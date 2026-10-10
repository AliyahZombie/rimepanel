//! World backups (tar.zst) with idle/interval automatic scheduling.
use crate::{
    instance::{self, Instance, Status},
    props,
    state::AppState,
    util,
};
use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::{path::Path, sync::Arc, time::Duration};

#[derive(Serialize)]
pub struct BackupInfo {
    pub name: String,
    pub size: u64,
    pub created: i64,
    pub tag: String,
}

pub fn list(app: &AppState, id: &str) -> Vec<BackupInfo> {
    let mut v = Vec::new();
    if let Ok(rd) = std::fs::read_dir(app.paths.backups(id)) {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            if !name.ends_with(".tar.zst") {
                continue;
            }
            let stem = name.trim_end_matches(".tar.zst");
            let (ts, tag) = stem.split_once('-').unwrap_or((stem, "manual"));
            let m = e.metadata().ok();
            v.push(BackupInfo {
                created: ts.parse().unwrap_or_else(|_| m.as_ref().map(util::mtime).unwrap_or(0)),
                size: m.map(|m| m.len()).unwrap_or(0),
                tag: tag.to_string(),
                name,
            });
        }
    }
    v.sort_by(|a, b| b.created.cmp(&a.created));
    v
}

/// Flush the world to disk and pause autosave while `f` runs.
pub async fn with_saves_paused<T, F, Fut>(inst: &Instance, f: F) -> Result<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let running = inst.status().await == Status::Running;
    if running {
        let _ = inst.send_command("save-off").await;
        let waiter = inst.wait_line("Saved the game", Duration::from_secs(90));
        let _ = inst.send_command("save-all flush").await;
        if !waiter.await {
            inst.push_log("[RimePanel] 等待存档写入超时，继续备份".into());
        }
    }
    let r = f().await;
    if running {
        let _ = inst.send_command("save-on").await;
    }
    r
}

pub async fn create(app: &AppState, inst: &Instance, tag: &str) -> Result<String> {
    let dirs = props::world_dirs(&inst.dir);
    if dirs.is_empty() {
        bail!("没有找到世界目录");
    }
    let id = inst.config().id;
    let out_dir = app.paths.backups(&id);
    std::fs::create_dir_all(&out_dir)?;
    let name = format!("{}-{}.tar.zst", util::now(), tag);
    let out = out_dir.join(&name);
    let base = inst.dir.clone();
    with_saves_paused(inst, || async {
        let out2 = out.clone();
        tokio::task::spawn_blocking(move || write_archive(&base, &dirs, &out2)).await?
    })
    .await
    .map_err(|e| {
        let _ = std::fs::remove_file(&out);
        e
    })?;
    {
        let mut c = inst.cfg.write().unwrap();
        c.last_backup = util::now();
    }
    instance::save(app, inst)?;
    prune(app, inst);
    inst.push_log(format!("[RimePanel] 备份完成 {name}"));
    Ok(name)
}

fn write_archive(base: &Path, dirs: &[String], out: &Path) -> Result<()> {
    let tmp = out.with_extension("partial");
    let f = std::fs::File::create(&tmp)?;
    let enc = zstd::Encoder::new(f, 3)?;
    let mut tar = tar::Builder::new(enc);
    tar.follow_symlinks(false);
    for d in dirs {
        tar.append_dir_all(d, base.join(d))?;
    }
    let enc = tar.into_inner()?;
    enc.finish()?;
    std::fs::rename(&tmp, out)?;
    Ok(())
}

fn prune(app: &AppState, inst: &Instance) {
    let cfg = inst.config();
    let keep = cfg.backup.keep.max(1) as usize;
    let autos: Vec<BackupInfo> = list(app, &cfg.id).into_iter().filter(|b| b.tag == "auto").collect();
    for b in autos.into_iter().skip(keep) {
        let _ = std::fs::remove_file(app.paths.backups(&cfg.id).join(&b.name));
    }
}

pub async fn restore(app: &AppState, inst: &Instance, name: &str) -> Result<()> {
    if !matches!(inst.status().await, Status::Stopped | Status::Crashed) {
        bail!("请先停止服务器");
    }
    let id = inst.config().id;
    let file = app.paths.backups(&id).join(util::clean_rel(name)?);
    if !file.exists() {
        bail!("备份不存在");
    }
    if !props::world_dirs(&inst.dir).is_empty() {
        create(app, inst, "pre-restore").await.context("恢复前快照失败")?;
    }
    let dir = inst.dir.clone();
    tokio::task::spawn_blocking(move || -> Result<()> {
        for d in props::world_dirs(&dir) {
            std::fs::remove_dir_all(dir.join(d))?;
        }
        util::untar_to(&file, &dir)
    })
    .await??;
    inst.push_log(format!("[RimePanel] 已从 {name} 恢复"));
    Ok(())
}

pub fn delete(app: &AppState, id: &str, name: &str) -> Result<()> {
    std::fs::remove_file(app.paths.backups(id).join(util::clean_rel(name)?))?;
    Ok(())
}

/// Scheduler: idle backups (nobody online for N minutes after activity) and
/// interval backups while the server runs.
pub fn start_scheduler(app: Arc<AppState>) {
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_secs(30));
        loop {
            tick.tick().await;
            for inst in app.instances.all() {
                let cfg = inst.config();
                let (status, players, last_seen, dirty) = {
                    let rt = inst.rt.lock().await;
                    (rt.status, rt.players, rt.last_player_seen, rt.dirty)
                };
                if status != Status::Running {
                    continue;
                }
                let online = players.map(|p| p.0).unwrap_or(0);
                let mut due = false;
                if cfg.backup.idle_enabled && dirty && online == 0 {
                    if let Some(t) = last_seen {
                        if t.elapsed() >= Duration::from_secs(cfg.backup.idle_minutes as u64 * 60) {
                            due = true;
                        }
                    }
                }
                if cfg.backup.interval_enabled
                    && util::now() - cfg.last_backup >= cfg.backup.interval_minutes as i64 * 60
                    && (dirty || online > 0)
                {
                    due = true;
                }
                if !due {
                    continue;
                }
                let a = app.clone();
                let i = inst.clone();
                let tid = app.tasks.spawn("backup", format!("自动备份 {}", cfg.name), Some(cfg.id.clone()), move |_ctx| async move {
                    let n = create(&a, &i, "auto").await?;
                    Ok(n)
                });
                let ok = app.tasks.wait(&tid).await.map(|t| t.state == crate::tasks::TaskState::Done).unwrap_or(false);
                if ok {
                    let mut rt = inst.rt.lock().await;
                    // stays clean until someone joins again
                    rt.dirty = online > 0;
                }
            }
        }
    });
}
