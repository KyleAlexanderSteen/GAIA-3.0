//! List the crates in the workspace.

use anyhow::Result;
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct SystemsArgs {
    #[arg(long, default_value = ".")]
    pub root: String,
}

pub async fn run(args: SystemsArgs) -> Result<()> {
    let crates = list_crates(Path::new(&args.root))?;
    println!("crates={}", crates.len());
    for name in crates {
        println!("{name}");
    }
    Ok(())
}

pub fn list_crates(root: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() && path.join("Cargo.toml").is_file() {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                if name.starts_with("gaia-") {
                    names.push(name.to_string());
                }
            }
        }
    }
    names.sort();
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn lists_a_crate_directory() {
        let root = std::env::temp_dir().join(format!("gaia-sys-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("gaia-demo")).unwrap();
        fs::write(root.join("gaia-demo/Cargo.toml"), "[package]\nname=\"gaia-demo\"\n").unwrap();
        let crates = list_crates(&root).unwrap();
        assert_eq!(crates, vec!["gaia-demo".to_string()]);
        let _ = fs::remove_dir_all(&root);
    }
}
