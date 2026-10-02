use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct AgentArgs {
    #[command(subcommand)]
    pub action: AgentAction,
}

#[derive(Subcommand)]
pub enum AgentAction {
    /// Create a new agent definition
    Create {
        /// Agent name
        #[arg(short, long)]
        name: String,
        /// Path to agent manifest TOML
        #[arg(short, long)]
        manifest: Option<String>,
    },
    /// Deploy an agent to the runtime
    Deploy {
        /// Agent name or ID
        #[arg(short, long)]
        name: String,
    },
}

pub async fn run(args: AgentArgs) -> Result<()> {
    match args.action {
        AgentAction::Create { .. } => {
            // TODO: POST /agents
            Err(super::not_implemented("agent create", "#1297"))
        }
        AgentAction::Deploy { .. } => {
            // TODO: POST /agents/{name}/deploy
            Err(super::not_implemented("agent deploy", "#1297"))
        }
    }
}
