//! Print the AI-001 chaos taxonomy.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct ChaosArgs {
    #[arg(long, default_value = "gaia-spec/chaos/ai-001.csv")]
    pub file: String,
}

pub async fn run(args: ChaosArgs) -> Result<()> {
    let rows = read_taxonomy(Path::new(&args.file))?;
    println!("modes={}", rows.len());
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

pub fn read_taxonomy(path: &Path) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(|err| anyhow!("did not read {}: {err}", path.display()))?;
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("");
    if !header.contains("kind") {
        return Err(anyhow!("taxonomy has no header"));
    }
    Ok(lines.filter(|line| !line.trim().is_empty()).map(|s| s.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn reads_observed_and_hypothesis_rows() {
        let path = std::env::temp_dir().join(format!("gaia-chaos-{}", std::process::id()));
        fs::write(&path, "id,kind,status\nhallucination,confabulation,observed\nloop,degradation,hypothesis\n").unwrap();
        let rows = read_taxonomy(&path).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().any(|row| row.contains("hypothesis")));
        let _ = fs::remove_file(path);
    }
}
