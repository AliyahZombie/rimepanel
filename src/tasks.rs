//! Background task manager: async downloads, archive work, installs.
use serde::Serialize;
use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, Mutex},
};
use tokio::sync::{broadcast, Semaphore};
use tokio_util::sync::CancellationToken;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskState {
    Running,
    Done,
    Failed,
    Cancelled,
}

#[derive(Serialize, Clone, Debug)]
pub struct TaskInfo {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub state: TaskState,
    /// 0..1, negative = indeterminate
    pub progress: f32,
    pub message: String,
    pub instance: Option<String>,
    pub created: i64,
    pub finished: Option<i64>,
    pub result: Option<serde_json::Value>,
}

struct Entry {
    info: TaskInfo,
    cancel: CancellationToken,
}

#[derive(Clone)]
pub struct TaskManager {
    inner: Arc<Mutex<HashMap<String, Entry>>>,
    pub events: broadcast::Sender<TaskInfo>,
    pub downloads: Arc<Semaphore>,
}

#[derive(Clone)]
pub struct TaskCtx {
    pub id: String,
    mgr: TaskManager,
    pub cancel: CancellationToken,
}

impl TaskCtx {
    pub fn progress(&self, p: f32, msg: impl Into<String>) {
        self.mgr.update(&self.id, |i| {
            i.progress = p;
            i.message = msg.into();
        });
    }
    pub fn message(&self, msg: impl Into<String>) {
        self.mgr.update(&self.id, |i| i.message = msg.into());
    }
    pub fn set_result(&self, v: serde_json::Value) {
        self.mgr.update(&self.id, |i| i.result = Some(v));
    }
    pub fn check(&self) -> anyhow::Result<()> {
        if self.cancel.is_cancelled() {
            anyhow::bail!("已取消");
        }
        Ok(())
    }
}

impl TaskManager {
    pub fn new(download_concurrency: usize) -> Self {
        let (tx, _) = broadcast::channel(512);
        Self {
            inner: Default::default(),
            events: tx,
            downloads: Arc::new(Semaphore::new(download_concurrency.max(1))),
        }
    }

    fn update(&self, id: &str, f: impl FnOnce(&mut TaskInfo)) {
        let snap = {
            let mut m = self.inner.lock().unwrap();
            let Some(e) = m.get_mut(id) else { return };
            f(&mut e.info);
            e.info.clone()
        };
        let _ = self.events.send(snap);
    }

    pub fn spawn<F, Fut>(
        &self,
        kind: &str,
        title: impl Into<String>,
        instance: Option<String>,
        f: F,
    ) -> String
    where
        F: FnOnce(TaskCtx) -> Fut + Send + 'static,
        Fut: Future<Output = anyhow::Result<String>> + Send + 'static,
    {
        let id = crate::util::new_id();
        let cancel = CancellationToken::new();
        let info = TaskInfo {
            id: id.clone(),
            kind: kind.into(),
            title: title.into(),
            state: TaskState::Running,
            progress: -1.0,
            message: "排队中".into(),
            instance,
            created: crate::util::now(),
            finished: None,
            result: None,
        };
        {
            let mut m = self.inner.lock().unwrap();
            m.insert(id.clone(), Entry { info: info.clone(), cancel: cancel.clone() });
            // keep the last 200 finished tasks
            if m.len() > 240 {
                let mut done: Vec<(i64, String)> = m
                    .values()
                    .filter(|e| e.info.state != TaskState::Running)
                    .map(|e| (e.info.created, e.info.id.clone()))
                    .collect();
                done.sort();
                for (_, k) in done.iter().take(m.len() - 200) {
                    m.remove(k);
                }
            }
        }
        let _ = self.events.send(info);
        let ctx = TaskCtx { id: id.clone(), mgr: self.clone(), cancel: cancel.clone() };
        let mgr = self.clone();
        let tid = id.clone();
        tokio::spawn(async move {
            let res = tokio::select! {
                // biased: the task body is always polled first, so guards it creates
                // on entry exist before a cancellation can drop it
                biased;
                r = f(ctx) => r,
                _ = cancel.cancelled() => Err(anyhow::anyhow!("已取消")),
            };
            mgr.update(&tid, |i| {
                i.finished = Some(crate::util::now());
                match res {
                    Ok(msg) => {
                        i.state = TaskState::Done;
                        i.progress = 1.0;
                        i.message = msg;
                    }
                    Err(e) => {
                        i.state = if cancel.is_cancelled() { TaskState::Cancelled } else { TaskState::Failed };
                        i.message = format!("{e:#}");
                        tracing::warn!("task {tid} failed: {e:#}");
                    }
                }
            });
        });
        id
    }

    pub fn list(&self) -> Vec<TaskInfo> {
        let mut v: Vec<TaskInfo> =
            self.inner.lock().unwrap().values().map(|e| e.info.clone()).collect();
        v.sort_by(|a, b| b.created.cmp(&a.created));
        v
    }

    pub fn get(&self, id: &str) -> Option<TaskInfo> {
        self.inner.lock().unwrap().get(id).map(|e| e.info.clone())
    }

    /// Ids of tasks still running for an instance.
    pub fn running_for(&self, inst: &str) -> Vec<String> {
        self.inner
            .lock()
            .unwrap()
            .values()
            .filter(|e| e.info.state == TaskState::Running && e.info.instance.as_deref() == Some(inst))
            .map(|e| e.info.id.clone())
            .collect()
    }

    pub fn cancel(&self, id: &str) -> bool {
        if let Some(e) = self.inner.lock().unwrap().get(id) {
            e.cancel.cancel();
            return true;
        }
        false
    }

    pub fn clear_finished(&self) {
        self.inner
            .lock()
            .unwrap()
            .retain(|_, e| e.info.state == TaskState::Running);
    }

    /// Wait for a task to finish; returns final info.
    pub async fn wait(&self, id: &str) -> Option<TaskInfo> {
        let mut rx = self.events.subscribe();
        loop {
            match self.get(id) {
                Some(i) if i.state != TaskState::Running => return Some(i),
                None => return None,
                _ => {}
            }
            match rx.recv().await {
                Ok(_) | Err(broadcast::error::RecvError::Lagged(_)) => {}
                Err(_) => return self.get(id),
            }
        }
    }
}
