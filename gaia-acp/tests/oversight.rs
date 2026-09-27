use gaia_acp::*;

#[test]
fn actor_cannot_pass_own_run() {
    assert_eq!(
        record_verdict("agent-a", "agent-a", true),
        Err(ReasonCode::CrossAgent)
    );
}

#[test]
fn other_agent_cannot_grade() {
    assert_eq!(
        record_verdict("agent-a", "agent-b", true),
        Err(ReasonCode::CrossAgent)
    );
}

#[test]
fn overseer_can_fail_the_run() {
    let v = record_verdict("agent-a", "overseer:core", false).unwrap();
    assert_eq!(v.grader_id, "overseer:core");
    assert!(!v.pass);
}

#[test]
fn actor_cannot_halt_wash() {
    let mut p = ControlPlane::start(1_700_000_000, "agent-a").unwrap();
    p.kill("agent-a");
    assert_eq!(
        halt_wash(&mut p, "agent-a", "agent-a"),
        Err(ReasonCode::EmergencyStop)
    );
}
