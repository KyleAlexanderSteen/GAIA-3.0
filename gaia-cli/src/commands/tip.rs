//! Check rising variance. Do not publish without a confirmation.

use anyhow::{anyhow, Result};
use clap::Args;

#[derive(Args)]
pub struct TipArgs {
    #[arg(long)]
    pub confirmed: Option<bool>,
}

pub async fn run(args: TipArgs) -> Result<()> {
    let series = [1.0, 1.1, 0.9, 1.0, 0.0, 3.0, 0.2, 2.8];
    let rising = gaia_kernel::tip::rising(&series).map_err(anyhow::Error::msg)?;
    println!("rising={rising}");
    match gaia_kernel::tip::publish(rising, args.confirmed) {
        Ok(state) => println!("warning={state}"),
        Err(reason) => return Err(anyhow!(reason)),
    }
    Ok(())
}
