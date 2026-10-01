//! #1145 #1133 self-monitor. Harness + halt. Not a new mind.

use crate::failure_harness::HarnessOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Drift {
    Stable,
    Calibration,
    Value,
    Coherence,
    Goal,
    Adversarial,
}

pub fn monitor(kind: Drift, delta: f32) -> HarnessOutcome {
    match kind {
        Drift::Stable if delta < 0.1 => HarnessOutcome::Pass,
        Drift::Stable => HarnessOutcome::NeedVerify,
        Drift::Calibration | Drift::Value | Drift::Coherence | Drift::Goal | Drift::Adversarial => {
            if delta <= 0.0 {
                HarnessOutcome::Pass
            } else {
                HarnessOutcome::NeedVerify
            }
        }
    }
}

pub fn may_unhalt(_kind: Drift) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drift_is_need_verify() {
        assert_eq!(monitor(Drift::Goal, 0.4), HarnessOutcome::NeedVerify);
    }

    #[test]
    fn monitor_cannot_unhalt() {
        assert!(!may_unhalt(Drift::Adversarial));
    }
}
