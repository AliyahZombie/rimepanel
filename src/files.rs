//! Sandboxed file manager for instance directories.
use crate::{
    library,
    state::{not_found, ApiResult, AppState},
    util,
};
use anyhow::{bail, Context};
use axum::{
    body::Body,
    extract::{Multipart, Path as AxPath, Query, State},
    http::header,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{io::Write, path::{Path, PathBuf}, sync::Arc};
use tokio::io::AsyncWriteExt;

#[derive(Deserialize)]
pub struct PathQ {
    #[serde(default)]
    pub path: String,
}

#[derive(Serialize)]
struct Entry {
    name: String,
    dir: bool,
    size: u64,
    mtime: i64,
    /// hard-linked to the resource library (or elsewhere)
    linked: bool,
}

fn root(app: &AppState, id: &str) -> ApiResult<PathBuf> {
    if app.instances.get(id).is_none() {
        return Err(not_found("实例"));
    }
    Ok(app.instance_dir(id))
}

pub async fn list(State(app): State<Arc<AppState>>, AxPath(id): AxPath<String>, Query(q): Query<PathQ>) -> ApiResult<Json<Value>> {
    let dir = util::safe_join(&root(&app, &id)?, &q.path)?;
    let mut v = Vec::new();
    for e in std::fs::read_dir(&dir).context("目录不存在")?.flatten() {
        let Ok(m) = e.metadata() else { continue };
        let name = e.file_name().to_string_lossy().to_string();
        if name.ends_with(".mcp-tmp") {
            continue;
        }
        v.push(Entry {
            dir: m.is_dir(),
            size: if m.is_dir() { 0 } else { m.len() },
            mtime: util::mtime(&m),
            linked: !m.is_dir() && util::nlink(&m) > 1,
            name,
        });
    }
    v.sort_by(|a, b| b.dir.cmp(&a.dir).then(a.name.to_lowercase().cmp(&b.name.to_lowercase())));
    Ok(Json(json!({ "path": q.path, "entries": v })))
}

pub async fn read(State(app): State<Arc<AppState>>, AxPath(id): AxPath<String>, Query(q): Query<PathQ>) -> ApiResult<Json<Value>> {
    let p = util::safe_join(&root(&app, &id)?, &q.path)?;
    let m = std::fs::metadata(&p)?;
    if m.len() > 8 * 1024 * 1024 {
        return Err(anyhow::anyhow!("文件过大（> 8MB），请下载后编辑").into());
    }
    let bytes = std::fs::read(&p)?;
    let text = String::from_utf8(bytes).map_err(|_| anyhow::anyhow!("不是文本文件"))?;
    Ok(Json(json!({ "content": text, "linked": util::nlink(&m) > 1 })))
}

pub async fn write(State(app): State<Arc<AppState>>, AxPath(id): AxPath<String>, Query(q): Query<PathQ>, body: String) -> ApiResult<Json<Value>> {
    let p = util::safe_join(&root(&app, &id)?, &q.path)?;
    // atomic replace: breaks any hard link instead of modifying the library copy
    util::atomic_write(&p, body.as_bytes())?;
    Ok(Json(json!({"ok": true})))
}

pub async fn upload(
    State(app): State<Arc<AppState>>,
    AxPath(id): AxPath<String>,
    Query(q): Query<PathQ>,
    mut mp: Multipart,
) -> ApiResult<Json<Value>> {
    let dir = util::safe_join(&root(&app, &id)?, &q.path)?;
    std::fs::create_dir_all(&dir)?;
    let mut saved = vec![];
    while let Some(mut field) = mp.next_field().await? {
        let Some(fname) = field.file_name().map(String::from) else { continue };
        let rel = util::clean_rel(&fname)?;
        let target = dir.join(&rel);
        if let Some(p) = target.parent() {
            std::fs::create_dir_all(p)?;
        }
        let tmp = target.with_file_name(format!(".{}.mcp-tmp", util::new_id()));
        let mut f = tokio::fs::File::create(&tmp).await?;
        let r: anyhow::Result<()> = async {
            while let Some(chunk) = field.chunk().await? {
                f.write_all(&chunk).await?;
            }
            f.flush().await?;
            Ok(())
        }
        .await;
        drop(f);
        if let Err(e) = r {
            let _ = std::fs::remove_file(&tmp);
            return Err(e.into());
        }
        std::fs::rename(&tmp, &target)?;
        saved.push(rel.to_string_lossy().to_string());
    }
    Ok(Json(json!({ "saved": saved })))
}

pub async fn stream_file(p: &Path, download_name: &str) -> ApiResult<Response> {
    let f = tokio::fs::File::open(p).await.context("文件不存在")?;
    let len = f.metadata().await?.len();
    let body = Body::from_stream(tokio_util::io::ReaderStream::new(f));
    let mime = mime_guess::from_path(p).first_or_octet_stream();
    let enc = percent_encoding::utf8_percent_encode(download_name, percent_encoding::NON_ALPHANUMERIC);
    Ok((
        [
            (header::CONTENT_TYPE, mime.to_string()),
            (header::CONTENT_LENGTH, len.to_string()),
            (header::CONTENT_DISPOSITION, format!("attachment; filename*=UTF-8''{enc}")),
        ],
        body,
    )
        .into_response())
}

pub async fn download(State(app): State<Arc<AppState>>, AxPath(id): AxPath<String>, Query(q): Query<PathQ>) -> ApiResult<Response> {
    let p = util::safe_join(&root(&app, &id)?, &q.path)?;
    if p.is_dir() {
        return Err(anyhow::anyhow!("目录请先压缩再下载").into());
    }
    let name = p.file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    stream_file(&p, &name).await
}

#[derive(Deserialize)]
pub struct RenameB {
    from: String,
    to: String,
}
pub async fn rename(State(app): State<Arc<AppState>>, AxPath(id): AxPath<String>, Json(b): Json<RenameB>) -> ApiResult<Json<Value>> {
    let r = root(&app, &id)?;
    let from = util::safe_join(&r, &b.from)?;
    let to = util::safe_join(&r, &b.to)?;
    if to.exists() {
        return Err(anyhow::anyhow!("目标已存在").into());
    }
    if let Some(p) = to.parent() {
        std::fs::create_dir_all(p)?;
    }
    std::fs::rename(from, to)?;
    Ok(Json(json!({"ok": true})))
}

#[derive(Deserialize)]
pub struct PathsB {
    paths: Vec<String>,
    #[serde(default)]
    dest: String,
}
pub async fn delete(State(app): State<Arc<AppState>>, AxPath(id): AxPath<String>, Json(b): Json<PathsB>) -> ApiResult<Json<Value>> {
    let r = root(&app, &id)?;
    for p in &b.paths {
        let full = util::safe_join(&r, p)?;
        if full == r.canonicalize()? {
            return Err(anyhow::anyhow!("不能删除根目录").into());
        }
        let m = std::fs::symlink_metadata(&full)?;
        if m.is_dir() {
            std::fs::remove_dir_all(&full)?;
        } else {
            // unlink only: the library object (if linked) is unaffected
            std::fs::remove_file(&full)?;
        }
    }
    Ok(Json(json!({"ok": true})))
}

pub async fn mkdir(State(app): State<Arc<AppState>>, AxPath(id): AxPath<String>, Json(b): Json<PathQ>) -> ApiResult<Json<Value>> {
    let p = util::safe_join(&root(&app, &id)?, &b.path)?;
    std::fs::create_dir_all(p)?;
    Ok(Json(json!({"ok": true})))
}

fn zip_add(z: &mut zip::ZipWriter<std::fs::File>, base: &Path, p: &Path) -> anyhow::Result<()> {
    let opts = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .large_file(true);
    let rel = p.strip_prefix(base)?.to_string_lossy().replace('\\', "/");
    let m = std::fs::symlink_metadata(p)?;
    if m.is_dir() {
        z.add_directory(format!("{rel}/"), opts)?;
        for e in std::fs::read_dir(p)?.flatten() {
            zip_add(z, base, &e.path())?;
        }
    } else if m.is_file() {
        z.start_file(rel, opts)?;
        let mut f = std::fs::File::open(p)?;
        std::io::copy(&mut f, z)?;
    }
    Ok(())
}

pub async fn compress(State(app): State<Arc<AppState>>, AxPath(id): AxPath<String>, Json(b): Json<PathsB>) -> ApiResult<Json<Value>> {
    let r = root(&app, &id)?;
    let mut srcs = vec![];
    for p in &b.paths {
        srcs.push(util::safe_join(&r, p)?);
    }
    if srcs.is_empty() {
        return Err(anyhow::anyhow!("未选择文件").into());
    }
    let dest = util::safe_join(&r, &b.dest)?;
    let task = app.tasks.spawn("compress", format!("压缩 {}", b.dest), Some(id), move |ctx| async move {
        ctx.progress(-1.0, "压缩中");
        tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
            let tmp = dest.with_file_name(format!(".{}.mcp-tmp", util::new_id()));
            let f = std::fs::File::create(&tmp)?;
            let mut z = zip::ZipWriter::new(f);
            for s in &srcs {
                let base = s.parent().context("bad path")?;
                if let Err(e) = zip_add(&mut z, base, s) {
                    let _ = std::fs::remove_file(&tmp);
                    return Err(e);
                }
            }
            z.finish()?.flush()?;
            std::fs::rename(&tmp, &dest)?;
            Ok("压缩完成".into())
        })
        .await?
    });
    Ok(Json(json!({ "task": task })))
}

