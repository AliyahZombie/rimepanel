//! Instance configuration and server process management.
use crate::{db::Db, java, props, slp, state::{AppState, Paths}, util};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
    process::Stdio,
    sync::{Arc, Mutex as StdMutex, RwLock},
    time::{Duration, Instant},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, Command},
    sync::{broadcast, Mutex},
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CoreSpec {
    /// vanilla | paper | purpur | fabric | quilt | forge | neoforge | custom
    pub kind: String,
    pub mc_version: String,
    #[serde(default)]
    pub loader_version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LaunchSpec {
    /// "auto" (managed runtime) or a path to a java binary
    #[serde(default = "auto")]
    pub java: String,
    #[serde(default = "d21")]
    pub java_major: u32,
    #[serde(default = "d1g")]
    pub min_mem: String,
    #[serde(default = "d4g")]
    pub max_mem: String,
    #[serde(default)]
    pub jvm_args: String,
    /// `-jar <jar>` launch
    #[serde(default)]
    pub jar: Option<String>,
    /// `@argsfile` launch (modern Forge/NeoForge)
    #[serde(default)]
    pub args_file: Option<String>,
    #[serde(default = "nogui")]
    pub server_args: String,
}

fn auto() -> String { "auto".into() }
fn d21() -> u32 { 21 }
fn d1g() -> String { "1G".into() }
fn d4g() -> String { "4G".into() }
fn nogui() -> String { "nogui".into() }

impl Default for LaunchSpec {
    fn default() -> Self {
        Self {
            java: auto(),
            java_major: 21,
            min_mem: d1g(),
            max_mem: d4g(),
            jvm_args: String::new(),
            jar: Some("server.jar".into()),
            args_file: None,
            server_args: nogui(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BackupPolicy {
    #[serde(default = "yes")]
    pub idle_enabled: bool,
    #[serde(default = "d10")]
    pub idle_minutes: u32,
    #[serde(default)]
    pub interval_enabled: bool,
    #[serde(default = "d360")]
    pub interval_minutes: u32,
    #[serde(default = "d10u")]
    pub keep: u32,
}
fn yes() -> bool { true }
fn d10() -> u32 { 10 }
fn d360() -> u32 { 360 }
fn d10u() -> u32 { 10 }

impl Default for BackupPolicy {
    fn default() -> Self {
        Self { idle_enabled: true, idle_minutes: 10, interval_enabled: false, interval_minutes: 360, keep: 10 }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InstanceConfig {
    pub id: String,
    pub name: String,
    pub created: i64,
    pub core: CoreSpec,
    #[serde(default)]
    pub launch: LaunchSpec,
    #[serde(default)]
    pub auto_start: bool,
    #[serde(default)]
    pub auto_restart: bool,
    #[serde(default)]
    pub backup: BackupPolicy,
    /// set while an install task owns the directory
    #[serde(default)]
    pub installing: bool,
    /// an install was interrupted (failed / cancelled) and the files may be partial
    #[serde(default)]
    pub incomplete: bool,
    #[serde(default)]
    pub resource_pack: Option<String>,
    #[serde(default)]
    pub last_backup: i64,
}

#[derive(Serialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Stopped,
    Starting,
    Running,
    Stopping,
    Crashed,
}

pub struct Runtime {
    pub status: Status,
    pub stdin: Option<ChildStdin>,
    pub pid: Option<u32>,
    pub started_at: Option<Instant>,
    pub players: Option<(u32, u32)>,
    pub last_player_seen: Option<Instant>,
    pub dirty: bool,
    pub stop_requested: bool,
    pub mem_kb: u64,
    pub cpu: f32,
    cpu_prev: Option<(u64, Instant)>,
    pub restarts: u32,
}

pub struct Instance {
    pub cfg: RwLock<InstanceConfig>,
    pub rt: Mutex<Runtime>,
    pub log: StdMutex<VecDeque<String>>,
    pub console: broadcast::Sender<String>,
    pub dir: PathBuf,
}

const LOG_CAP: usize = 3000;

impl Instance {
    fn new(cfg: InstanceConfig, dir: PathBuf) -> Arc<Self> {
        let (tx, _) = broadcast::channel(1024);
        Arc::new(Self {
            cfg: RwLock::new(cfg),
            rt: Mutex::new(Runtime {
                status: Status::Stopped,
                stdin: None,
                pid: None,
                started_at: None,
                players: None,
                last_player_seen: None,
                dirty: false,
                stop_requested: false,
                mem_kb: 0,
                cpu: 0.0,
                cpu_prev: None,
                restarts: 0,
            }),
            log: StdMutex::new(VecDeque::with_capacity(LOG_CAP)),
            console: tx,
            dir,
        })
    }

    pub fn config(&self) -> InstanceConfig {
        self.cfg.read().unwrap().clone()
    }

    pub fn push_log(&self, line: String) {
        {
            let mut l = self.log.lock().unwrap();
            if l.len() >= LOG_CAP {
                l.pop_front();
            }
            l.push_back(line.clone());
        }
        let _ = self.console.send(line);
    }

    pub fn recent_log(&self) -> Vec<String> {
        self.log.lock().unwrap().iter().cloned().collect()
    }

    pub async fn send_command(&self, cmd: &str) -> Result<()> {
        let mut rt = self.rt.lock().await;
        let stdin = rt.stdin.as_mut().context("服务器未运行")?;
        stdin.write_all(format!("{}\n", cmd.trim_end()).as_bytes()).await?;
        stdin.flush().await?;
        drop(rt);
        self.push_log(format!("> {}", cmd.trim_end()));
        Ok(())
    }

    pub async fn status(&self) -> Status {
        self.rt.lock().await.status
    }

    /// Wait until a console line matching `needle` appears.
    pub async fn wait_line(&self, needle: &str, timeout: Duration) -> bool {
        let mut rx = self.console.subscribe();
        tokio::time::timeout(timeout, async {
            loop {
                match rx.recv().await {
                    Ok(l) if l.contains(needle) => return true,
                    Ok(_) | Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(_) => return false,
                }
            }
        })
        .await
        .unwrap_or(false)
    }
}

#[derive(Serialize)]
pub struct InstanceView {
    #[serde(flatten)]
    pub cfg: InstanceConfig,
    pub status: Status,
    pub players: Option<(u32, u32)>,
    pub uptime: Option<u64>,
    pub mem_kb: u64,
    pub cpu: f32,
    pub port: u16,
}

pub struct InstanceManager {
    pub map: RwLock<HashMap<String, Arc<Instance>>>,
}

impl InstanceManager {
    pub fn load(db: &Db, paths: &Paths) -> Result<Self> {
        let mut map = HashMap::new();
        for mut cfg in db.load_instances::<InstanceConfig>()? {
            cfg.installing = false;
            let dir = paths.instance(&cfg.id);
            let _ = std::fs::create_dir_all(&dir);
            map.insert(cfg.id.clone(), Instance::new(cfg, dir));
        }
        Ok(Self { map: RwLock::new(map) })
    }

    pub fn get(&self, id: &str) -> Option<Arc<Instance>> {
        self.map.read().unwrap().get(id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<Instance>> {
        self.map.read().unwrap().values().cloned().collect()
    }

    pub fn insert(&self, app: &AppState, cfg: InstanceConfig) -> Result<Arc<Instance>> {
        let dir = app.paths.instance(&cfg.id);
        std::fs::create_dir_all(&dir)?;
        app.db.save_instance(&cfg.id, &cfg)?;
        let inst = Instance::new(cfg.clone(), dir);
        self.map.write().unwrap().insert(cfg.id.clone(), inst.clone());
        Ok(inst)
    }

    pub fn remove(&self, id: &str) {
        self.map.write().unwrap().remove(id);
    }

    pub async fn stop_all(&self) {
        let all = self.all();
        let futs = all.into_iter().map(|i| async move {
            if i.status().await == Status::Running || i.status().await == Status::Starting {
                let _ = stop(&i, Duration::from_secs(60)).await;
            }
        });
        futures_util::future::join_all(futs).await;
    }
}

pub async fn view(i: &Instance) -> InstanceView {
    let rt = i.rt.lock().await;
    InstanceView {
        cfg: i.config(),
        status: rt.status,
        players: rt.players,
        uptime: rt.started_at.map(|t| t.elapsed().as_secs()),
        mem_kb: rt.mem_kb,
        cpu: rt.cpu,
        port: props::port(&i.dir),
    }
}

/// Owns the `installing` flag for the lifetime of an install task.
///
/// The task manager cancels by *dropping* the task future, so code placed after an
/// `.await` never runs on cancel. Clearing the flag in `Drop` is the only place that
/// is guaranteed to execute on success, failure, cancellation and panic alike.
pub struct InstallGuard {
    app: Arc<AppState>,
    inst: Arc<Instance>,
    cancel: tokio_util::sync::CancellationToken,
    /// a brand new instance: remove it entirely when the user cancels its creation
    fresh: bool,
    done: bool,
}

impl InstallGuard {
    pub fn new(app: Arc<AppState>, inst: Arc<Instance>, cancel: tokio_util::sync::CancellationToken, fresh: bool) -> Self {
        inst.cfg.write().unwrap().installing = true;
        Self { app, inst, cancel, fresh, done: false }
    }
    /// Mark success: the flag is cleared and the incomplete marker removed.
    pub fn finish(mut self) {
        self.done = true;
    }
}

impl Drop for InstallGuard {
    fn drop(&mut self) {
        let id = self.inst.config().id;
        if !self.done && self.fresh && self.cancel.is_cancelled() {
            // creation cancelled by the user: nothing worth keeping
            self.app.instances.remove(&id);
            let _ = self.app.db.delete_instance(&id);
            let dir = self.inst.dir.clone();
            std::thread::spawn(move || {
                let _ = std::fs::remove_dir_all(dir);
            });
            tracing::info!("instance {id} creation cancelled, removed");
            return;
        }
        {
            let mut c = self.inst.cfg.write().unwrap();
            c.installing = false;
            if self.done {
                c.incomplete = false;
            } else if self.fresh {
                c.incomplete = true;
            }
        }
        let _ = save(&self.app, &self.inst);
    }
}

pub fn save(app: &AppState, i: &Instance) -> Result<()> {
    let cfg = i.config();
    app.db.save_instance(&cfg.id, &cfg)
}

fn build_command(app: &AppState, i: &Instance) -> Result<Command> {
    let cfg = i.config();
    let l = &cfg.launch;
    let java = java::resolve(app, &l.java, l.java_major)?;
    let mut cmd = Command::new(&java);
    cmd.current_dir(&i.dir);
    cmd.arg(format!("-Xms{}", l.min_mem)).arg(format!("-Xmx{}", l.max_mem));
    for a in l.jvm_args.split_whitespace() {
        cmd.arg(a);
    }
    if let Some(f) = l.args_file.as_ref().filter(|s| !s.is_empty()) {
        if i.dir.join("user_jvm_args.txt").exists() {
            cmd.arg("@user_jvm_args.txt");
        }
        cmd.arg(format!("@{f}"));
    } else if let Some(j) = l.jar.as_ref().filter(|s| !s.is_empty()) {
        if !i.dir.join(j).exists() {
            bail!("找不到服务端核心 {j}");
        }
        cmd.arg("-jar").arg(j);
    } else {
        bail!("未配置启动方式");
    }
    for a in l.server_args.split_whitespace() {
        cmd.arg(a);
    }
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    #[cfg(unix)]
    cmd.process_group(0);
    cmd.kill_on_drop(false);
    Ok(cmd)
}

pub fn start(
    app: Arc<AppState>,
    i: Arc<Instance>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>> {
    Box::pin(start_inner(app, i))
}

async fn start_inner(app: Arc<AppState>, i: Arc<Instance>) -> Result<()> {
    let c = i.config();
    if c.installing {
        bail!("实例正在安装中");
    }
    if c.incomplete {
        bail!("实例安装未完成，请删除后重新创建，或在设置中重新安装核心");
    }
    drop(c);
    let mut rt = i.rt.lock().await;
    if matches!(rt.status, Status::Running | Status::Starting | Status::Stopping) {
        bail!("服务器已在运行");
    }
    let mut cmd = build_command(&app, &i)?;
    let mut child: Child = cmd.spawn().context("启动 Java 进程失败")?;
    rt.pid = child.id();
    rt.stdin = child.stdin.take();
    rt.status = Status::Starting;
    rt.started_at = Some(Instant::now());
    rt.stop_requested = false;
    rt.players = None;
    rt.cpu_prev = None;
    drop(rt);
    i.push_log(format!("[RimePanel] 服务器启动中 (pid {})", child.id().unwrap_or(0)));

    let out = child.stdout.take().unwrap();
    let err = child.stderr.take().unwrap();
    let i1 = i.clone();
    tokio::spawn(async move {
        let mut lines = BufReader::new(out).lines();
        while let Ok(Some(l)) = lines.next_line().await {
            if l.contains("Done (") && l.contains(")! For help") {
                let mut rt = i1.rt.lock().await;
                if rt.status == Status::Starting {
                    rt.status = Status::Running;
                }
            }
            i1.push_log(l);
        }
    });
    let i2 = i.clone();
    tokio::spawn(async move {
        let mut lines = BufReader::new(err).lines();
        while let Ok(Some(l)) = lines.next_line().await {
            i2.push_log(l);
        }
    });

    let i3 = i.clone();
    let app2 = app.clone();
    tokio::spawn(async move {
        let code = child.wait().await.ok();
        let (requested, crashed) = {
            let mut rt = i3.rt.lock().await;
            rt.stdin = None;
            rt.pid = None;
            rt.players = None;
            rt.started_at = None;
            rt.mem_kb = 0;
            rt.cpu = 0.0;
            let ok = code.map(|c| c.success()).unwrap_or(false);
            let crashed = !rt.stop_requested && !ok;
            rt.status = if crashed { Status::Crashed } else { Status::Stopped };
            (rt.stop_requested, crashed)
        };
        i3.push_log(format!(
            "[RimePanel] 进程已退出 ({})",
            code.map(|c| c.to_string()).unwrap_or_else(|| "unknown".into())
        ));
        if crashed && !requested && i3.config().auto_restart {
            let n = {
                let mut rt = i3.rt.lock().await;
                rt.restarts += 1;
                rt.restarts
            };
            if n <= 5 {
                let delay = Duration::from_secs(5 * n as u64);
                i3.push_log(format!("[RimePanel] {} 秒后自动重启 (第 {n} 次)", delay.as_secs()));
                tokio::time::sleep(delay).await;
                if let Err(e) = start(app2, i3.clone()).await {
                    i3.push_log(format!("[RimePanel] 自动重启失败: {e:#}"));
                }
            } else {
                i3.push_log("[RimePanel] 连续崩溃次数过多，停止自动重启".into());
            }
        }
    });
    Ok(())
}

/// Graceful stop: `stop` command, then SIGTERM, then SIGKILL the process group.
pub async fn stop(i: &Instance, grace: Duration) -> Result<()> {
    let pid = {
        let mut rt = i.rt.lock().await;
        if !matches!(rt.status, Status::Running | Status::Starting) {
            bail!("服务器未运行");
        }
        rt.status = Status::Stopping;
        rt.stop_requested = true;
        rt.restarts = 0;
        rt.pid
    };
    let _ = i.send_command("stop").await;
    if wait_exit(i, grace).await {
        return Ok(());
    }
    if let Some(pid) = pid {
        signal_group(pid, libc::SIGTERM);
        if wait_exit(i, Duration::from_secs(15)).await {
            return Ok(());
        }
        signal_group(pid, libc::SIGKILL);
    }
    wait_exit(i, Duration::from_secs(5)).await;
    Ok(())
}

pub async fn kill(i: &Instance) -> Result<()> {
    let pid = {
        let mut rt = i.rt.lock().await;
        rt.stop_requested = true;
        rt.pid
    };
    if let Some(pid) = pid {
        signal_group(pid, libc::SIGKILL);
    }
    Ok(())
}

fn signal_group(pid: u32, sig: i32) {
    #[cfg(unix)]
    unsafe {
        libc::kill(-(pid as i32), sig);
        libc::kill(pid as i32, sig);
    }
}

async fn wait_exit(i: &Instance, t: Duration) -> bool {
    let deadline = Instant::now() + t;
    while Instant::now() < deadline {
        if i.rt.lock().await.pid.is_none() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    false
}

fn proc_stats(pid: u32) -> Option<(u64, u64)> {
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    let rss = status
        .lines()
        .find(|l| l.starts_with("VmRSS:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let after = stat.rsplit_once(')')?.1;
    let f: Vec<&str> = after.split_whitespace().collect();
    let ticks = f.get(11)?.parse::<u64>().ok()? + f.get(12)?.parse::<u64>().ok()?;
    Some((rss, ticks))
}

/// Background monitor: player counts (SLP), memory/cpu, autostart.
pub fn start_background(app: Arc<AppState>) {
    let a = app.clone();
    tokio::spawn(async move {
        for i in a.instances.all() {
            if i.config().auto_start {
                if let Err(e) = start(a.clone(), i.clone()).await {
                    tracing::warn!("autostart {} failed: {e:#}", i.config().name);
                }
            }
        }
    });
    tokio::spawn(async move {
        let hz = unsafe { libc::sysconf(libc::_SC_CLK_TCK) }.max(1) as f32;
        let mut tick = tokio::time::interval(Duration::from_secs(5));
        loop {
            tick.tick().await;
            for i in app.instances.all() {
                let (status, pid) = {
                    let rt = i.rt.lock().await;
                    (rt.status, rt.pid)
                };
                if !matches!(status, Status::Running | Status::Starting) {
                    continue;
                }
                if let Some(pid) = pid {
                    if let Some((rss, ticks)) = proc_stats(pid) {
                        let mut rt = i.rt.lock().await;
                        rt.mem_kb = rss;
                        if let Some((pt, at)) = rt.cpu_prev {
                            let dt = at.elapsed().as_secs_f32().max(0.1);
                            rt.cpu = (ticks.saturating_sub(pt) as f32 / hz) / dt * 100.0;
                        }
                        rt.cpu_prev = Some((ticks, Instant::now()));
                    }
                }
                let port = props::port(&i.dir);
                if let Ok(s) = slp::ping(port).await {
                    let mut rt = i.rt.lock().await;
                    if rt.status == Status::Starting {
                        rt.status = Status::Running;
                    }
                    rt.players = Some((s.online, s.max));
                    if s.online > 0 {
                        rt.last_player_seen = Some(Instant::now());
                        rt.dirty = true;
                    } else if rt.last_player_seen.is_none() {
                        rt.last_player_seen = Some(Instant::now());
                    }
                }
            }
        }
    });
}

pub fn new_config(name: &str, core: CoreSpec) -> InstanceConfig {
    InstanceConfig {
        id: util::new_id(),
        name: name.to_string(),
        created: util::now(),
        core,
        launch: LaunchSpec::default(),
        auto_start: false,
        auto_restart: false,
        backup: BackupPolicy::default(),
        installing: false,
        incomplete: false,
        resource_pack: None,
        last_backup: 0,
    }
}
