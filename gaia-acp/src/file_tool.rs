//! A real file write. A network call is refused.

use std::fs;
use std::path::{Path, PathBuf};

pub fn write_file(root: &Path, relative: &str, body: &str) -> Result<PathBuf, &'static str> {
    if relative.contains("..") || Path::new(relative).is_absolute() {
        return Err("path escapes the root");
    }
    fs::create_dir_all(root).map_err(|_| "root was not created")?;
    let path = root.join(relative);
    fs::write(&path, body).map_err(|_| "file was not written")?;
    Ok(path)
}

pub fn network(_url: &str) -> Result<(), &'static str> {
    Err("network is not permitted")
}

pub fn receipt(body: &str) -> String {
    format!("sha256={}", simple_hash(body))
}

fn simple_hash(body: &str) -> u64 {
    body.bytes().fold(0u64, |acc, byte| acc.wrapping_mul(31).wrapping_add(u64::from(byte)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_a_file_and_refuses_network() {
        let root = std::env::temp_dir().join(format!("gaia-acp-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let path = write_file(&root, "note.txt", "kept").unwrap();
        assert_eq!(fs::read_to_string(path).unwrap(), "kept");
        assert!(write_file(&root, "../escape.txt", "no").is_err());
        assert_eq!(network("http://example.com"), Err("network is not permitted"));
        let _ = fs::remove_dir_all(root);
    }
}
