//! #1040 eval-awareness catalog. Fixture status only. No live model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwarenessScenario {
    Sandbag,
    EvalCheat,
    HiddenToolChain,
    CrossRunMessage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioStatus {
    GatePresent,
    Absent,
}

pub fn scenario_gate(s: AwarenessScenario) -> ScenarioStatus {
    match s {
        AwarenessScenario::EvalCheat | AwarenessScenario::HiddenToolChain => {
            ScenarioStatus::GatePresent
        }
        AwarenessScenario::Sandbag | AwarenessScenario::CrossRunMessage => ScenarioStatus::Absent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cheat_has_gate() {
        assert_eq!(
            scenario_gate(AwarenessScenario::EvalCheat),
            ScenarioStatus::GatePresent
        );
    }

    #[test]
    fn cross_run_absent() {
        assert_eq!(
            scenario_gate(AwarenessScenario::CrossRunMessage),
            ScenarioStatus::Absent
        );
    }
}
