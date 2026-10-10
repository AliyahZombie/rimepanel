//! HTTP router: REST API, SSE events, console WebSocket, embedded frontend.
use crate::{
    auth, backup, clientmods, files,
    install::{self, AssembleReq},
    instance::{self, CoreSpec, Status},
    java, library, modrinth, props,
    state::{not_found, ApiResult, AppState},
    util, versions, worlds,
};
use anyhow::Context;
use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        DefaultBodyLimit, Multipart, Path, Query, State,
    },
    http::{header, StatusCode, Uri},
    middleware,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse, Response,
    },
    routing::{delete, get, post, put},
    Json, Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{collections::BTreeMap, convert::Infallible, sync::Arc, time::Duration};
use tokio::io::AsyncWriteExt;

type S = State<Arc<AppState>>;

#[derive(rust_embed::RustEmbed)]
#[folder = "web/"]
struct Assets;

pub fn router(app: Arc<AppState>) -> Router {
    let api = Router::new()
        // auth
        .route("/api/auth/state", get(auth::state))
        .route("/api/auth/setup", post(auth::setup))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/auth/password", post(auth::change_password))
        // system
        .route("/api/system", get(system))
        .route("/api/events", get(events))
        .route("/api/settings", get(get_settings).put(put_settings))
        // tasks
        .route("/api/tasks", get(tasks_list))
        .route("/api/tasks/clear", post(tasks_clear))
        .route("/api/tasks/:id/cancel", post(task_cancel))
        // versions / java
        .route("/api/versions/:kind", get(game_versions))
        .route("/api/versions/:kind/loaders", get(loader_versions))
        .route("/api/java", get(java_list))
        .route("/api/java/install", post(java_install))
        .route("/api/java/available", get(java_available))
        .route("/api/java/custom", post(java_custom_add))
        .route("/api/java/custom/remove", post(java_custom_remove))
        .route("/api/java/:major", delete(java_uninstall))
        // instances
        .route("/api/instances", get(inst_list).post(inst_create))
        .route("/api/instances/:id", get(inst_get).patch(inst_patch).delete(inst_delete))
        .route("/api/instances/:id/start", post(inst_start))
        .route("/api/instances/:id/stop", post(inst_stop))
        .route("/api/instances/:id/restart", post(inst_restart))
        .route("/api/instances/:id/kill", post(inst_kill))
        .route("/api/instances/:id/command", post(inst_command))
        .route("/api/instances/:id/console", get(inst_console))
        .route("/api/instances/:id/properties", get(props_get).put(props_put))
        .route("/api/instances/:id/eula", post(inst_eula))
        .route("/api/instances/:id/cancel-install", post(inst_cancel_install))
        .route("/api/instances/:id/core", post(inst_core))
        .route("/api/instances/:id/content", get(content_list))
        .route("/api/instances/:id/content/remove", post(content_remove))
        .route("/api/instances/:id/content/link", post(content_link))
        .route("/api/instances/:id/modrinth", post(inst_modrinth))
        .route("/api/instances/:id/clientmods/scan", post(cm_scan))
        .route("/api/instances/:id/clientmods/disable", post(cm_disable))
        .route("/api/instances/:id/clientmods/enable", post(cm_enable))
        .route("/api/instances/:id/resourcepack", post(inst_rp))
        .route("/api/instances/:id/backups", get(bk_list).post(bk_create))
        .route("/api/instances/:id/backups/:name", delete(bk_delete))
        .route("/api/instances/:id/backups/:name/restore", post(bk_restore))
        .route("/api/instances/:id/backups/:name/download", get(bk_download))
        .route("/api/instances/:id/backups/:name/to-world", post(bk_to_world))
        .route("/api/instances/:id/world/apply", post(world_apply))
        .route("/api/instances/:id/world/save", post(world_save))
        // files
        .route("/api/instances/:id/files", get(files::list))
        .route("/api/instances/:id/files/read", get(files::read))
        .route("/api/instances/:id/files/write", put(files::write))
        .route("/api/instances/:id/files/upload", post(files::upload))
        .route("/api/instances/:id/files/download", get(files::download))
        .route("/api/instances/:id/files/rename", post(files::rename))
        .route("/api/instances/:id/files/delete", post(files::delete))
        .route("/api/instances/:id/files/mkdir", post(files::mkdir))
        .route("/api/instances/:id/files/compress", post(files::compress))
        .route("/api/instances/:id/files/extract", post(files::extract))
        .route("/api/instances/:id/files/to-library", post(files::to_library))
        // library
        .route("/api/library", get(lib_list))
        .route("/api/library/stats", get(lib_stats))
        .route("/api/library/upload", post(lib_upload))
        .route("/api/library/gc", post(lib_gc))
        .route("/api/library/modrinth", post(lib_modrinth))
        .route("/api/library/:sha1", delete(lib_delete).patch(lib_patch))
        .route("/api/worlds", get(worlds_list))
        .route("/api/worlds/upload", post(worlds_upload))
        .route("/api/worlds/:id", delete(worlds_delete).patch(worlds_rename))
        .route("/api/recipes", get(recipes_list).post(recipes_save))
        .route("/api/recipes/:id", delete(recipes_delete))
        // modrinth proxy
        .route("/api/modrinth/search", get(mr_search))
        .route("/api/modrinth/project/:id", get(mr_project))
        .route("/api/modrinth/project/:id/versions", get(mr_versions))
        // public resource pack download for clients
        .route("/rp/:file", get(resource_pack));

    api.fallback(static_handler)
        .layer(middleware::from_fn_with_state(app.clone(), auth::require))
        .layer(DefaultBodyLimit::disable())
        .with_state(app)
}

