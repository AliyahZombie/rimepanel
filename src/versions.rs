//! Version lists for every supported server core.
use crate::{net, state::AppState};
use anyhow::{bail, Context, Result};
use serde_json::{json, Value};

pub const MANIFEST: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
const TTL: i64 = 3600;



/// `[{id, type}]` newest first.
pub async fn game_versions(app: &AppState, kind: &str) -> Result<Value> {
    let list: Vec<Value> = match kind {
        "vanilla" => {
            let m = net::get_json_cached(app, MANIFEST, TTL).await?;
            m["versions"]
                .as_array()
                .context("manifest")?
                .iter()
                .map(|v| json!({"id": v["id"], "type": v["type"]}))
                .collect()
        }
        "fabric" => {
            let m = net::get_json_cached(app, "https://meta.fabricmc.net/v2/versions/game", TTL).await?;
            stable_list(&m)
        }
        "quilt" => {
            let m = net::get_json_cached(app, "https://meta.quiltmc.org/v3/versions/game", TTL).await?;
            stable_list(&m)
        }
        "paper" => {
            let m = net::get_json_cached(app, "https://fill.papermc.io/v3/projects/paper", TTL).await?;
            let mut out = Vec::new();
            if let Some(obj) = m["versions"].as_object() {
                for (_, arr) in obj {
                    for v in arr.as_array().into_iter().flatten() {
                        let id = v.as_str().unwrap_or_default();
                        let t = if id.contains("-pre") || id.contains("-rc") { "snapshot" } else { "release" };
                        out.push(json!({"id": id, "type": t}));
                    }
                }
            }
            out.sort_by(|a, b| cmp_release(b["id"].as_str().unwrap_or(""), a["id"].as_str().unwrap_or("")));
            out
        }
        "purpur" => {
            let m = net::get_json_cached(app, "https://api.purpurmc.org/v2/purpur", TTL).await?;
            let mut v: Vec<Value> = m["versions"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|x| json!({"id": x, "type": "release"}))
                .collect();
            v.reverse();
            v
        }
        "forge" => {
            let m = forge_meta(app).await?;
            let mut keys: Vec<&String> = m.as_object().context("forge meta")?.keys().collect();
            keys.sort_by(|a, b| cmp_ver(b, a));
            keys.into_iter().map(|k| json!({"id": k, "type": "release"})).collect()
        }
        "neoforge" => {
            let all = neoforge_all(app).await?;
            let mut mcs: Vec<String> = Vec::new();
            for v in &all {
                if let Some(mc) = neoforge_mc(v) {
                    if !mcs.contains(&mc) {
                        mcs.push(mc);
                    }
                }
            }
            mcs.sort_by(|a, b| cmp_ver(b, a));
            mcs.into_iter().map(|k| json!({"id": k, "type": "release"})).collect()
        }
        _ => bail!("未知核心类型 {kind}"),
    };
    Ok(Value::Array(list))
}

/// Loader / build versions for a given game version, newest first.
pub async fn loader_versions(app: &AppState, kind: &str, mc: &str) -> Result<Value> {
    let list: Vec<Value> = match kind {
        "fabric" => {
            let m = net::get_json_cached(app, "https://meta.fabricmc.net/v2/versions/loader", TTL).await?;
            stable_list(&m)
        }
        "quilt" => {
            let m = net::get_json_cached(app, "https://meta.quiltmc.org/v3/versions/loader", TTL).await?;
            m.as_array()
                .into_iter()
                .flatten()
                .map(|v| {
                    let id = v["version"].as_str().unwrap_or("");
                    json!({"id": id, "type": if id.contains("beta") {"beta"} else {"release"}})
                })
                .collect()
        }
        "paper" => {
            let url = format!("https://fill.papermc.io/v3/projects/paper/versions/{mc}/builds");
            let m = net::get_json_cached(app, &url, 600).await?;
            m.as_array()
                .into_iter()
                .flatten()
                .map(|b| json!({"id": b["id"].to_string(), "type": b["channel"].as_str().unwrap_or("").to_lowercase()}))
                .collect()
        }
        "purpur" => {
            let m = net::get_json_cached(app, &format!("https://api.purpurmc.org/v2/purpur/{mc}"), 600).await?;
            let mut v: Vec<Value> = m["builds"]["all"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|b| json!({"id": b, "type": "release"}))
                .collect();
            v.reverse();
            v
        }
        "forge" => {
            let m = forge_meta(app).await?;
            let mut v: Vec<Value> = m[mc]
                .as_array()
                .into_iter()
                .flatten()
                .map(|x| json!({"id": x, "type": "release"}))
                .collect();
            v.reverse();
            v
        }
        "neoforge" => {
            let all = neoforge_all(app).await?;
            let mut v: Vec<Value> = all
                .iter()
                .filter(|x| neoforge_mc(x).as_deref() == Some(mc))
                .map(|x| json!({"id": x, "type": if x.contains("beta") {"beta"} else {"release"}}))
                .collect();
            v.reverse();
            v
        }
        _ => vec![],
    };
    Ok(Value::Array(list))
}

