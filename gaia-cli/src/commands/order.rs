//! Print the AI-002 order taxonomy.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct OrderArgs {
    #[arg(long, default_value = "gaia-spec/chaos/ai-002.csv")]
    pub file: String,
}

pub async fn run(args: OrderArgs) -> Result<()> {
    let rows = read_order(Path::new(&args.file))?;
    println!("controls={}", rows.len());
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

pub fn read_order(path: &Path) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(|err| anyhow!("did not read {}: {err}", path.display()))?;
    let mut lines = text.lines();
    if !lines.next().unwrap_or("").contains("layer") {
        return Err(anyhow!("order taxonomy has no header"));
    }
    Ok(lines.filter(|line| !line.trim().is_empty()).map(|s| s.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn reads_observed_and_convention_controls() {
        let path = std::env::temp_dir().join(format!("gaia-order-{}", std::process::id()));
        fs::write(&path, "id,layer,status\nsandbox,system,observed\nframework,organizational,convention\n").unwrap();
        let rows = read_order(&path).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().any(|row| row.contains("convention")));
        let _ = fs::remove_file(path);
    }
}
