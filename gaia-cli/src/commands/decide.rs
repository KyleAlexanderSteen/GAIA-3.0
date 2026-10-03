//! Print perspectives or decide 24 against 21 from the fixture.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;

#[derive(Args)]
pub struct DecideArgs {
    /// perspectives, or areas.
    pub kind: String,
}

pub async fn run(args: DecideArgs) -> Result<()> {
    match args.kind.as_str() {
        "perspectives" => {
            print!("{}", fs::read_to_string("gaia-spec/chaos/persp-001.csv")?);
            Ok(())
        }
        "areas" => {
            let text = fs::read_to_string("gaia-spec/chaos/persp-003.csv")?;
            let mut left = 0;
            let mut right = 0;
            for line in text.lines().skip(1).filter(|line| !line.is_empty()) {
                let cols: Vec<&str> = line.split(',').collect();
                if cols.get(1) == Some(&"1") { left += 1; }
                if cols.get(2) == Some(&"1") { right += 1; }
            }
            match gaia_kernel::decide::choose("list-24", "tree-21", left, right) {
                Ok(choice) => {
                    println!("winner={} kept={} not_taken={} score={left}-{right}", choice.winner, choice.kept, choice.not_taken);
                    print!("{text}");
                    Ok(())
                }
                Err(reason) => Err(anyhow!(reason)),
            }
        }
        _ => Err(anyhow!("kind must be perspectives or areas")),
    }
}
