//! ACP-local grounding gate. Same contract as `gaia-runtime::enforce_grounding`.
//! Lives here so the control plane does not depend on wasmtime.

use crate::types::ReasonCode;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GroundingClaim {
    pub required: bool,
    pub source_ids: Vec<String>,
    pub ungrounded_opt_in: bool,
    /// Composite faithfulness in thousandths. `None` skips the score gate.
    pub composite_millis: Option<u16>,
    pub min_composite_millis: Option<u16>,
}

fn to_millis(v: f32) -> u16 {
    (v.clamp(0.0, 1.0) * 1000.0).round() as u16
}

impl GroundingClaim {
    pub fn required(source_ids: Vec<String>) -> Self {
        Self {
            required: true,
            source_ids,
            ungrounded_opt_in: false,
            composite_millis: None,
            min_composite_millis: None,
        }
    }

    pub fn ungrounded(opt_in: bool) -> Self {
        Self {
            required: false,
            source_ids: Vec::new(),
            ungrounded_opt_in: opt_in,
            composite_millis: None,
            min_composite_millis: None,
        }
    }

    /// Attach a precomputed composite score and minimum. ACP does not score.
    pub fn with_faithfulness(mut self, composite: f32, min_composite: f32) -> Self {
        self.composite_millis = Some(to_millis(composite));
        self.min_composite_millis = Some(to_millis(min_composite));
        self
    }

    pub fn enforce(&self) -> Result<(), ReasonCode> {
        if self.required && self.source_ids.is_empty() {
            return Err(ReasonCode::GroundingViolation);
        }
        if !self.required && self.source_ids.is_empty() && !self.ungrounded_opt_in {
            return Err(ReasonCode::UngroundedOptInRequired);
        }
        if let (Some(score), Some(min)) = (self.composite_millis, self.min_composite_millis) {
            if score < min {
                return Err(ReasonCode::GroundingViolation);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_empty_is_violation() {
        assert_eq!(
            GroundingClaim::required(vec![]).enforce(),
            Err(ReasonCode::GroundingViolation),
        );
    }

    #[test]
    fn required_with_source_ok() {
        assert!(GroundingClaim::required(vec!["chunk-1".into()])
            .enforce()
            .is_ok());
    }

    #[test]
    fn ungrounded_without_opt_in_denied() {
        assert_eq!(
            GroundingClaim::ungrounded(false).enforce(),
            Err(ReasonCode::UngroundedOptInRequired),
        );
    }

    #[test]
    fn low_composite_is_violation() {
        let claim = GroundingClaim::required(vec!["chunk-1".into()]).with_faithfulness(0.2, 0.5);
        assert_eq!(claim.enforce(), Err(ReasonCode::GroundingViolation));
    }

    #[test]
    fn high_composite_passes() {
        let claim = GroundingClaim::required(vec!["chunk-1".into()]).with_faithfulness(0.8, 0.5);
        assert!(claim.enforce().is_ok());
    }
}
