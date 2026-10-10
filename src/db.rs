use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{de::DeserializeOwned, Serialize};
use std::{path::Path, sync::Mutex};

pub struct Db {
    conn: Mutex<Connection>,
}

#[derive(Serialize, Clone, Debug)]
pub struct LibRow {
    pub sha1: String,
    pub kind: String,
    pub name: String,
    pub size: i64,
    pub source: String,
    pub meta: serde_json::Value,
    pub created: i64,
}

#[derive(Serialize, Clone, Debug)]
pub struct WorldRow {
    pub id: String,
    pub name: String,
    pub size: i64,
    pub source: String,
    pub created: i64,
}

impl Db {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
             CREATE TABLE IF NOT EXISTS instances (id TEXT PRIMARY KEY, data TEXT NOT NULL);
             CREATE TABLE IF NOT EXISTS library (
                sha1 TEXT PRIMARY KEY, kind TEXT NOT NULL, name TEXT NOT NULL, size INTEGER NOT NULL,
                source TEXT NOT NULL, meta TEXT NOT NULL, created INTEGER NOT NULL);
             CREATE INDEX IF NOT EXISTS library_kind ON library(kind);
             CREATE TABLE IF NOT EXISTS worlds (
                id TEXT PRIMARY KEY, name TEXT NOT NULL, size INTEGER NOT NULL, source TEXT NOT NULL, created INTEGER NOT NULL);
             CREATE TABLE IF NOT EXISTS recipes (id TEXT PRIMARY KEY, name TEXT NOT NULL, data TEXT NOT NULL, created INTEGER NOT NULL);",
        )?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    fn c(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn get_setting(&self, k: &str) -> Option<String> {
        self.c()
            .query_row("SELECT value FROM settings WHERE key=?1", [k], |r| r.get(0))
            .optional()
            .ok()
            .flatten()
    }

    pub fn set_setting(&self, k: &str, v: &str) -> Result<()> {
        self.c().execute(
            "INSERT INTO settings(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            params![k, v],
        )?;
        Ok(())
    }

    pub fn all_settings(&self) -> Vec<(String, String)> {
        let c = self.c();
        let mut st = c.prepare("SELECT key,value FROM settings").unwrap();
        st.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
            .unwrap()
            .flatten()
            .collect()
    }

    // ----- instances -----
    pub fn save_instance<T: Serialize>(&self, id: &str, v: &T) -> Result<()> {
        let s = serde_json::to_string(v)?;
        self.c().execute(
            "INSERT INTO instances(id,data) VALUES(?1,?2) ON CONFLICT(id) DO UPDATE SET data=excluded.data",
            params![id, s],
        )?;
        Ok(())
    }
    pub fn delete_instance(&self, id: &str) -> Result<()> {
        self.c().execute("DELETE FROM instances WHERE id=?1", [id])?;
        Ok(())
    }
    pub fn load_instances<T: DeserializeOwned>(&self) -> Result<Vec<T>> {
        let c = self.c();
        let mut st = c.prepare("SELECT data FROM instances")?;
        let rows: Vec<String> = st.query_map([], |r| r.get(0))?.flatten().collect();
        Ok(rows
            .into_iter()
            .filter_map(|s| serde_json::from_str(&s).ok())
            .collect())
    }

    // ----- library -----
    pub fn lib_upsert(&self, row: &LibRow) -> Result<()> {
        self.c().execute(
            "INSERT INTO library(sha1,kind,name,size,source,meta,created) VALUES(?1,?2,?3,?4,?5,?6,?7)
             ON CONFLICT(sha1) DO UPDATE SET
               kind=CASE WHEN library.kind='other' THEN excluded.kind ELSE library.kind END,
               source=CASE WHEN excluded.source!='upload' THEN excluded.source ELSE library.source END,
               meta=CASE WHEN excluded.meta!='{}' THEN excluded.meta ELSE library.meta END",
            params![row.sha1, row.kind, row.name, row.size, row.source, row.meta.to_string(), row.created],
        )?;
        Ok(())
    }
    fn map_lib(r: &rusqlite::Row) -> rusqlite::Result<LibRow> {
        let meta: String = r.get(5)?;
        Ok(LibRow {
            sha1: r.get(0)?,
            kind: r.get(1)?,
            name: r.get(2)?,
            size: r.get(3)?,
            source: r.get(4)?,
            meta: serde_json::from_str(&meta).unwrap_or_default(),
            created: r.get(6)?,
        })
    }
    pub fn lib_get(&self, sha1: &str) -> Option<LibRow> {
        self.c()
            .query_row(
                "SELECT sha1,kind,name,size,source,meta,created FROM library WHERE sha1=?1",
                [sha1],
                Self::map_lib,
            )
            .optional()
            .ok()
            .flatten()
    }
    pub fn lib_list(&self, kind: Option<&str>) -> Vec<LibRow> {
        let c = self.c();
        let sql = "SELECT sha1,kind,name,size,source,meta,created FROM library";
        match kind {
            Some(k) => {
                let mut st = c.prepare(&format!("{sql} WHERE kind=?1 ORDER BY created DESC")).unwrap();
                st.query_map([k], Self::map_lib).unwrap().flatten().collect()
            }
            None => {
                let mut st = c.prepare(&format!("{sql} ORDER BY created DESC")).unwrap();
                st.query_map([], Self::map_lib).unwrap().flatten().collect()
            }
        }
    }
    pub fn lib_delete(&self, sha1: &str) -> Result<()> {
        self.c().execute("DELETE FROM library WHERE sha1=?1", [sha1])?;
        Ok(())
    }
    pub fn lib_find_server(&self, name: &str) -> Option<LibRow> {
        self.c()
            .query_row(
                "SELECT sha1,kind,name,size,source,meta,created FROM library WHERE kind IN ('server','installer') AND name=?1",
                [name],
                Self::map_lib,
            )
            .optional()
            .ok()
            .flatten()
    }

    // ----- worlds -----
    pub fn world_insert(&self, w: &WorldRow) -> Result<()> {
        self.c().execute(
            "INSERT INTO worlds(id,name,size,source,created) VALUES(?1,?2,?3,?4,?5)",
            params![w.id, w.name, w.size, w.source, w.created],
        )?;
        Ok(())
    }
    pub fn world_list(&self) -> Vec<WorldRow> {
        let c = self.c();
        let mut st = c
            .prepare("SELECT id,name,size,source,created FROM worlds ORDER BY created DESC")
            .unwrap();
        st.query_map([], |r| {
            Ok(WorldRow {
                id: r.get(0)?,
                name: r.get(1)?,
                size: r.get(2)?,
                source: r.get(3)?,
                created: r.get(4)?,
            })
        })
        .unwrap()
        .flatten()
        .collect()
    }
    pub fn world_rename(&self, id: &str, name: &str) -> Result<()> {
        self.c().execute("UPDATE worlds SET name=?2 WHERE id=?1", params![id, name])?;
        Ok(())
    }
    pub fn world_delete(&self, id: &str) -> Result<()> {
        self.c().execute("DELETE FROM worlds WHERE id=?1", [id])?;
        Ok(())
    }

    // ----- recipes -----
    pub fn recipe_save(&self, id: &str, name: &str, data: &serde_json::Value) -> Result<()> {
        self.c().execute(
            "INSERT INTO recipes(id,name,data,created) VALUES(?1,?2,?3,?4)
             ON CONFLICT(id) DO UPDATE SET name=excluded.name, data=excluded.data",
            params![id, name, data.to_string(), crate::util::now()],
        )?;
        Ok(())
    }
    pub fn recipe_list(&self) -> Vec<serde_json::Value> {
        let c = self.c();
        let mut st = c
            .prepare("SELECT id,name,data,created FROM recipes ORDER BY created DESC")
            .unwrap();
        st.query_map([], |r| {
            let id: String = r.get(0)?;
            let name: String = r.get(1)?;
            let data: String = r.get(2)?;
            let created: i64 = r.get(3)?;
            Ok(serde_json::json!({
                "id": id, "name": name, "created": created,
                "data": serde_json::from_str::<serde_json::Value>(&data).unwrap_or_default()
            }))
        })
        .unwrap()
        .flatten()
        .collect()
    }
    pub fn recipe_delete(&self, id: &str) -> Result<()> {
        self.c().execute("DELETE FROM recipes WHERE id=?1", [id])?;
        Ok(())
    }
}
