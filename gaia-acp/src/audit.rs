use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::types::ReasonCode;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaneState {
    Unregistered,
    Registered,
    Verified,
    SessionStarted,
    ManifestIssued,
    Stopped,
    Killed,
}

impl PlaneState {
    pub fn can_transition(self, next: PlaneState) -> bool {
        use PlaneState::*;
        matches!(
            (self, next),
            (Unregistered, Registered)
                | (Registered, Verified)
                | (Verified, SessionStarted)
                | (SessionStarted, ManifestIssued)
                | (ManifestIssued, ManifestIssued)
                | (ManifestIssued, Stopped)
                | (ManifestIssued, Killed)
                | (SessionStarted, Stopped)
                | (SessionStarted, Killed)
                | (Stopped, ManifestIssued)
                | (Stopped, Killed)
                | (_, Killed)
        )
    }

    pub fn allows_calls(self) -> bool {
        matches!(self, PlaneState::ManifestIssued)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaneEvent {
    ServerRegistered,
    ServerVerified,
    SessionStarted,
    ManifestIssued,
    ToolProposed,
    PolicyEvaluated,
    ApprovalRequested,
    ApprovalGranted,
    ApprovalDenied,
    CallAllowed,
    CallDenied,
    EgressDenied,
    ExecutionCompleted,
    ExecutionFailed,
    CredentialRevoked,
    ServerStopped,
    ServerKilled,
    AnomalyDetected,
    EmergencyStop,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActionReceipt {
    pub sequence: u64,
    pub event: PlaneEvent,
    pub previous_hash: String,
    pub hash: String,
    pub agent_id: String,
    pub tool: String,
    pub action_class: String,
    pub request_hash: String,
    pub reason: String,
    pub outcome: String,
    pub policy_version: String,
}

impl ActionReceipt {
    pub fn serialize_public(&self) -> String {
        serde_json::to_string(self).expect("receipt json")
    }

    pub fn leaks_sensitive(&self, secrets: &[&str]) -> bool {
        let s = self.serialize_public();
        secrets.iter().any(|x| !x.is_empty() && s.contains(x))
    }

    pub fn recompute_hash(&self) -> String {
        hash_fields(HashInput {
            previous_hash: &self.previous_hash,
            sequence: self.sequence,
            event: self.event,
            agent_id: &self.agent_id,
            tool: &self.tool,
            request_hash: &self.request_hash,
            reason: &self.reason,
            outcome: &self.outcome,
            action_class: &self.action_class,
        })
    }
}

struct HashInput<'a> {
    previous_hash: &'a str,
    sequence: u64,
    event: PlaneEvent,
    agent_id: &'a str,
    tool: &'a str,
    request_hash: &'a str,
    reason: &'a str,
    outcome: &'a str,
    action_class: &'a str,
}

fn hash_fields(input: HashInput<'_>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.previous_hash.as_bytes());
    hasher.update(input.sequence.to_le_bytes());
    hasher.update(format!("{:?}", input.event).as_bytes());
    hasher.update(input.agent_id.as_bytes());
    hasher.update(input.tool.as_bytes());
    hasher.update(input.request_hash.as_bytes());
    hasher.update(input.reason.as_bytes());
    hasher.update(input.outcome.as_bytes());
    hasher.update(input.action_class.as_bytes());
    hex::encode(hasher.finalize())
}

/// Structured input for [`AuditChain::push`].
///
/// Replaces the previous 7-argument flat signature. Consistent with the
/// `HashInput` pattern in this file and `MemStoreParams` / `IngestParams`
/// in `gaia-earth`.
pub struct AuditPushInput<'a> {
    pub event: PlaneEvent,
    pub agent_id: &'a str,
    pub tool: &'a str,
    pub action_class: &'a str,
    pub request_hash: &'a str,
    pub reason: ReasonCode,
    pub outcome: &'a str,
}

#[derive(Debug, Default)]
pub struct AuditChain {
    receipts: Vec<ActionReceipt>,
}

impl AuditChain {
    pub fn len(&self) -> usize {
        self.receipts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.receipts.is_empty()
    }

    pub fn last_hash(&self) -> String {
        self.receipts
            .last()
            .map(|r| r.hash.clone())
            .unwrap_or_else(|| "0".into())
    }

    pub fn push(&mut self, input: AuditPushInput<'_>) -> ActionReceipt {
        let previous_hash = self.last_hash();
        let sequence = self.receipts.len() as u64 + 1;
        let reason_s = input.reason.as_str();
        let hash = hash_fields(HashInput {
            previous_hash: &previous_hash,
            sequence,
            event: input.event,
            agent_id: input.agent_id,
            tool: input.tool,
            request_hash: input.request_hash,
            reason: reason_s,
            outcome: input.outcome,
            action_class: input.action_class,
        });
        let receipt = ActionReceipt {
            sequence,
            event: input.event,
            previous_hash,
            hash,
            agent_id: input.agent_id.into(),
            tool: input.tool.into(),
            action_class: input.action_class.into(),
            request_hash: input.request_hash.into(),
            reason: reason_s.into(),
            outcome: input.outcome.into(),
            policy_version: crate::policy::POLICY_VERSION.into(),
        };
        self.receipts.push(receipt.clone());
        receipt
    }

