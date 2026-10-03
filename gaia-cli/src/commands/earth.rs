//! Print the EARTH-001 interaction map.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct EarthArgs {
    #[arg(long, default_value = "gaia-spec/chaos/earth-001.csv")]
    pub file: String,
}

pub async fn run(args: EarthArgs) -> Result<()> {
    let rows = read_earth(Path::new(&args.file))?;
    println!("links={}", rows.len());
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

pub fn read_earth(path: &Path) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(|err| anyhow!("did not read {}: {err}", path.display()))?;
    let mut lines = text.lines();
    if !lines.next().unwrap_or("").contains("evidence") {
        return Err(anyhow!("earth map has no header"));
    }
    Ok(lines.filter(|line| !line.trim().is_empty()).map(|s| s.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn map_labels_model_and_fixture() {
        let path = std::env::temp_dir().join(format!("gaia-earth-{}", std::process::id()));
        fs::write(&path, "id,link,evidence\nboundaries,six,modeled\nensemble,fixture,fixture\n").unwrap();
        let rows = read_earth(&path).unwrap();
        assert!(rows.iter().any(|row| row.contains("fixture")));
        let _ = fs::remove_file(path);
    }
}
