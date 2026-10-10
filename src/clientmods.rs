//! Detection of client-only mods. Layered: Modrinth metadata -> jar metadata
//! -> known list -> crash log analysis. Detected jars are moved to
//! `mods-disabled/` (never deleted) so the user can restore them.
use crate::{modrinth, state::AppState, util};
use anyhow::Result;
use serde::Serialize;
use std::{io::Read, path::Path};

#[derive(Serialize, Clone, Debug)]
pub struct Finding {
    pub file: String,
    pub mod_id: String,
    pub reason: String,
    /// "high" or "medium"
    pub confidence: String,
}

const KNOWN_CLIENT: &[&str] = &[
    "optifine", "sodium", "iris", "oculus", "embeddium", "rubidium", "magnesium", "sodiumextra",
    "sodium-extra", "reeses_sodium_options", "reeses-sodium-options", "indium", "xaerominimap",
    "xaeroworldmap", "xaeroplus", "modmenu", "betterf3", "dynamic_fps", "dynamicfps", "fpsreducer",
    "lambdynlights", "citresewn", "continuity", "notenoughanimations", "firstperson",
    "firstpersonmod", "skinlayers3d", "3dskinlayers", "sodiumdynamiclights", "entity_texture_features",
    "entity_model_features", "controlling", "mousetweaks", "toastcontrol", "betterpingdisplay",
    "legendarytooltips", "itemphysiclite", "cullleaves", "smoothboot", "immediatelyfast",
    "fancymenu", "drippyloadingscreen", "zoomify", "okzoomer", "ok_zoomer", "chat_heads",
    "chatheads", "enhancedblockentities", "betterbeds", "cit_resewn", "ryoamiclights",
    "visuality", "shulkerboxtooltip_client", "particular", "fallingleaves", "wakes",
    "presencefootsteps", "soundphysics", "sound_physics_remastered", "auudio", "loadmyresources",
    "betteradvancements", "jeresources_client", "blur", "screenshot_viewer", "rrls",
    "durabilitytooltip", "inventoryprofilesnext", "mixintrace", "nochatreports_client",
];

fn read_entry(z: &mut zip::ZipArchive<std::fs::File>, name: &str) -> Option<String> {
    let mut e = z.by_name(name).ok()?;
    let mut s = String::new();
    e.read_to_string(&mut s).ok()?;
    Some(s)
}

/// Inspect jar metadata. Returns (mod id, client-only reason).
pub fn inspect_jar(path: &Path) -> (String, Option<(String, String)>) {
    let Ok(f) = std::fs::File::open(path) else { return (String::new(), None) };
    let Ok(mut z) = zip::ZipArchive::new(f) else { return (String::new(), None) };
    if let Some(s) = read_entry(&mut z, "fabric.mod.json") {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
            let id = v["id"].as_str().unwrap_or("").to_string();
            if v["environment"].as_str() == Some("client") {
                return (id, Some(("fabric.mod.json 声明 environment=client".into(), "high".into())));
            }
            return (id, None);
        }
    }
    if let Some(s) = read_entry(&mut z, "quilt.mod.json") {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
            let id = v["quilt_loader"]["id"].as_str().unwrap_or("").to_string();
            if v["minecraft"]["environment"].as_str() == Some("client") {
                return (id, Some(("quilt.mod.json 声明 environment=client".into(), "high".into())));
            }
            return (id, None);
        }
    }
    for toml_name in ["META-INF/neoforge.mods.toml", "META-INF/mods.toml"] {
        if let Some(s) = read_entry(&mut z, toml_name) {
            let Ok(v) = s.parse::<toml::Value>() else { continue };
            let first = v.get("mods").and_then(|m| m.as_array()).and_then(|a| a.first());
            let id = first
                .and_then(|m| m.get("modId"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if v.get("clientSideOnly").and_then(|x| x.as_bool()) == Some(true) {
                return (id, Some((format!("{toml_name} 声明 clientSideOnly"), "high".into())));
            }
            // dependency on minecraft with side=CLIENT
            if let Some(deps) = v.get("dependencies").and_then(|d| d.get(&id)).and_then(|d| d.as_array()) {
                for d in deps {
                    let mid = d.get("modId").and_then(|x| x.as_str()).unwrap_or("");
                    let side = d.get("side").and_then(|x| x.as_str()).unwrap_or("");
                    if (mid == "minecraft" || mid == "forge" || mid == "neoforge") && side == "CLIENT" {
                        return (id, Some((format!("{toml_name} 依赖 side=CLIENT"), "medium".into())));
                    }
                }
            }
            let display = first.and_then(|m| m.get("displayTest")).and_then(|x| x.as_str()).unwrap_or("");
            if display == "IGNORE_ALL_VERSION" {
                return (id, Some(("displayTest=IGNORE_ALL_VERSION（通常是纯客户端模组）".into(), "medium".into())));
            }
            return (id, None);
        }
    }
    // OptiFine has no loader metadata
    if z.by_name("optifine/Config.class").is_ok() || z.by_name("net/optifine/Config.class").is_ok() {
        return ("optifine".into(), Some(("OptiFine".into(), "high".into())));
    }
    (String::new(), None)
}

pub async fn scan(app: &AppState, dir: &Path) -> Result<Vec<Finding>> {
    let mods = dir.join("mods");
    let mut jars = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&mods) {
        for e in rd.flatten() {
            let n = e.file_name().to_string_lossy().to_string();
            if n.to_lowercase().ends_with(".jar") {
                jars.push(e.path());
            }
        }
    }
    let jars2 = jars.clone();
    let hashed: Vec<(std::path::PathBuf, String)> = tokio::task::spawn_blocking(move || {
        jars2
            .into_iter()
            .filter_map(|p| util::sha1_file(&p).ok().map(|h| (p, h)))
            .collect()
    })
    .await?;
    let hashes: Vec<String> = hashed.iter().map(|(_, h)| h.clone()).collect();
    let remote = modrinth::lookup_hashes(app, &hashes).await.unwrap_or_default();
    let crash_suspects = crash_suspects(dir);

    let mut out = Vec::new();
    for (path, sha) in hashed {
        let file = path.file_name().unwrap().to_string_lossy().to_string();
        let p2 = path.clone();
        let (mod_id, local) = tokio::task::spawn_blocking(move || inspect_jar(&p2)).await?;
        if let Some(p) = remote.get(&sha) {
            if p["server_side"].as_str() == Some("unsupported") {
                out.push(Finding {
                    file,
                    mod_id: mod_id.clone(),
                    reason: format!("Modrinth: {} 不支持服务端", p["title"].as_str().unwrap_or("")),
                    confidence: "high".into(),
                });
                continue;
            }
            if p["server_side"].as_str().is_some() {
                // Modrinth says the server can run it — trust that over heuristics
                continue;
            }
        }
        if let Some((reason, conf)) = local {
            out.push(Finding { file, mod_id, reason, confidence: conf });
            continue;
        }
        let lid = mod_id.to_lowercase();
        let lfile = file.to_lowercase();
        if (!lid.is_empty() && KNOWN_CLIENT.contains(&lid.as_str()))
            || KNOWN_CLIENT.iter().any(|k| lfile.starts_with(&format!("{k}-")) || lfile.starts_with(&format!("{k}_")))
        {
            out.push(Finding { file, mod_id, reason: "已知客户端模组名单".into(), confidence: "medium".into() });
            continue;
        }
        if crash_suspects.iter().any(|s| s == &file) {
            out.push(Finding {
                file,
                mod_id,
                reason: "崩溃日志显示其加载了客户端类".into(),
                confidence: "medium".into(),
            });
        }
    }
    Ok(out)
}

