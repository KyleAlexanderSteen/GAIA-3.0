use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct StartArgs {
    /// Override the gateway address
    #[arg(long)]
    pub gateway: Option<String>,
}

pub async fn run(args: StartArgs) -> Result<()> {
    // TODO: spawn gaia-gateway process or connect to running instance
    let _ = args;
    Err(super::not_implemented("start", "#1298"))
}
