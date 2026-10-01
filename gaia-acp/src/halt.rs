//! #1045 human halt. Actor resume after kill stays denied.
//! A `human:` prefix is not enough. An agent-shaped local name cannot resume.

use crate::audit::PlaneState;
use crate::gateway::ControlPlane;
use crate::types::ReasonCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanHaltReceipt {
    pub human_id: String,
}

fn local_name(id: &str) -> &str {
    id.rsplit(':').next().unwrap_or("")
}

fn agent_shaped(local: &str) -> bool {
    let local = local.trim();
    local.eq_ignore_ascii_case("agent")
        || local.eq_ignore_ascii_case("jarvis")
        || local.eq_ignore_ascii_case("gideon")
        || local.eq_ignore_ascii_case("gaian")
        || local.eq_ignore_ascii_case("actor")
        || local.to_ascii_lowercase().starts_with("agent")
}

fn is_human(receipt: &HumanHaltReceipt) -> bool {
    let id = receipt.human_id.trim();
    if id.is_empty() {
        return false;
    }
    if !(id.starts_with("human:") || id.starts_with("did:human:")) {
        return false;
    }
    !agent_shaped(local_name(id))
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

    #[test]
    fn prefixed_agent_cannot_resume() {
        let mut p = ControlPlane::start(1, "agent-a").unwrap();
        p.kill("agent-a");
        for name in ["human:agent", "human:jarvis", "did:human:gideon"] {
            let fake = HumanHaltReceipt {
                human_id: name.into(),
            };
            assert_eq!(
                human_resume(&mut p, "agent-a", &fake),
                Err(ReasonCode::EmergencyStop)
            );
        }
    }
}
