//! Print the EARTH-002 review table.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct CollectiveArgs {
    #[arg(long, default_value = "gaia-spec/chaos/earth-002.csv")]
    pub file: String,
}

pub async fn run(args: CollectiveArgs) -> Result<()> {
    let rows = read_collective(Path::new(&args.file))?;
    println!("rows={}", rows.len());
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

pub fn read_collective(path: &Path) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(|err| anyhow!("did not read {}: {err}", path.display()))?;
    let mut lines = text.lines();
    if !lines.next().unwrap_or("").contains("measure") {
        return Err(anyhow!("review table has no header"));
    }
    Ok(lines.filter(|line| !line.trim().is_empty()).map(|s| s.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn table_keeps_task_separate_from_a_mind() {
        let path = std::env::temp_dir().join(format!("gaia-collective-{}", std::process::id()));
        fs::write(&path, "id,system,task,measure\nmaze,slime,path,tube\n").unwrap();
        assert_eq!(read_collective(&path).unwrap().len(), 1);
        let _ = fs::remove_file(path);
    }
}
