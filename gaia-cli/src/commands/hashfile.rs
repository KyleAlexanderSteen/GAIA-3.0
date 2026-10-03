//! Hash a local file. A URL is rejected.

use anyhow::{anyhow, Result};
use clap::Args;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct HashArgs {
    pub path: String,
}

pub async fn run(args: HashArgs) -> Result<()> {
    let (kind, hex) = hash_path(&args.path)?;
    println!("{kind} {hex}");
    Ok(())
}

pub fn hash_path(path: &str) -> Result<(&'static str, String)> {
    let lower = path.to_ascii_lowercase();
    if lower.starts_with("http://") || lower.starts_with("https://") {
        return Err(anyhow!("url rejected; nothing was hashed"));
    }
    let bytes = fs::read(Path::new(path)).map_err(|err| anyhow!("did not read {path}: {err}"))?;
    let hex = format!("{:x}", Sha256::digest(bytes));
    Ok(("sha256", hex))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn hashes_a_file_and_rejects_a_url() {
        let path = std::env::temp_dir().join(format!("gaia-hash-{}", std::process::id()));
        fs::write(&path, b"gaia").unwrap();
        let (kind, hex) = hash_path(path.to_str().unwrap()).unwrap();
        assert_eq!(kind, "sha256");
        assert_eq!(hex.len(), 64);
        assert!(hash_path("https://example.com").is_err());
        let _ = fs::remove_file(path);
    }
}
