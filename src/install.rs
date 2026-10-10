//! Server core installation and "assembly" (拼装服务器): core + modpack +
//! mods + resource pack + datapacks + world template -> a ready instance.
use crate::{
    clientmods, curseforge,
    instance::{self, CoreSpec, Instance, LaunchSpec},
    java, library, modrinth, net, props,
    state::AppState,
    tasks::TaskCtx,
    util, versions,
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::Path, process::Stdio, sync::Arc};
use tokio::io::{AsyncBufReadExt, BufReader};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "source", rename_all = "lowercase")]
pub enum PackRef {
    Modrinth { version_id: String },
    Library { sha1: String },
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "source", rename_all = "lowercase")]
pub enum ContentRef {
    Modrinth { version_id: String },
    Library { sha1: String },
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct AssembleReq {
    pub name: String,
    #[serde(default)]
    pub core: Option<CoreSpec>,
    #[serde(default)]
    pub eula: bool,
    #[serde(default)]
    pub max_mem: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub modpack: Option<PackRef>,
    #[serde(default)]
    pub mods: Vec<ContentRef>,
    #[serde(default)]
    pub resourcepack: Option<ContentRef>,
    #[serde(default)]
    pub datapacks: Vec<ContentRef>,
    #[serde(default)]
    pub world: Option<String>,
    #[serde(default)]
    pub properties: BTreeMap<String, String>,
    #[serde(default = "yes")]
    pub strip_client: bool,
    #[serde(default)]
    pub save_recipe: Option<String>,
}
fn yes() -> bool { true }

/// Library-backed server jar (the "server jar archive"): reuse when present.
async fn server_jar(
    app: &AppState,
    name: &str,
    url: &str,
    sha1: Option<String>,
    sha256: Option<String>,
    kind: &str,
    ctx: &TaskCtx,
) -> Result<String> {
    if let Some(r) = app.db.lib_find_server(name) {
        if library::object_path(app, &r.sha1).exists() {
            ctx.message(format!("复用存档的核心 {name}"));
            return Ok(r.sha1);
        }
    }
    ctx.message(format!("下载 {name}"));
    net::fetch_to_library(
        app,
        net::Download {
            urls: vec![url.to_string()],
            sha1,
            sha256,
            size: None,
            kind,
            name: name.into(),
            source: "official",
            meta: json!({}),
        },
        Some(ctx),
    )
    .await
}

async fn vanilla_server(app: &AppState, mc: &str, ctx: &TaskCtx) -> Result<String> {
    let vj = versions::vanilla_json(app, mc).await?;
    let s = &vj["downloads"]["server"];
    let url = s["url"].as_str().with_context(|| format!("{mc} 没有官方服务端"))?;
    server_jar(app, &format!("vanilla-{mc}.jar"), url, s["sha1"].as_str().map(String::from), None, "server", ctx).await
}

async fn java_major_for(app: &AppState, mc: &str) -> u32 {
    match versions::vanilla_json(app, mc).await {
        Ok(v) => v["javaVersion"]["majorVersion"].as_u64().map(|x| x as u32).unwrap_or_else(|| java::default_major_for(mc)),
        Err(_) => java::default_major_for(mc),
    }
}

/// Run an installer jar inside the instance dir, streaming its output.
async fn run_installer(java: &str, dir: &Path, args: &[&str], inst: &Instance, ctx: &TaskCtx) -> Result<()> {
    let mut child = tokio::process::Command::new(java)
        .args(args)
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .context("无法运行安装器")?;
    let out = child.stdout.take().unwrap();
    let err = child.stderr.take().unwrap();
    let c2 = ctx.clone();
    let mut lines = BufReader::new(out).lines();
    let mut elines = BufReader::new(err).lines();
    loop {
        tokio::select! {
            l = lines.next_line() => match l? {
                Some(l) => { c2.message(l.chars().take(160).collect::<String>()); inst.push_log(format!("[安装器] {l}")); }
                None => break,
            },
            l = elines.next_line() => if let Ok(Some(l)) = l { inst.push_log(format!("[安装器] {l}")); },
            _ = ctx.cancel.cancelled() => { let _ = child.kill().await; bail!("已取消"); }
        }
    }
    let st = child.wait().await?;
    if !st.success() {
        bail!("安装器退出码 {st}，详见控制台日志");
    }
    Ok(())
}

fn find_args_file(dir: &Path) -> Option<String> {
    for base in ["libraries/net/minecraftforge/forge", "libraries/net/neoforged/neoforge", "libraries/net/neoforged/forge"] {
        let p = dir.join(base);
        if let Ok(rd) = std::fs::read_dir(&p) {
            let mut v: Vec<_> = rd.flatten().collect();
            v.sort_by_key(|e| std::cmp::Reverse(e.metadata().map(|m| util::mtime(&m)).unwrap_or(0)));
            for e in v {
                let f = e.path().join("unix_args.txt");
                if f.exists() {
                    return f.strip_prefix(dir).ok().map(|r| r.to_string_lossy().to_string());
                }
            }
        }
    }
    None
}

/// Install (or replace) the server core of an instance. Returns the launch spec.
pub async fn install_core(app: &AppState, inst: &Instance, core: &CoreSpec, ctx: &TaskCtx) -> Result<LaunchSpec> {
    let dir = &inst.dir;
    let mc = core.mc_version.as_str();
    let mut launch = inst.config().launch;
    launch.args_file = None;
    launch.jar = Some("server.jar".into());
    if core.kind == "custom" {
        return Ok(launch);
    }
    if mc.is_empty() {
        bail!("未指定游戏版本");
    }
    let major = java_major_for(app, mc).await;
    launch.java_major = major;
    ctx.progress(-1.0, format!("准备 Java {major}"));
    let java_bin = java::ensure(app, major, ctx).await?;

    match core.kind.as_str() {
        "vanilla" => {
            let sha = vanilla_server(app, mc, ctx).await?;
            library::link(app, &sha, &dir.join("server.jar"))?;
        }
        "paper" => {
            let build = match core.loader_version.as_deref().filter(|s| !s.is_empty()) {
                Some(b) => format!("https://fill.papermc.io/v3/projects/paper/versions/{mc}/builds/{b}"),
                None => format!("https://fill.papermc.io/v3/projects/paper/versions/{mc}/builds/latest"),
            };
            let b = net::get_json(app, &build).await?;
            let d = &b["downloads"]["server:default"];
            let name = d["name"].as_str().unwrap_or("paper.jar").to_string();
            let sha = server_jar(app, &name, d["url"].as_str().context("paper 下载地址")?,
                None, d["checksums"]["sha256"].as_str().map(String::from), "server", ctx).await?;
            library::link(app, &sha, &dir.join("server.jar"))?;
        }
        "purpur" => {
            let build = core.loader_version.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| "latest".into());
            let url = format!("https://api.purpurmc.org/v2/purpur/{mc}/{build}/download");
            let sha = server_jar(app, &format!("purpur-{mc}-{build}.jar"), &url, None, None, "server", ctx).await?;
            if build == "latest" {
                // never cache "latest" under a fixed name
                if let Some(r) = app.db.lib_get(&sha) {
                    library::register(app, &sha, "server", &format!("purpur-{mc}-{}.jar", &sha[..8]), "official", r.meta)?;
                }
            }
            library::link(app, &sha, &dir.join("server.jar"))?;
        }
        "fabric" => {
            let loader = match core.loader_version.clone().filter(|s| !s.is_empty()) {
                Some(l) => l,
                None => versions::loader_versions(app, "fabric", mc).await?[0]["id"].as_str().context("fabric loader")?.to_string(),
            };
            let inst_v = net::get_json_cached(app, "https://meta.fabricmc.net/v2/versions/installer", 3600).await?;
            let installer = inst_v[0]["version"].as_str().context("fabric installer")?.to_string();
            let url = format!("https://meta.fabricmc.net/v2/versions/loader/{mc}/{loader}/{installer}/server/jar");
            let sha = server_jar(app, &format!("fabric-server-{mc}-{loader}-{installer}.jar"), &url, None, None, "server", ctx).await?;
            library::link(app, &sha, &dir.join("fabric-server-launch.jar"))?;
            // pre-place vanilla server.jar so the launcher doesn't download it again
            let van = vanilla_server(app, mc, ctx).await?;
            library::link(app, &van, &dir.join("server.jar"))?;
            launch.jar = Some("fabric-server-launch.jar".into());
        }
        "quilt" => {
            let loader = match core.loader_version.clone().filter(|s| !s.is_empty()) {
                Some(l) => l,
                None => versions::loader_versions(app, "quilt", mc).await?[0]["id"].as_str().context("quilt loader")?.to_string(),
            };
            let iv = net::get_json_cached(app, "https://meta.quiltmc.org/v3/versions/installer", 3600).await?;
            let url = iv[0]["url"].as_str().context("quilt installer")?.to_string();
            let iname = format!("quilt-installer-{}.jar", iv[0]["version"].as_str().unwrap_or("x"));
            let sha = server_jar(app, &iname, &url, None, None, "installer", ctx).await?;
            library::link(app, &sha, &dir.join("quilt-installer.jar"))?;
            ctx.progress(-1.0, "运行 Quilt 安装器");
            run_installer(&java_bin, dir, &["-jar", "quilt-installer.jar", "install", "server", mc, &loader, "--download-server", "--install-dir=."], inst, ctx).await?;
            let _ = std::fs::remove_file(dir.join("quilt-installer.jar"));
            launch.jar = Some("quilt-server-launch.jar".into());
        }
        "forge" | "neoforge" => {
            let ver = match core.loader_version.clone().filter(|s| !s.is_empty()) {
                Some(v) => v,
                None => versions::loader_versions(app, &core.kind, mc).await?[0]["id"].as_str().context("没有可用的加载器版本")?.to_string(),
            };
            let (url, name) = if core.kind == "forge" {
                (format!("https://maven.minecraftforge.net/net/minecraftforge/forge/{ver}/forge-{ver}-installer.jar"), format!("forge-{ver}-installer.jar"))
            } else {
                (format!("https://maven.neoforged.net/releases/net/neoforged/neoforge/{ver}/neoforge-{ver}-installer.jar"), format!("neoforge-{ver}-installer.jar"))
            };
            let sha = server_jar(app, &name, &url, None, None, "installer", ctx).await?;
            library::link(app, &sha, &dir.join("installer.jar"))?;
            ctx.progress(-1.0, format!("运行 {} 安装器（首次较慢）", core.kind));
            run_installer(&java_bin, dir, &["-jar", "installer.jar", "--installServer"], inst, ctx).await?;
            for f in ["installer.jar", "installer.jar.log", "installer.log", "run.bat"] {
                let _ = std::fs::remove_file(dir.join(f));
            }
            if let Some(a) = find_args_file(dir) {
                launch.args_file = Some(a);
                launch.jar = None;
            } else {
                // legacy forge: forge-<ver>.jar / forge-<ver>-universal.jar
                let jar = std::fs::read_dir(dir)?
                    .flatten()
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .find(|n| n.starts_with("forge-") && n.ends_with(".jar") && !n.contains("installer"))
                    .context("安装完成但未找到启动文件")?;
                launch.jar = Some(jar);
            }
        }
        k => bail!("不支持的核心类型 {k}"),
    }
    Ok(launch)
}

