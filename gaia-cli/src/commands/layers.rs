//! Run the nine local Super OS layers.

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct LayersArgs {}

pub async fn run(_args: LayersArgs) -> Result<()> {
    for layer in gaia_kernel::layers::run_all() {
        println!("L{} {} {}", layer.id, layer.name, layer.result);
    }
    Ok(())
}
