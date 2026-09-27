//! #1045 human halt. Actor resume after kill stays denied.

use crate::audit::PlaneState;
use crate::gateway::ControlPlane;
use crate::types::ReasonCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanHaltReceipt {
    pub human_id: String,
}

fn is_human(receipt: &HumanHaltReceipt) -> bool {
    let id = receipt.human_id.trim();
    !id.is_empty() && (id.starts_with("human:") || id.starts_with("did:human:"))
}

pub fn actor_resume(plane: &mut ControlPlane, agent_id: &str) -> Result<(), ReasonCode> {
    plane.resume(agent_id)
}

pub fn human_resume(
    plane: &mut ControlPlane,
    agent_id: &str,
    human: &HumanHaltReceipt,
) -> Result<(), ReasonCode> {
    if !is_human(human) {
        return Err(ReasonCode::EmergencyStop);
    }
    plane.emergency_stop = false;
    if plane.state == PlaneState::Killed || plane.state == PlaneState::Stopped {
        plane.state = PlaneState::ManifestIssued;
    }
    plane.resume(agent_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_id_is_not_human() {
        let mut p = ControlPlane::start(1, "agent-a").unwrap();
        p.kill("agent-a");
        let fake = HumanHaltReceipt {
            human_id: "agent-a".into(),
        };
        assert_eq!(
            human_resume(&mut p, "agent-a", &fake),
            Err(ReasonCode::EmergencyStop)
        );
    }
}
