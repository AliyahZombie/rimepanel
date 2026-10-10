use crate::{db::Db, instance::InstanceManager, tasks::TaskManager};
use axum::{http::StatusCode, response::IntoResponse, Json};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
    time::Instant,
};

pub struct Paths {
    pub root: PathBuf,
}

impl Paths {
    pub fn objects(&self) -> PathBuf {
        self.root.join("library/objects")
    }
    pub fn worlds(&self) -> PathBuf {
        self.root.join("library/worlds")
    }
    pub fn instances(&self) -> PathBuf {
        self.root.join("instances")
    }
    pub fn instance(&self, id: &str) -> PathBuf {
        self.instances().join(id)
    }
    pub fn runtimes(&self) -> PathBuf {
        self.root.join("runtimes")
    }
    pub fn backups(&self, id: &str) -> PathBuf {
        self.root.join("backups").join(id)
    }
    pub fn tmp(&self) -> PathBuf {
        self.root.join("tmp")
    }
    pub fn cache(&self) -> PathBuf {
        self.root.join("cache")
    }
}

pub struct AppState {
    pub paths: Paths,
    pub db: Db,
    pub tasks: TaskManager,
    pub http: reqwest::Client,
    pub instances: InstanceManager,
    pub sessions: Mutex<HashMap<String, Instant>>,
}

impl AppState {
    pub async fn init(root: PathBuf) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&root)?;
        let root = root.canonicalize()?;
        let paths = Paths { root };
        for d in [
            paths.objects(),
            paths.worlds(),
            paths.instances(),
            paths.runtimes(),
            paths.root.join("backups"),
            paths.tmp(),
            paths.cache(),
        ] {
            std::fs::create_dir_all(d)?;
        }
        // stale temp files from an interrupted run
        if let Ok(rd) = std::fs::read_dir(paths.tmp()) {
            for e in rd.flatten() {
                let _ = std::fs::remove_file(e.path());
            }
        }
        let db = Db::open(&paths.root.join("panel.db"))?;
        let conc: usize = db
            .get_setting("download_concurrency")
            .and_then(|v| v.parse().ok())
            .unwrap_or(8);
        let http = reqwest::Client::builder()
            .user_agent(concat!("RimePanel/", env!("CARGO_PKG_VERSION"), ""))
            .connect_timeout(std::time::Duration::from_secs(15))
            .build()?;
        let instances = InstanceManager::load(&db, &paths)?;
        Ok(Self {
            paths,
            db,
            tasks: TaskManager::new(conc),
            http,
            instances,
            sessions: Mutex::new(HashMap::new()),
        })
    }

    pub fn setting(&self, k: &str) -> Option<String> {
        self.db.get_setting(k).filter(|s| !s.is_empty())
    }

    pub fn instance_dir(&self, id: &str) -> PathBuf {
        self.paths.instance(id)
    }

    #[allow(dead_code)]
    pub fn root(&self) -> &Path {
        &self.paths.root
    }
}

/// API error type: everything becomes `{ "error": "..." }`.
pub struct AppError(pub StatusCode, pub String);

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(e: E) -> Self {
        let e: anyhow::Error = e.into();
        AppError(StatusCode::BAD_REQUEST, format!("{e:#}"))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.0, Json(serde_json::json!({ "error": self.1 }))).into_response()
    }
}

pub type ApiResult<T> = Result<T, AppError>;

pub fn not_found(what: &str) -> AppError {
    AppError(StatusCode::NOT_FOUND, format!("{what} 不存在"))
}
