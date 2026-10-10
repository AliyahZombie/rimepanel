//! HTTP helpers: JSON fetch, mirror rewriting, verified downloads into the library.
use crate::{library, state::AppState, tasks::TaskCtx};
use anyhow::{anyhow, bail, Context, Result};
use futures_util::StreamExt;
use sha1::{Digest, Sha1};
use std::time::Duration;
use tokio::io::AsyncWriteExt;

const BMCL: &str = "https://bmclapi2.bangbang93.com";

/// Rewrite an official URL to the BMCLAPI mirror (when enabled).
pub fn mirror_url(app: &AppState, url: &str) -> Option<String> {
    if app.setting("mirror").as_deref() != Some("bmclapi") {
        return None;
    }
    let rules: [(&str, String); 9] = [
        ("https://piston-meta.mojang.com", BMCL.to_string()),
        ("https://launchermeta.mojang.com", BMCL.to_string()),
        ("https://piston-data.mojang.com", BMCL.to_string()),
        ("https://launcher.mojang.com", BMCL.to_string()),
        ("https://maven.minecraftforge.net", format!("{BMCL}/maven")),
        ("https://files.minecraftforge.net/maven", format!("{BMCL}/maven")),
        ("https://maven.fabricmc.net", format!("{BMCL}/maven")),
        ("https://meta.fabricmc.net", format!("{BMCL}/fabric-meta")),
        ("https://maven.neoforged.net/releases", format!("{BMCL}/maven")),
    ];
    for (from, to) in rules.iter() {
        if let Some(rest) = url.strip_prefix(from) {
            return Some(format!("{to}{rest}"));
        }
    }
    None
}

pub fn candidates(app: &AppState, url: &str) -> Vec<String> {
    let mut v = Vec::new();
    if let Some(m) = mirror_url(app, url) {
        v.push(m);
    }
    v.push(url.to_string());
    v
}

pub async fn get_json(app: &AppState, url: &str) -> Result<serde_json::Value> {
    let mut last = anyhow!("no url");
    for u in candidates(app, url) {
        for attempt in 0..3 {
            match app
                .http
                .get(&u)
                .timeout(Duration::from_secs(30))
                .send()
                .await
                .and_then(|r| r.error_for_status())
            {
                Ok(r) => match r.json::<serde_json::Value>().await {
                    Ok(v) => return Ok(v),
                    Err(e) => last = e.into(),
                },
                Err(e) => last = e.into(),
            }
            tokio::time::sleep(Duration::from_millis(400 * (attempt + 1))).await;
        }
    }
    Err(last.context(format!("请求失败: {url}")))
}

/// Cached JSON (for version lists) — refreshed after `ttl` seconds.
pub async fn get_json_cached(app: &AppState, url: &str, ttl: i64) -> Result<serde_json::Value> {
    let key = hex::encode(Sha1::digest(url.as_bytes()));
    let path = app.paths.cache().join(format!("{key}.json"));
    if let Ok(m) = std::fs::metadata(&path) {
        if crate::util::now() - crate::util::mtime(&m) < ttl {
            if let Ok(s) = std::fs::read(&path) {
                if let Ok(v) = serde_json::from_slice(&s) {
                    return Ok(v);
                }
            }
        }
    }
    match get_json(app, url).await {
        Ok(v) => {
            let _ = crate::util::atomic_write(&path, v.to_string().as_bytes());
            Ok(v)
        }
        Err(e) => {
            // stale cache beats nothing
            if let Ok(s) = std::fs::read(&path) {
                if let Ok(v) = serde_json::from_slice(&s) {
                    return Ok(v);
                }
            }
            Err(e)
        }
    }
}

pub struct Download<'a> {
    pub urls: Vec<String>,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
    pub size: Option<u64>,
    pub kind: &'a str,
    pub name: String,
    pub source: &'a str,
    pub meta: serde_json::Value,
}

