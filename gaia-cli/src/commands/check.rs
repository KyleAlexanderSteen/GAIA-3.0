//! Check a claim. Physical needs an instrument. A value is not a rule.

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct CheckArgs {
    /// physical, value, or metaphor.
    pub kind: String,
    #[arg(long)]
    pub instrument: bool,
    #[arg(long)]
    pub rule: bool,
}

pub async fn run(args: CheckArgs) -> Result<()> {
    let verdict = gaia_kernel::honest::check(&args.kind, args.instrument, args.rule);
    println!("kind={} accepted={} reason={}", verdict.kind, verdict.accepted, verdict.reason);
    Ok(())
}
