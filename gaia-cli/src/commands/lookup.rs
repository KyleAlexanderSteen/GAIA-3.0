//! Return matrix rows for one domain.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct LookupArgs {
    pub domain: String,
    #[arg(long, default_value = "gaia-spec/bands/matrix.csv")]
    pub file: String,
}

pub async fn run(args: LookupArgs) -> Result<()> {
    let rows = lookup(Path::new(&args.file), &args.domain)?;
    println!("hits={}", rows.len());
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

pub fn lookup(path: &Path, domain: &str) -> Result<Vec<String>> {
    let text = fs::read_to_string(path).map_err(|err| anyhow!("did not read {}: {err}", path.display()))?;
    let mut lines = text.lines();
    if !lines.next().unwrap_or("").contains("domain") {
        return Err(anyhow!("matrix has no header"));
    }
    let want = domain.to_ascii_lowercase();
    Ok(lines
        .filter(|line| line.split(',').next().unwrap_or("").eq_ignore_ascii_case(&want))
        .map(|line| line.to_string())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn returns_the_domain_and_empty_for_a_miss() {
        let path = std::env::temp_dir().join(format!("gaia-lookup-{}", std::process::id()));
        fs::write(&path, "domain,band\nknowledge,normal\nskills,super\n").unwrap();
        assert_eq!(lookup(&path, "knowledge").unwrap().len(), 1);
        assert!(lookup(&path, "absent").unwrap().is_empty());
        let _ = fs::remove_file(path);
    }
}