fn ok() -> Json<Value> {
    Json(json!({"ok": true}))
}

fn get_inst(app: &AppState, id: &str) -> ApiResult<Arc<instance::Instance>> {
    app.instances.get(id).ok_or_else(|| not_found("实例"))
}

// ---------------- static ----------------
async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    let (file, data) = match Assets::get(path) {
        Some(d) if !path.is_empty() => (path.to_string(), d),
        _ => ("index.html".to_string(), Assets::get("index.html").expect("index.html")),
    };
    let mime = mime_guess::from_path(&file).first_or_octet_stream();
    let cache = if file == "index.html" { "no-cache" } else { "public, max-age=3600" };
    (
        [(header::CONTENT_TYPE, mime.to_string()), (header::CACHE_CONTROL, cache.to_string())],
        Body::from(data.data.into_owned()),
    )
        .into_response()
}

// ---------------- system / settings ----------------
fn meminfo() -> (u64, u64) {
    let s = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let get = |k: &str| {
        s.lines()
            .find(|l| l.starts_with(k))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0)
    };
    (get("MemTotal:"), get("MemAvailable:"))
}

fn disk(path: &std::path::Path) -> (u64, u64) {
    #[cfg(unix)]
    unsafe {
        let c = std::ffi::CString::new(path.to_string_lossy().as_bytes()).unwrap();
        let mut st: libc::statvfs = std::mem::zeroed();
        if libc::statvfs(c.as_ptr(), &mut st) == 0 {
            let bs = st.f_frsize as u64;
            return (st.f_blocks as u64 * bs, st.f_bavail as u64 * bs);
        }
    }
    (0, 0)
}

async fn system(State(app): S) -> Json<Value> {
    let (mt, ma) = meminfo();
    let (dt, df) = disk(&app.paths.root);
    let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let self_rss = std::fs::read_to_string("/proc/self/status")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("VmRSS:"))
        .and_then(|l| l.split_whitespace().nth(1).map(String::from))
        .unwrap_or_default();
    Json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "os": std::env::consts::OS, "arch": std::env::consts::ARCH,
        "cpus": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
        "mem_total_kb": mt, "mem_avail_kb": ma,
        "disk_total": dt, "disk_free": df,
        "load": load.split_whitespace().take(3).collect::<Vec<_>>(),
        "panel_rss_kb": self_rss.parse::<u64>().unwrap_or(0),
        "data_dir": app.paths.root.to_string_lossy(),
    }))
}

const PUBLIC_SETTINGS: &[&str] = &["mirror", "curseforge_key", "public_url", "monaco_cdn", "download_concurrency"];

async fn get_settings(State(app): S) -> Json<Value> {
    let mut m = serde_json::Map::new();
    for (k, v) in app.db.all_settings() {
        if PUBLIC_SETTINGS.contains(&k.as_str()) {
            let v = if k == "curseforge_key" && !v.is_empty() { "********".to_string() } else { v };
            m.insert(k, Value::String(v));
        }
    }
    Json(Value::Object(m))
}

async fn put_settings(State(app): S, Json(b): Json<BTreeMap<String, String>>) -> ApiResult<Json<Value>> {
    for (k, v) in b {
        if !PUBLIC_SETTINGS.contains(&k.as_str()) {
            continue;
        }
        if k == "curseforge_key" && v == "********" {
            continue;
        }
        app.db.set_setting(&k, v.trim())?;
    }
    Ok(ok())
}

/// Server-sent events: task updates + instance snapshots.
async fn events(State(app): S) -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let rx = app.tasks.events.subscribe();
    let iv = tokio::time::interval(Duration::from_secs(2));
    let stream = futures_util::stream::unfold((rx, iv, app), |(mut rx, mut iv, app)| async move {
        let ev = tokio::select! {
            r = rx.recv() => match r {
                Ok(t) => Event::default().event("task").json_data(t).unwrap_or_default(),
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => Event::default().comment("lag"),
                Err(_) => return None,
            },
            _ = iv.tick() => {
                let mut views = vec![];
                for i in app.instances.all() {
                    views.push(instance::view(&i).await);
                }
                Event::default().event("instances").json_data(views).unwrap_or_default()
            }
        };
        Some((Ok(ev), (rx, iv, app)))
    });
    Sse::new(stream).keep_alive(KeepAlive::default())
}