/// Resolve a ContentRef into a library sha1 (downloading from Modrinth if needed).
async fn content_sha(app: &AppState, r: &ContentRef, kind: &str, ctx: &TaskCtx) -> Result<(String, String)> {
    match r {
        ContentRef::Library { sha1 } => {
            let row = app.db.lib_get(sha1).context("资源库中找不到该文件")?;
            Ok((row.sha1, row.name))
        }
        ContentRef::Modrinth { version_id } => {
            let v = modrinth::version(app, version_id).await?;
            modrinth::fetch_version(app, &v, kind, Some(ctx)).await
        }
    }
}

pub fn set_eula(dir: &Path) -> Result<()> {
    util::atomic_write(&dir.join("eula.txt"), b"# Accepted via RimePanel (https://aka.ms/MinecraftEULA)\neula=true\n")
}

/// Configure a resource pack served by the panel itself.
pub fn apply_resource_pack(app: &AppState, inst: &Instance, sha1: Option<&str>) -> Result<()> {
    let mut changes = BTreeMap::new();
    match sha1 {
        Some(s) => {
            let base = app.setting("public_url").unwrap_or_default();
            let base = base.trim_end_matches('/');
            changes.insert("resource-pack".into(), format!("{base}/rp/{s}.zip"));
            changes.insert("resource-pack-sha1".into(), s.into());
        }
        None => {
            changes.insert("resource-pack".into(), String::new());
            changes.insert("resource-pack-sha1".into(), String::new());
        }
    }
    props::update(&inst.dir, &changes)?;
    inst.cfg.write().unwrap().resource_pack = sha1.map(String::from);
    Ok(())
}

