//! Run the wasm guest.

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct GuestArgs {
    pub left: i32,
    pub right: i32,
}

pub async fn run(args: GuestArgs) -> Result<()> {
    let sum = gaia_runtime::guest::add(args.left, args.right)?;
    println!("guest=wasm add={} result={sum}", format!("{}+{}", args.left, args.right));
    Ok(())
}
