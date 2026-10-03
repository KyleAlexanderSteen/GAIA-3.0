//! Run one real tool. Network is refused.

use anyhow::{anyhow, Result};
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct ToolArgs {
    /// write or network.
    pub kind: String,
    pub target: String,
    pub body: Option<String>,
}

pub async fn run(args: ToolArgs) -> Result<()> {
    match args.kind.as_str() {
        "write" => {
            let root = PathBuf::from(std::env::var("GAIA_HOME").unwrap_or_else(|_| ".".into())).join("tools");
            let path = gaia_acp::file_tool::write_file(&root, &args.target, args.body.as_deref().unwrap_or(""))
                .map_err(anyhow::Error::msg)?;
            println!("wrote={}", path.display());
            println!("{}", gaia_acp::file_tool::receipt(args.body.as_deref().unwrap_or("")));
            Ok(())
        }
        "network" => Err(anyhow!(gaia_acp::file_tool::network(&args.target).unwrap_err())),
        _ => Err(anyhow!("kind must be write or network")),
    }
}