    pub fn receipts(&self) -> &[ActionReceipt] {
        &self.receipts
    }

    pub fn receipts_mut(&mut self) -> &mut [ActionReceipt] {
        &mut self.receipts
    }

    pub fn chain_ok(&self) -> bool {
        let mut prev = "0".to_string();
        for (i, r) in self.receipts.iter().enumerate() {
            if r.sequence != i as u64 + 1 || r.previous_hash != prev {
                return false;
            }
            if r.hash != r.recompute_hash() {
                return false;
            }
            prev = r.hash.clone();
        }
        true
    }
}


/// Versioned envelope for a persisted audit chain.
#[derive(Debug, Serialize, Deserialize)]
struct PersistedAuditLog {
    format_version: u32,
    receipts: Vec<ActionReceipt>,
}

impl AuditChain {
    /// Persist a verified chain with write-sync-rename. Callers must serialize
    /// concurrent writers targeting the same path.
    pub fn persist_to(&self, path: impl AsRef<Path>) -> Result<(), String> {
        if !self.chain_ok() {
            return Err("refusing to persist an invalid audit chain".into());
        }
        let path = path.as_ref();
        let parent = path.parent().filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent).map_err(|e| format!("create audit directory: {e}"))?;
        let bytes = serde_json::to_vec(&PersistedAuditLog {
            format_version: 1,
            receipts: self.receipts.clone(),
        }).map_err(|e| format!("serialize audit log: {e}"))?;
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("system clock: {e}"))?
            .as_nanos();
        let temp = path.with_extension(format!("audit-tmp-{}-{nonce}", std::process::id()));
        let result = (|| -> Result<(), String> {
            let mut file = File::create(&temp).map_err(|e| format!("create temp audit log: {e}"))?;
            file.write_all(&bytes).map_err(|e| format!("write audit log: {e}"))?;
            file.sync_all().map_err(|e| format!("sync audit log: {e}"))?;
            fs::rename(&temp, path).map_err(|e| format!("commit audit log: {e}"))?;
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temp);
        }
        result
    }

    /// Load and verify the persisted chain; reject unsupported formats or tampering.
    pub fn load_from(path: impl AsRef<Path>) -> Result<Self, String> {
        let bytes = fs::read(path).map_err(|e| format!("read audit log: {e}"))?;
        let persisted: PersistedAuditLog = serde_json::from_slice(&bytes)
            .map_err(|e| format!("parse audit log: {e}"))?;
        if persisted.format_version != 1 {
            return Err(format!("unsupported audit log format: {}", persisted.format_version));
        }
        let chain = Self { receipts: persisted.receipts };
        if !chain.chain_ok() {
            return Err("audit chain verification failed".into());
        }
        Ok(chain)
    }
}

#[cfg(test)]
mod persistence_tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_path() -> std::path::PathBuf {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("gaia-audit-{}-{nonce}.json", std::process::id()))
    }

    #[test]
    fn persisted_chain_roundtrips_and_verifies() {
        let path = temp_path();
        let mut chain = AuditChain::default();
        chain.push(AuditPushInput {
            event: PlaneEvent::ToolProposed, agent_id: "agent-test", tool: "read",
            action_class: "local_read", request_hash: "sha256:request",
            reason: ReasonCode::StateInvalid, outcome: "denied",
        });
        chain.push(AuditPushInput {
            event: PlaneEvent::CallDenied, agent_id: "agent-test", tool: "read",
            action_class: "local_read", request_hash: "sha256:request",
            reason: ReasonCode::StateInvalid, outcome: "denied",
        });
        chain.persist_to(&path).unwrap();
        let restored = AuditChain::load_from(&path).unwrap();
        assert_eq!(restored.len(), 2);
        assert!(restored.chain_ok());
        assert_eq!(restored.last_hash(), chain.last_hash());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn persisted_chain_rejects_tampered_receipt() {
        let path = temp_path();
        let mut chain = AuditChain::default();
        chain.push(AuditPushInput {
            event: PlaneEvent::CallDenied, agent_id: "agent-test", tool: "read",
            action_class: "local_read", request_hash: "sha256:request",
            reason: ReasonCode::StateInvalid, outcome: "denied",
        });
        chain.persist_to(&path).unwrap();
        let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        value["receipts"][0]["outcome"] = serde_json::Value::String("allowed".into());
        fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(AuditChain::load_from(&path).unwrap_err().contains("verification failed"));
        let _ = fs::remove_file(path);
    }
}
