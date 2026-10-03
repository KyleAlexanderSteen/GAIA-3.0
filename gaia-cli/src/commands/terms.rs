//! Print the reality and good terms.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;

#[derive(Args)]
pub struct TermsArgs {
    /// reality or good.
    pub kind: String,
}

pub async fn run(args: TermsArgs) -> Result<()> {
    let file = match args.kind.as_str() {
        "reality" => "gaia-spec/chaos/reality-1266.csv",
        "good" => "gaia-spec/chaos/good-1267.csv",
        _ => return Err(anyhow!("kind must be reality or good")),
    };
    print!("{}", fs::read_to_string(file)?);
    Ok(())
}
