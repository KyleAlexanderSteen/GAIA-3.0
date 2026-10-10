use std::sync::Arc;
use std::time::Duration;

use gaia_acp::{
    ActionClass, CapabilityManifest, HumanApprovalReceipt, PolicyDecision, PolicyEngine,
    PolicyEvaluationContext, ProposedAction, RevocationList, SignedIntent,
};
use gaia_kernel::broker::{Broker, Capabilities};
use gaia_kernel::executor::{Executor, IntentReceipt};
use gaia_kernel::{Principal, PrincipalKind};
use gaia_runtime::{SandboxManager, SandboxProfile};

/// Trusted authorization source for guarded component execution.
///
/// Production implementations must bind the intent to an authenticated caller,
/// verify the intent's cryptographic authenticity, resolve capabilities from
/// trusted control-plane state, and persist approval consumption atomically.
/// Request payloads and conversational text are never trusted approval inputs.
pub trait AuthorizationProvider: Send + Sync {
    fn verify_intent(&self, intent: &SignedIntent) -> Result<(), String>;

    fn resolve_manifest(
        &self,
        intent: &SignedIntent,
        action: &ProposedAction,
    ) -> Result<CapabilityManifest, String>;

    fn revocations(&self) -> Result<RevocationList, String>;

    fn emergency_stop(&self) -> Result<bool, String>;

    fn resolve_approval(
        &self,
        intent: &SignedIntent,
        action: &ProposedAction,
    ) -> Result<Option<HumanApprovalReceipt>, String>;

    fn consumed_approval_ids(&self) -> Result<Vec<String>, String>;

    /// Atomically marks a single-use approval consumed. Returns false if
    /// another request already consumed it. Implementations used in production
    /// must make this durable across processes/restarts.
    fn consume_approval_once(&self, approval_id: &str) -> Result<bool, String>;
}

/// Canonical local dispatch entry used by the CLI and gateway.
/// This narrow path only admits the existing local intent grammar; it does not
/// execute arbitrary system calls or WASM components.
pub fn dispatch_intent(id: &str, payload: &str) -> IntentReceipt {
    let now = unix_now();
    let manifest = CapabilityManifest::local_reader("gaia-local", now);
    let intent = SignedIntent {
        intent_id: id.to_owned(),
        goal_class: "local-parse".into(),
        issued_at: now,
        expires_at: now.saturating_add(60),
    };
    let action = ProposedAction {
        agent_id: manifest.agent_id.clone(),
        tool: "local_parse".into(),
        method: "ADMIT".into(),
        target: "intent".into(),
        action_class: ActionClass::LocalParse,
        payload: payload.to_owned(),
        nonce: manifest.nonce.clone(),
        gateway_id: manifest.gateway_id.clone(),
        server_id: manifest.server_id.clone(),
        resource_id: manifest.resource_id.clone(),
        wants_delegation: false,
    };
    let revoked = RevocationList::default();
    let decision = PolicyEngine::evaluate(PolicyEvaluationContext {
        now,
        intent: &intent,
        manifest: &manifest,
        action: &action,
        approval: None,
        revoked: &revoked,
        emergency_stop: false,
        consumed_approvals: &[],
        untrusted: None,
    });
    if !decision.is_allow() {
        return IntentReceipt {
            id: id.to_owned(),
            status: "refused",
            detail: decision.reason().as_str().to_owned(),
        };
    }

    let broker = Arc::new(Broker::new());
    let principal = Principal::generate(PrincipalKind::Node);
    let executor = Executor::new(&principal, Capabilities::default(), broker);
    executor.run_intent(id, payload)
}