/// Look in the latest crash report / log for "invalid dist DEDICATED_SERVER" or
/// client class loading errors and collect jar names mentioned nearby.
pub fn crash_suspects(dir: &Path) -> Vec<String> {
    let mut texts = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir.join("crash-reports")) {
        let mut v: Vec<_> = rd.flatten().collect();
        v.sort_by_key(|e| e.metadata().map(|m| util::mtime(&m)).unwrap_or(0));
        if let Some(last) = v.last() {
            if let Ok(s) = std::fs::read_to_string(last.path()) {
                texts.push(s);
            }
        }
    }
    if let Ok(s) = std::fs::read_to_string(dir.join("logs/latest.log")) {
        texts.push(s);
    }
    let re = regex_lite::Regex::new(r"([A-Za-z0-9_\-\.\+ ]+\.jar)").unwrap();
    let mut out = Vec::new();
    for t in texts {
        if !(t.contains("invalid dist DEDICATED_SERVER")
            || t.contains("net/minecraft/client")
            || t.contains("net.minecraft.client"))
        {
            continue;
        }
        for line in t.lines() {
            if line.contains("client") || line.contains("Mod File") || line.contains("mods/") {
                for c in re.captures_iter(line) {
                    let n = c[1].trim().rsplit('/').next().unwrap_or("").to_string();
                    if !n.is_empty() && dir.join("mods").join(&n).exists() && !out.contains(&n) {
                        out.push(n);
                    }
                }
            }
        }
    }
    out
}

pub fn disable(dir: &Path, files: &[String]) -> Result<usize> {
    let to = dir.join("mods-disabled");
    std::fs::create_dir_all(&to)?;
    let mut n = 0;
    for f in files {
        let name = util::clean_rel(f)?;
        let src = dir.join("mods").join(&name);
        if src.exists() {
            std::fs::rename(&src, to.join(&name))?;
            n += 1;
        }
    }
    Ok(n)
}

pub fn enable(dir: &Path, files: &[String]) -> Result<usize> {
    let mut n = 0;
    std::fs::create_dir_all(dir.join("mods"))?;
    for f in files {
        let name = util::clean_rel(f)?;
        let src = dir.join("mods-disabled").join(&name);
        if src.exists() {
            std::fs::rename(&src, dir.join("mods").join(&name))?;
            n += 1;
        }
    }
    Ok(n)
}
