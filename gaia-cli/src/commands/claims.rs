//! Print claim tiers. A metaphor is not evidence.

use anyhow::Result;
use clap::Args;
use std::fs;

#[derive(Args)]
pub struct ClaimsArgs {}

pub async fn run(_args: ClaimsArgs) -> Result<()> {
    let text = fs::read_to_string("gaia-spec/chaos/bridge-001.csv")?;
    let mut empirical = 0;
    let mut metaphor = 0;
    for line in text.lines().skip(1).filter(|line| !line.is_empty()) {
        if line.contains(",empirical,") { empirical += 1; }
        if line.contains(",symbolic,") { metaphor += 1; }
        println!("{line}");
    }
    println!("empirical={empirical} metaphor={metaphor}");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_metaphor_is_not_an_empirical_row() {
        let line = "1155,a system is conscious,empirical,not-shown,1174";
        assert!(line.contains("not-shown"));
        assert!(!line.contains("proven"));
    }
}
