use serde::{Deserialize, Serialize};

use crate::types::{ActionClass, RiskTier};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IdentityKind {
    Agent,
    Gateway,
    Server,
    Approver,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrincipalId {
    pub kind: IdentityKind,
    pub id: String,
}

#[derive(Debug, Clone, Default)]
pub struct RevocationList {
    ids: Vec<String>,
}

impl RevocationList {
    pub fn revoke(&mut self, id: impl Into<String>) {
        let id = id.into();
        if !self.ids.contains(&id) {
            self.ids.push(id);
        }
    }

    pub fn is_revoked(&self, id: &str) -> bool {
        self.ids.iter().any(|x| x == id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityManifest {
    pub manifest_id: String,
    pub issuer_id: String,
    pub agent_id: String,
    pub gateway_id: String,
    pub server_id: String,
    pub resource_id: String,
    pub task_id: String,
    pub issued_at: u64,
    pub not_before: u64,
    pub expires_at: u64,
    pub allowed_tools: Vec<String>,
    pub allowed_paths: Vec<String>,
    pub allowed_destinations: Vec<String>,
    pub max_actions: u32,
    pub actions_used: u32,
    pub max_risk: RiskTier,
    pub policy_version: String,
    pub nonce: String,
    pub allow_delegation: bool,
    pub revocation_epoch: u64,
}

impl CapabilityManifest {
    pub fn local_reader(agent_id: &str, now: u64) -> Self {
        Self {
            manifest_id: format!("man-{agent_id}"),
            issuer_id: "issuer-local".into(),
            agent_id: agent_id.into(),
            gateway_id: "gateway-local".into(),
            server_id: "server-local".into(),
            resource_id: "repo-local".into(),
            task_id: format!("task-{agent_id}"),
            issued_at: now,
            not_before: now,
            expires_at: now + 3600,
            allowed_tools: vec![
                "local_parse".into(),
                "local_read".into(),
                "scratch_write".into(),
            ],
            allowed_paths: vec!["scratch/".into(), "docs/".into()],
            allowed_destinations: vec![],
            max_actions: 8,
            actions_used: 0,
            max_risk: RiskTier::T2,
            policy_version: crate::policy::POLICY_VERSION.into(),
            nonce: format!("nonce-{agent_id}"),
            allow_delegation: false,
            revocation_epoch: 0,
        }
    }

    pub fn expired(&self, now: u64) -> bool {
        now >= self.expires_at
    }

    pub fn not_yet_valid(&self, now: u64) -> bool {
        now < self.not_before
    }

    pub fn budget_exceeded(&self) -> bool {
        self.actions_used >= self.max_actions
    }

    pub fn tool_allowed(&self, tool: &str) -> bool {
        self.allowed_tools.iter().any(|t| t == tool)
    }

    pub fn path_allowed(&self, path: &str) -> bool {
        if path.contains("..") {
            return false;
        }
        self.allowed_paths.iter().any(|p| path.starts_with(p))
    }

    pub fn dest_allowed(&self, dest: &str) -> bool {
        self.allowed_destinations.iter().any(|d| d == dest)
    }

    pub fn risk_allowed(&self, class: ActionClass) -> bool {
        class.risk() <= self.max_risk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(agent_id: &str, now: u64) -> CapabilityManifest {
        CapabilityManifest::local_reader(agent_id, now)
    }

    #[test]
    fn fresh_manifest_is_not_expired() {
        let m = manifest("a1", 1000);
        assert!(!m.expired(1000));
        assert!(!m.expired(1000 + 3599));
    }

    #[test]
    fn manifest_expires_at_boundary() {
        let m = manifest("a1", 1000);
        // expires_at = now + 3600 = 4600
        assert!(m.expired(4600));
    }

    #[test]
    fn not_yet_valid_before_not_before() {
        let mut m = manifest("a1", 1000);
        m.not_before = 2000;
        assert!(m.not_yet_valid(1999));
        assert!(!m.not_yet_valid(2000));
    }

    #[test]
    fn budget_exceeded_when_actions_used_eq_max() {
        let mut m = manifest("a1", 0);
        m.max_actions = 3;
        m.actions_used = 3;
        assert!(m.budget_exceeded());
        m.actions_used = 2;
        assert!(!m.budget_exceeded());
    }

    #[test]
    fn tool_allowed_exact_match() {
        let m = manifest("a1", 0);
        assert!(m.tool_allowed("local_read"));
        assert!(!m.tool_allowed("exec_shell"));
    }

    #[test]
    fn path_allowed_prefix_check() {
        let m = manifest("a1", 0);
        assert!(m.path_allowed("docs/notes.txt"));
        assert!(m.path_allowed("scratch/tmp"));
        assert!(!m.path_allowed("secrets/vault"));
    }

    #[test]
    fn path_with_traversal_is_denied() {
        let m = manifest("a1", 0);
        assert!(!m.path_allowed("docs/../secrets"));
    }

    #[test]
    fn revocation_list_idempotent() {
        let mut rev = RevocationList::default();
        rev.revoke("id-x");
        rev.revoke("id-x");
        assert!(rev.is_revoked("id-x"));
        assert!(!rev.is_revoked("id-y"));
    }
}
