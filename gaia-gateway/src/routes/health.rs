use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubsystemHealth {
    pub name: String,
    pub state: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthReport {
    pub subsystems: Vec<SubsystemHealth>,
    pub network: String,
}

/// Report only what this gateway process can currently establish.
/// The gateway has a live health route; downstream services are not wired
/// into AppState yet, so they must never be reported READY.
pub fn health_report() -> HealthReport {
    HealthReport {
        subsystems: vec![
            SubsystemHealth {
                name: "gateway".into(),
                state: "READY".into(),
                reason: "health handler is responding".into(),
            },
            SubsystemHealth {
                name: "orchestrator".into(),
                state: "NOT_IMPLEMENTED".into(),
                reason: "AppState has no orchestrator client".into(),
            },
            SubsystemHealth {
                name: "ACP".into(),
                state: "NOT_IMPLEMENTED".into(),
                reason: "gateway execution is not wired through the ACP control plane".into(),
            },
            SubsystemHealth {
                name: "MemOS".into(),
                state: "NOT_IMPLEMENTED".into(),
                reason: "gateway does not persist intent results to MemOS".into(),
            },
            SubsystemHealth {
                name: "sandbox".into(),
                state: "NOT_IMPLEMENTED".into(),
                reason: "gateway intent handler does not invoke gaia-runtime sandbox".into(),
            },
            SubsystemHealth {
                name: "audit".into(),
                state: "DEGRADED".into(),
                reason: "local receipt append exists; full orchestrated audit integration is absent".into(),
            },
        ],
        // The current gateway intent handler does not execute through the
        // runtime sandbox, so it cannot truthfully claim that network access
        // is disabled for the complete execution path.
        network: "UNKNOWN".into(),
    }
}

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(health_report()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_never_reports_unwired_subsystems_ready() {
        let report = health_report();
        assert_eq!(report.subsystems[0].name, "gateway");
        assert_eq!(report.subsystems[0].state, "READY");
        for subsystem in report.subsystems.iter().filter(|s| s.name != "gateway") {
            assert_ne!(
                subsystem.state, "READY",
                "unwired subsystem {} must not report READY",
                subsystem.name
            );
            assert!(!subsystem.reason.is_empty());
        }
    }

    #[test]
    fn health_does_not_claim_network_disabled_without_sandbox_wiring() {
        assert_eq!(health_report().network, "UNKNOWN");
    }
}
