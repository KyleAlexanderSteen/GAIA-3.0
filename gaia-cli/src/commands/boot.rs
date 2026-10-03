//! Run the local boot path.

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct BootArgs {}

pub async fn run(_args: BootArgs) -> Result<()> {
    for stage in gaia_kernel::boot::boot() {
        println!("stage={} done={} note={}", stage.name, stage.done, stage.note);
    }
    match gaia_kernel::boot::probe("disk") {
        Ok(state) => println!("driver=disk {state}"),
        Err(reason) => println!("driver=disk miss={reason}"),
    }
    println!("guest={}", gaia_kernel::boot::run_guest().map_err(anyhow::Error::msg)?);
    Ok(())
}
