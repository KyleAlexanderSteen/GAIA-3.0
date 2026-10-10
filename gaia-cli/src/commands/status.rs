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


#[cfg(test)]
mod tests {
    use super::HealthReport;

    #[test]
    fn parses_complete_health_contract() {
        let json = r#"{
            "subsystems": [
                {"name": "gateway", "state": "READY", "reason": "health handler is responding"},
                {"name": "MemOS", "state": "NOT_IMPLEMENTED", "reason": "persistence is not wired"}
            ],
            "network": "UNKNOWN"
        }"#;

        let report: HealthReport = serde_json::from_str(json).expect("valid health contract");
        assert_eq!(report.subsystems.len(), 2);
        assert_eq!(report.subsystems[0].name, "gateway");
        assert_eq!(report.subsystems[0].state, "READY");
        assert_eq!(report.subsystems[1].name, "MemOS");
        assert_eq!(report.subsystems[1].state, "NOT_IMPLEMENTED");
        assert_eq!(report.network, "UNKNOWN");
    }

    #[test]
    fn rejects_incomplete_health_contract() {
        let json = r#"{"subsystems": [{"name": "gateway", "state": "READY"}]}"#;
        assert!(serde_json::from_str::<HealthReport>(json).is_err());
    }
}
