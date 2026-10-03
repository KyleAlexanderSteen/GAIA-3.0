//! A local person. No DID resolution and no wearable.

use anyhow::{anyhow, Result};
use clap::Args;

#[derive(Args)]
pub struct PersonArgs {
    pub name: String,
    #[arg(long, default_value = "en")]
    pub language: String,
}

pub async fn run(args: PersonArgs) -> Result<()> {
    let id = gaia_kernel::person::local_id(&args.name).map_err(anyhow::Error::msg)?;
    let language = gaia_kernel::person::language(&args.language).map_err(anyhow::Error::msg)?;
    println!("id={id} language={language}");
    println!("health={}", gaia_kernel::person::health("watch", None).unwrap_err());
    println!("earth={}", gaia_kernel::person::earth_link(false).unwrap());
    let path = std::path::PathBuf::from(std::env::var("GAIA_HOME").unwrap_or_else(|_| ".".into())).join("person").join("vault.txt");
    gaia_kernel::person::vault_write(&path, "only-local").map_err(anyhow::Error::msg)?;
    println!("vault={}", gaia_kernel::person::vault_read(&path).map_err(anyhow::Error::msg)?);
    println!("{}", gaia_kernel::person::health_note("home", 72).map_err(anyhow::Error::msg)?);
    Ok(())
}