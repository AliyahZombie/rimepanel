//! Single-admin authentication with argon2 and HttpOnly session cookies.
use crate::state::{ApiResult, AppError, AppState};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;
use std::{sync::Arc, time::{Duration, Instant}};

const COOKIE: &str = "mcp_session";
const TTL: Duration = Duration::from_secs(7 * 24 * 3600);

fn token_from(headers: &HeaderMap) -> Option<String> {
    let c = headers.get(header::COOKIE)?.to_str().ok()?;
    c.split(';')
        .filter_map(|p| p.trim().split_once('='))
        .find(|(k, _)| *k == COOKIE)
        .map(|(_, v)| v.to_string())
}

pub fn is_logged_in(app: &AppState, headers: &HeaderMap) -> bool {
    let Some(t) = token_from(headers) else { return false };
    let mut s = app.sessions.lock().unwrap();
    match s.get(&t) {
        Some(at) if at.elapsed() < TTL => {
            s.insert(t, Instant::now());
            true
        }
        Some(_) => {
            s.remove(&t);
            false
        }
        None => false,
    }
}

pub async fn require(State(app): State<Arc<AppState>>, req: Request, next: Next) -> Response {
    let path = req.uri().path();
    let open = path.starts_with("/api/auth/") || !path.starts_with("/api/");
    if open || is_logged_in(&app, req.headers()) {
        return next.run(req).await;
    }
    (StatusCode::UNAUTHORIZED, Json(json!({"error": "未登录"}))).into_response()
}

fn hash(pw: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(pw.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| anyhow::anyhow!("{e}"))
}

fn verify(pw: &str, stored: &str) -> bool {
    PasswordHash::new(stored)
        .map(|h| Argon2::default().verify_password(pw.as_bytes(), &h).is_ok())
        .unwrap_or(false)
}

fn session_response(app: &AppState) -> Response {
    let t = crate::util::random_token();
    app.sessions.lock().unwrap().insert(t.clone(), Instant::now());
    let cookie = format!("{COOKIE}={t}; Path=/; HttpOnly; SameSite=Strict; Max-Age={}", TTL.as_secs());
    ([(header::SET_COOKIE, cookie)], Json(json!({"ok": true}))).into_response()
}

pub async fn state(State(app): State<Arc<AppState>>, headers: HeaderMap) -> Json<serde_json::Value> {
    Json(json!({
        "setup_needed": app.setting("admin_password").is_none(),
        "logged_in": is_logged_in(&app, &headers),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

#[derive(Deserialize)]
pub struct PwBody {
    password: String,
    #[serde(default)]
    new_password: Option<String>,
}

pub async fn setup(State(app): State<Arc<AppState>>, Json(b): Json<PwBody>) -> ApiResult<Response> {
    if app.setting("admin_password").is_some() {
        return Err(AppError(StatusCode::FORBIDDEN, "已初始化".into()));
    }
    if b.password.len() < 6 {
        return Err(AppError(StatusCode::BAD_REQUEST, "密码至少 6 位".into()));
    }
    app.db.set_setting("admin_password", &hash(&b.password)?)?;
    Ok(session_response(&app))
}

pub async fn login(State(app): State<Arc<AppState>>, Json(b): Json<PwBody>) -> ApiResult<Response> {
    let Some(stored) = app.setting("admin_password") else {
        return Err(AppError(StatusCode::BAD_REQUEST, "尚未初始化".into()));
    };
    if !verify(&b.password, &stored) {
        tokio::time::sleep(Duration::from_millis(600)).await;
        return Err(AppError(StatusCode::UNAUTHORIZED, "密码错误".into()));
    }
    Ok(session_response(&app))
}

pub async fn logout(State(app): State<Arc<AppState>>, headers: HeaderMap) -> Response {
    if let Some(t) = token_from(&headers) {
        app.sessions.lock().unwrap().remove(&t);
    }
    let cookie = format!("{COOKIE}=; Path=/; HttpOnly; Max-Age=0");
    ([(header::SET_COOKIE, cookie)], Json(json!({"ok": true}))).into_response()
}

pub async fn change_password(
    State(app): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(b): Json<PwBody>,
) -> ApiResult<Json<serde_json::Value>> {
    if !is_logged_in(&app, &headers) {
        return Err(AppError(StatusCode::UNAUTHORIZED, "未登录".into()));
    }
    let stored = app.setting("admin_password").unwrap_or_default();
    if !verify(&b.password, &stored) {
        return Err(AppError(StatusCode::UNAUTHORIZED, "当前密码错误".into()));
    }
    let np = b.new_password.unwrap_or_default();
    if np.len() < 6 {
        return Err(AppError(StatusCode::BAD_REQUEST, "新密码至少 6 位".into()));
    }
    app.db.set_setting("admin_password", &hash(&np)?)?;
    let keep = token_from(&headers);
    app.sessions.lock().unwrap().retain(|k, _| Some(k) == keep.as_ref());
    Ok(Json(json!({"ok": true})))
}
