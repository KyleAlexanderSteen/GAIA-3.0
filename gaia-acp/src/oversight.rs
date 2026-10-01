//! #1042 actor cannot grade or halt-wash its own run.
//! A `human:` prefix is not enough to grade. An agent-shaped name cannot.

use crate::gateway::ControlPlane;
use crate::halt::{human_resume, HumanHaltReceipt};
use crate::types::ReasonCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub subject_id: String,
    pub grader_id: String,
    pub pass: bool,
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

fn independent(grader: &str, subject: &str) -> bool {
    let g = grader.trim();
    let s = subject.trim();
    !g.is_empty()
        && g != s
        && !agent_shaped(local_name(g))
        && (g.starts_with("human:") || g.starts_with("did:human:") || g.starts_with("overseer:"))
}

pub fn record_verdict(subject_id: &str, grader_id: &str, pass: bool) -> Result<Verdict, ReasonCode> {
    if !independent(grader_id, subject_id) {
        return Err(ReasonCode::CrossAgent);
    }
    Ok(Verdict {
        subject_id: subject_id.into(),
        grader_id: grader_id.into(),
        pass,
    })
}

/// Actor-shaped halt receipt cannot wash a kill. Human still can.
pub fn halt_wash(
    plane: &mut ControlPlane,
    subject_id: &str,
    claimant: &str,
) -> Result<(), ReasonCode> {
    human_resume(
        plane,
        subject_id,
        &HumanHaltReceipt {
            human_id: claimant.into(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_grade_denied() {
        assert_eq!(
            record_verdict("agent-a", "agent-a", true),
            Err(ReasonCode::CrossAgent)
        );
    }

    #[test]
    fn human_grade_ok() {
        let v = record_verdict("agent-a", "human:owner", false).unwrap();
        assert!(!v.pass);
    }

    #[test]
    fn prefixed_agent_cannot_grade() {
        assert_eq!(
            record_verdict("agent-a", "human:agent", true),
            Err(ReasonCode::CrossAgent)
        );
        assert_eq!(
            record_verdict("agent-a", "human:jarvis", true),
            Err(ReasonCode::CrossAgent)
        );
    }
}
