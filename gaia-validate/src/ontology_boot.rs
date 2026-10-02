//! In-memory boot of the GAIA 3.0 ontology chain.
//! This process starts. It does not claim a Super OS.

use crate::ontology_gate::{admit_power, GateError, PowerClaim};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootReport {
    pub product: &'static str,
    pub repo: &'static str,
    pub domains: usize,
    pub act_enabled: bool,
    pub consciousness_axiom: bool,
}

#[derive(Debug, Clone)]
pub struct OntologyBoot {
    domains: Vec<&'static str>,
}

impl OntologyBoot {
    pub fn boot() -> Self {
        Self {
            domains: vec![
                "U0 meta-knowledge",
                "U1 mathematics",
                "U2 physical universe",
                "U3 earth",
                "U4 life",
                "U5 mind",
                "U6 consciousness",
                "U7 humanity",
                "U8 language",
                "U9 technology",
                "U10 culture",
                "U11 ethics",
                "U12 systems",
            ],
        }
    }

    pub fn report(&self) -> BootReport {
        BootReport {
            product: "GAIA 3.0",
            repo: "GAIA-2.0",
            domains: self.domains.len(),
            act_enabled: false,
            consciousness_axiom: false,
        }
    }

    pub fn submit(&self, claim: &PowerClaim) -> Result<(), GateError> {
        admit_power(claim)
    }
}

pub fn boot_line() -> String {
    let boot = OntologyBoot::boot();
    let report = boot.report();
    format!(
        "booted {} on {} domains={} act_enabled={} consciousness_axiom={}",
        report.product, report.repo, report.domains, report.act_enabled, report.consciousness_axiom
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ontology_gate::PowerState;

    #[test]
    fn boot_starts_with_thirteen_domains_and_act_off() {
        let report = OntologyBoot::boot().report();
        assert_eq!(report.domains, 13);
        assert!(!report.act_enabled);
        assert!(!report.consciousness_axiom);
    }

    #[test]
    fn boot_rejects_an_operational_claim() {
        let boot = OntologyBoot::boot();
        let claim = PowerClaim {
            name: "super autonomy".into(),
            state: PowerState::Operational,
            has_mechanism: false,
            has_evidence: false,
        };
        assert_eq!(boot.submit(&claim), Err(GateError::PowerWithoutEvidence));
    }
}
