//! Copy a note from node A to node B.

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct FederateArgs {
    pub note: String,
}

pub async fn run(args: FederateArgs) -> Result<()> {
    let (mut a, mut b) = gaia_kernel::node_pair::pair();
    gaia_kernel::node_pair::write(&mut a, "note", &args.note);
    let copied = gaia_kernel::node_pair::replicate(&a, &mut b).map_err(anyhow::Error::msg)?;
    println!("from={} to={} copied={} value={}", a.id, b.id, copied, b.records.get("note").map(String::as_str).unwrap_or(""));
    let crossed = gaia_kernel::node_pair::copy_over_localhost(&args.note).map_err(anyhow::Error::msg)?;
    println!("localhost={crossed}");
    Ok(())
}
