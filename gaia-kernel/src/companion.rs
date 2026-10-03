//! Local companion memory. A note stays on this machine.

use std::fs;
use std::path::Path;

pub fn remember(path: &Path, note: &str) -> Result<(), String> {
    if note.trim().is_empty() {
        return Err("empty note".into());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    let mut text = fs::read_to_string(path).unwrap_or_default();
    text.push_str(note.trim());
    text.push('\n');
    fs::write(path, text).map_err(|err| err.to_string())
}

pub fn recall(path: &Path) -> Result<Vec<String>, String> {
    let text = fs::read_to_string(path).map_err(|_| "no memory".to_string())?;
    Ok(text.lines().filter(|line| !line.is_empty()).map(|s| s.to_string()).collect())
}

pub fn local_model_url(url: &str) -> Result<(), String> {
    if url.starts_with("http://127.0.0.1:11434") || url.starts_with("http://localhost:11434") {
        Ok(())
    } else {
        Err("model url is not local".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remembers_a_note_and_rejects_a_remote_model() {
        let path = std::env::temp_dir().join(format!("gaia-companion-{}", std::process::id()));
        let _ = fs::remove_file(&path);
        remember(&path, "watered the plant").unwrap();
        assert_eq!(recall(&path).unwrap(), vec!["watered the plant".to_string()]);
        assert!(local_model_url("http://example.com").is_err());
        let _ = fs::remove_file(path);
    }
}
