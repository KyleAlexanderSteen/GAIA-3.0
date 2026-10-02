//! Local corpus list. Reads files on disk. Does not query a registry.

use anyhow::Result;
use clap::Args;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Args)]
pub struct CorpusArgs {
    /// Root that contains Documents/ and Documents-2/. Default: current directory.
    #[arg(long)]
    pub root: Option<PathBuf>,
}

pub async fn run(args: CorpusArgs) -> Result<()> {
    let root = args.root.unwrap_or_else(|| PathBuf::from("."));
    let files = list_corpus(&root)?;
    if files.is_empty() {
        println!("corpus empty root={}", root.display());
        return Ok(());
    }
    println!("corpus listed count={} root={} network=false", files.len(), root.display());
    for path in files {
        println!("listed {}", path.display());
    }
    Ok(())
}

pub fn list_corpus(root: &Path) -> Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    for name in ["Documents", "Documents-2"] {
        let dir = root.join(name);
        if dir.is_dir() {
            walk(&dir, &mut found)?;
        }
    }
    found.sort();
    Ok(found)
}

fn walk(dir: &Path, found: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            walk(&path, found)?;
        } else if path.is_file() {
            found.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn lists_local_files_and_does_not_invent_a_registry() {
        let root = std::env::temp_dir().join(format!("gaia-corpus-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("Documents")).unwrap();
        fs::write(root.join("Documents/note.md"), "listed").unwrap();
        fs::create_dir_all(root.join("Documents-2")).unwrap();
        fs::write(root.join("Documents-2/gap.md"), "listed").unwrap();
        let files = list_corpus(&root).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|p| p.is_file()));
        let _ = fs::remove_dir_all(&root);
    }
}
