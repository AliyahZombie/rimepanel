use anyhow::{anyhow, bail, Result};
use sha1::{Digest, Sha1};
use std::io::Read;
use std::path::{Component, Path, PathBuf};

pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn new_id() -> String {
    use rand::Rng;
    let mut r = rand::thread_rng();
    (0..10)
        .map(|_| {
            let c = r.gen_range(0..36u8);
            (if c < 10 { b'0' + c } else { b'a' + c - 10 }) as char
        })
        .collect()
}

pub fn random_token() -> String {
    use rand::RngCore;
    let mut b = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut b);
    hex::encode(b)
}

/// Normalise a user supplied relative path; rejects absolute paths and `..`.
pub fn clean_rel(p: &str) -> Result<PathBuf> {
    let mut out = PathBuf::new();
    for c in Path::new(p.trim_start_matches(['/', '\\'])).components() {
        match c {
            Component::Normal(s) => out.push(s),
            Component::CurDir => {}
            _ => bail!("非法路径"),
        }
    }
    Ok(out)
}

/// Join `rel` onto `root` and make sure the result (following symlinks of
/// existing ancestors) stays inside `root`.
pub fn safe_join(root: &Path, rel: &str) -> Result<PathBuf> {
    let rel = clean_rel(rel)?;
    let root_c = root.canonicalize()?;
    let full = root_c.join(&rel);
    let mut probe = full.clone();
    loop {
        if probe.exists() {
            let c = probe.canonicalize()?;
            if !c.starts_with(&root_c) {
                bail!("路径越界");
            }
            break;
        }
        if !probe.pop() {
            break;
        }
    }
    Ok(full)
}

pub fn sha1_file(path: &Path) -> Result<String> {
    let mut f = std::fs::File::open(path)?;
    let mut h = Sha1::new();
    let mut buf = vec![0u8; 256 * 1024];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(hex::encode(h.finalize()))
}

/// Write via temp file + rename. Rename replaces the directory entry, so a
/// hard-linked file's other links (the resource library) are never modified.
pub fn atomic_write(path: &Path, data: &[u8]) -> Result<()> {
    let dir = path.parent().ok_or_else(|| anyhow!("no parent"))?;
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(format!(".{}.mcp-tmp", new_id()));
    std::fs::write(&tmp, data)?;
    if let Ok(meta) = std::fs::metadata(path) {
        let _ = std::fs::set_permissions(&tmp, meta.permissions());
    }
    std::fs::rename(&tmp, path).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        e.into()
    })
}

pub fn dir_size(p: &Path) -> u64 {
    let mut total = 0;
    if let Ok(rd) = std::fs::read_dir(p) {
        for e in rd.flatten() {
            if let Ok(m) = e.metadata() {
                if m.is_dir() {
                    total += dir_size(&e.path());
                } else {
                    total += m.len();
                }
            }
        }
    }
    total
}

pub fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for e in std::fs::read_dir(src)? {
        let e = e?;
        let ft = e.file_type()?;
        let to = dst.join(e.file_name());
        if ft.is_dir() {
            copy_dir(&e.path(), &to)?;
        } else if ft.is_file() {
            std::fs::copy(e.path(), &to)?;
        }
    }
    Ok(())
}

#[cfg(unix)]
pub fn nlink(m: &std::fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt;
    m.nlink()
}
#[cfg(not(unix))]
pub fn nlink(_m: &std::fs::Metadata) -> u64 {
    1
}

pub fn mtime(m: &std::fs::Metadata) -> i64 {
    m.modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Extract a zip archive into `dest`, refusing entries that escape it.
pub fn unzip_to(archive: &Path, dest: &Path, strip_prefix: Option<&str>) -> Result<usize> {
    let f = std::fs::File::open(archive)?;
    let mut z = zip::ZipArchive::new(f)?;
    let mut n = 0;
    for i in 0..z.len() {
        let mut ent = z.by_index(i)?;
        let Some(name) = ent.enclosed_name() else { continue };
        let name = name.to_path_buf();
        let rel = match strip_prefix {
            Some(p) => match name.strip_prefix(p) {
                Ok(r) => r.to_path_buf(),
                Err(_) => continue,
            },
            None => name,
        };
        if rel.as_os_str().is_empty() {
            continue;
        }
        let out = dest.join(&rel);
        if ent.is_dir() {
            std::fs::create_dir_all(&out)?;
        } else {
            if let Some(p) = out.parent() {
                std::fs::create_dir_all(p)?;
            }
            let _ = std::fs::remove_file(&out);
            let mut w = std::fs::File::create(&out)?;
            std::io::copy(&mut ent, &mut w)?;
            n += 1;
        }
    }
    Ok(n)
}

/// Extract a tar (optionally gz/zst compressed) archive safely.
pub fn untar_to(archive: &Path, dest: &Path) -> Result<()> {
    let f = std::fs::File::open(archive)?;
    let name = archive.to_string_lossy().to_lowercase();
    let reader: Box<dyn Read> = if name.ends_with(".gz") || name.ends_with(".tgz") {
        Box::new(flate2::read::GzDecoder::new(f))
    } else if name.ends_with(".zst") {
        Box::new(zstd::Decoder::new(f)?)
    } else {
        Box::new(f)
    };
    let mut ar = tar::Archive::new(reader);
    ar.set_preserve_permissions(true);
    std::fs::create_dir_all(dest)?;
    for e in ar.entries()? {
        let mut e = e?;
        // unpack_in refuses paths escaping `dest`
        e.unpack_in(dest)?;
    }
    Ok(())
}

pub fn human_size(n: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut v = n as f64;
    let mut i = 0;
    while v >= 1024.0 && i < units.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    format!("{:.1} {}", v, units[i])
}
