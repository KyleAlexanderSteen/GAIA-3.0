use gaia_acp::*;

fn now() -> u64 {
    1_700_000_000
}

fn action() -> ProposedAction {
    ProposedAction {
        agent_id: "agent-a".into(),
        tool: "local_read".into(),
        method: "call".into(),
        target: "docs/a.md".into(),
        action_class: ActionClass::LocalRead,
        payload: "ok".into(),
        nonce: "nonce-agent-a".into(),
        gateway_id: "gateway-local".into(),
        server_id: "server-local".into(),
        resource_id: "repo-local".into(),
        wants_delegation: false,
    }
}

#[test]
fn actor_cannot_resume_after_kill() {
    let mut p = ControlPlane::start(now(), "agent-a").unwrap();
    p.kill("agent-a");
    assert_eq!(actor_resume(&mut p, "agent-a"), Err(ReasonCode::EmergencyStop));
    let mut m = CapabilityManifest::local_reader("agent-a", now());
    assert!(!p.invoke(&mut m, &action(), None, None).executed);
}

#[test]
fn agent_shaped_receipt_cannot_clear_halt() {
    let mut p = ControlPlane::start(now(), "agent-a").unwrap();
    p.kill("agent-a");
    let fake = HumanHaltReceipt {
        human_id: "agent-a".into(),
    };
    assert_eq!(
        human_resume(&mut p, "agent-a", &fake),
        Err(ReasonCode::EmergencyStop)
    );
}

#[test]
fn human_can_clear_halt() {
    let mut p = ControlPlane::start(now(), "agent-a").unwrap();
    p.kill("agent-a");
    let human = HumanHaltReceipt {
        human_id: "human:owner".into(),
    };
    assert!(human_resume(&mut p, "agent-a", &human).is_ok());
    let mut m = CapabilityManifest::local_reader("agent-a", now());
    assert!(p.invoke(&mut m, &action(), None, None).executed);
}
