//! Print CHAOS-001, CHAOS-002, and CHAOS-004.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct ChaosBandArgs {
    /// Which band: good, bad, or contain.
    pub band: String,
}

pub async fn run(args: ChaosBandArgs) -> Result<()> {
    let file = match args.band.as_str() {
        "good" => "gaia-spec/chaos/chaos-001.csv",
        "bad" => "gaia-spec/chaos/chaos-002.csv",
        "contain" => "gaia-spec/chaos/chaos-004.csv",
        _ => return Err(anyhow!("band must be good, bad, or contain")),
    };
    let rows = read_band(Path::new(file))?;
    println!("band={} rows={}", args.band, rows.len());
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

pub fn read_band(path: &Path) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(|err| anyhow!("did not read {}: {err}", path.display()))?;
    let mut lines = text.lines();
    if lines.next().unwrap_or("").is_empty() {
        return Err(anyhow!("band has no header"));
    }
    Ok(lines.filter(|line| !line.trim().is_empty()).map(|s| s.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn reads_a_band_file() {
        let path = std::env::temp_dir().join(format!("gaia-band-{}", std::process::id()));
        fs::write(&path, "id,side\nmutation,helps\n").unwrap();
        assert_eq!(read_band(&path).unwrap().len(), 1);
        let _ = fs::remove_file(path);
    }
}