// ---------------- tasks ----------------
async fn tasks_list(State(app): S) -> Json<Value> {
    Json(json!(app.tasks.list()))
}
async fn tasks_clear(State(app): S) -> Json<Value> {
    app.tasks.clear_finished();
    ok()
}
async fn task_cancel(State(app): S, Path(id): Path<String>) -> Json<Value> {
    Json(json!({"ok": app.tasks.cancel(&id)}))
}

// ---------------- versions / java ----------------
async fn game_versions(State(app): S, Path(kind): Path<String>) -> ApiResult<Json<Value>> {
    Ok(Json(versions::game_versions(&app, &kind).await?))
}
#[derive(Deserialize)]
struct McQ {
    #[serde(default)]
    mc: String,
}
async fn loader_versions(State(app): S, Path(kind): Path<String>, Query(q): Query<McQ>) -> ApiResult<Json<Value>> {
    Ok(Json(versions::loader_versions(&app, &kind, &q.mc).await?))
}
async fn java_list(State(app): S) -> Json<Value> {
    let a = app.clone();
    let list = tokio::task::spawn_blocking(move || java::list(&a)).await.unwrap_or_default();
    // which instances need which major
    let mut usage: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for i in app.instances.all() {
        let c = i.config();
        usage.entry(c.launch.java_major).or_default().push(c.name);
    }
    Json(json!({"runtimes": list, "required": usage}))
}
async fn java_available(State(app): S) -> ApiResult<Json<Value>> {
    Ok(Json(java::available(&app).await?))
}
#[derive(Deserialize)]
struct JavaPathB {
    path: String,
}
async fn java_custom_add(State(app): S, Json(b): Json<JavaPathB>) -> ApiResult<Json<Value>> {
    let p = std::path::PathBuf::from(b.path.trim());
    let p2 = p.clone();
    let ver = tokio::task::spawn_blocking(move || java::probe(&p2)).await?
        .context("无法识别该 Java（请填写 bin/java 可执行文件路径）")?;
    let mut v = java::custom_paths(&app);
    let s = p.to_string_lossy().to_string();
    if !v.contains(&s) {
        v.push(s);
    }
    java::set_custom_paths(&app, &v)?;
    Ok(Json(json!({"version": ver, "major": java::parse_major(&ver)})))
}
async fn java_custom_remove(State(app): S, Json(b): Json<JavaPathB>) -> ApiResult<Json<Value>> {
    let v: Vec<String> = java::custom_paths(&app).into_iter().filter(|p| p != &b.path).collect();
    java::set_custom_paths(&app, &v)?;
    Ok(ok())
}
async fn java_uninstall(State(app): S, Path(major): Path<u32>) -> ApiResult<Json<Value>> {
    for i in app.instances.all() {
        let c = i.config();
        if c.launch.java_major == major && c.launch.java == "auto" && i.status().await != Status::Stopped && i.status().await != Status::Crashed {
            return Err(anyhow::anyhow!("实例「{}」正在使用 Java {major}，请先停止", c.name).into());
        }
    }
    java::uninstall(&app, major)?;
    Ok(ok())
}
#[derive(Deserialize)]
struct MajorB {
    major: u32,
}
async fn java_install(State(app): S, Json(b): Json<MajorB>) -> Json<Value> {
    let a = app.clone();
    let t = app.tasks.spawn("java", format!("安装 Java {}", b.major), None, move |ctx| async move {
        let p = java::install(&a, b.major, &ctx).await?;
        Ok(format!("已安装 {p}"))
    });
    Json(json!({"task": t}))
}

// ---------------- instances ----------------
async fn inst_list(State(app): S) -> Json<Value> {
    let mut v = vec![];
    for i in app.instances.all() {
        v.push(instance::view(&i).await);
    }
    v.sort_by(|a, b| a.cfg.created.cmp(&b.cfg.created));
    Json(json!(v))
}

async fn inst_create(State(app): S, Json(req): Json<AssembleReq>) -> ApiResult<Json<Value>> {
    let (id, task) = install::start_assemble(app, req)?;
    Ok(Json(json!({"id": id, "task": task})))
}

async fn inst_get(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    Ok(Json(json!(instance::view(&i).await)))
}

#[derive(Deserialize)]
struct PatchB {
    name: Option<String>,
    launch: Option<instance::LaunchSpec>,
    auto_start: Option<bool>,
    auto_restart: Option<bool>,
    backup: Option<instance::BackupPolicy>,
}
async fn inst_patch(State(app): S, Path(id): Path<String>, Json(b): Json<PatchB>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    {
        let mut c = i.cfg.write().unwrap();
        if let Some(n) = b.name.filter(|s| !s.trim().is_empty()) {
            c.name = n.trim().into();
        }
        if let Some(l) = b.launch {
            c.launch = l;
        }
        if let Some(v) = b.auto_start {
            c.auto_start = v;
        }
        if let Some(v) = b.auto_restart {
            c.auto_restart = v;
        }
        if let Some(v) = b.backup {
            c.backup = v;
        }
    }
    instance::save(&app, &i)?;
    Ok(ok())
}

