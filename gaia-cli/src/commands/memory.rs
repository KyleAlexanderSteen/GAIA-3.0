use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct MemoryArgs {
    #[command(subcommand)]
    pub action: MemoryAction,
}

#[derive(Subcommand)]
pub enum MemoryAction {
    /// List memory cubes
    List,
    /// Search memory
    Search {
        #[arg()]
        query: String,
    },
}

pub async fn run(args: MemoryArgs) -> Result<()> {
    match args.action {
        MemoryAction::List => {
            // TODO: GET /memory
            Err(super::not_implemented("memory list", "#1297"))
        }
        MemoryAction::Search { .. } => {
            // TODO: GET /memory/search?q=...
            Err(super::not_implemented("memory search", "#1297"))
        }
    }
}
