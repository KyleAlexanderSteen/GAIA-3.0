//! Record a run. A missing risk or person is a refusal.

use anyhow::{anyhow, Result};
use clap::Args;
use std::fs;

#[derive(Args)]
pub struct GovernArgs {
    pub risk: String,
    #[arg(long)]
    pub person: String,
}

pub async fn run(args: GovernArgs) -> Result<()> {
    let record = gaia_kernel::govern::allow_run(&args.risk, &args.person).map_err(anyhow::Error::msg)?;
    fs::create_dir_all("governance")?;
    let line = format!("person={} decision={} risk={}\n", record.person, record.decision, args.risk);
    use std::io::Write;
    let mut file = fs::OpenOptions::new().create(true).append(true).open("governance/oversight.log")?;
    file.write_all(line.as_bytes())?;
    println!("person={} decision={}", record.person, record.decision);
    print!("{}", fs::read_to_string("gaia-spec/governance/vendors.csv")?);
    Ok(())
}
