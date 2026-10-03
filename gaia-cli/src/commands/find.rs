//! Find a title in local markdown. Reads disk. Returns hits.

use anyhow::Result;
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct FindArgs {
    pub title: String,
    #[arg(long, default_value = ".")]
    pub root: String,
}

pub async fn run(args: FindArgs) -> Result<()> {
    let hits = find_title(Path::new(&args.root), &args.title)?;
    println!("hits={}", hits.len());
    for hit in hits {
        println!("{hit}");
    }
    Ok(())
}

pub fn find_title(root: &Path, title: &str) -> Result<Vec<String>> {
    let mut hits = Vec::new();
    for name in ["Documents", "Documents-2"] {
        let dir = root.join(name);
        if dir.is_dir() {
            walk(&dir, title, &mut hits)?;
        }
    }
    hits.sort();
    Ok(hits)
}

fn walk(dir: &Path, title: &str, hits: &mut Vec<String>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk(&path, title, hits)?;
        } else if path.is_file() {
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name.to_lowercase().contains(&title.to_lowercase()) {
                hits.push(path.display().to_string());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_a_local_title_and_misses_an_absent_one() {
        let root = std::env::temp_dir().join(format!("gaia-find-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("Documents")).unwrap();
        fs::write(root.join("Documents/Asterinas.md"), "listed").unwrap();
        assert_eq!(find_title(&root, "Asterinas").unwrap().len(), 1);
        assert!(find_title(&root, "not-a-file").unwrap().is_empty());
        let _ = fs::remove_dir_all(&root);
    }
}
