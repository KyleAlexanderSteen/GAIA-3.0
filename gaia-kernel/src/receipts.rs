//! Local receipt file. Written only after a recorded echo. Not an audit store.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub fn ledger_path() -> PathBuf {
    let home = std::env::var("GAIA_HOME").unwrap_or_else(|_| ".gaia".into());
    PathBuf::from(home).join("receipts.jsonl")
}

pub fn append_recorded(id: &str, detail: &str) -> Result<(), String> {
    let path = ledger_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("receipt dir: {e}"))?;
    }
    let line = format!(r#"{{"id":{id:?},"status":"recorded","detail":{detail:?}}}"#);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|e| format!("receipt open: {e}"))?;
    writeln!(file, "{line}").map_err(|e| format!("receipt write: {e}"))?;
    Ok(())
}

pub fn read_recent(limit: usize) -> Result<Vec<String>, String> {
    let path = ledger_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(&path).map_err(|e| format!("receipt read: {e}"))?;
    let mut lines: Vec<String> = text.lines().filter(|l| !l.is_empty()).map(str::to_string).collect();
    if lines.len() > limit {
        lines = lines.split_off(lines.len() - limit);
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recorded_echo_can_be_read_back() {
        let dir = std::env::temp_dir().join(format!("gaia-receipt-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        std::env::set_var("GAIA_HOME", &dir);
        append_recorded("id-1", "echo: hi").unwrap();
        let rows = read_recent(10).unwrap();
        assert_eq!(rows.len(), 1);
        assert!(rows[0].contains("recorded"));
        assert!(rows[0].contains("echo: hi"));
        let _ = fs::remove_dir_all(&dir);
    }
}