#[derive(Deserialize)]
struct DelQ {
    #[serde(default)]
    keep_backups: bool,
}
async fn inst_delete(State(app): S, Path(id): Path<String>, Query(q): Query<DelQ>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    if !matches!(i.status().await, Status::Stopped | Status::Crashed) {
        return Err(anyhow::anyhow!("请先停止服务器").into());
    }
    if i.config().installing {
        // deleting an instance that is still installing: cancel its tasks first
        let owned = app.tasks.running_for(&id);
        for t in &owned {
            app.tasks.cancel(t);
        }
        for _ in 0..50 {
            if !i.config().installing || app.instances.get(&id).is_none() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        if app.instances.get(&id).is_none() {
            return Ok(ok()); // the cancelled creation already removed it
        }
        if owned.is_empty() {
            // no task owns it: a stale flag, safe to delete anyway
            tracing::warn!("instance {id} had a stale installing flag");
        } else if i.config().installing {
            return Err(anyhow::anyhow!("安装任务仍在收尾，请稍后再试").into());
        }
    }
    app.instances.remove(&id);
    app.db.delete_instance(&id)?;
    let dir = i.dir.clone();
    let bdir = app.paths.backups(&id);
    tokio::task::spawn_blocking(move || {
        let _ = std::fs::remove_dir_all(dir);
        if !q.keep_backups {
            let _ = std::fs::remove_dir_all(bdir);
        }
    })
    .await?;
    Ok(ok())
}

async fn inst_start(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    if !i.dir.join("eula.txt").exists()
        || !std::fs::read_to_string(i.dir.join("eula.txt")).unwrap_or_default().contains("eula=true")
    {
        return Err(anyhow::anyhow!("EULA_REQUIRED").into());
    }
    match instance::start(app.clone(), i.clone()).await {
        Ok(_) => Ok(ok()),
        Err(e) if e.to_string().starts_with("JAVA_MISSING:") => {
            // install the required runtime in the background, then start
            let major = i.config().launch.java_major;
            let a = app.clone();
            let t = app.tasks.spawn("java", format!("安装 Java {major} 并启动 {}", i.config().name), Some(id), move |ctx| async move {
                java::install(&a, major, &ctx).await?;
                ctx.progress(-1.0, "启动服务器");
                instance::start(a.clone(), i).await?;
                Ok(format!("Java {major} 已安装，服务器启动中"))
            });
            Ok(Json(json!({"task": t, "java_installing": major})))
        }
        Err(e) => Err(e.into()),
    }
}
async fn inst_stop(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    tokio::spawn(async move {
        let _ = instance::stop(&i, Duration::from_secs(60)).await;
    });
    Ok(ok())
}
async fn inst_restart(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    let a = app.clone();
    tokio::spawn(async move {
        if matches!(i.status().await, Status::Running | Status::Starting) {
            let _ = instance::stop(&i, Duration::from_secs(60)).await;
        }
        if let Err(e) = instance::start(a, i.clone()).await {
            i.push_log(format!("[RimePanel] 重启失败: {e:#}"));
        }
    });
    Ok(ok())
}
async fn inst_kill(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    instance::kill(&i).await?;
    Ok(ok())
}
#[derive(Deserialize)]
struct CmdB {
    command: String,
}
async fn inst_command(State(app): S, Path(id): Path<String>, Json(b): Json<CmdB>) -> ApiResult<Json<Value>> {
    get_inst(&app, &id)?.send_command(&b.command).await?;
    Ok(ok())
}

async fn inst_console(State(app): S, Path(id): Path<String>, ws: WebSocketUpgrade) -> ApiResult<Response> {
    let i = get_inst(&app, &id)?;
    Ok(ws.on_upgrade(move |sock| console_ws(sock, i)))
}

async fn console_ws(sock: WebSocket, i: Arc<instance::Instance>) {
    let (mut tx, mut rx) = sock.split();
    let mut sub = i.console.subscribe();
    let hist = json!({"type": "history", "lines": i.recent_log()}).to_string();
    if tx.send(Message::Text(hist)).await.is_err() {
        return;
    }
    let i2 = i.clone();
    let mut send_task = tokio::spawn(async move {
        let mut batch: Vec<String> = Vec::new();
        let mut flush = tokio::time::interval(Duration::from_millis(80));
        loop {
            tokio::select! {
                r = sub.recv() => match r {
                    Ok(l) => { batch.push(l); if batch.len() < 400 { continue; } }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(_) => break,
                },
                _ = flush.tick() => {}
            }
            if !batch.is_empty() {
                let msg = json!({"type": "lines", "lines": std::mem::take(&mut batch)}).to_string();
                if tx.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        }
    });
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(m)) = rx.next().await {
            if let Message::Text(t) = m {
                if let Err(e) = i2.send_command(&t).await {
                    i2.push_log(format!("[RimePanel] {e:#}"));
                }
            }
        }
    });
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}

