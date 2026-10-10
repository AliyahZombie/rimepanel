//! World template library (copied into instances, never hard-linked).
use crate::{backup, db::WorldRow, instance::Instance, props, state::AppState, util};
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

fn finish(app: &AppState, staged: &Path, name: &str, source: &str) -> Result<String> {
    let id = util::new_id();
    let dst = app.paths.worlds().join(&id);
    std::fs::rename(staged, &dst).or_else(|_| util::copy_dir(staged, &dst))?;
    let _ = std::fs::remove_file(dst.join("session.lock"));
    app.db.world_insert(&WorldRow {
        id: id.clone(),
        name: name.into(),
        size: util::dir_size(&dst) as i64,
        source: source.into(),
        created: util::now(),
    })?;
    Ok(id)
}

/// Find the shallowest directory containing level.dat.
fn find_level_root(dir: &Path) -> Option<PathBuf> {
    let mut queue = vec![(dir.to_path_buf(), 0)];
    while let Some((d, depth)) = queue.first().cloned() {
        queue.remove(0);
        if d.join("level.dat").exists() {
            return Some(d);
        }
        if depth > 3 {
            continue;
        }
        if let Ok(rd) = std::fs::read_dir(&d) {
            for e in rd.flatten() {
                if e.path().is_dir() {
                    queue.push((e.path(), depth + 1));
                }
            }
        }
    }
    None
}

pub async fn from_instance(app: &AppState, inst: &Instance, name: &str) -> Result<String> {
    let level = props::level_name(&inst.dir);
    let src = inst.dir.join(&level);
    if !src.join("level.dat").exists() {
        bail!("实例中没有世界 {level}");
    }
    let staged = app.paths.tmp().join(util::new_id());
    backup::with_saves_paused(inst, || async {
        let (s, d) = (src.clone(), staged.clone());
        tokio::task::spawn_blocking(move || util::copy_dir(&s, &d)).await?
    })
    .await?;
    let r = finish(app, &staged, name, &format!("实例 {}", inst.config().name));
    let _ = std::fs::remove_dir_all(&staged);
    r
}

pub async fn from_archive(app: &AppState, archive: &Path, name: &str, source: &str) -> Result<String> {
    let staged = app.paths.tmp().join(util::new_id());
    let (a, s) = (archive.to_path_buf(), staged.clone());
    tokio::task::spawn_blocking(move || -> Result<()> {
        let n = a.to_string_lossy().to_lowercase();
        if n.ends_with(".zip") {
            util::unzip_to(&a, &s, None)?;
        } else {
            util::untar_to(&a, &s)?;
        }
        Ok(())
    })
    .await??;
    let root = find_level_root(&staged).context("压缩包中没有 level.dat")?;
    let r = finish(app, &root, name, source);
    let _ = std::fs::remove_dir_all(&staged);
    r
}

pub async fn from_backup(app: &AppState, inst_id: &str, backup_name: &str, name: &str) -> Result<String> {
    let file = app.paths.backups(inst_id).join(util::clean_rel(backup_name)?);
    from_archive(app, &file, name, &format!("备份 {backup_name}")).await
}

pub async fn apply(app: &AppState, inst: &Instance, world_id: &str) -> Result<()> {
    use crate::instance::Status;
    if !matches!(inst.status().await, Status::Stopped | Status::Crashed) {
        bail!("请先停止服务器");
    }
    let src = app.paths.worlds().join(util::clean_rel(world_id)?);
    if !src.is_dir() {
        bail!("世界模板不存在");
    }
    if !props::world_dirs(&inst.dir).is_empty() {
        backup::create(app, inst, "pre-world").await?;
    }
    let dir = inst.dir.clone();
    tokio::task::spawn_blocking(move || -> Result<()> {
        for d in props::world_dirs(&dir) {
            std::fs::remove_dir_all(dir.join(d))?;
        }
        util::copy_dir(&src, &dir.join(props::level_name(&dir)))
    })
    .await??;
    Ok(())
}

pub fn delete(app: &AppState, id: &str) -> Result<()> {
    let p = app.paths.worlds().join(util::clean_rel(id)?);
    if p.exists() {
        std::fs::remove_dir_all(p)?;
    }
    app.db.world_delete(id)
}
