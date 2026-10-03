//! Print the AI-003 adaptive alignment model.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct AdaptArgs {
    #[arg(long, default_value = "gaia-spec/chaos/ai-003.csv")]
    pub file: String,
}

pub async fn run(args: AdaptArgs) -> Result<()> {
    let rows = read_model(Path::new(&args.file))?;
    println!("rows={}", rows.len());
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

pub fn read_model(path: &Path) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(|err| anyhow!("did not read {}: {err}", path.display()))?;
    let mut lines = text.lines();
    if !lines.next().unwrap_or("").contains("extreme") {
        return Err(anyhow!("model has no header"));
    }
    Ok(lines.filter(|line| !line.trim().is_empty()).map(|s| s.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn model_names_both_extremes_and_forbids_self_adjust() {
        let path = std::env::temp_dir().join(format!("gaia-adapt-{}", std::process::id()));
        fs::write(&path, "id,extreme\ntoo-much,too-much\nself-adjust,forbidden\n").unwrap();
        let rows = read_model(&path).unwrap();
        assert!(rows.iter().any(|row| row.contains("forbidden")));
        let _ = fs::remove_file(path);
    }
}
