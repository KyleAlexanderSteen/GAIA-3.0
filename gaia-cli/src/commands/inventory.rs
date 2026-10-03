//! Print required OS and AI components.

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct InventoryArgs {}

pub async fn run(_args: InventoryArgs) -> Result<()> {
    let parts = gaia_kernel::inventory::required();
    let missing = parts.iter().filter(|part| !part.present).count();
    println!("parts={} missing={}", parts.len(), missing);
    for part in parts {
        println!("{} {} present={}", part.stack, part.name, part.present);
    }
    println!("superintelligence={}", gaia_kernel::inventory::grants_superintelligence());
    Ok(())
}
