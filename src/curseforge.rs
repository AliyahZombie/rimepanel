//! CurseForge modpack import (manifest.json format), following PCL's approach:
//! resolve file ids through the CF API (or the MCIM mirror when no key is set),
//! fall back to constructed forgecdn URLs when `downloadUrl` is null.
use crate::{instance::CoreSpec, library, net, state::AppState, tasks::TaskCtx, util};
use anyhow::{Context, Result};
use futures_util::{stream, StreamExt};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde_json::{json, Value};
use std::path::Path;

const OFFICIAL: &str = "https://api.curseforge.com";
const MCIM: &str = "https://mod.mcimirror.top/curseforge";

pub fn manifest_core(m: &Value) -> CoreSpec {
    let mc = m["minecraft"]["version"].as_str().unwrap_or_default().to_string();
    let loaders = m["minecraft"]["modLoaders"].as_array().cloned().unwrap_or_default();
    let primary = loaders
        .iter()
        .find(|l| l["primary"].as_bool() == Some(true))
        .or_else(|| loaders.first())
        .and_then(|l| l["id"].as_str())
        .unwrap_or("");
    let (kind, ver) = primary.split_once('-').unwrap_or(("vanilla", ""));
    match kind {
        "forge" => CoreSpec { kind: "forge".into(), mc_version: mc.clone(), loader_version: Some(format!("{mc}-{ver}")) },
        "neoforge" => CoreSpec { kind: "neoforge".into(), mc_version: mc, loader_version: Some(ver.into()) },
        "fabric" => CoreSpec { kind: "fabric".into(), mc_version: mc, loader_version: Some(ver.into()) },
        "quilt" => CoreSpec { kind: "quilt".into(), mc_version: mc, loader_version: Some(ver.into()) },
        _ => CoreSpec { kind: "vanilla".into(), mc_version: mc, loader_version: None },
    }
}

async fn cf_post(app: &AppState, path: &str, body: Value) -> Result<Value> {
    let key = app.setting("curseforge_key");
    let mut bases = vec![];
    if key.is_some() {
        bases.push(OFFICIAL);
    }
    bases.push(MCIM);
    let mut last = anyhow::anyhow!("CurseForge API 不可用");
    for base in bases {
        let mut req = app.http.post(format!("{base}{path}")).json(&body);
        if base == OFFICIAL {
            req = req.header("x-api-key", key.clone().unwrap_or_default());
        }
        match req.send().await.and_then(|r| r.error_for_status()) {
            Ok(r) => return Ok(r.json().await?),
            Err(e) => last = e.into(),
        }
    }
    Err(last)
}

fn cdn_urls(file_id: u64, name: &str) -> Vec<String> {
    let enc = utf8_percent_encode(name, NON_ALPHANUMERIC).to_string();
    let (a, b) = (file_id / 1000, file_id % 1000);
    vec![
        format!("https://edge.forgecdn.net/files/{a}/{b}/{enc}"),
        format!("https://mediafilez.forgecdn.net/files/{a}/{b}/{enc}"),
        format!("https://mod.mcimirror.top/files/{a}/{b}/{enc}"),
    ]
}

#[derive(serde::Serialize, Default)]
pub struct CfReport {
    pub installed: usize,
    pub skipped_client: Vec<String>,
    pub skipped_other: Vec<String>,
    pub failed: Vec<String>,
}