pub async fn assemble(app: Arc<AppState>, req: AssembleReq, inst: Arc<Instance>, ctx: TaskCtx) -> Result<String> {
    let dir = inst.dir.clone();
    let mut notes: Vec<String> = vec![];

    // 1. modpack decides the core when present
    let mut core = req.core.clone().unwrap_or_default();
    let mut pack: Option<(std::path::PathBuf, bool)> = None; // (path, is_mrpack)
    if let Some(p) = &req.modpack {
        ctx.progress(-1.0, "获取整合包");
        let sha = match p {
            PackRef::Library { sha1 } => sha1.clone(),
            PackRef::Modrinth { version_id } => {
                let v = modrinth::version(&app, version_id).await?;
                modrinth::fetch_version(&app, &v, "modpack", Some(&ctx)).await?.0
            }
        };
        let path = library::object_path(&app, &sha);
        let is_mr = modrinth::read_zip_json(&path, "modrinth.index.json").is_ok();
        core = if is_mr {
            modrinth::mrpack_core(&modrinth::read_zip_json(&path, "modrinth.index.json")?)
        } else {
            curseforge::manifest_core(&modrinth::read_zip_json(&path, "manifest.json").context("无法识别的整合包格式")?)
        };
        pack = Some((path, is_mr));
    }
    if core.kind.is_empty() {
        bail!("请选择服务端核心");
    }
    inst.cfg.write().unwrap().core = core.clone();
    instance::save(&app, &inst)?;

    // 2. core
    let mut launch = install_core(&app, &inst, &core, &ctx).await?;
    if let Some(m) = &req.max_mem {
        launch.max_mem = m.clone();
    }
    inst.cfg.write().unwrap().launch = launch;
    instance::save(&app, &inst)?;

    // 3. modpack content
    if let Some((path, is_mr)) = &pack {
        if *is_mr {
            let n = modrinth::apply_mrpack(&app, &dir, path, &ctx).await?;
            notes.push(format!("整合包文件 {n} 个"));
        } else {
            let r = curseforge::apply(&app, &dir, path, &ctx).await?;
            notes.push(format!("模组 {} 个", r.installed));
            if !r.skipped_client.is_empty() {
                notes.push(format!("跳过客户端文件 {} 个", r.skipped_client.len()));
            }
            if !r.failed.is_empty() {
                notes.push(format!("{} 个下载失败，可手动上传到资源库", r.failed.len()));
                ctx.set_result(json!({"failed": r.failed}));
            }
        }
    }

    // 4. extra mods / plugins
    let folder = modrinth::mods_folder(&core.kind);
    for (i, m) in req.mods.iter().enumerate() {
        ctx.progress(i as f32 / req.mods.len().max(1) as f32, "链接模组");
        if let ContentRef::Modrinth { version_id } = m {
            modrinth::install_into(&app, &dir, &core, version_id, true, &ctx).await?;
            continue;
        }
        let (sha, name) = content_sha(&app, m, "mod", &ctx).await?;
        // explicit choice wins over a same-named file from the pack
        library::link(&app, &sha, &dir.join(folder).join(&name))?;
    }

    // 5. world (copied, never linked: worlds are mutable)
    let level = props::level_name(&dir);
    if let Some(wid) = &req.world {
        ctx.progress(-1.0, "复制世界");
        let src = app.paths.worlds().join(wid);
        if !src.is_dir() {
            bail!("世界模板不存在");
        }
        let dst = dir.join(&level);
        tokio::task::spawn_blocking(move || -> Result<()> {
            let _ = std::fs::remove_dir_all(&dst);
            util::copy_dir(&src, &dst)
        })
        .await??;
    }

    // 6. datapacks
    for d in &req.datapacks {
        let (sha, name) = content_sha(&app, d, "datapack", &ctx).await?;
        library::link(&app, &sha, &dir.join(&level).join("datapacks").join(&name))?;
    }

    // 7. resource pack (served by the panel)
    if let Some(r) = &req.resourcepack {
        let (sha, _) = content_sha(&app, r, "resourcepack", &ctx).await?;
        apply_resource_pack(&app, &inst, Some(&sha))?;
    }

    // 8. properties / eula
    let mut changes = req.properties.clone();
    if let Some(p) = req.port {
        changes.insert("server-port".into(), p.to_string());
    }
    if !changes.is_empty() {
        props::update(&dir, &changes)?;
    }
    if req.eula {
        set_eula(&dir)?;
    }

    // 9. strip client-only mods
    if req.strip_client && dir.join("mods").is_dir() {
        ctx.progress(-1.0, "识别客户端模组");
        let found = clientmods::scan(&app, &dir).await.unwrap_or_default();
        let high: Vec<String> = found.iter().filter(|f| f.confidence == "high").map(|f| f.file.clone()).collect();
        if !high.is_empty() {
            clientmods::disable(&dir, &high)?;
            notes.push(format!("已隔离 {} 个客户端模组", high.len()));
        }
        let medium = found.iter().filter(|f| f.confidence != "high").count();
        if medium > 0 {
            notes.push(format!("{medium} 个疑似客户端模组待确认"));
        }
    }

    instance::save(&app, &inst)?;
    if let Some(name) = &req.save_recipe {
        let mut r = serde_json::to_value(&req)?;
        r["save_recipe"] = Value::Null;
        app.db.recipe_save(&util::new_id(), name, &r)?;
    }
    Ok(if notes.is_empty() { "实例已就绪".into() } else { format!("实例已就绪：{}", notes.join("，")) })
}

/// Create the instance record then run assembly as a background task.
pub fn start_assemble(app: Arc<AppState>, req: AssembleReq) -> Result<(String, String)> {
    if req.name.trim().is_empty() {
        bail!("请填写实例名称");
    }
    let mut cfg = instance::new_config(req.name.trim(), req.core.clone().unwrap_or_default());
    cfg.installing = true;
    let inst = app.instances.insert(&app, cfg.clone())?;
    let a = app.clone();
    let id = cfg.id.clone();
    let i2 = inst.clone();
    let task = app.tasks.spawn("assemble", format!("创建实例 {}", cfg.name), Some(id.clone()), move |ctx| async move {
        let guard = instance::InstallGuard::new(a.clone(), i2.clone(), ctx.cancel.clone(), true);
        let r = assemble(a.clone(), req, i2.clone(), ctx).await;
        if r.is_ok() {
            guard.finish();
        }
        r
    });
    Ok((id, task))
}
