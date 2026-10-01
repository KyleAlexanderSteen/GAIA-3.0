//! Jarvis act boundary for #1150.
//!
//! A proposal is not an act. A deny, a missing receipt, a halted session,
//! or an agent-shaped signer never reaches `executed`. A grant is single-use
//! and must name the same tool and target as the proposal.
//!
//! This does not spawn a process, open a network, or close #1150.

/// What Jarvis wants done. Not permission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProposedAct {
    pub tool: String,
    pub target: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActDecision {
    Grant,
    Deny,
}

/// Human receipt. Chat text is not a receipt. An agent id is not a human.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanActReceipt {
    pub id: String,
    pub human_id: String,
    pub decision: ActDecision,
    pub tool: String,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActError {
    Halted,
    ApprovalMissing,
    NotHuman,
    Denied,
    Mismatch,
    Replay,
    EmptyField(&'static str),
}

impl std::fmt::Display for ActError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Halted => write!(f, "session is halted; act refused"),
            Self::ApprovalMissing => write!(f, "no human receipt"),
            Self::NotHuman => write!(f, "signer is not a human principal"),
            Self::Denied => write!(f, "human denied the act"),
            Self::Mismatch => write!(f, "receipt does not match the proposal"),
            Self::Replay => write!(f, "receipt already consumed"),
            Self::EmptyField(name) => write!(f, "field `{name}` must not be empty"),
        }
    }
}

impl std::error::Error for ActError {}

pub fn is_human_principal(id: &str) -> bool {
    let id = id.trim();
    !id.is_empty() && (id.starts_with("human:") || id.starts_with("did:human:"))
}

/// Records only acts that passed the gate. The vec is the proof.
#[derive(Debug, Default)]
pub struct ActGate {
    executed: Vec<String>,
    consumed: Vec<String>,
}

impl ActGate {
    pub fn executed(&self) -> &[String] {
        &self.executed
    }

    pub fn submit(
        &mut self,
        proposal: &ProposedAct,
        receipt: Option<&HumanActReceipt>,
        halted: bool,
    ) -> Result<(), ActError> {
        if proposal.tool.trim().is_empty() {
            return Err(ActError::EmptyField("tool"));
        }
        if proposal.target.trim().is_empty() {
            return Err(ActError::EmptyField("target"));
        }
        if halted {
            return Err(ActError::Halted);
        }
        let Some(receipt) = receipt else {
            return Err(ActError::ApprovalMissing);
        };
        if !is_human_principal(&receipt.human_id) {
            return Err(ActError::NotHuman);
        }
        if receipt.decision != ActDecision::Grant {
            return Err(ActError::Denied);
        }
        if receipt.tool != proposal.tool || receipt.target != proposal.target {
            return Err(ActError::Mismatch);
        }
        if receipt.id.trim().is_empty() {
            return Err(ActError::EmptyField("id"));
        }
        if self.consumed.iter().any(|id| id == &receipt.id) {
            return Err(ActError::Replay);
        }
        self.consumed.push(receipt.id.clone());
        self.executed
            .push(format!("{}:{}", proposal.tool, proposal.target));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proposal() -> ProposedAct {
        ProposedAct {
            tool: "read".into(),
            target: "gaia-agents".into(),
        }
    }

    fn grant(id: &str) -> HumanActReceipt {
        HumanActReceipt {
            id: id.into(),
            human_id: "human:kyle".into(),
            decision: ActDecision::Grant,
            tool: "read".into(),
            target: "gaia-agents".into(),
        }
    }

    #[test]
    fn deny_never_executes() {
        let mut gate = ActGate::default();
        let mut receipt = grant("r1");
        receipt.decision = ActDecision::Deny;
        let err = gate.submit(&proposal(), Some(&receipt), false).unwrap_err();
        assert_eq!(err, ActError::Denied);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn missing_receipt_never_executes() {
        let mut gate = ActGate::default();
        let err = gate.submit(&proposal(), None, false).unwrap_err();
        assert_eq!(err, ActError::ApprovalMissing);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn agent_cannot_grant() {
        let mut gate = ActGate::default();
        let mut receipt = grant("r1");
        receipt.human_id = "agent:jarvis".into();
        let err = gate.submit(&proposal(), Some(&receipt), false).unwrap_err();
        assert_eq!(err, ActError::NotHuman);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn halt_blocks_a_valid_grant() {
        let mut gate = ActGate::default();
        let receipt = grant("r1");
        let err = gate.submit(&proposal(), Some(&receipt), true).unwrap_err();
        assert_eq!(err, ActError::Halted);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn human_grant_executes_once() {
        let mut gate = ActGate::default();
        let receipt = grant("r1");
        gate.submit(&proposal(), Some(&receipt), false).unwrap();
        assert_eq!(gate.executed(), ["read:gaia-agents".to_string()]);
        let err = gate.submit(&proposal(), Some(&receipt), false).unwrap_err();
        assert_eq!(err, ActError::Replay);
        assert_eq!(gate.executed().len(), 1);
    }
}