pub async fn apply(app: &AppState, dir: &Path, pack: &Path, ctx: &TaskCtx) -> Result<CfReport> {
    let manifest = crate::modrinth::read_zip_json(pack, "manifest.json")?;
    let entries: Vec<(u64, u64)> = manifest["files"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|f| f["required"].as_bool().unwrap_or(true))
        .filter_map(|f| Some((f["projectID"].as_u64()?, f["fileID"].as_u64()?)))
        .collect();
    let mut report = CfReport::default();
    ctx.progress(-1.0, format!("解析 {} 个 CurseForge 文件", entries.len()));
    let file_ids: Vec<u64> = entries.iter().map(|e| e.1).collect();
    let mod_ids: Vec<u64> = entries.iter().map(|e| e.0).collect();
    let files = cf_post(app, "/v1/mods/files", json!({ "fileIds": file_ids }))
        .await
        .context("获取 CurseForge 文件信息失败（可在设置中填写 API Key）")?;
    let mods = cf_post(app, "/v1/mods", json!({ "modIds": mod_ids, "filterPcOnly": false }))
        .await
        .unwrap_or(json!({"data": []}));
    let mut class_of = std::collections::HashMap::new();
    for m in mods["data"].as_array().into_iter().flatten() {
        if let (Some(id), Some(c)) = (m["id"].as_u64(), m["classId"].as_u64()) {
            class_of.insert(id, (c, m["name"].as_str().unwrap_or("").to_string()));
        }
    }
    let mut jobs = Vec::new();
    for f in files["data"].as_array().into_iter().flatten() {
        let fid = f["id"].as_u64().unwrap_or(0);
        let mid = f["modId"].as_u64().unwrap_or(0);
        let name = f["fileName"].as_str().unwrap_or("").to_string();
        let gv: Vec<&str> = f["gameVersions"].as_array().into_iter().flatten().filter_map(|x| x.as_str()).collect();
        if gv.contains(&"Client") && !gv.contains(&"Server") {
            report.skipped_client.push(name);
            continue;
        }
        let class = class_of.get(&mid).map(|c| c.0).unwrap_or(6);
        let folder = match class {
            6 => "mods",
            6945 => "datapacks",
            _ => {
                // resource packs / shaders / worlds are client-side content
                report.skipped_other.push(name);
                continue;
            }
        };
        let sha1 = f["hashes"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|h| h["algo"].as_u64() == Some(1))
            .and_then(|h| h["value"].as_str())
            .map(String::from);
        let mut urls = vec![];
        if let Some(u) = f["downloadUrl"].as_str() {
            urls.push(u.to_string());
        }
        urls.extend(cdn_urls(fid, &name));
        jobs.push((name, folder, sha1, urls, mid, fid));
    }
    let total = jobs.len();
    let done = std::sync::atomic::AtomicUsize::new(0);
    let futs: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = (String, Result<()>)> + Send + '_>>> = jobs
        .into_iter()
        .map(|(name, folder, sha1, urls, mid, fid)| {
            let done = &done;
            Box::pin(async move {
                let r = async {
                    let sha = net::fetch_to_library(app, net::Download {
                        urls,
                        sha1,
                        sha256: None,
                        size: None,
                        kind: if folder == "mods" { "mod" } else { "datapack" },
                        name: name.clone(),
                        source: "curseforge",
                        meta: json!({"cf_mod_id": mid, "cf_file_id": fid}),
                    }, None).await?;
                    let target = if folder == "datapacks" {
                        dir.join(crate::props::level_name(dir)).join("datapacks").join(&name)
                    } else {
                        dir.join(folder).join(&name)
                    };
                    library::link(app, &sha, &target)?;
                    Ok(())
                }.await;
                let n = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                ctx.progress(n as f32 / total.max(1) as f32, format!("模组 {n}/{total}  {name}"));
                (name, r)
            }) as _
        })
        .collect();
    let results: Vec<(String, Result<()>)> = stream::iter(futs)
        .buffer_unordered(8)
        .collect()
        .await;
    for (name, r) in results {
        match r {
            Ok(_) => report.installed += 1,
            Err(e) => report.failed.push(format!("{name}: {e:#}")),
        }
    }
    let overrides = manifest["overrides"].as_str().unwrap_or("overrides").to_string();
    let (p, d) = (pack.to_path_buf(), dir.to_path_buf());
    tokio::task::spawn_blocking(move || util::unzip_to(&p, &d, Some(&overrides))).await??;
    Ok(report)
}
