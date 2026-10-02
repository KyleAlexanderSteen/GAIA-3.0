//! Local receipt ledger. Only written after an admitted echo. Not an audit store.
use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub fn ledger_path() -> PathBuf {
    let home = std::env::var("GAIA_HOME").unwrap_or_else(|_| ".gaia".into());
    PathBuf::from(home).join("receipts.jsonl")
}

pub fn append_recorded(id: &str, detail: &str) -> Result<()> {
    let path = ledger_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let line = serde_json::json!({
        "id": id,
        "status": "recorded",
        "detail": detail,
    });
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{line}")?;
    Ok(())
}

pub fn read_recent(limit: usize) -> Result<Vec<String>> {
    let path = ledger_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(path)?;
    let mut lines: Vec<String> = text.lines().filter(|l| !l.is_empty()).map(str::to_string).collect();
    if lines.len() > limit {
        lines = lines.split_off(lines.len() - limit);
    }
    Ok(lines)
}
