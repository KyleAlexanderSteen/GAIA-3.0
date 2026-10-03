//! Print ORDER-001 through ORDER-004.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct OrderBandArgs {
    /// Which band: good, bad, rigid, or adapt.
    pub band: String,
}

pub async fn run(args: OrderBandArgs) -> Result<()> {
    let file = match args.band.as_str() {
        "good" => "gaia-spec/chaos/order-001.csv",
        "bad" => "gaia-spec/chaos/order-002.csv",
        "rigid" => "gaia-spec/chaos/order-003.csv",
        "adapt" => "gaia-spec/chaos/order-004.csv",
        _ => return Err(anyhow!("band must be good, bad, rigid, or adapt")),
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
    fn reads_an_order_band() {
        let path = std::env::temp_dir().join(format!("gaia-orderband-{}", std::process::id()));
        fs::write(&path, "id,form\nlaw,law\n").unwrap();
        assert_eq!(read_band(&path).unwrap().len(), 1);
        let _ = fs::remove_file(path);
    }
}
