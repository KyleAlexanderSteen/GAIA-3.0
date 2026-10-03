//! Print an organization catalog and a local majority.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct OrgArgs {
    /// feedback, governance, ecology, or agents.
    pub band: String,
    /// Votes as 1 or 0, for the agents band.
    #[arg(long)]
    pub votes: Vec<u8>,
}

pub async fn run(args: OrgArgs) -> Result<()> {
    if args.band == "agents" && !args.votes.is_empty() {
        let votes: Vec<bool> = args.votes.iter().map(|vote| *vote == 1).collect();
        match gaia_kernel::consensus::majority(&votes) {
            Ok(decision) => println!("decision={decision}"),
            Err(reason) => return Err(anyhow!(reason)),
        }
    }
    let file = match args.band.as_str() {
        "feedback" => "gaia-spec/chaos/org-001.csv",
        "governance" => "gaia-spec/chaos/org-002.csv",
        "ecology" => "gaia-spec/chaos/org-004.csv",
        "agents" => "gaia-spec/chaos/org-005.csv",
        _ => return Err(anyhow!("band must be feedback, governance, ecology, or agents")),
    };
    let text = fs::read_to_string(Path::new(file))?;
    let rows = text.lines().skip(1).filter(|line| !line.is_empty()).count();
    println!("band={} rows={rows}", args.band);
    print!("{text}");
    Ok(())
}
