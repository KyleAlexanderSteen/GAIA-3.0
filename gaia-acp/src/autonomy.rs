//! Local autonomy caps and confirmation domains (#220, #187).
//! No live plant, money rail, or peer vault dump.

use crate::approval::HumanApprovalReceipt;
use crate::types::{ActionClass, ProposedAction, ReasonCode};

/// Software-enforced levels. 4–5 exist only as a named refuse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AutonomyLevel {
    Observe = 0,
    Suggest = 1,
    LocalAct = 2,
    BoundedRemediate = 3,
    PlantAct = 4,
    Unbounded = 5,
}

impl AutonomyLevel {
    pub fn default_level() -> Self {
        AutonomyLevel::Suggest
    }

    pub fn software_max() -> Self {
        AutonomyLevel::BoundedRemediate
    }

    pub fn requires_written_artifact(self) -> bool {
        self >= AutonomyLevel::PlantAct
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmDomain {
    Money,
    Legal,
    Medical,
    Irreversible,
    OutboundMessage,
    LifeSafety,
    Vault,
}

impl ConfirmDomain {
    pub fn from_action(action: &ProposedAction) -> Option<Self> {
        let t = action.target.to_ascii_lowercase();
        let tool = action.tool.to_ascii_lowercase();
        if tool.contains("vault") || t.contains("/vault") || t.contains("memory-dump") {
            return Some(ConfirmDomain::Vault);
        }
        if matches!(action.action_class, ActionClass::Destructive) || t.contains("life-safety") {
            return Some(ConfirmDomain::LifeSafety);
        }
        if t.contains("payment") || t.contains("wire") || tool.contains("ledger") {
            return Some(ConfirmDomain::Money);
        }
        if t.contains("legal") || tool.contains("contract") {
            return Some(ConfirmDomain::Legal);
        }
        if t.contains("medical") || t.contains("/phi/") {
            return Some(ConfirmDomain::Medical);
        }
        if matches!(action.action_class, ActionClass::MergeDeployPublish) {
            return Some(ConfirmDomain::Irreversible);
        }
        if matches!(action.action_class, ActionClass::ExternalWrite) {
            return Some(ConfirmDomain::OutboundMessage);
        }
        None
    }
}

/// Peer envelope: identity + intent + purpose. Never raw memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerEnvelope {
    pub did: String,
    pub intent_id: String,
    pub purpose: String,
    pub raw_memory: bool,
    /// Specialist agents report only to Core (#220).
    pub specialist: bool,
}

impl PeerEnvelope {
    pub fn validate(&self) -> Result<(), ReasonCode> {
        if self.did.is_empty() || self.intent_id.is_empty() || self.purpose.is_empty() {
            return Err(ReasonCode::Malformed);
        }
        if self.raw_memory {
            return Err(ReasonCode::VaultDumpDenied);
        }
        if self.purpose.eq_ignore_ascii_case("dump-vault") {
            return Err(ReasonCode::VaultDumpDenied);
        }
        if self.specialist {
            let p = self.purpose.to_ascii_lowercase();
            if p.contains("command") || p.contains("peer-direct") || p.contains("dump") {
                return Err(ReasonCode::CrossAgent);
            }
        }
        Ok(())
    }
}

pub fn gate(
    level: AutonomyLevel,
    action: &ProposedAction,
    approval: Option<&HumanApprovalReceipt>,
) -> Result<(), ReasonCode> {
    if level.requires_written_artifact() {
        return Err(ReasonCode::AutonomyCap);
    }
    if level < AutonomyLevel::LocalAct
        && matches!(
            action.action_class,
            ActionClass::RepoWrite
                | ActionClass::ExternalWrite
                | ActionClass::NetworkEgress
                | ActionClass::MergeDeployPublish
                | ActionClass::Destructive
        )
    {
        return Err(ReasonCode::AutonomyCap);
    }
    if let Some(domain) = ConfirmDomain::from_action(action) {
        match domain {
            ConfirmDomain::LifeSafety | ConfirmDomain::Vault => {
                if approval.is_none() {
                    return Err(if domain == ConfirmDomain::LifeSafety {
                        ReasonCode::LifeSafetyDenied
                    } else {
                        ReasonCode::VaultDumpDenied
                    });
                }
            }
            ConfirmDomain::Money
            | ConfirmDomain::Legal
            | ConfirmDomain::Medical
            | ConfirmDomain::Irreversible
            | ConfirmDomain::OutboundMessage => {
                if approval.is_none() {
                    return Err(ReasonCode::ConfirmRequired);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ActionClass, ProposedAction};

    fn action(class: ActionClass, target: &str, tool: &str) -> ProposedAction {
        ProposedAction {
            agent_id: "agent-a".into(),
            tool: tool.into(),
            method: "call".into(),
            target: target.into(),
            action_class: class,
            payload: String::new(),
            nonce: String::new(),
            gateway_id: "gw".into(),
            server_id: "srv".into(),
            resource_id: "repo".into(),
            wants_delegation: false,
        }
    }

    #[test]
    fn default_level_is_suggest() {
        assert_eq!(AutonomyLevel::default_level(), AutonomyLevel::Suggest);
    }

    #[test]
    fn software_max_is_bounded_remediate() {
        assert_eq!(
            AutonomyLevel::software_max(),
            AutonomyLevel::BoundedRemediate,
        );
    }

    #[test]
    fn plant_act_requires_written_artifact() {
        assert!(AutonomyLevel::PlantAct.requires_written_artifact());
        assert!(AutonomyLevel::Unbounded.requires_written_artifact());
        assert!(!AutonomyLevel::BoundedRemediate.requires_written_artifact());
    }

    #[test]
    fn gate_plant_act_is_autonomy_cap() {
        let a = action(ActionClass::LocalRead, "docs/", "local_read");
        assert_eq!(
            gate(AutonomyLevel::PlantAct, &a, None),
            Err(ReasonCode::AutonomyCap),
        );
    }

    #[test]
    fn gate_observe_blocks_repo_write() {
        let a = action(ActionClass::RepoWrite, "repo/x.rs", "git_commit");
        assert_eq!(
            gate(AutonomyLevel::Observe, &a, None),
            Err(ReasonCode::AutonomyCap),
        );
    }

    #[test]
    fn gate_local_act_allows_scratch_write() {
        let a = action(ActionClass::ScratchWrite, "scratch/tmp", "scratch_write");
        assert!(gate(AutonomyLevel::LocalAct, &a, None).is_ok());
    }

    #[test]
    fn gate_money_target_requires_approval() {
        let a = action(ActionClass::ExternalWrite, "payment/wire", "ledger");
        assert_eq!(
            gate(AutonomyLevel::LocalAct, &a, None),
            Err(ReasonCode::ConfirmRequired),
        );
    }

    #[test]
    fn gate_vault_target_requires_approval() {
        let a = action(ActionClass::LocalRead, "/vault/dump", "vault_tool");
        assert_eq!(
            gate(AutonomyLevel::BoundedRemediate, &a, None),
            Err(ReasonCode::VaultDumpDenied),
        );
    }

    #[test]
    fn peer_envelope_rejects_raw_memory() {
        let env = PeerEnvelope {
            did: "did:key:z1".into(),
            intent_id: "i1".into(),
            purpose: "query".into(),
            raw_memory: true,
            specialist: false,
        };
        assert_eq!(env.validate(), Err(ReasonCode::VaultDumpDenied));
    }

    #[test]
    fn peer_envelope_rejects_dump_vault_purpose() {
        let env = PeerEnvelope {
            did: "did:key:z1".into(),
            intent_id: "i1".into(),
            purpose: "dump-vault".into(),
            raw_memory: false,
            specialist: false,
        };
        assert_eq!(env.validate(), Err(ReasonCode::VaultDumpDenied));
    }

    #[test]
    fn peer_envelope_specialist_cannot_command() {
        let env = PeerEnvelope {
            did: "did:key:z1".into(),
            intent_id: "i1".into(),
            purpose: "peer-direct-command".into(),
            raw_memory: false,
            specialist: true,
        };
        assert_eq!(env.validate(), Err(ReasonCode::CrossAgent));
    }

    #[test]
    fn peer_envelope_valid_passes() {
        let env = PeerEnvelope {
            did: "did:key:z1".into(),
            intent_id: "i1".into(),
            purpose: "share-summary".into(),
            raw_memory: false,
            specialist: false,
        };
        assert!(env.validate().is_ok());
    }
}
