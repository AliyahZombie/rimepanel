//! Content-addressed resource library. Files live once under
//! `library/objects/xx/<sha1>` and are hard-linked into instances.
use crate::{db::LibRow, state::AppState, util};
use anyhow::{bail, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};

pub const KINDS: &[&str] = &[
    "mod", "plugin", "resourcepack", "datapack", "modpack", "server", "installer", "other",
];

pub fn object_path(app: &AppState, sha1: &str) -> PathBuf {
    let s = sha1.to_lowercase();
    app.paths.objects().join(&s[..2.min(s.len())]).join(&s)
}

pub fn register(
    app: &AppState,
    sha1: &str,
    kind: &str,
    name: &str,
    source: &str,
    meta: serde_json::Value,
) -> Result<()> {
    let p = object_path(app, sha1);
    let size = std::fs::metadata(&p).map(|m| m.len() as i64).unwrap_or(0);
    app.db.lib_upsert(&LibRow {
        sha1: sha1.to_lowercase(),
        kind: kind.into(),
        name: name.into(),
        size,
        source: source.into(),
        meta,
        created: util::now(),
    })
}

/// Move a finished temp file into the object store.
pub fn import_tmp(
    app: &AppState,
    tmp: &Path,
    sha1: &str,
    kind: &str,
    name: &str,
    source: &str,
    meta: serde_json::Value,
) -> Result<()> {
    let dst = object_path(app, sha1);
    if dst.exists() {
        let _ = std::fs::remove_file(tmp);
    } else {
        std::fs::create_dir_all(dst.parent().unwrap())?;
        if std::fs::rename(tmp, &dst).is_err() {
            std::fs::copy(tmp, &dst)?;
            let _ = std::fs::remove_file(tmp);
        }
        // objects are read-only so an accidental in-place write fails loudly
        set_readonly(&dst);
    }
    register(app, sha1, kind, name, source, meta)
}

#[cfg(unix)]
fn set_readonly(p: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(p, std::fs::Permissions::from_mode(0o444));
}
#[cfg(not(unix))]
fn set_readonly(_p: &Path) {}

/// Import an existing file (keeps the original).
pub fn import_copy(app: &AppState, src: &Path, kind: &str, source: &str) -> Result<String> {
    let sha = util::sha1_file(src)?;
    let name = src.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let dst = object_path(app, &sha);
    if !dst.exists() {
        std::fs::create_dir_all(dst.parent().unwrap())?;
        let tmp = app.paths.tmp().join(util::new_id());
        // try hard link first (same filesystem, zero copy)
        if std::fs::hard_link(src, &tmp).is_err() {
            std::fs::copy(src, &tmp)?;
        }
        std::fs::rename(&tmp, &dst)?;
        set_readonly(&dst);
    }
    register(app, &sha, kind, &name, source, serde_json::json!({}))?;
    Ok(sha)
}

/// Place an object at `dest`: hard link, falling back to a copy (cross-device).
/// Returns true when a hard link was used.
pub fn link(app: &AppState, sha1: &str, dest: &Path) -> Result<bool> {
    let src = object_path(app, sha1);
    if !src.exists() {
        bail!("资源库中缺少对象 {sha1}");
    }
    if let Some(p) = dest.parent() {
        std::fs::create_dir_all(p)?;
    }
    if let Ok(m) = std::fs::symlink_metadata(dest) {
        if m.is_dir() {
            bail!("目标是目录: {}", dest.display());
        }
        std::fs::remove_file(dest)?;
    }
    match std::fs::hard_link(&src, dest) {
        Ok(_) => Ok(true),
        Err(_) => {
            std::fs::copy(&src, dest)?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = std::fs::set_permissions(dest, std::fs::Permissions::from_mode(0o644));
            }
            Ok(false)
        }
    }
}

#[derive(Serialize)]
pub struct LibItem {
    #[serde(flatten)]
    pub row: LibRow,
    pub links: u64,
}

pub fn list(app: &AppState, kind: Option<&str>) -> Vec<LibItem> {
    app.db
        .lib_list(kind)
        .into_iter()
        .map(|row| {
            let links = std::fs::metadata(object_path(app, &row.sha1))
                .map(|m| util::nlink(&m).saturating_sub(1))
                .unwrap_or(0);
            LibItem { row, links }
        })
        .collect()
}

#[derive(Serialize)]
pub struct Stats {
    pub objects: usize,
    pub bytes: u64,
    pub saved_bytes: u64,
    pub worlds: usize,
}

pub fn stats(app: &AppState) -> Stats {
    let mut bytes = 0;
    let mut saved = 0;
    let rows = app.db.lib_list(None);
    for r in &rows {
        if let Ok(m) = std::fs::metadata(object_path(app, &r.sha1)) {
            bytes += m.len();
            saved += m.len() * util::nlink(&m).saturating_sub(1);
        }
    }
    Stats { objects: rows.len(), bytes, saved_bytes: saved, worlds: app.db.world_list().len() }
}

pub fn delete(app: &AppState, sha1: &str) -> Result<()> {
    // Removing the object never breaks instances: their hard links keep the data.
    let p = object_path(app, sha1);
    if p.exists() {
        std::fs::remove_file(&p)?;
    }
    app.db.lib_delete(sha1)
}

/// Remove objects no instance links to. Server jars are kept (jar archive).
pub fn gc(app: &AppState, include_servers: bool) -> Result<(usize, u64)> {
    let mut n = 0;
    let mut freed = 0;
    for r in app.db.lib_list(None) {
        if !include_servers && (r.kind == "server" || r.kind == "installer") {
            continue;
        }
        let p = object_path(app, &r.sha1);
        match std::fs::metadata(&p) {
            Ok(m) if util::nlink(&m) <= 1 => {
                freed += m.len();
                delete(app, &r.sha1)?;
                n += 1;
            }
            Err(_) => {
                app.db.lib_delete(&r.sha1)?;
            }
            _ => {}
        }
    }
    Ok((n, freed))
}

/// Guess library kind from a filename.
pub fn guess_kind(name: &str, hint: Option<&str>) -> String {
    if let Some(h) = hint {
        if KINDS.contains(&h) {
            return h.into();
        }
    }
    let l = name.to_lowercase();
    if l.ends_with(".mrpack") {
        "modpack".into()
    } else if l.ends_with(".jar") {
        "mod".into()
    } else if l.ends_with(".zip") {
        "resourcepack".into()
    } else {
        "other".into()
    }
}
