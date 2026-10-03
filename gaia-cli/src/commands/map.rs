//! Print the band matrix from disk.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct MapArgs {
    #[arg(long, default_value = "gaia-spec/bands/matrix.csv")]
    pub file: String,
}

pub async fn run(args: MapArgs) -> Result<()> {
    let rows = read_matrix(Path::new(&args.file))?;
    println!("rows={}", rows.len());
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

pub fn read_matrix(path: &Path) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(|err| anyhow!("did not read {}: {err}", path.display()))?;
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("");
    if !header.contains("domain") {
        return Err(anyhow!("matrix has no header"));
    }
    Ok(lines.filter(|line| !line.trim().is_empty()).map(|line| line.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn reads_rows_from_a_matrix_file() {
        let path = std::env::temp_dir().join(format!("gaia-map-{}", std::process::id()));
        fs::write(&path, "domain,band\nknowledge,normal\nskills,super\n").unwrap();
        let rows = read_matrix(&path).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows[0].contains("knowledge"));
        let _ = fs::remove_file(path);
    }
}
