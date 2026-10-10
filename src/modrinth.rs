//! Modrinth API v2: search, versions, installs and .mrpack modpacks.
use crate::{instance::CoreSpec, library, net, state::AppState, tasks::TaskCtx, util};
use anyhow::{bail, Context, Result};
use futures_util::{stream, StreamExt};
use serde_json::{json, Value};
use std::path::Path;

pub const API: &str = "https://api.modrinth.com/v2";

pub fn loaders_for(kind: &str) -> Vec<&'static str> {
    match kind {
        "fabric" => vec!["fabric"],
        "quilt" => vec!["quilt", "fabric"],
        "forge" => vec!["forge"],
        "neoforge" => vec!["neoforge"],
        "paper" => vec!["paper", "spigot", "bukkit"],
        "purpur" => vec!["purpur", "paper", "spigot", "bukkit"],
        _ => vec![],
    }
}

pub fn is_plugin_core(kind: &str) -> bool {
    matches!(kind, "paper" | "purpur")
}

pub fn mods_folder(kind: &str) -> &'static str {
    if is_plugin_core(kind) { "plugins" } else { "mods" }
}

pub async fn search(
    app: &AppState,
    q: &str,
    project_type: &str,
    mc: Option<&str>,
    loader: Option<&str>,
    offset: u32,
) -> Result<Value> {
    let mut facets: Vec<Vec<String>> = vec![vec![format!("project_type:{project_type}")]];
    if let Some(v) = mc.filter(|s| !s.is_empty()) {
        facets.push(vec![format!("versions:{v}")]);
    }
    if let Some(l) = loader.filter(|s| !s.is_empty()) {
        let ls = loaders_for(l);
        if !ls.is_empty() && project_type != "resourcepack" && project_type != "datapack" {
            facets.push(ls.iter().map(|x| format!("categories:{x}")).collect());
        }
    }
    if project_type == "mod" || project_type == "modpack" {
        // hide client-only projects when browsing for a server
        facets.push(vec!["server_side:required".into(), "server_side:optional".into()]);
    }
    let url = reqwest::Url::parse_with_params(
        &format!("{API}/search"),
        &[
            ("query", q.to_string()),
            ("facets", serde_json::to_string(&facets)?),
            ("limit", "20".into()),
            ("offset", offset.to_string()),
            ("index", if q.is_empty() { "downloads".into() } else { "relevance".into() }),
        ],
    )?;
    net::get_json(app, url.as_str()).await
}

pub async fn project_versions(
    app: &AppState,
    id: &str,
    mc: Option<&str>,
    loader: Option<&str>,
) -> Result<Value> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(l) = loader.filter(|s| !s.is_empty()) {
        let ls = loaders_for(l);
        if !ls.is_empty() {
            params.push(("loaders", serde_json::to_string(&ls)?));
        }
    }
    if let Some(v) = mc.filter(|s| !s.is_empty()) {
        params.push(("game_versions", serde_json::to_string(&[v])?));
    }
    let url = reqwest::Url::parse_with_params(&format!("{API}/project/{id}/version"), &params)?;
    net::get_json(app, url.as_str()).await
}

pub async fn version(app: &AppState, id: &str) -> Result<Value> {
    net::get_json(app, &format!("{API}/version/{id}")).await
}

pub async fn project(app: &AppState, id: &str) -> Result<Value> {
    net::get_json(app, &format!("{API}/project/{id}")).await
}

fn primary_file(v: &Value) -> Result<&Value> {
    let files = v["files"].as_array().context("版本没有文件")?;
    files
        .iter()
        .find(|f| f["primary"].as_bool() == Some(true))
        .or_else(|| files.first())
        .context("版本没有文件")
}

/// Download a version's primary file into the library. Returns (sha1, filename).
pub async fn fetch_version(
    app: &AppState,
    v: &Value,
    kind: &str,
    ctx: Option<&TaskCtx>,
) -> Result<(String, String)> {
    let f = primary_file(v)?;
    let name = f["filename"].as_str().unwrap_or("file").to_string();
    let meta = json!({
        "project_id": v["project_id"], "version_id": v["id"],
        "version": v["version_number"], "title": v["name"],
    });
    let sha = net::fetch_to_library(
        app,
        net::Download {
            urls: vec![f["url"].as_str().unwrap_or_default().to_string()],
            sha1: f["hashes"]["sha1"].as_str().map(String::from),
            sha256: None,
            size: f["size"].as_u64(),
            kind,
            name: name.clone(),
            source: "modrinth",
            meta,
        },
        ctx,
    )
    .await?;
    Ok((sha, name))
}

