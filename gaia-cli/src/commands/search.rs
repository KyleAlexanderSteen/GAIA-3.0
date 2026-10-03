//! Search local knowledge files. No remote index.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct SearchArgs {
    pub query: String,
    #[arg(long, default_value = "gaia-spec")]
    pub root: String,
}

pub async fn run(args: SearchArgs) -> Result<()> {
    let hits = search(Path::new(&args.root), &args.query)?;
    println!("hits={}", hits.len());
    for hit in hits {
        println!("{hit}");
    }
    Ok(())
}

pub fn search(root: &Path, query: &str) -> Result<Vec<String>> {
    if !root.is_dir() {
        return Err(anyhow!("did not read {}", root.display()));
    }
    let mut hits = Vec::new();
    walk(root, &query.to_ascii_lowercase(), &mut hits)?;
    hits.sort();
    Ok(hits)
}

fn walk(dir: &Path, query: &str, hits: &mut Vec<String>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk(&path, query, hits)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let text = fs::read_to_string(&path).unwrap_or_default().to_ascii_lowercase();
            if text.contains(query) {
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
    fn finds_a_catalog_and_misses_an_absent_query() {
        let root = std::env::temp_dir().join(format!("gaia-search-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("magic.csv"), "id,kind\nin-context,claimed\n").unwrap();
        assert_eq!(search(&root, "in-context").unwrap().len(), 1);
        assert!(search(&root, "absent").unwrap().is_empty());
        let _ = fs::remove_dir_all(&root);
    }
}
