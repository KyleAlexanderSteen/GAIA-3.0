//! Capability revocation enforcer. In-process only. Author: Kyle Steen
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct CapabilityToken {
    pub token_id: String,
    pub issuer: String,
    pub subject: String,
    pub parent_token_id: Option<String>,
    pub expires_at: u64,
    pub operations: Vec<String>,
    pub resource: String,
    pub policy_version: String,
}

#[derive(Debug, Clone)]
pub struct RevocationRecord {
    pub token_id: String,
    pub reason: String,
    pub revoked_by: String,
    pub epoch: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthzDecision {
    Allow,
    Deny { code: DenyCode },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DenyCode {
    TokenNotFound,
    TokenExpired,
    TokenRevoked,
    AncestorRevoked,
    OperationNotAllowed,
    ResourceNotAllowed,
    SubjectMismatch,
}

#[derive(Debug, Clone)]
pub struct AuthzAuditEvent {
    pub event_id: u64,
    pub token_id: String,
    pub subject: String,
    pub resource: String,
    pub operation: String,
    pub decision: AuthzDecision,
    pub epoch: u64,
}

pub struct CapabilityEnforcer {
    tokens: HashMap<String, CapabilityToken>,
    revocations: HashMap<String, RevocationRecord>,
    audit_log: Vec<AuthzAuditEvent>,
    epoch: u64,
}

impl CapabilityEnforcer {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            revocations: HashMap::new(),
            audit_log: Vec::new(),
            epoch: 0,
        }
    }

    pub fn register(&mut self, token: CapabilityToken) {
        self.tokens.insert(token.token_id.clone(), token);
    }

    pub fn revoke(&mut self, token_id: &str, reason: &str, revoked_by: &str) -> bool {
        if self.revocations.contains_key(token_id) {
            return false;
        }
        self.epoch += 1;
        self.revocations.insert(
            token_id.to_owned(),
            RevocationRecord {
                token_id: token_id.to_owned(),
                reason: reason.to_owned(),
                revoked_by: revoked_by.to_owned(),
                epoch: self.epoch,
            },
        );
        true
    }

    pub fn authorize(
        &mut self,
        token_id: &str,
        subject: &str,
        resource: &str,
        operation: &str,
    ) -> AuthzDecision {
        self.epoch += 1;
        let epoch = self.epoch;
        let decision = self.evaluate(token_id, subject, resource, operation);
        self.audit_log.push(AuthzAuditEvent {
            event_id: self.audit_log.len() as u64 + 1,
            token_id: token_id.to_owned(),
            subject: subject.to_owned(),
            resource: resource.to_owned(),
            operation: operation.to_owned(),
            decision: decision.clone(),
            epoch,
        });
        decision
    }

    fn evaluate(
        &self,
        token_id: &str,
        subject: &str,
        resource: &str,
        operation: &str,
    ) -> AuthzDecision {
        let Some(token) = self.tokens.get(token_id) else {
            return AuthzDecision::Deny {
                code: DenyCode::TokenNotFound,
            };
        };
        if token.subject != subject {
            return AuthzDecision::Deny {
                code: DenyCode::SubjectMismatch,
            };
        }
        if unix_now() > token.expires_at {
            return AuthzDecision::Deny {
                code: DenyCode::TokenExpired,
            };
        }
        if self.revocations.contains_key(token_id) {
            return AuthzDecision::Deny {
                code: DenyCode::TokenRevoked,
            };
        }
        if let Some(parent_id) = &token.parent_token_id {
            if self.ancestor_revoked(parent_id) {
                return AuthzDecision::Deny {
                    code: DenyCode::AncestorRevoked,
                };
            }
        }
        if !token.operations.iter().any(|op| op == operation) {
            return AuthzDecision::Deny {
                code: DenyCode::OperationNotAllowed,
            };
        }
        if !resource_matches(&token.resource, resource) {
            return AuthzDecision::Deny {
                code: DenyCode::ResourceNotAllowed,
            };
        }
        AuthzDecision::Allow
    }

    fn ancestor_revoked(&self, token_id: &str) -> bool {
        let mut visited: HashSet<String> = HashSet::new();
        let mut current = token_id.to_owned();
        loop {
            if !visited.insert(current.clone()) {
                break;
            }
            if self.revocations.contains_key(&current) {
                return true;
            }
            let Some(token) = self.tokens.get(&current) else {
                break;
            };
            let Some(parent_id) = &token.parent_token_id else {
                break;
            };
            current = parent_id.clone();
        }
        false
    }