/// Install a version (and its required dependencies) into an instance.
pub async fn install_into(
    app: &AppState,
    dir: &Path,
    core: &CoreSpec,
    version_id: &str,
    with_deps: bool,
    ctx: &TaskCtx,
) -> Result<Vec<String>> {
    let mut queue = vec![version_id.to_string()];
    let mut seen_projects = std::collections::HashSet::new();
    let mut installed = Vec::new();
    while let Some(vid) = queue.pop() {
        ctx.check()?;
        let v = version(app, &vid).await?;
        let pid = v["project_id"].as_str().unwrap_or_default().to_string();
        if !seen_projects.insert(pid.clone()) {
            continue;
        }
        let proj = project(app, &pid).await.unwrap_or(Value::Null);
        let ptype = proj["project_type"].as_str().unwrap_or("mod");
        let loaders: Vec<&str> =
            v["loaders"].as_array().into_iter().flatten().filter_map(|x| x.as_str()).collect();
        let is_plugin = loaders.iter().any(|l| ["bukkit", "spigot", "paper", "purpur", "folia"].contains(l));
        let (kind, folder) = match ptype {
            "resourcepack" => ("resourcepack", "resourcepacks".to_string()),
            "shader" => bail!("光影是客户端资源，服务器不需要"),
            "modpack" => bail!("整合包请通过「拼装 / 新建实例」导入"),
            _ if loaders.contains(&"datapack") => {
                ("datapack", format!("{}/datapacks", crate::props::level_name(dir)))
            }
            _ if is_plugin || is_plugin_core(&core.kind) => ("plugin", "plugins".into()),
            _ => ("mod", "mods".into()),
        };
        ctx.message(format!("下载 {}", proj["title"].as_str().unwrap_or(&pid)));
        let (sha, name) = fetch_version(app, &v, kind, Some(ctx)).await?;
        let mut meta_upd = app.db.lib_get(&sha).map(|r| r.meta).unwrap_or_default();
        meta_upd["title"] = proj["title"].clone();
        meta_upd["icon_url"] = proj["icon_url"].clone();
        meta_upd["server_side"] = proj["server_side"].clone();
        meta_upd["client_side"] = proj["client_side"].clone();
        library::register(app, &sha, kind, &name, "modrinth", meta_upd)?;
        if kind == "resourcepack" {
            installed.push(format!("resourcepack:{sha}"));
            continue;
        }
        library::link(app, &sha, &dir.join(&folder).join(&name))?;
        installed.push(name);
        if with_deps {
            for d in v["dependencies"].as_array().into_iter().flatten() {
                if d["dependency_type"].as_str() != Some("required") {
                    continue;
                }
                if let Some(dv) = d["version_id"].as_str() {
                    queue.push(dv.to_string());
                } else if let Some(dp) = d["project_id"].as_str() {
                    if seen_projects.contains(dp) {
                        continue;
                    }
                    let vs = project_versions(app, dp, Some(&core.mc_version), Some(&core.kind)).await?;
                    if let Some(first) = vs.as_array().and_then(|a| a.first()) {
                        queue.push(first["id"].as_str().unwrap_or_default().to_string());
                    }
                }
            }
        }
    }
    Ok(installed)
}

/// Read core info from a .mrpack's modrinth.index.json.
pub fn mrpack_core(index: &Value) -> CoreSpec {
    let deps = &index["dependencies"];
    let mc = deps["minecraft"].as_str().unwrap_or_default().to_string();
    let pick = |k: &str| deps[k].as_str().map(String::from);
    if let Some(v) = pick("neoforge") {
        CoreSpec { kind: "neoforge".into(), mc_version: mc, loader_version: Some(v) }
    } else if let Some(v) = pick("forge") {
        let full = format!("{}-{}", mc, v);
        CoreSpec { kind: "forge".into(), mc_version: mc, loader_version: Some(full) }
    } else if let Some(v) = pick("quilt-loader") {
        CoreSpec { kind: "quilt".into(), mc_version: mc, loader_version: Some(v) }
    } else if let Some(v) = pick("fabric-loader") {
        CoreSpec { kind: "fabric".into(), mc_version: mc, loader_version: Some(v) }
    } else {
        CoreSpec { kind: "vanilla".into(), mc_version: mc, loader_version: None }
    }
}

