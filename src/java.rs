//! Managed Java runtimes (Eclipse Temurin via the Adoptium API).
use crate::{net, state::AppState, tasks::TaskCtx};
use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Serialize, Clone)]
pub struct Runtime {
    pub major: u32,
    pub path: String,
    pub version: String,
    pub managed: bool,
    /// managed | system | custom
    pub source: String,
}

fn arch() -> &'static str {
    match std::env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "aarch64",
        "arm" => "arm",
        "riscv64" => "riscv64",
        "powerpc64" => "ppc64le",
        "s390x" => "s390x",
        _ => "x64",
    }
}

pub fn managed_bin(app: &AppState, major: u32) -> PathBuf {
    app.paths.runtimes().join(format!("java-{major}")).join("bin/java")
}

/// "1.8.0_402" -> 8, "21.0.4" -> 21, "25" -> 25
pub fn parse_major(v: &str) -> Option<u32> {
    let mut it = v.trim_matches('"').split(|c: char| c == '.' || c == '_' || c == '+' || c == '-');
    let a: u32 = it.next()?.parse().ok()?;
    if a == 1 {
        it.next()?.parse().ok()
    } else {
        Some(a)
    }
}

fn release_version(home: &Path) -> Option<String> {
    let s = std::fs::read_to_string(home.join("release")).ok()?;
    s.lines()
        .find(|l| l.starts_with("JAVA_VERSION="))
        .map(|l| l[13..].trim_matches('"').to_string())
}

/// Probe a java binary: (version string). Reads the `release` file when
/// present, otherwise runs `java -version`.
pub fn probe(bin: &Path) -> Option<String> {
    let real = bin.canonicalize().ok()?;
    if let Some(home) = real.parent().and_then(|p| p.parent()) {
        if let Some(v) = release_version(home) {
            return Some(v);
        }
    }
    let out = std::process::Command::new(&real).arg("-version").output().ok()?;
    let text = String::from_utf8_lossy(&out.stderr).to_string() + &String::from_utf8_lossy(&out.stdout);
    let line = text.lines().find(|l| l.contains("version"))?;
    let v = line.split('"').nth(1)?.to_string();
    Some(v)
}