/// Resolve trusted authority, authorize with ACP, enforce the human approval
/// boundary, then compile and execute inside the deny-by-default WASM sandbox.
///
/// Ordering is intentional: no component compilation or execution occurs
/// before trusted intent verification and ACP authorization. A single-use
/// approval is atomically consumed before sandbox work, so concurrent requests
/// cannot both execute with the same receipt. Failed execution does not restore
/// a consumed approval.
pub async fn dispatch_component_authorized(
    component_bytes: &[u8],
    now: u64,
    intent: &SignedIntent,
    action: &ProposedAction,
    authority: &dyn AuthorizationProvider,
) -> Result<(), String> {
    authority
        .verify_intent(intent)
        .map_err(|e| format!("intent-verification-failed: {e}"))?;

    let manifest = authority
        .resolve_manifest(intent, action)
        .map_err(|e| format!("capability-resolution-failed: {e}"))?;
    let revoked = authority
        .revocations()
        .map_err(|e| format!("revocation-resolution-failed: {e}"))?;
    let emergency_stop = authority
        .emergency_stop()
        .map_err(|e| format!("emergency-stop-resolution-failed: {e}"))?;
    let approval = authority
        .resolve_approval(intent, action)
        .map_err(|e| format!("approval-resolution-failed: {e}"))?;
    let consumed_approvals = authority
        .consumed_approval_ids()
        .map_err(|e| format!("approval-replay-state-failed: {e}"))?;

    let decision = PolicyEngine::evaluate(PolicyEvaluationContext {
        now,
        intent,
        manifest: &manifest,
        action,
        approval: approval.as_ref(),
        revoked: &revoked,
        emergency_stop,
        consumed_approvals: &consumed_approvals,
        untrusted: None,
    });
    match decision {
        PolicyDecision::Allow { .. } => {}
        PolicyDecision::Deny { reason } | PolicyDecision::RequireApproval { reason } => {
            return Err(reason.as_str().to_owned());
        }
    }

    if let Some(receipt) = approval.as_ref().filter(|receipt| receipt.single_use) {
        let consumed = authority
            .consume_approval_once(&receipt.id)
            .map_err(|e| format!("approval-consumption-failed: {e}"))?;
        if !consumed {
            return Err(gaia_acp::ReasonCode::ApprovalReplay.as_str().to_owned());
        }
    }

    // Do not derive sandbox privileges from the request. This profile is
    // deliberately deny-by-default until a reviewed capability-to-profile
    // mapping exists for a specific, supported capability.
    let sandbox = SandboxManager::new(SandboxProfile::default())
        .map_err(|e| format!("sandbox-init-failed: {e}"))?;
    let component = sandbox
        .compile(component_bytes)
        .map_err(|e| format!("sandbox-compile-failed: {e}"))?;
    sandbox
        .execute_component_with_load(&component, Duration::from_secs(5), 5_000)
        .await
        .0
        .map_err(|e| format!("sandbox-execution-failed: {e}"))
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct TestAuthority {
        manifest: CapabilityManifest,
        revoked: RevocationList,
        approval: Option<HumanApprovalReceipt>,
        consumed: Mutex<Vec<String>>,
        verify_ok: bool,
        emergency_stop: bool,
    }

    impl AuthorizationProvider for TestAuthority {
        fn verify_intent(&self, _intent: &SignedIntent) -> Result<(), String> {
            if self.verify_ok { Ok(()) } else { Err("invalid test signature".into()) }
        }

        fn resolve_manifest(
            &self,
            _intent: &SignedIntent,
            _action: &ProposedAction,
        ) -> Result<CapabilityManifest, String> {
            Ok(self.manifest.clone())
        }

        fn revocations(&self) -> Result<RevocationList, String> {
            Ok(self.revoked.clone())
        }

        fn emergency_stop(&self) -> Result<bool, String> {
            Ok(self.emergency_stop)
        }

        fn resolve_approval(
            &self,
            _intent: &SignedIntent,
            _action: &ProposedAction,
        ) -> Result<Option<HumanApprovalReceipt>, String> {
            Ok(self.approval.clone())
        }

        fn consumed_approval_ids(&self) -> Result<Vec<String>, String> {
            Ok(self.consumed.lock().unwrap().clone())
        }

        fn consume_approval_once(&self, approval_id: &str) -> Result<bool, String> {
            let mut consumed = self.consumed.lock().unwrap();
            if consumed.iter().any(|id| id == approval_id) {
                return Ok(false);
            }
            consumed.push(approval_id.to_owned());
            Ok(true)
        }
    }

    fn fixture(action_class: ActionClass, approval_required: bool) -> (SignedIntent, ProposedAction, TestAuthority) {
        let now = unix_now();
        let mut manifest = CapabilityManifest::local_reader("test-agent", now);
        let tool = if approval_required { "external_write" } else { "local_parse" };
        manifest.allowed_tools.push(tool.into());
        manifest.max_risk = gaia_acp::RiskTier::T3;
        let intent = SignedIntent {
            intent_id: "dispatch-test".into(),
            goal_class: "test".into(),
            issued_at: now,
            expires_at: now + 30,
        };
        let action = ProposedAction {
            agent_id: manifest.agent_id.clone(),
            tool: tool.into(),
            method: if approval_required { "WRITE" } else { "ADMIT" }.into(),
            target: if approval_required { "https://example.com/resource" } else { "intent" }.into(),
            action_class,
            payload: "fixture".into(),
            nonce: manifest.nonce.clone(),
            gateway_id: manifest.gateway_id.clone(),
            server_id: manifest.server_id.clone(),
            resource_id: manifest.resource_id.clone(),
            wants_delegation: false,
        };
        let approval = if approval_required {
            Some(HumanApprovalReceipt::grant_for(
                "approval-1", "human-reviewer", &intent, &action, now + 20,
            ))
        } else {
            None
        };
        let authority = TestAuthority {
            manifest,
            revoked: RevocationList::default(),
            approval,
            consumed: Mutex::new(Vec::new()),
            verify_ok: true,
            emergency_stop: false,
        };
        (intent, action, authority)
    }

    #[test]
    fn recorded_receipt_preserves_caller_id_and_detail() {
        let receipt = dispatch_intent("shared-1", "echo: same path");
        assert_eq!(receipt.id, "shared-1");
        assert_eq!(receipt.status, "recorded");
        assert_eq!(receipt.detail, "same path");
    }

    #[test]
    fn unsupported_intent_is_not_reported_as_success() {
        let receipt = dispatch_intent("shared-2", "perform an unsupported action");
        assert_eq!(receipt.status, "not-executed");
        assert_ne!(receipt.status, "done");
    }

    #[test]
    fn empty_intent_is_refused() {
        assert_eq!(dispatch_intent("shared-3", " ").status, "refused");
    }

    #[tokio::test]
    async fn approval_is_required_before_invalid_component_is_compiled() {
        let (intent, action, mut authority) = fixture(ActionClass::ExternalWrite, true);
        authority.approval = None;
        let error = dispatch_component_authorized(&[0, 1, 2], unix_now(), &intent, &action, &authority)
            .await
            .unwrap_err();
        assert_eq!(error, gaia_acp::ReasonCode::ApprovalMissing.as_str());
        assert!(authority.consumed.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn intent_verification_failure_stops_before_capability_resolution() {
        let (intent, action, mut authority) = fixture(ActionClass::LocalParse, false);
        authority.verify_ok = false;
        let error = dispatch_component_authorized(&[0, 1, 2], unix_now(), &intent, &action, &authority)
            .await
            .unwrap_err();
        assert!(error.starts_with("intent-verification-failed:"));
    }

    #[tokio::test]
    async fn emergency_stop_denies_before_sandbox_compilation() {
        let (intent, action, mut authority) = fixture(ActionClass::LocalParse, false);
        authority.emergency_stop = true;
        let error = dispatch_component_authorized(&[0, 1, 2], unix_now(), &intent, &action, &authority)
            .await
            .unwrap_err();
        assert_eq!(error, gaia_acp::ReasonCode::EmergencyStop.as_str());
    }

    #[tokio::test]
    async fn single_use_approval_is_consumed_before_sandbox_work() {
        let (intent, action, authority) = fixture(ActionClass::ExternalWrite, true);
        // Invalid component bytes prove the approval was consumed after policy
        // authorization but before compilation/execution.
        let error = dispatch_component_authorized(&[0, 1, 2], unix_now(), &intent, &action, &authority)
            .await
            .unwrap_err();
        assert!(error.starts_with("sandbox-compile-failed:"));
        assert_eq!(authority.consumed.lock().unwrap().as_slice(), &["approval-1"]);
        let replay = dispatch_component_authorized(&[0, 1, 2], unix_now(), &intent, &action, &authority)
            .await
            .unwrap_err();
        assert_eq!(replay, gaia_acp::ReasonCode::ApprovalReplay.as_str());
    }
}