    pub fn audit_log(&self) -> &[AuthzAuditEvent] {
        &self.audit_log
    }

    pub fn revocation_record(&self, token_id: &str) -> Option<&RevocationRecord> {
        self.revocations.get(token_id)
    }
}

impl Default for CapabilityEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

fn resource_matches(selector: &str, resource: &str) -> bool {
    if selector == "*" {
        return true;
    }
    if let Some(prefix) = selector.strip_suffix("/*") {
        return resource == prefix || resource.starts_with(&format!("{prefix}/"));
    }
    selector == resource
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn future_unix(secs: u64) -> u64 {
    unix_now() + secs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_token(id: &str, subject: &str, ops: &[&str], resource: &str) -> CapabilityToken {
        CapabilityToken {
            token_id: id.to_owned(),
            issuer: "did:key:gaia:issuer".to_owned(),
            subject: subject.to_owned(),
            parent_token_id: None,
            expires_at: future_unix(3600),
            operations: ops.iter().map(|s| s.to_string()).collect(),
            resource: resource.to_owned(),
            policy_version: "v0.1".to_owned(),
        }
    }

    #[test]
    fn conformance_revoked_agent_case() {
        let mut e = CapabilityEnforcer::new();
        e.register(make_token(
            "grant-001",
            "did:key:gaia:agent-researcher",
            &["memory.read"],
            "memcube:project/*",
        ));
        assert_eq!(
            e.authorize(
                "grant-001",
                "did:key:gaia:agent-researcher",
                "memcube:project/notes",
                "memory.read",
            ),
            AuthzDecision::Allow
        );
        assert!(e.revoke("grant-001", "user-requested", "did:key:gaia:human"));
        assert_eq!(
            e.authorize(
                "grant-001",
                "did:key:gaia:agent-researcher",
                "memcube:project/notes",
                "memory.read",
            ),
            AuthzDecision::Deny {
                code: DenyCode::TokenRevoked
            }
        );
    }

    #[test]
    fn deny_delegated_token_when_parent_revoked() {
        let mut e = CapabilityEnforcer::new();
        e.register(make_token("parent", "did:agent-a", &["memory.read"], "memcube:project/*"));
        let mut child = make_token("child", "did:agent-b", &["memory.read"], "memcube:project/notes");
        child.parent_token_id = Some("parent".into());
        e.register(child);
        e.revoke("parent", "security-incident", "did:issuer");
        assert_eq!(
            e.authorize("child", "did:agent-b", "memcube:project/notes", "memory.read"),
            AuthzDecision::Deny {
                code: DenyCode::AncestorRevoked
            }
        );
    }
    #[test]
    fn authorization_branches_have_exact_deny_codes() {
        let mut e = CapabilityEnforcer::new();

        assert_eq!(
            e.authorize("missing", "did:agent-a", "memcube:project/notes", "memory.read"),
            AuthzDecision::Deny { code: DenyCode::TokenNotFound }
        );

        let mut expired = make_token("expired", "did:agent-a", &["memory.read"], "memcube:project/*");
        expired.expires_at = unix_now().saturating_sub(1);
        e.register(expired);
        assert_eq!(
            e.authorize("expired", "did:agent-a", "memcube:project/notes", "memory.read"),
            AuthzDecision::Deny { code: DenyCode::TokenExpired }
        );

        e.register(make_token("subject", "did:agent-a", &["memory.read"], "memcube:project/*"));
        assert_eq!(
            e.authorize("subject", "did:agent-b", "memcube:project/notes", "memory.read"),
            AuthzDecision::Deny { code: DenyCode::SubjectMismatch }
        );

        e.register(make_token("operation", "did:agent-a", &["memory.read"], "memcube:project/*"));
        assert_eq!(
            e.authorize("operation", "did:agent-a", "memcube:project/notes", "memory.write"),
            AuthzDecision::Deny { code: DenyCode::OperationNotAllowed }
        );

        e.register(make_token("resource", "did:agent-a", &["memory.read"], "memcube:project/*"));
        assert_eq!(
            e.authorize("resource", "did:agent-a", "memcube:other/notes", "memory.read"),
            AuthzDecision::Deny { code: DenyCode::ResourceNotAllowed }
        );
    }

    #[test]
    fn resource_matching_covers_selector_boundaries() {
        assert!(resource_matches("*", "anything"));
        assert!(resource_matches("memcube:project/*", "memcube:project"));
        assert!(resource_matches("memcube:project/*", "memcube:project/notes"));
        assert!(resource_matches("memcube:project/*", "memcube:project/notes/today"));
        assert!(resource_matches("memcube:project", "memcube:project"));
        assert!(!resource_matches("memcube:project/*", "memcube:projects/notes"));
        assert!(!resource_matches("memcube:project/*", "memcube:project-other"));
        assert!(!resource_matches("memcube:project", "memcube:project/notes"));
    }

    #[test]
    fn returned_decision_matches_audit_event_exactly() {
        let mut e = CapabilityEnforcer::new();
        e.register(make_token(
            "grant-audit",
            "did:agent-a",
            &["memory.read"],
            "memcube:project/*",
        ));

        let decision = e.authorize(
            "grant-audit",
            "did:agent-a",
            "memcube:project/notes",
            "memory.read",
        );

        assert_eq!(decision, AuthzDecision::Allow);
        let event = e.audit_log().last().expect("authorization must be audited");
        assert_eq!(event.event_id, 1);
        assert_eq!(event.token_id, "grant-audit");
        assert_eq!(event.subject, "did:agent-a");
        assert_eq!(event.resource, "memcube:project/notes");
        assert_eq!(event.operation, "memory.read");
        assert_eq!(event.decision, decision);
        assert_eq!(event.epoch, 1);
    }

    #[test]
    fn revoked_and_ancestor_revoked_decisions_match_audit() {
        let mut e = CapabilityEnforcer::new();
        e.register(make_token(
            "revoked",
            "did:agent-a",
            &["memory.read"],
            "memcube:project/*",
        ));
        assert!(e.revoke("revoked", "user-requested", "did:human"));
        let revoked_decision = e.authorize(
            "revoked",
            "did:agent-a",
            "memcube:project/notes",
            "memory.read",
        );
        assert_eq!(
            revoked_decision,
            AuthzDecision::Deny { code: DenyCode::TokenRevoked }
        );
        assert_eq!(e.audit_log().last().unwrap().decision, revoked_decision);

        e.register(make_token(
            "parent-audit",
            "did:agent-a",
            &["memory.read"],
            "memcube:project/*",
        ));
        let mut child = make_token(
            "child-audit",
            "did:agent-b",
            &["memory.read"],
            "memcube:project/notes",
        );
        child.parent_token_id = Some("parent-audit".into());
        e.register(child);
        assert!(e.revoke("parent-audit", "security-incident", "did:issuer"));

        let ancestor_decision = e.authorize(
            "child-audit",
            "did:agent-b",
            "memcube:project/notes",
            "memory.read",
        );
        assert_eq!(
            ancestor_decision,
            AuthzDecision::Deny { code: DenyCode::AncestorRevoked }
        );
        assert_eq!(e.audit_log().last().unwrap().decision, ancestor_decision);
    }

    #[test]
    fn audit_event_order_and_epoch_are_deterministic() {
        let mut e = CapabilityEnforcer::new();
        e.register(make_token(
            "grant-order",
            "did:agent-a",
            &["memory.read"],
            "memcube:project/*",
        ));

        let first = e.authorize(
            "grant-order",
            "did:agent-a",
            "memcube:project/notes",
            "memory.read",
        );
        let second = e.authorize(
            "grant-order",
            "did:agent-a",
            "memcube:project/notes",
            "memory.write",
        );

        assert_eq!(first, AuthzDecision::Allow);
        assert_eq!(
            second,
            AuthzDecision::Deny { code: DenyCode::OperationNotAllowed }
        );
        assert_eq!(e.audit_log().len(), 2);
        assert_eq!(e.audit_log()[0].event_id, 1);
        assert_eq!(e.audit_log()[0].epoch, 1);
        assert_eq!(e.audit_log()[0].decision, first);
        assert_eq!(e.audit_log()[1].event_id, 2);
        assert_eq!(e.audit_log()[1].epoch, 2);
        assert_eq!(e.audit_log()[1].decision, second);
    }

}