async fn props_get(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    Ok(Json(json!(props::read(&i.dir))))
}
async fn props_put(State(app): S, Path(id): Path<String>, Json(b): Json<BTreeMap<String, String>>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    props::update(&i.dir, &b)?;
    Ok(ok())
}
/// Cancel every running task that owns this instance (install / core change).
/// A cancelled *creation* removes the instance; a cancelled core change keeps it.
async fn inst_cancel_install(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    let owned = app.tasks.running_for(&id);
    for t in &owned {
        app.tasks.cancel(t);
    }
    for _ in 0..50 {
        if !i.config().installing || app.instances.get(&id).is_none() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    if owned.is_empty() && i.config().installing {
        // nobody owns the flag any more: clear it so the instance is usable again
        {
            let mut c = i.cfg.write().unwrap();
            c.installing = false;
            c.incomplete = true;
        }
        instance::save(&app, &i)?;
    }
    Ok(Json(json!({"removed": app.instances.get(&id).is_none(), "cancelled": owned.len()})))
}

async fn inst_eula(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    install::set_eula(&get_inst(&app, &id)?.dir)?;
    Ok(ok())
}

async fn inst_core(State(app): S, Path(id): Path<String>, Json(core): Json<CoreSpec>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    if !matches!(i.status().await, Status::Stopped | Status::Crashed) {
        return Err(anyhow::anyhow!("请先停止服务器").into());
    }
    if i.config().installing {
        return Err(anyhow::anyhow!("实例正在安装中").into());
    }
    let a = app.clone();
    let t = app.tasks.spawn("core", format!("更换核心 {} {}", core.kind, core.mc_version), Some(id), move |ctx| async move {
        let guard = instance::InstallGuard::new(a.clone(), i.clone(), ctx.cancel.clone(), false);
        let l = install::install_core(&a, &i, &core, &ctx).await?;
        {
            let mut c = i.cfg.write().unwrap();
            c.launch = l;
            c.core = core;
        }
        guard.finish(); // clears installing + incomplete, saves
        Ok("核心已安装".to_string())
    });
    Ok(Json(json!({"task": t})))
}

// ----- content (mods / plugins) -----
async fn content_list(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    let dir = i.dir.clone();
    let level = props::level_name(&dir);
    let folders = vec![
        "mods".to_string(),
        "mods-disabled".to_string(),
        "plugins".to_string(),
        format!("{level}/datapacks"),
    ];
    let rows = tokio::task::spawn_blocking(move || {
        let mut out = vec![];
        for f in folders {
            let Ok(rd) = std::fs::read_dir(dir.join(&f)) else { continue };
            for e in rd.flatten() {
                let Ok(m) = e.metadata() else { continue };
                if !m.is_file() {
                    continue;
                }
                let name = e.file_name().to_string_lossy().to_string();
                let sha = util::sha1_file(&e.path()).unwrap_or_default();
                out.push((f.clone(), name, m.len(), util::nlink(&m) > 1, sha));
            }
        }
        out
    })
    .await?;
    let v: Vec<Value> = rows
        .into_iter()
        .map(|(folder, name, size, linked, sha)| {
            let lib = app.db.lib_get(&sha);
            json!({
                "folder": folder, "file": name, "size": size, "linked": linked, "sha1": sha,
                "title": lib.as_ref().and_then(|r| r.meta["title"].as_str().map(String::from)),
                "icon_url": lib.as_ref().and_then(|r| r.meta["icon_url"].as_str().map(String::from)),
                "source": lib.as_ref().map(|r| r.source.clone()),
            })
        })
        .collect();
    Ok(Json(json!(v)))
}

#[derive(Deserialize)]
struct ContentB {
    folder: String,
    file: String,
}
async fn content_remove(State(app): S, Path(id): Path<String>, Json(b): Json<ContentB>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    let p = util::safe_join(&i.dir, &format!("{}/{}", b.folder, b.file))?;
    std::fs::remove_file(p)?;
    Ok(ok())
}

#[derive(Deserialize)]
struct LinkB {
    sha1: String,
}
async fn content_link(State(app): S, Path(id): Path<String>, Json(b): Json<LinkB>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    let row = app.db.lib_get(&b.sha1).ok_or_else(|| not_found("资源"))?;
    let core = i.config().core;
    match row.kind.as_str() {
        "resourcepack" => install::apply_resource_pack(&app, &i, Some(&row.sha1))?,
        "datapack" => {
            library::link(&app, &row.sha1, &i.dir.join(props::level_name(&i.dir)).join("datapacks").join(&row.name))?;
        }
        "plugin" => {
            library::link(&app, &row.sha1, &i.dir.join("plugins").join(&row.name))?;
        }
        "mod" => {
            library::link(&app, &row.sha1, &i.dir.join(modrinth::mods_folder(&core.kind)).join(&row.name))?;
        }
        "server" => {
            library::link(&app, &row.sha1, &i.dir.join("server.jar"))?;
        }
        k => return Err(anyhow::anyhow!("该类型 ({k}) 不能直接加入实例").into()),
    }
    instance::save(&app, &i)?;
    Ok(ok())
}

#[derive(Deserialize)]
struct MrInstallB {
    version_id: String,
    #[serde(default = "tru")]
    deps: bool,
}
fn tru() -> bool {
    true
}
async fn inst_modrinth(State(app): S, Path(id): Path<String>, Json(b): Json<MrInstallB>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    let a = app.clone();
    let t = app.tasks.spawn("modrinth", "安装 Modrinth 资源", Some(id), move |ctx| async move {
        let core = i.config().core;
        let installed = modrinth::install_into(&a, &i.dir, &core, &b.version_id, b.deps, &ctx).await?;
        let mut names = vec![];
        for n in installed {
            if let Some(sha) = n.strip_prefix("resourcepack:") {
                install::apply_resource_pack(&a, &i, Some(sha))?;
                instance::save(&a, &i)?;
                names.push("服务器资源包".to_string());
            } else {
                names.push(n);
            }
        }
        Ok(format!("已安装 {}", names.join(", ")))
    });
    Ok(Json(json!({"task": t})))
}

async fn cm_scan(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    let a = app.clone();
    let t = app.tasks.spawn("clientmods", "识别客户端模组", Some(id), move |ctx| async move {
        ctx.progress(-1.0, "扫描 mods 目录");
        let found = clientmods::scan(&a, &i.dir).await?;
        let n = found.len();
        ctx.set_result(json!(found));
        Ok(format!("发现 {n} 个客户端模组"))
    });
    Ok(Json(json!({"task": t})))
}
#[derive(Deserialize)]
struct FilesB {
    files: Vec<String>,
}
async fn cm_disable(State(app): S, Path(id): Path<String>, Json(b): Json<FilesB>) -> ApiResult<Json<Value>> {
    let n = clientmods::disable(&get_inst(&app, &id)?.dir, &b.files)?;
    Ok(Json(json!({"moved": n})))
}
async fn cm_enable(State(app): S, Path(id): Path<String>, Json(b): Json<FilesB>) -> ApiResult<Json<Value>> {
    let n = clientmods::enable(&get_inst(&app, &id)?.dir, &b.files)?;
    Ok(Json(json!({"moved": n})))
}

#[derive(Deserialize)]
struct RpB {
    sha1: Option<String>,
}
async fn inst_rp(State(app): S, Path(id): Path<String>, Json(b): Json<RpB>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    install::apply_resource_pack(&app, &i, b.sha1.as_deref().filter(|s| !s.is_empty()))?;
    instance::save(&app, &i)?;
    Ok(ok())
}

// ----- backups -----
async fn bk_list(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    get_inst(&app, &id)?;
    Ok(Json(json!(backup::list(&app, &id))))
}
async fn bk_create(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    let a = app.clone();
    let t = app.tasks.spawn("backup", format!("备份 {}", i.config().name), Some(id), move |ctx| async move {
        ctx.progress(-1.0, "打包世界");
        backup::create(&a, &i, "manual").await
    });
    Ok(Json(json!({"task": t})))
}
async fn bk_delete(State(app): S, Path((id, name)): Path<(String, String)>) -> ApiResult<Json<Value>> {
    backup::delete(&app, &id, &name)?;
    Ok(ok())
}
async fn bk_restore(State(app): S, Path((id, name)): Path<(String, String)>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    let a = app.clone();
    let t = app.tasks.spawn("restore", format!("恢复 {name}"), Some(id), move |ctx| async move {
        ctx.progress(-1.0, "恢复中");
        backup::restore(&a, &i, &name).await?;
        Ok("恢复完成".into())
    });
    Ok(Json(json!({"task": t})))
}
async fn bk_download(State(app): S, Path((id, name)): Path<(String, String)>) -> ApiResult<Response> {
    let p = app.paths.backups(&id).join(util::clean_rel(&name)?);
    files::stream_file(&p, &name).await
}
#[derive(Deserialize)]
struct NameB {
    name: String,
}
async fn bk_to_world(State(app): S, Path((id, name)): Path<(String, String)>, Json(b): Json<NameB>) -> ApiResult<Json<Value>> {
    let a = app.clone();
    let t = app.tasks.spawn("world", format!("另存世界 {}", b.name), Some(id.clone()), move |ctx| async move {
        ctx.progress(-1.0, "解包备份");
        worlds::from_backup(&a, &id, &name, &b.name).await?;
        Ok("已加入世界库".into())
    });
    Ok(Json(json!({"task": t})))
}
#[derive(Deserialize)]
struct WorldB {
    world_id: String,
}
async fn world_apply(State(app): S, Path(id): Path<String>, Json(b): Json<WorldB>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    let a = app.clone();
    let t = app.tasks.spawn("world", "替换世界", Some(id), move |ctx| async move {
        ctx.progress(-1.0, "复制世界");
        worlds::apply(&a, &i, &b.world_id).await?;
        Ok("世界已替换".into())
    });
    Ok(Json(json!({"task": t})))
}
async fn world_save(State(app): S, Path(id): Path<String>, Json(b): Json<NameB>) -> ApiResult<Json<Value>> {
    let i = get_inst(&app, &id)?;
    let a = app.clone();
    let t = app.tasks.spawn("world", format!("保存世界 {}", b.name), Some(id), move |ctx| async move {
        ctx.progress(-1.0, "复制世界");
        worlds::from_instance(&a, &i, &b.name).await?;
        Ok("已加入世界库".into())
    });
    Ok(Json(json!({"task": t})))
}

// ---------------- library ----------------
#[derive(Deserialize)]
struct KindQ {
    kind: Option<String>,
}
async fn lib_list(State(app): S, Query(q): Query<KindQ>) -> Json<Value> {
    Json(json!(library::list(&app, q.kind.as_deref().filter(|s| !s.is_empty()))))
}
async fn lib_stats(State(app): S) -> Json<Value> {
    let a = app.clone();
    let s = tokio::task::spawn_blocking(move || library::stats(&a)).await.unwrap();
    Json(json!(s))
}

/// Stream a multipart upload to a temp file; returns (tmp path, filename).
async fn save_upload(app: &AppState, mp: &mut Multipart) -> anyhow::Result<Vec<(std::path::PathBuf, String)>> {
    let mut out = vec![];
    while let Some(mut field) = mp.next_field().await? {
        let Some(name) = field.file_name().map(|s| s.rsplit(['/', '\\']).next().unwrap_or(s).to_string()) else {
            continue;
        };
        let ext = std::path::Path::new(&name)
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();
        let ext = if name.ends_with(".tar.gz") { ".tar.gz".into() } else if name.ends_with(".tar.zst") { ".tar.zst".into() } else { ext };
        let tmp = app.paths.tmp().join(format!("{}{}", util::new_id(), ext));
        let mut f = tokio::fs::File::create(&tmp).await?;
        while let Some(c) = field.chunk().await? {
            f.write_all(&c).await?;
        }
        f.flush().await?;
        out.push((tmp, name));
    }
    Ok(out)
}

async fn lib_upload(State(app): S, Query(q): Query<KindQ>, mut mp: Multipart) -> ApiResult<Json<Value>> {
    let files = save_upload(&app, &mut mp).await?;
    let mut res = vec![];
    for (tmp, name) in files {
        let kind = library::guess_kind(&name, q.kind.as_deref());
        let a = app.clone();
        let sha = tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
            let sha = util::sha1_file(&tmp)?;
            library::import_tmp(&a, &tmp, &sha, &kind, &name, "upload", json!({}))?;
            Ok(sha)
        })
        .await??;
        res.push(sha);
    }
    Ok(Json(json!({"sha1": res})))
}
async fn lib_delete(State(app): S, Path(sha): Path<String>) -> ApiResult<Json<Value>> {
    library::delete(&app, &sha)?;
    Ok(ok())
}
#[derive(Deserialize)]
struct LibPatchB {
    kind: Option<String>,
    name: Option<String>,
}
async fn lib_patch(State(app): S, Path(sha): Path<String>, Json(b): Json<LibPatchB>) -> ApiResult<Json<Value>> {
    let mut row = app.db.lib_get(&sha).ok_or_else(|| not_found("资源"))?;
    if let Some(k) = b.kind.filter(|k| library::KINDS.contains(&k.as_str())) {
        row.kind = k;
    }
    if let Some(n) = b.name.filter(|n| !n.trim().is_empty()) {
        row.name = n.trim().into();
    }
    app.db.lib_delete(&sha)?;
    app.db.lib_upsert(&row)?;
    Ok(ok())
}
#[derive(Deserialize)]
struct GcB {
    #[serde(default)]
    include_servers: bool,
}
async fn lib_gc(State(app): S, Json(b): Json<GcB>) -> ApiResult<Json<Value>> {
    let a = app.clone();
    let (n, freed) = tokio::task::spawn_blocking(move || library::gc(&a, b.include_servers)).await??;
    Ok(Json(json!({"removed": n, "freed": freed})))
}
#[derive(Deserialize)]
struct LibMrB {
    version_id: String,
    kind: Option<String>,
}
async fn lib_modrinth(State(app): S, Json(b): Json<LibMrB>) -> ApiResult<Json<Value>> {
    let a = app.clone();
    let t = app.tasks.spawn("library", "下载到资源库", None, move |ctx| async move {
        let v = modrinth::version(&a, &b.version_id).await?;
        let p = modrinth::project(&a, v["project_id"].as_str().unwrap_or_default()).await.unwrap_or_default();
        let kind = b.kind.unwrap_or_else(|| match p["project_type"].as_str() {
            Some("modpack") => "modpack".into(),
            Some("resourcepack") => "resourcepack".into(),
            Some("plugin") => "plugin".into(),
            _ => {
                let ls: Vec<&str> = v["loaders"].as_array().into_iter().flatten().filter_map(|x| x.as_str()).collect();
                if ls.contains(&"datapack") { "datapack".into() }
                else if ls.iter().any(|l| ["bukkit","spigot","paper","purpur"].contains(l)) { "plugin".into() }
                else { "mod".into() }
            }
        });
        let (sha, name) = modrinth::fetch_version(&a, &v, &kind, Some(&ctx)).await?;
        let mut meta = a.db.lib_get(&sha).map(|r| r.meta).unwrap_or_default();
        meta["title"] = p["title"].clone();
        meta["icon_url"] = p["icon_url"].clone();
        meta["server_side"] = p["server_side"].clone();
        library::register(&a, &sha, &kind, &name, "modrinth", meta)?;
        Ok(format!("已加入资源库 {name}"))
    });
    Ok(Json(json!({"task": t})))
}

