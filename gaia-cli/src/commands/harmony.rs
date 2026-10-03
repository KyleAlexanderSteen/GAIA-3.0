//! Print HARMONY-001 through HARMONY-005.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct HarmonyArgs {
    /// Which scale: person, family, community, civilization, or planet.
    pub scale: String,
}

pub async fn run(args: HarmonyArgs) -> Result<()> {
    let file = match args.scale.as_str() {
        "person" => "gaia-spec/chaos/harmony-001.csv",
        "family" => "gaia-spec/chaos/harmony-002.csv",
        "community" => "gaia-spec/chaos/harmony-003.csv",
        "civilization" => "gaia-spec/chaos/harmony-004.csv",
        "planet" => "gaia-spec/chaos/harmony-005.csv",
        _ => return Err(anyhow!("scale must be person, family, community, civilization, or planet")),
    };
    let rows = read_scale(Path::new(file))?;
    println!("scale={} rows={}", args.scale, rows.len());
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

pub fn read_scale(path: &Path) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(|err| anyhow!("did not read {}: {err}", path.display()))?;
    let mut lines = text.lines();
    if lines.next().unwrap_or("").is_empty() {
        return Err(anyhow!("harmony file has no header"));
    }
    Ok(lines.filter(|line| !line.trim().is_empty()).map(|s| s.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn reads_a_harmony_scale() {
        let path = std::env::temp_dir().join(format!("gaia-harmony-{}", std::process::id()));
        fs::write(&path, "id,factor\nautonomy,choice\n").unwrap();
        assert_eq!(read_scale(&path).unwrap().len(), 1);
        let _ = fs::remove_file(path);
    }
}
