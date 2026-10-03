//! Accept a local MCP frame. HTTP stays off.

use anyhow::{anyhow, Result};
use clap::Args;

#[derive(Args)]
pub struct McpArgs {
    /// stdio, http, or oauth.
    pub transport: String,
    pub frame: String,
}

pub async fn run(args: McpArgs) -> Result<()> {
    gaia_kernel::mcp_stdio::accept_transport(&args.transport).map_err(anyhow::Error::msg)?;
    let method = gaia_kernel::mcp_stdio::parse_frame(&args.frame).map_err(anyhow::Error::msg)?;
    println!("transport=stdio method={method} budget=1");
    Ok(())
}
