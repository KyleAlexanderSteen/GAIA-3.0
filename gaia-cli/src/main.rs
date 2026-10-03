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
    /// Print required OS and AI components.
    Inventory(commands::inventory::InventoryArgs),
    /// Start the GAIA runtime
    Start(commands::start::StartArgs),
    /// Print reality or good terms.
    Terms(commands::terms::TermsArgs),
    /// Manage agents
    Agent(commands::agent::AgentArgs),
    /// Print the adaptive alignment model.
    Adapt(commands::adapt::AdaptArgs),
    /// Send an intent to the orchestrator
    Intent(commands::intent::IntentArgs),
    /// Run the nine local layers.
    Layers(commands::layers::LayersArgs),
    /// Inspect or query memory
    Memory(commands::memory::MemoryArgs),
    /// Print the AI order taxonomy.
    Order(commands::order::OrderArgs),
    /// Print an organization band.
    Org(commands::org::OrgArgs),
    /// Print an order band: good, bad, rigid, or adapt.
    OrderBand(commands::orderband::OrderBandArgs),
    /// View the audit log
    Audit(commands::audit::AuditArgs),
    /// Print the Earth interaction map.
    Earth(commands::earth::EarthArgs),
    /// Revoke (stop) a running agent
    Reading(commands::reading::ReadingArgs),
    Revoke(commands::revoke::RevokeArgs),
    /// Print the listed knowledge/skill/power/magic bands. Grants nothing.
    Bands(commands::bands::BandsArgs),
    /// Print claim tiers.
    Claims(commands::claims::ClaimsArgs),
    /// Run the local boot path.
    Boot(commands::boot::BootArgs),
    /// Check a claim.
    Check(commands::check::CheckArgs),
    /// Print the AI chaos taxonomy.
    Chaos(commands::chaos::ChaosArgs),
    /// Return matrix rows for one domain.
    Lookup(commands::lookup::LookupArgs),
    /// Print the collective intelligence review.
    Collective(commands::collective::CollectiveArgs),
    /// List local Documents and Documents-2 files. No network.
    Corpus(commands::corpus::CorpusArgs),
    /// Print the frozen planetary criteria.
    Criteria(commands::criteria::CriteriaArgs),
    /// Run the wasm guest add.
    Guest(commands::guest::GuestArgs),
    /// Ask a local model. Misses loud if no server is set.
    Model(commands::model::ModelArgs),
    /// Start a host process and print its pid.
    Proc(commands::proc::ProcArgs),
    /// Find a title in the local corpus.
    Find(commands::find::FindArgs),
    /// Record a governed run.
    Govern(commands::govern::GovernArgs),
    /// Hash a local file. Rejects a URL.
    Hash(commands::hashfile::HashArgs),
    /// Name two goods and call the pluralism gate.
    Loss(commands::loss::LossArgs),
    /// Print the band matrix.
    Map(commands::map::MapArgs),
    /// List workspace crates.
    Systems(commands::systems::SystemsArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Init(args)   => commands::init::run(args).await,
        Commands::Inventory(args) => commands::inventory::run(args).await,
        Commands::Start(args)  => commands::start::run(args).await,
        Commands::Terms(args)  => commands::terms::run(args).await,
        Commands::Agent(args)  => commands::agent::run(args).await,
        Commands::Adapt(args)  => commands::adapt::run(args).await,
        Commands::Intent(args) => commands::intent::run(args).await,
        Commands::Layers(args) => commands::layers::run(args).await,
        Commands::Memory(args) => commands::memory::run(args).await,
        Commands::Order(args)  => commands::order::run(args).await,
        Commands::Org(args)    => commands::org::run(args).await,
        Commands::OrderBand(args) => commands::orderband::run(args).await,
        Commands::Audit(args)  => commands::audit::run(args).await,
        Commands::Earth(args)  => commands::earth::run(args).await,
        Commands::Reading(args) => commands::reading::run(args).await,
        Commands::Revoke(args) => commands::revoke::run(args).await,
        Commands::Bands(args)  => commands::bands::run(args).await,
        Commands::Claims(args) => commands::claims::run(args).await,
        Commands::Boot(args)   => commands::boot::run(args).await,
        Commands::Check(args)  => commands::check::run(args).await,
        Commands::Chaos(args)  => commands::chaos::run(args).await,
        Commands::Lookup(args) => commands::lookup::run(args).await,
        Commands::Collective(args) => commands::collective::run(args).await,
        Commands::Corpus(args) => commands::corpus::run(args).await,
        Commands::Criteria(args) => commands::criteria::run(args).await,
        Commands::Guest(args)  => commands::guest::run(args).await,
        Commands::Model(args)  => commands::model::run(args).await,
        Commands::Proc(args)   => commands::proc::run(args).await,
        Commands::Find(args)   => commands::find::run(args).await,
        Commands::Govern(args) => commands::govern::run(args).await,
        Commands::Hash(args)   => commands::hashfile::run(args).await,
        Commands::Loss(args)   => commands::loss::run(args).await,
        Commands::Map(args)    => commands::map::run(args).await,
        Commands::Systems(args)=> commands::systems::run(args).await,
    }
}
