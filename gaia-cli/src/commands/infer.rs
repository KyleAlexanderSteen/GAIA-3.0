//! Local completion. Remote providers are not implemented.

use anyhow::{anyhow, Result};
use clap::Args;

#[derive(Args)]
pub struct InferArgs {
    pub prompt: String,
    #[arg(long)]
    pub remote: Option<String>,
}

pub async fn run(args: InferArgs) -> Result<()> {
    if let Some(name) = args.remote {
        return Err(anyhow!(gaia_inference::remote(&name).unwrap_err()));
    }
    let completion = gaia_inference::complete_local(&args.prompt).map_err(anyhow::Error::msg)?;
    println!("provider=local");
    println!("{}", completion.text);
    Ok(())
}