fn stable_list(m: &Value) -> Vec<Value> {
    m.as_array()
        .into_iter()
        .flatten()
        .map(|v| {
            json!({
                "id": v["version"],
                "type": if v["stable"].as_bool().unwrap_or(true) { "release" } else { "snapshot" }
            })
        })
        .collect()
}

async fn forge_meta(app: &AppState) -> Result<Value> {
    net::get_json_cached(
        app,
        "https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json",
        TTL,
    )
    .await
}

async fn neoforge_all(app: &AppState) -> Result<Vec<String>> {
    let m = net::get_json_cached(
        app,
        "https://maven.neoforged.net/api/maven/versions/releases/net/neoforged/neoforge",
        TTL,
    )
    .await?;
    Ok(m["versions"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|v| v.as_str().map(String::from))
        .collect())
}

/// NeoForge version -> Minecraft version.
/// `21.1.77` -> `1.21.1`, `21.0.x` -> `1.21`, `26.1.0.5` -> `26.1`, `26.1.2.3` -> `26.1.2`.
pub fn neoforge_mc(v: &str) -> Option<String> {
    let base = v.split('-').next()?;
    let p: Vec<u32> = base.split('.').map(|x| x.parse().ok()).collect::<Option<Vec<_>>>()?;
    match p.as_slice() {
        [a, b, c, _d] if *a >= 25 => Some(if *c == 0 { format!("{a}.{b}") } else { format!("{a}.{b}.{c}") }),
        [a, b, _] if *a >= 20 && *a < 25 => Some(if *b == 0 { format!("1.{a}") } else { format!("1.{a}.{b}") }),
        _ => None,
    }
}

/// Compare `1.21.11` / `1.21.11-rc3`: base version first, release above pre-releases.
pub fn cmp_release(a: &str, b: &str) -> std::cmp::Ordering {
    let (ab, asuf) = a.split_once('-').unwrap_or((a, ""));
    let (bb, bsuf) = b.split_once('-').unwrap_or((b, ""));
    cmp_ver(ab, bb)
        .then_with(|| asuf.is_empty().cmp(&bsuf.is_empty()))
        .then_with(|| cmp_ver(asuf, bsuf))
}

/// Compare dotted version strings numerically (non-numeric suffixes sort lower).
pub fn cmp_ver(a: &str, b: &str) -> std::cmp::Ordering {
    let pa: Vec<i64> = a.split(|c: char| !c.is_ascii_digit()).filter_map(|x| x.parse().ok()).collect();
    let pb: Vec<i64> = b.split(|c: char| !c.is_ascii_digit()).filter_map(|x| x.parse().ok()).collect();
    pa.cmp(&pb)
}

/// Fetch the vanilla version json for `mc`.
pub async fn vanilla_json(app: &AppState, mc: &str) -> Result<Value> {
    let m = net::get_json_cached(app, MANIFEST, TTL).await?;
    let url = m["versions"]
        .as_array()
        .context("manifest")?
        .iter()
        .find(|v| v["id"].as_str() == Some(mc))
        .and_then(|v| v["url"].as_str())
        .with_context(|| format!("找不到原版版本 {mc}"))?
        .to_string();
    net::get_json_cached(app, &url, 86400 * 30).await
}
