//! #1042 actor cannot grade or halt-wash its own run.

use crate::gateway::ControlPlane;
use crate::halt::{human_resume, HumanHaltReceipt};
use crate::types::ReasonCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    pub subject_id: String,
    pub grader_id: String,
    pub pass: bool,
}

fn independent(grader: &str, subject: &str) -> bool {
    let g = grader.trim();
    let s = subject.trim();
    !g.is_empty()
        && g != s
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
}