#[derive(Deserialize)]
pub struct ExtractB {
    path: String,
    dest: String,
}
pub async fn extract(State(app): State<Arc<AppState>>, AxPath(id): AxPath<String>, Json(b): Json<ExtractB>) -> ApiResult<Json<Value>> {
    let r = root(&app, &id)?;
    let src = util::safe_join(&r, &b.path)?;
    let dest = util::safe_join(&r, &b.dest)?;
    let task = app.tasks.spawn("extract", format!("解压 {}", b.path), Some(id), move |ctx| async move {
        ctx.progress(-1.0, "解压中");
        tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
            let n = src.to_string_lossy().to_lowercase();
            if n.ends_with(".zip") || n.ends_with(".jar") || n.ends_with(".mrpack") {
                util::unzip_to(&src, &dest, None)?;
            } else if n.ends_with(".tar") || n.ends_with(".tar.gz") || n.ends_with(".tgz") || n.ends_with(".tar.zst") {
                util::untar_to(&src, &dest)?;
            } else {
                bail!("不支持的压缩格式");
            }
            Ok("解压完成".into())
        })
        .await?
    });
    Ok(Json(json!({ "task": task })))
}

#[derive(Deserialize)]
pub struct ToLibB {
    path: String,
    #[serde(default)]
    kind: Option<String>,
}
/// Move an instance file into the library and replace it with a hard link.
pub async fn to_library(State(app): State<Arc<AppState>>, AxPath(id): AxPath<String>, Json(b): Json<ToLibB>) -> ApiResult<Json<Value>> {
    let p = util::safe_join(&root(&app, &id)?, &b.path)?;
    if !p.is_file() {
        return Err(anyhow::anyhow!("只能导入文件").into());
    }
    let name = p.file_name().unwrap().to_string_lossy().to_string();
    let kind = library::guess_kind(&name, b.kind.as_deref());
    let a = app.clone();
    let sha = tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
        let sha = library::import_copy(&a, &p, &kind, "upload")?;
        library::link(&a, &sha, &p)?;
        Ok(sha)
    })
    .await??;
    Ok(Json(json!({ "sha1": sha })))
}
