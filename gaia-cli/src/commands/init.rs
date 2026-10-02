use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct InitArgs {
    /// Profile to initialise (e.g. developer, sovereign)
    #[arg(long, default_value = "developer")]
    pub profile: String,

    /// Gateway base URL
    #[arg(long, default_value = "http://localhost:7700")]
    pub gateway: String,
}

pub async fn run(args: InitArgs) -> Result<()> {
    // TODO: write ~/.gaia/config.toml with profile + gateway URL
    let _ = args;
    Err(super::not_implemented("init", "#1297"))
}