pub fn read_zip_json(path: &Path, entry: &str) -> Result<Value> {
    let f = std::fs::File::open(path)?;
    let mut z = zip::ZipArchive::new(f)?;
    let e = z.by_name(entry).with_context(|| format!("整合包中缺少 {entry}"))?;
    Ok(serde_json::from_reader(e)?)
}

/// Apply a .mrpack: download server-side files (via library + hard links) and overrides.
pub async fn apply_mrpack(app: &AppState, dir: &Path, pack: &Path, ctx: &TaskCtx) -> Result<usize> {
    let index = read_zip_json(pack, "modrinth.index.json")?;
    let files: Vec<Value> = index["files"].as_array().cloned().unwrap_or_default();
    let wanted: Vec<Value> = files
        .into_iter()
        .filter(|f| f["env"]["server"].as_str() != Some("unsupported"))
        .collect();
    let total = wanted.len();
    let done = std::sync::atomic::AtomicUsize::new(0);
    let futs: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + '_>>> = wanted
        .into_iter()
        .map(|f| {
            let done = &done;
            Box::pin(async move {
                let rel = util::clean_rel(f["path"].as_str().unwrap_or_default())?;
                let name = rel.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                let top = rel.components().next().map(|c| c.as_os_str().to_string_lossy().to_string()).unwrap_or_default();
                let kind = match top.as_str() {
                    "mods" => "mod",
                    "resourcepacks" => "resourcepack",
                    "datapacks" => "datapack",
                    "plugins" => "plugin",
                    _ => "other",
                };
                let urls = f["downloads"].as_array().into_iter().flatten()
                    .filter_map(|u| u.as_str().map(String::from)).collect();
                let sha = net::fetch_to_library(app, net::Download {
                    urls,
                    sha1: f["hashes"]["sha1"].as_str().map(String::from),
                    sha256: None,
                    size: f["fileSize"].as_u64(),
                    kind,
                    name: name.clone(),
                    source: "modrinth",
                    meta: json!({}),
                }, None).await?;
                library::link(app, &sha, &dir.join(&rel))?;
                let n = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                ctx.progress(n as f32 / total.max(1) as f32, format!("整合包文件 {n}/{total}  {name}"));
                Ok(())
            }) as _
        })
        .collect();
    let results: Vec<Result<()>> = stream::iter(futs)
        .buffer_unordered(8)
        .collect()
        .await;
    for r in results {
        r?;
    }
    ctx.progress(-1.0, "解压覆盖文件");
    let (p, d) = (pack.to_path_buf(), dir.to_path_buf());
    tokio::task::spawn_blocking(move || -> Result<()> {
        util::unzip_to(&p, &d, Some("overrides"))?;
        util::unzip_to(&p, &d, Some("server-overrides"))?;
        Ok(())
    })
    .await??;
    Ok(total)
}

/// Batch lookup of projects by file sha1: sha1 -> project json.
pub async fn lookup_hashes(app: &AppState, hashes: &[String]) -> Result<std::collections::HashMap<String, Value>> {
    let mut out = std::collections::HashMap::new();
    if hashes.is_empty() {
        return Ok(out);
    }
    let resp: Value = app
        .http
        .post(format!("{API}/version_files"))
        .json(&json!({"hashes": hashes, "algorithm": "sha1"}))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let mut pids = Vec::new();
    let mut sha_to_pid = Vec::new();
    if let Some(obj) = resp.as_object() {
        for (sha, v) in obj {
            if let Some(p) = v["project_id"].as_str() {
                pids.push(p.to_string());
                sha_to_pid.push((sha.clone(), p.to_string()));
            }
        }
    }
    if pids.is_empty() {
        return Ok(out);
    }
    pids.sort();
    pids.dedup();
    let mut projects = std::collections::HashMap::new();
    for chunk in pids.chunks(200) {
        let url = reqwest::Url::parse_with_params(
            &format!("{API}/projects"),
            &[("ids", serde_json::to_string(chunk)?)],
        )?;
        let v = net::get_json(app, url.as_str()).await?;
        for p in v.as_array().into_iter().flatten() {
            if let Some(id) = p["id"].as_str() {
                projects.insert(id.to_string(), p.clone());
            }
        }
    }
    for (sha, pid) in sha_to_pid {
        if let Some(p) = projects.get(&pid) {
            out.insert(sha, p.clone());
        }
    }
    Ok(out)
}
