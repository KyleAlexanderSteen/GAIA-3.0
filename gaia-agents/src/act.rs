//! Jarvis act boundary for #1150. Identity slice is #1164.
//!
//! A proposal is not an act. A deny, a missing receipt, a halted session,
//! a prefix-only signer, or an agent-shaped name never reaches `executed`.
//! A grant is single-use and must name the same tool and target.
//!
//! The allowlist is a developer roster, not a DID resolver. Membership does
//! not rescue an agent-shaped local name. This does not spawn a process,
//! open a network, or close #1150. The runtime is not Super Intelligence.

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

/// Principals allowed to grant or clear a halt. Empty means nobody.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HumanAllowlist {
    ids: Vec<String>,
}

impl HumanAllowlist {
    pub fn new(ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
        }
    }

    /// Developer profile used by the tests. Not a verified identity.
    pub fn developer() -> Self {
        Self::new(["human:kyle"])
    }

    pub fn contains(&self, id: &str) -> bool {
        self.ids.iter().any(|row| row == id.trim())
    }
}

impl Default for HumanAllowlist {
    fn default() -> Self {
        Self { ids: Vec::new() }
    }
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
            Self::NotHuman => write!(f, "signer is not on the human allowlist"),
            Self::Denied => write!(f, "human denied the act"),
            Self::Mismatch => write!(f, "receipt does not match the proposal"),
            Self::Replay => write!(f, "receipt already consumed"),
            Self::EmptyField(name) => write!(f, "field `{name}` must not be empty"),
        }
    }
}

impl std::error::Error for ActError {}

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

/// Prefix is not enough. The id must be on the allowlist, and the local name
/// must not be agent-shaped even if someone put it on the list.
pub fn is_human_principal(id: &str, allow: &HumanAllowlist) -> bool {
    let id = id.trim();
    if id.is_empty() {
        return false;
    }
    if !(id.starts_with("human:") || id.starts_with("did:human:")) {
        return false;
    }
    if agent_shaped(local_name(id)) {
        return false;
    }
    allow.contains(id)
}

/// Records only acts that passed the gate. The vec is the proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActGate {
    executed: Vec<String>,
    consumed: Vec<String>,
    allow: HumanAllowlist,
}

impl Default for ActGate {
    fn default() -> Self {
        Self {
            executed: Vec::new(),
            consumed: Vec::new(),
            allow: HumanAllowlist::default(),
        }
    }
}

impl ActGate {
    pub fn with_allowlist(allow: HumanAllowlist) -> Self {
        Self {
            allow,
            ..Self::default()
        }
    }

    pub fn executed(&self) -> &[String] {
        &self.executed
    }

    pub fn allowlist(&self) -> &HumanAllowlist {
        &self.allow
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
        if !is_human_principal(&receipt.human_id, &self.allow) {
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

    fn gate() -> ActGate {
        ActGate::with_allowlist(HumanAllowlist::developer())
    }

    #[test]
    fn deny_never_executes() {
        let mut gate = gate();
        let mut receipt = grant("r1");
        receipt.decision = ActDecision::Deny;
        let err = gate.submit(&proposal(), Some(&receipt), false).unwrap_err();
        assert_eq!(err, ActError::Denied);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn missing_receipt_never_executes() {
        let mut gate = gate();
        let err = gate.submit(&proposal(), None, false).unwrap_err();
        assert_eq!(err, ActError::ApprovalMissing);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn agent_cannot_grant() {
        let mut gate = gate();
        let mut receipt = grant("r1");
        receipt.human_id = "agent:jarvis".into();
        let err = gate.submit(&proposal(), Some(&receipt), false).unwrap_err();
        assert_eq!(err, ActError::NotHuman);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn prefixed_agent_name_cannot_grant() {
        let mut poisoned = HumanAllowlist::new(["human:agent", "human:jarvis", "human:kyle"]);
        let mut gate = ActGate::with_allowlist(poisoned.clone());
        for name in ["human:agent", "human:jarvis", "did:human:gideon"] {
            let mut receipt = grant("r1");
            receipt.human_id = name.into();
            let err = gate.submit(&proposal(), Some(&receipt), false).unwrap_err();
            assert_eq!(err, ActError::NotHuman);
        }
        assert!(gate.executed().is_empty());
        poisoned = HumanAllowlist::developer();
        assert!(is_human_principal("human:kyle", &poisoned));
        assert!(!is_human_principal("human:agent", &HumanAllowlist::new(["human:agent"])));
    }

    #[test]
    fn prefix_alone_is_not_enough() {
        let mut gate = ActGate::default();
        let err = gate
            .submit(&proposal(), Some(&grant("r1")), false)
            .unwrap_err();
        assert_eq!(err, ActError::NotHuman);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn halt_blocks_a_valid_grant() {
        let mut gate = gate();
        let receipt = grant("r1");
        let err = gate.submit(&proposal(), Some(&receipt), true).unwrap_err();
        assert_eq!(err, ActError::Halted);
        assert!(gate.executed().is_empty());
    }

    #[test]
    fn human_grant_executes_once() {
        let mut gate = gate();
        let receipt = grant("r1");
        gate.submit(&proposal(), Some(&receipt), false).unwrap();
        assert_eq!(gate.executed(), ["read:gaia-agents".to_string()]);
        let err = gate.submit(&proposal(), Some(&receipt), false).unwrap_err();
        assert_eq!(err, ActError::Replay);
        assert_eq!(gate.executed().len(), 1);
    }
}
