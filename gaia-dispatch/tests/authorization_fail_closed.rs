//! Security regression tests for the authorization-to-sandbox boundary.
//!
//! Invalid component bytes are deliberate sentinels in deny-path tests: if
//! policy is bypassed, the result becomes a sandbox compile error instead of
//! the expected ACP denial. The failure-path test uses a real trapping WASM
//! component and proves a failed run does not make its single-use approval
//! reusable.

use std::sync::Mutex;

use gaia_acp::{
    ActionClass, CapabilityManifest, HumanApprovalReceipt, ProposedAction, RevocationList,
    RiskTier, SignedIntent,
};
use gaia_dispatch::{dispatch_component_authorized, AuthorizationProvider};

const TRAPPING_COMPONENT: &str = r#"
(component
  (core module $m
    (func (export "run")
      unreachable
    )
  )
  (core instance $i (instantiate $m))
  (func (export "run") (canon lift (core func $i "run")))
)
"#;

struct TestAuthority {
    manifest: CapabilityManifest,
    revoked: RevocationList,
    approval: Option<HumanApprovalReceipt>,
    consumed: Mutex<Vec<String>>,
}

impl AuthorizationProvider for TestAuthority {
    fn verify_intent(&self, _intent: &SignedIntent) -> Result<(), String> {
        Ok(())
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
        Ok(false)
    }

    fn resolve_approval(
        &self,
        _intent: &SignedIntent,
        _action: &ProposedAction,
    ) -> Result<Option<HumanApprovalReceipt>, String> {
        Ok(self.approval.clone())
    }

    fn consumed_approval_ids(&self) -> Result<Vec<String>, String> {
        Ok(self.consumed.lock().expect("approval mutex poisoned").clone())
    }

    fn consume_approval_once(&self, approval_id: &str) -> Result<bool, String> {
        let mut consumed = self.consumed.lock().expect("approval mutex poisoned");
        if consumed.iter().any(|id| id == approval_id) {
            return Ok(false);
        }
        consumed.push(approval_id.to_owned());
        Ok(true)
    }
}

fn fixture(requires_approval: bool) -> (u64, SignedIntent, ProposedAction, TestAuthority) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_secs();
    let mut manifest = CapabilityManifest::local_reader("policy-test-agent", now);
    let (tool, method, target, action_class) = if requires_approval {
        ("external_write", "WRITE", "https://example.com/resource", ActionClass::ExternalWrite)
    } else {
        ("local_parse", "ADMIT", "intent", ActionClass::LocalParse)
    };
    manifest.allowed_tools.push(tool.to_owned());
    manifest.max_risk = RiskTier::T3;

    let intent = SignedIntent {
        intent_id: "fail-closed-test-intent".into(),
        goal_class: "test".into(),
        issued_at: now,
        expires_at: now + 30,
    };
    let action = ProposedAction {
        agent_id: manifest.agent_id.clone(),
        tool: tool.into(),
        method: method.into(),
        target: target.into(),
        action_class,
        payload: "security regression fixture".into(),
        nonce: manifest.nonce.clone(),
        gateway_id: manifest.gateway_id.clone(),
        server_id: manifest.server_id.clone(),
        resource_id: manifest.resource_id.clone(),
        wants_delegation: false,
    };
    let approval = requires_approval.then(|| {
        HumanApprovalReceipt::grant_for(
            "single-use-approval",
            "human-reviewer",
            &intent,
            &action,
            now + 20,
        )
    });
    let authority = TestAuthority {
        manifest,
        revoked: RevocationList::default(),
        approval,
        consumed: Mutex::new(Vec::new()),
    };
    (now, intent, action, authority)
}

#[tokio::test]
async fn explicit_policy_denial_stops_before_component_compilation() {
    let (now, intent, action, mut authority) = fixture(false);
    authority.manifest.allowed_tools.retain(|tool| tool != &action.tool);

    // If execution crosses the policy boundary, these bytes produce a compile
    // error. The required ACP denial proves the sandbox was never reached.
    let error = dispatch_component_authorized(&[0, 1, 2], now, &intent, &action, &authority)
        .await
        .expect_err("an unlisted tool must be denied");
    assert_eq!(error, gaia_acp::ReasonCode::ToolNotListed.as_str());
    assert!(authority.consumed.lock().expect("approval mutex poisoned").is_empty());
}

#[tokio::test]
async fn missing_required_approval_stops_before_component_compilation() {
    let (now, intent, action, mut authority) = fixture(true);
    authority.approval = None;

    // Invalid bytes make any accidental compile/execution path observable.
    let error = dispatch_component_authorized(&[0, 1, 2], now, &intent, &action, &authority)
        .await
        .expect_err("an external write without approval must be denied");
    assert_eq!(error, gaia_acp::ReasonCode::ApprovalMissing.as_str());
    assert!(authority.consumed.lock().expect("approval mutex poisoned").is_empty());
}

#[tokio::test]
async fn failed_execution_is_not_success_and_does_not_restore_single_use_approval() {
    let (now, intent, action, authority) = fixture(true);
    let bytes = wat::parse_str(TRAPPING_COMPONENT).expect("valid trapping component fixture");

    let error = dispatch_component_authorized(&bytes, now, &intent, &action, &authority)
        .await
        .expect_err("a trapping component must not be reported as successful");
    assert!(
        error.starts_with("sandbox-execution-failed:"),
        "runtime failure must remain an execution failure, got: {error}"
    );
    assert_eq!(
        authority.consumed.lock().expect("approval mutex poisoned").as_slice(),
        &["single-use-approval"]
    );

    let replay = dispatch_component_authorized(&bytes, now, &intent, &action, &authority)
        .await
        .expect_err("a consumed approval must not authorize a retry");
    assert_eq!(replay, gaia_acp::ReasonCode::ApprovalReplay.as_str());
}
