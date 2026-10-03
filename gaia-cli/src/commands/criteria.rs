//! Print the frozen EARTH-003 criteria.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct CriteriaArgs {
    #[arg(long, default_value = "gaia-spec/chaos/earth-003.csv")]
    pub file: String,
}

pub async fn run(args: CriteriaArgs) -> Result<()> {
    let rows = read_criteria(Path::new(&args.file))?;
    println!("criteria={}", rows.len());
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

pub fn read_criteria(path: &Path) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(|err| anyhow!("did not read {}: {err}", path.display()))?;
    let mut lines = text.lines();
    if !lines.next().unwrap_or("").contains("frozen") {
        return Err(anyhow!("criteria have no header"));
    }
    Ok(lines.filter(|line| !line.trim().is_empty()).map(|s| s.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn criteria_are_frozen_and_do_not_claim_a_mind() {
        let path = std::env::temp_dir().join(format!("gaia-criteria-{}", std::process::id()));
        fs::write(&path, "id,property,measure,counts_against,status,frozen\nstop,end,all-fail,ends,criteria-frozen,2026-10-02\n").unwrap();
        let rows = read_criteria(&path).unwrap();
        assert!(rows[0].contains("criteria-frozen"));
        assert!(!rows[0].contains("conscious"));
        let _ = fs::remove_file(path);
    }
}
