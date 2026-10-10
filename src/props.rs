//! server.properties read/update preserving comments and order.
use anyhow::Result;
use std::{collections::BTreeMap, path::Path};

pub fn read(dir: &Path) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    if let Ok(s) = std::fs::read_to_string(dir.join("server.properties")) {
        for line in s.lines() {
            let t = line.trim();
            if t.starts_with('#') || t.is_empty() {
                continue;
            }
            if let Some((k, v)) = t.split_once('=') {
                m.insert(k.trim().to_string(), unescape(v.trim()));
            }
        }
    }
    m
}

fn unescape(v: &str) -> String {
    v.replace("\\:", ":").replace("\\=", "=").replace("\\\\", "\\")
}
fn escape(v: &str) -> String {
    v.replace('\\', "\\\\").replace(':', "\\:").replace('=', "\\=")
}

pub fn update(dir: &Path, changes: &BTreeMap<String, String>) -> Result<()> {
    let path = dir.join("server.properties");
    let old = std::fs::read_to_string(&path).unwrap_or_default();
    let mut seen = std::collections::HashSet::new();
    let mut out = String::new();
    for line in old.lines() {
        let t = line.trim();
        if !t.starts_with('#') {
            if let Some((k, _)) = t.split_once('=') {
                let k = k.trim();
                if let Some(v) = changes.get(k) {
                    out.push_str(&format!("{k}={}\n", escape(v)));
                    seen.insert(k.to_string());
                    continue;
                }
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    for (k, v) in changes {
        if !seen.contains(k) {
            out.push_str(&format!("{k}={}\n", escape(v)));
        }
    }
    crate::util::atomic_write(&path, out.as_bytes())
}

pub fn port(dir: &Path) -> u16 {
    read(dir)
        .get("server-port")
        .and_then(|p| p.parse().ok())
        .unwrap_or(25565)
}

pub fn level_name(dir: &Path) -> String {
    read(dir)
        .get("level-name")
        .cloned()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "world".into())
}

/// World directories: `world`, plus Bukkit-style `world_nether` / `world_the_end`.
pub fn world_dirs(dir: &Path) -> Vec<String> {
    let level = level_name(dir);
    let mut v = Vec::new();
    for name in [level.clone(), format!("{level}_nether"), format!("{level}_the_end")] {
        if dir.join(&name).is_dir() {
            v.push(name);
        }
    }
    v
}