async fn worlds_list(State(app): S) -> Json<Value> {
    Json(json!(app.db.world_list()))
}
async fn worlds_upload(State(app): S, mut mp: Multipart) -> ApiResult<Json<Value>> {
    let files = save_upload(&app, &mut mp).await?;
    let (tmp, name) = files.into_iter().next().context("未上传文件")?;
    let a = app.clone();
    let wname = name.trim_end_matches(".zip").trim_end_matches(".tar.gz").trim_end_matches(".tar.zst").to_string();
    let t = app.tasks.spawn("world", format!("导入世界 {wname}"), None, move |ctx| async move {
        ctx.progress(-1.0, "解压世界");
        let r = worlds::from_archive(&a, &tmp, &wname, "上传").await;
        let _ = std::fs::remove_file(&tmp);
        r.map(|_| "世界已导入".to_string())
    });
    Ok(Json(json!({"task": t})))
}
async fn worlds_delete(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    worlds::delete(&app, &id)?;
    Ok(ok())
}
async fn worlds_rename(State(app): S, Path(id): Path<String>, Json(b): Json<NameB>) -> ApiResult<Json<Value>> {
    app.db.world_rename(&id, &b.name)?;
    Ok(ok())
}

async fn recipes_list(State(app): S) -> Json<Value> {
    Json(json!(app.db.recipe_list()))
}
#[derive(Deserialize)]
struct RecipeB {
    name: String,
    data: Value,
}
async fn recipes_save(State(app): S, Json(b): Json<RecipeB>) -> ApiResult<Json<Value>> {
    app.db.recipe_save(&util::new_id(), &b.name, &b.data)?;
    Ok(ok())
}
async fn recipes_delete(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    app.db.recipe_delete(&id)?;
    Ok(ok())
}