fn system_candidates(app: &AppState) -> Vec<(PathBuf, &'static str)> {
    let mut v: Vec<(PathBuf, &'static str)> = Vec::new();
    if let Ok(h) = std::env::var("JAVA_HOME") {
        v.push((PathBuf::from(h).join("bin/java"), "system"));
    }
    for base in ["/usr/lib/jvm", "/usr/java", "/opt/java", "/usr/local/java"] {
        if let Ok(rd) = std::fs::read_dir(base) {
            for e in rd.flatten() {
                v.push((e.path().join("bin/java"), "system"));
            }
        }
    }
    if let Some(path) = std::env::var_os("PATH") {
        for d in std::env::split_paths(&path) {
            v.push((d.join("java"), "system"));
        }
    }
    for p in custom_paths(app) {
        v.push((PathBuf::from(p), "custom"));
    }
    v
}

pub fn custom_paths(app: &AppState) -> Vec<String> {
    app.setting("java_custom")
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn set_custom_paths(app: &AppState, v: &[String]) -> Result<()> {
    app.db.set_setting("java_custom", &serde_json::to_string(v)?)
}

/// All runtimes: managed ones plus detected system / user-added javas.
pub fn list(app: &AppState) -> Vec<Runtime> {
    let mut v = Vec::new();
    let mut seen = std::collections::HashSet::new();
    if let Ok(rd) = std::fs::read_dir(app.paths.runtimes()) {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if let Some(m) = name.strip_prefix("java-").and_then(|m| m.parse::<u32>().ok()) {
                let bin = e.path().join("bin/java");
                if bin.exists() {
                    seen.insert(bin.canonicalize().unwrap_or(bin.clone()));
                    let version = release_version(&e.path()).unwrap_or_default();
                    v.push(Runtime { major: m, path: bin.to_string_lossy().into(), version, managed: true, source: "managed".into() });
                }
            }
        }
    }
    for (bin, src) in system_candidates(app) {
        let Ok(real) = bin.canonicalize() else { continue };
        if !real.is_file() || !seen.insert(real.clone()) {
            continue;
        }
        if let Some(ver) = probe(&real) {
            if let Some(m) = parse_major(&ver) {
                v.push(Runtime { major: m, path: real.to_string_lossy().into(), version: ver, managed: false, source: src.into() });
            }
        }
    }
    v.sort_by(|a, b| b.major.cmp(&a.major).then(b.managed.cmp(&a.managed)));
    v
}

/// Resolve the java binary for an instance setting: "auto" picks a runtime
/// whose major version matches exactly (managed first, then system/custom);
/// anything else is used as a path.
pub fn resolve(app: &AppState, setting: &str, major: u32) -> Result<String> {
    if !setting.is_empty() && setting != "auto" {
        if !Path::new(setting).exists() {
            bail!("Java 路径不存在: {setting}");
        }
        return Ok(setting.to_string());
    }
    let p = managed_bin(app, major);
    if p.exists() {
        return Ok(p.to_string_lossy().into());
    }
    if let Some(r) = list(app).into_iter().find(|r| r.major == major) {
        return Ok(r.path);
    }
    bail!("JAVA_MISSING:{major}")
}

static INSTALL_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

/// Ensure a managed runtime exists; downloads it when missing.
pub async fn ensure(app: &AppState, major: u32, ctx: &TaskCtx) -> Result<String> {
    if let Ok(p) = resolve(app, "auto", major) {
        return Ok(p);
    }
    install(app, major, ctx).await
}

pub async fn install(app: &AppState, major: u32, ctx: &TaskCtx) -> Result<String> {
    ctx.progress(-1.0, format!("等待其他 Java 安装完成"));
    let _g = INSTALL_LOCK.lock().await;
    let existing = managed_bin(app, major);
    if existing.exists() {
        return Ok(existing.to_string_lossy().into());
    }
    ctx.progress(-1.0, format!("获取 Java {major} 信息"));
    let url = format!(
        "https://api.adoptium.net/v3/assets/latest/{major}/hotspot?os=linux&architecture={}&image_type=jre&vendor=eclipse",
        arch()
    );
    let v = net::get_json(app, &url).await?;
    let mut pkg = v.as_array().and_then(|a| a.first()).map(|x| x["binary"]["package"].clone());
    if pkg.as_ref().map(|p| p["link"].is_null()).unwrap_or(true) {
        // some versions ship no JRE build: fall back to JDK
        let url = url.replace("image_type=jre", "image_type=jdk");
        let v = net::get_json(app, &url).await?;
        pkg = v.as_array().and_then(|a| a.first()).map(|x| x["binary"]["package"].clone());
    }
    let pkg = pkg.context("Adoptium 未提供该版本")?;
    let link = pkg["link"].as_str().context("无下载地址")?.to_string();
    let checksum = pkg["checksum"].as_str().map(|s| s.to_string());
    let tmp = app.paths.tmp().join(format!("jre-{major}-{}.tar.gz", crate::util::new_id()));
    net::fetch_to_file(app, &link, &tmp, Some(ctx)).await?;
    if let Some(c) = checksum {
        let t = tmp.clone();
        let got = tokio::task::spawn_blocking(move || -> Result<String> {
            use sha2::Digest;
            let mut f = std::fs::File::open(&t)?;
            let mut h = sha2::Sha256::new();
            std::io::copy(&mut f, &mut h)?;
            Ok(hex::encode(h.finalize()))
        })
        .await??;
        if !got.eq_ignore_ascii_case(&c) {
            let _ = std::fs::remove_file(&tmp);
            bail!("Java 包校验失败");
        }
    }
    ctx.progress(-1.0, "解压 Java");
    let dest = app.paths.runtimes().join(format!("java-{major}"));
    let staging = app.paths.runtimes().join(format!(".java-{major}-staging"));
    let t2 = tmp.clone();
    let st2 = staging.clone();
    tokio::task::spawn_blocking(move || -> Result<()> {
        let _ = std::fs::remove_dir_all(&st2);
        crate::util::untar_to(&t2, &st2)?;
        Ok(())
    })
    .await??;
    let _ = std::fs::remove_file(&tmp);
    // archive has a single top-level dir (e.g. jdk-21.0.4+7-jre)
    let inner = std::fs::read_dir(&staging)?
        .flatten()
        .find(|e| e.path().is_dir())
        .map(|e| e.path())
        .context("解压结果异常")?;
    let _ = std::fs::remove_dir_all(&dest);
    std::fs::rename(&inner, &dest)?;
    let _ = std::fs::remove_dir_all(&staging);
    let bin = managed_bin(app, major);
    if !bin.exists() {
        bail!("安装后未找到 java 可执行文件");
    }
    Ok(bin.to_string_lossy().into())
}

/// Java major version required by a Minecraft version (from its version json).
pub fn default_major_for(mc: &str) -> u32 {
    // fallback heuristic when the version json is unavailable
    let parts: Vec<u32> = mc.split('.').filter_map(|p| p.parse().ok()).collect();
    match parts.as_slice() {
        [1, minor, rest @ ..] => {
            let patch = rest.first().copied().unwrap_or(0);
            if *minor >= 21 || (*minor == 20 && patch >= 5) {
                21
            } else if *minor >= 18 {
                17
            } else if *minor == 17 {
                16
            } else {
                8
            }
        }
        // date-based versions (26.1+) require Java 25
        [y, ..] if *y >= 26 => 25,
        _ => 25,
    }
}

/// Remove a managed runtime.
pub fn uninstall(app: &AppState, major: u32) -> Result<()> {
    let dir = app.paths.runtimes().join(format!("java-{major}"));
    if !dir.exists() {
        bail!("Java {major} 不是面板托管的运行时");
    }
    std::fs::remove_dir_all(dir)?;
    Ok(())
}

/// Feature releases offered by Adoptium (cached for a day).
pub async fn available(app: &AppState) -> Result<serde_json::Value> {
    net::get_json_cached(app, "https://api.adoptium.net/v3/info/available_releases", 86400).await
}
