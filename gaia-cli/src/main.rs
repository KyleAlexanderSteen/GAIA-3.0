use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod ledger;

#[derive(Parser)]
#[command(
    name = "gaia",
    about = "GAIA sovereign runtime CLI",
    version,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialise a GAIA profile
    Init(commands::init::InitArgs),
    /// Start the GAIA runtime
    Start(commands::start::StartArgs),
    /// Manage agents
    Agent(commands::agent::AgentArgs),
    /// Send an intent to the orchestrator
    Intent(commands::intent::IntentArgs),
    /// Inspect or query memory
    Memory(commands::memory::MemoryArgs),
    /// View the audit log
    Audit(commands::audit::AuditArgs),
    /// Revoke (stop) a running agent
    Revoke(commands::revoke::RevokeArgs),
    /// Print the listed knowledge/skill/power/magic bands. Grants nothing.
    Bands(commands::bands::BandsArgs),
    /// List local Documents and Documents-2 files. No network.
    Corpus(commands::corpus::CorpusArgs),
    /// Ask a local model. Misses loud if no server is set.
    Model(commands::model::ModelArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Init(args)   => commands::init::run(args).await,
        Commands::Start(args)  => commands::start::run(args).await,
        Commands::Agent(args)  => commands::agent::run(args).await,
        Commands::Intent(args) => commands::intent::run(args).await,
        Commands::Memory(args) => commands::memory::run(args).await,
        Commands::Audit(args)  => commands::audit::run(args).await,
        Commands::Revoke(args) => commands::revoke::run(args).await,
        Commands::Bands(args)  => commands::bands::run(args).await,
        Commands::Corpus(args) => commands::corpus::run(args).await,
        Commands::Model(args)  => commands::model::run(args).await,
    }
}
