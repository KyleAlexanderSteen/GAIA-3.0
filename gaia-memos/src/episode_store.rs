//! SQLite episodes + FTS5 + snapshots (open_files JSON).
//! License: Apache-2.0
//! Also compiles #1152 ledger and #1120–#1125 layers.

#[path = "consent.rs"]
pub mod consent_ledger;
#[path = "layers.rs"]
pub mod memory_layers;

use rusqlite::{params, Connection, OptionalExtension};

pub struct EpisodeStore {
    conn: Connection,
}

impl EpisodeStore {
    pub fn open(path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS episodes (
                t_unix_ms INTEGER NOT NULL,
                text TEXT NOT NULL,
                modality TEXT NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS episodes_fts USING fts5(text)",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS snapshots (
                id TEXT PRIMARY KEY,
                label TEXT NOT NULL,
                current_step TEXT NOT NULL,
                open_files TEXT NOT NULL DEFAULT '[]'
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn insert(&self, t_unix_ms: i64, text: &str, modality: &str) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO episodes (t_unix_ms, text, modality) VALUES (?1, ?2, ?3)",
            params![t_unix_ms, text, modality],
        )?;
        let rowid = self.conn.last_insert_rowid();
        self.conn.execute(
            "INSERT INTO episodes_fts(rowid, text) VALUES (?1, ?2)",
            params![rowid, text],
        )?;
        Ok(())
    }

    pub fn recent(&self, k: usize) -> rusqlite::Result<Vec<(i64, String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT t_unix_ms, text, modality FROM episodes ORDER BY t_unix_ms DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![k as i64], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        rows.collect()
    }

    pub fn search(&self, vague: &str, k: usize) -> rusqlite::Result<Vec<(i64, String, String)>> {
        let q = vague.trim().replace('"', "");
        if q.is_empty() {
            return self.recent(k);
        }
        let mut stmt = self.conn.prepare(
            "SELECT e.t_unix_ms, e.text, e.modality
             FROM episodes_fts f
             JOIN episodes e ON e.rowid = f.rowid
             WHERE episodes_fts MATCH ?1
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![q, k as i64], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        rows.collect()
    }

    fn rowids_for_text(&self, text: &str) -> rusqlite::Result<Vec<i64>> {
        let mut stmt = self
            .conn
            .prepare("SELECT rowid FROM episodes WHERE text = ?1")?;
        let rows = stmt.query_map(params![text], |row| row.get(0))?;
        rows.collect()
    }

    pub fn forget(&self, text: &str) -> rusqlite::Result<usize> {
        let ids = self.rowids_for_text(text)?;
        let n = ids.len();
        for id in ids {
            self.conn
                .execute("DELETE FROM episodes_fts WHERE rowid = ?1", params![id])?;
            self.conn
                .execute("DELETE FROM episodes WHERE rowid = ?1", params![id])?;
        }
        Ok(n)
    }

    pub fn correct(&self, old_text: &str, new_text: &str) -> rusqlite::Result<usize> {
        let ids = self.rowids_for_text(old_text)?;
        let n = ids.len();
        for id in ids {
            self.conn.execute(
                "UPDATE episodes SET text = ?1 WHERE rowid = ?2",
                params![new_text, id],
            )?;
            self.conn
                .execute("DELETE FROM episodes_fts WHERE rowid = ?1", params![id])?;
            self.conn.execute(
                "INSERT INTO episodes_fts(rowid, text) VALUES (?1, ?2)",
                params![id, new_text],
            )?;
        }
        Ok(n)
    }

    pub fn save_snapshot(
        &self,
        id: &str,
        label: &str,
        current_step: &str,
        open_files_json: &str,
    ) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO snapshots (id, label, current_step, open_files) VALUES (?1, ?2, ?3, ?4)",
            params![id, label, current_step, open_files_json],
        )?;
        Ok(())
    }

    pub fn find_snapshot(
        &self,
        prompt: &str,
    ) -> rusqlite::Result<Option<(String, String, String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, label, current_step, open_files FROM snapshots
             WHERE id = ?1 OR label = ?1 OR instr(?1, label) > 0
             ORDER BY rowid DESC LIMIT 1",
        )?;
        stmt.query_row(params![prompt], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .optional()
    }
}
