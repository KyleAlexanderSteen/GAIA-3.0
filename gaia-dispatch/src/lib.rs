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

/// Canonical local dispatch entry used by the CLI and gateway.
/// Policy-checks the request before invoking the existing narrow kernel admission path.
/// This function does not execute arbitrary system calls or WASM components.
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

/// Execute a WASM component only after ACP capability/policy checks and, when
/// required by the action class, a valid human approval receipt.
/// The caller must supply the manifest and action from trusted control-plane
/// state; conversational text is not an approval mechanism.
pub async fn dispatch_component(
    component_bytes: &[u8],
    now: u64,
    intent: &SignedIntent,
    manifest: &CapabilityManifest,
    action: &ProposedAction,
    approval: Option<&HumanApprovalReceipt>,
    revoked: &RevocationList,
    consumed_approvals: &[String],
    emergency_stop: bool,
) -> Result<(), String> {
    let decision = PolicyEngine::evaluate(PolicyEvaluationContext {
        now,
        intent,
        manifest,
        action,
        approval,
        revoked,
        emergency_stop,
        consumed_approvals,
        untrusted: None,
    });
    match decision {
        PolicyDecision::Allow { .. } => {}
        PolicyDecision::Deny { reason } | PolicyDecision::RequireApproval { reason } => {
            return Err(reason.as_str().to_owned());
        }
    }

    // Keep network, inherited environment, and filesystem access disabled unless
    // a future reviewed capability-to-profile mapping explicitly grants them.
    let sandbox = SandboxManager::new(SandboxProfile::default())
        .map_err(|e| format!("sandbox-init-failed: {e}"))?;
    let component = sandbox
        .compile(component_bytes)
        .map_err(|e| format!("sandbox-compile-failed: {e}"))?;
    sandbox
        .execute_component_with_load(
            &component,
            Duration::from_secs(5),
            5_000,
        )
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

    #[test]
    fn elevated_action_without_approval_is_denied() {
        let now = unix_now();
        let mut manifest = CapabilityManifest::local_reader("gaia-local", now);
        manifest.allowed_tools.push("external_write".into());
        manifest.max_risk = gaia_acp::RiskTier::T3;
        let intent = SignedIntent {
            intent_id: "approval-test".into(),
            goal_class: "dangerous".into(),
            issued_at: now,
            expires_at: now + 30,
        };
        let action = ProposedAction {
            agent_id: manifest.agent_id.clone(),
            tool: "external_write".into(),
            method: "WRITE".into(),
            target: "https://example.com/resource".into(),
            action_class: ActionClass::ExternalWrite,
            payload: "test".into(),
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
        assert!(!decision.is_allow());
        assert_eq!(decision.reason(), gaia_acp::ReasonCode::ApprovalMissing);
    }
}