/// Download into the content-addressed library (deduplicated by sha1).
/// Returns the sha1. Progress is reported to `ctx` when given.
pub async fn fetch_to_library(
    app: &AppState,
    d: Download<'_>,
    ctx: Option<&TaskCtx>,
) -> Result<String> {
    if let Some(s) = &d.sha1 {
        let s = s.to_lowercase();
        if library::object_path(app, &s).exists() {
            library::register(app, &s, d.kind, &d.name, d.source, d.meta.clone())?;
            return Ok(s);
        }
    }
    let _permit = app.tasks.downloads.acquire().await?;
    let mut urls = Vec::new();
    for u in &d.urls {
        urls.extend(candidates(app, u));
    }
    let mut last = anyhow!("没有可用的下载地址");
    for u in &urls {
        for attempt in 0..3u64 {
            if let Some(c) = ctx {
                c.check()?;
            }
            match download_once(app, u, &d, ctx).await {
                Ok(sha) => {
                    return Ok(sha);
                }
                Err(e) => {
                    tracing::debug!("download {u} failed: {e:#}");
                    last = e;
                    if last.to_string().contains("校验失败") {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(500 * (attempt + 1))).await;
                }
            }
        }
    }
    Err(last.context(format!("下载失败: {}", d.name)))
}

async fn download_once(
    app: &AppState,
    url: &str,
    d: &Download<'_>,
    ctx: Option<&TaskCtx>,
) -> Result<String> {
    let resp = app
        .http
        .get(url)
        .timeout(Duration::from_secs(600))
        .send()
        .await?
        .error_for_status()?;
    let total = resp.content_length().or(d.size);
    let tmp = app.paths.tmp().join(crate::util::new_id());
    let mut f = tokio::fs::File::create(&tmp).await?;
    let mut h1 = Sha1::new();
    let mut h256 = sha2::Sha256::new();
    let mut done: u64 = 0;
    let mut stream = resp.bytes_stream();
    let mut last_report = std::time::Instant::now();
    let res: Result<()> = async {
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            h1.update(&chunk);
            if d.sha256.is_some() {
                sha2::Digest::update(&mut h256, &chunk);
            }
            f.write_all(&chunk).await?;
            done += chunk.len() as u64;
            if let Some(c) = ctx {
                if last_report.elapsed() > Duration::from_millis(300) {
                    last_report = std::time::Instant::now();
                    let p = total.map(|t| done as f32 / t.max(1) as f32).unwrap_or(-1.0);
                    c.progress(
                        p,
                        format!("{} {}", d.name, crate::util::human_size(done)),
                    );
                }
            }
        }
        f.flush().await?;
        Ok(())
    }
    .await;
    drop(f);
    if let Err(e) = res {
        let _ = tokio::fs::remove_file(&tmp).await;
        return Err(e);
    }
    let sha = hex::encode(h1.finalize());
    if let Some(exp) = &d.sha1 {
        if !exp.eq_ignore_ascii_case(&sha) {
            let _ = tokio::fs::remove_file(&tmp).await;
            bail!("sha1 校验失败 ({})", d.name);
        }
    }
    if let Some(exp) = &d.sha256 {
        let got = hex::encode(sha2::Digest::finalize(h256));
        if !exp.eq_ignore_ascii_case(&got) {
            let _ = tokio::fs::remove_file(&tmp).await;
            bail!("sha256 校验失败 ({})", d.name);
        }
    }
    library::import_tmp(app, &tmp, &sha, d.kind, &d.name, d.source, d.meta.clone())
        .context("写入资源库失败")?;
    Ok(sha)
}

/// Download a URL to a plain file (no library), e.g. JDK archives.
pub async fn fetch_to_file(
    app: &AppState,
    url: &str,
    dest: &std::path::Path,
    ctx: Option<&TaskCtx>,
) -> Result<()> {
    let resp = app.http.get(url).send().await?.error_for_status()?;
    let total = resp.content_length();
    let mut f = tokio::fs::File::create(dest).await?;
    let mut stream = resp.bytes_stream();
    let mut done = 0u64;
    let mut last = std::time::Instant::now();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        f.write_all(&chunk).await?;
        done += chunk.len() as u64;
        if let Some(c) = ctx {
            c.check()?;
            if last.elapsed() > Duration::from_millis(300) {
                last = std::time::Instant::now();
                c.progress(
                    total.map(|t| done as f32 / t.max(1) as f32).unwrap_or(-1.0),
                    format!("下载中 {}", crate::util::human_size(done)),
                );
            }
        }
    }
    f.flush().await?;
    Ok(())
}
