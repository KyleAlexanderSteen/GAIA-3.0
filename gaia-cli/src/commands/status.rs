use anyhow::{Context, Result};
use clap::Args;
use serde::Deserialize;

#[derive(Args)]
pub struct StatusArgs {
    /// Gateway base URL
    #[arg(long, default_value = "http://127.0.0.1:7700")]
    pub gateway: String,
}

#[derive(Debug, Deserialize)]
struct SubsystemHealth {
    name: String,
    state: String,
    reason: String,
}

#[derive(Debug, Deserialize)]
struct HealthReport {
    subsystems: Vec<SubsystemHealth>,
    network: String,
}

pub async fn run(args: StatusArgs) -> Result<()> {
    let url = format!("{}/health", args.gateway.trim_end_matches('/'));
    let response = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .with_context(|| format!("gateway health request failed at {url}"))?
        .error_for_status()
        .with_context(|| format!("gateway health endpoint returned an error at {url}"))?;
    let report: HealthReport = response
        .json()
        .await
        .with_context(|| format!("gateway returned an invalid health contract at {url}"))?;

    println!("{:<14} {:<18} REASON", "SUBSYSTEM", "STATE");
    for subsystem in report.subsystems {
        println!(
            "{:<14} {:<18} {}",
            subsystem.name, subsystem.state, subsystem.reason
        );
    }
    println!("{:<14} {:<18}", "network", report.network);
    Ok(())
}