// ---------------- modrinth proxy ----------------
#[derive(Deserialize)]
struct MrSearchQ {
    #[serde(default)]
    q: String,
    #[serde(default = "mod_t")]
    r#type: String,
    mc: Option<String>,
    loader: Option<String>,
    #[serde(default)]
    offset: u32,
}
fn mod_t() -> String {
    "mod".into()
}
async fn mr_search(State(app): S, Query(q): Query<MrSearchQ>) -> ApiResult<Json<Value>> {
    Ok(Json(modrinth::search(&app, &q.q, &q.r#type, q.mc.as_deref(), q.loader.as_deref(), q.offset).await?))
}
async fn mr_project(State(app): S, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    Ok(Json(modrinth::project(&app, &id).await?))
}
#[derive(Deserialize)]
struct MrVerQ {
    mc: Option<String>,
    loader: Option<String>,
}
async fn mr_versions(State(app): S, Path(id): Path<String>, Query(q): Query<MrVerQ>) -> ApiResult<Json<Value>> {
    Ok(Json(modrinth::project_versions(&app, &id, q.mc.as_deref(), q.loader.as_deref()).await?))
}

// ---------------- public ----------------
async fn resource_pack(State(app): S, Path(file): Path<String>) -> Response {
    let sha = file.trim_end_matches(".zip").to_lowercase();
    if sha.len() != 40 || !sha.chars().all(|c| c.is_ascii_hexdigit()) {
        return StatusCode::NOT_FOUND.into_response();
    }
    match app.db.lib_get(&sha) {
        Some(r) if r.kind == "resourcepack" => {
            match files::stream_file(&library::object_path(&app, &sha), &r.name).await {
                Ok(r) => r,
                Err(e) => e.into_response(),
            }
        }
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}
