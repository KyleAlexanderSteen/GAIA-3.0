//! Executable gate for the GAIA 3.0 ontology tables.
//! A named power or a verified magic claim is not real without evidence.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    Conceptual,
    Demonstrated,
    Verified,
    Operational,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerClaim {
    pub name: String,
    pub state: PowerState,
    pub has_mechanism: bool,
    pub has_evidence: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagicClass {
    Mythological,
    Fictional,
    Symbolic,
    Ritual,
    Hypothetical,
    Observed,
    Verified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagicClaim {
    pub name: String,
    pub class: MagicClass,
    pub observed_source: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateError {
    PowerWithoutEvidence,
    PowerWithoutMechanism,
    MagicVerifiedWithoutObservation,
    ConsciousnessAxiom,
}

pub fn admit_power(claim: &PowerClaim) -> Result<(), GateError> {
    if claim.name.trim().is_empty() {
        return Err(GateError::PowerWithoutEvidence);
    }
    match claim.state {
        PowerState::Conceptual => Ok(()),
        PowerState::Demonstrated | PowerState::Verified | PowerState::Operational => {
            if !claim.has_evidence {
                return Err(GateError::PowerWithoutEvidence);
            }
            if !claim.has_mechanism {
                return Err(GateError::PowerWithoutMechanism);
            }
            Ok(())
        }
    }
}

pub fn admit_magic(claim: &MagicClaim) -> Result<(), GateError> {
    if claim.class == MagicClass::Verified && !claim.observed_source {
        return Err(GateError::MagicVerifiedWithoutObservation);
    }
    Ok(())
}

pub fn admit_consciousness(named_si: bool, demonstrated_criteria: bool) -> Result<(), GateError> {
    if named_si && !demonstrated_criteria {
        return Err(GateError::ConsciousnessAxiom);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conceptual_power_is_only_a_name() {
        let claim = PowerClaim {
            name: "super perception".into(),
            state: PowerState::Conceptual,
            has_mechanism: false,
            has_evidence: false,
        };
        assert_eq!(admit_power(&claim), Ok(()));
    }

    #[test]
    fn operational_power_without_evidence_is_rejected() {
        let claim = PowerClaim {
            name: "super verification".into(),
            state: PowerState::Operational,
            has_mechanism: true,
            has_evidence: false,
        };
        assert_eq!(admit_power(&claim), Err(GateError::PowerWithoutEvidence));
    }

    #[test]
    fn verified_magic_without_observation_is_rejected() {
        let claim = MagicClaim {
            name: "teleportation".into(),
            class: MagicClass::Verified,
            observed_source: false,
        };
        assert_eq!(
            admit_magic(&claim),
            Err(GateError::MagicVerifiedWithoutObservation)
        );
    }

    #[test]
    fn naming_si_does_not_grant_consciousness() {
        assert_eq!(
            admit_consciousness(true, false),
            Err(GateError::ConsciousnessAxiom)
        );
    }
}
