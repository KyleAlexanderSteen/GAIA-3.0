use anyhow::Result;
use clap::Args;
use std::fs;

#[derive(Args)]
pub struct InitArgs {
    /// Profile to initialise (e.g. developer, sovereign)
    #[arg(long, default_value = "developer")]
    pub profile: String,

    /// Gateway base URL. Stored only. This command does not connect.
    #[arg(long, default_value = "http://localhost:7700")]
    pub gateway: String,
}

pub async fn run(args: InitArgs) -> Result<()> {
    if args.profile != "developer" {
        return Err(super::not_implemented(
            "init --profile other than developer",
            "#1297",
        ));
    }
    let path = crate::ledger::ledger_path()
        .parent()
        .expect("ledger path has a parent")
        .join("profile.toml");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let body = format!(
        "profile = \"developer\"\ngateway = \"{}\"\ngateway_connected = false\nruntime = \"not-started\"\n",
        args.gateway.replace('"', "")
    );
    fs::write(&path, body)?;
    println!(
        "profile written path={} profile=developer runtime=not-started",
        path.display()
    );
    Ok(())
}
