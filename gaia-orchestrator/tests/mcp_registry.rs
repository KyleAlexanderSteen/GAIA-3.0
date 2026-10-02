//! #23 depth: in-process MCP only. No TCP. No mDNS.

use gaia_memos::MemOs;
use gaia_orchestrator::{
    DiscoveryStub, IntentEngine, IntentSigner, JsonRpcRequest, McpMessage, McpRegistry,
};

#[test]
fn registry_lists_local_aip_manifests() {
    let reg = McpRegistry::local();
    assert!(!reg.live_wire());
    assert!(reg.listen_tcp("127.0.0.1:0").is_err());
    assert!(reg.list().iter().any(|a| a.name == "gaia-local-researcher"));
}

#[test]
fn unsigned_mcp_is_rejected() {
    let mut reg = McpRegistry::local();
    let msg = McpMessage::unsigned("research.summarize", "{}");
    assert!(reg.invoke(&msg).unwrap_err().contains("unsigned"));
    assert_eq!(reg.handler_invocations, 0);
}

#[test]
fn signed_tool_invokes_from_intent() {
    let mut mem = MemOs::new();
    let g = IntentEngine::local_stub()
        .parse("research and summarize CARE", &mut mem)
        .unwrap();
    let out = McpRegistry::local()
        .invoke_from_intent(&g, &IntentSigner::generate())
        .unwrap();
    assert!(out.starts_with("mcp-not-executed:research.summarize"));
    assert!(out.contains(&format!("intent={}", g.id)));
    assert!(out.contains("audit="));
}

#[test]
fn discovery_stub_lists_without_network() {
    let reg = McpRegistry::local();
    let disc = DiscoveryStub;
    disc.advertise(&reg.list()[0]).unwrap();
    assert!(!disc.browse_local(&reg).is_empty());
    assert!(disc.browse_mdns().is_err());
}

#[test]
fn forged_mcp_token_is_rejected() {
    let mut reg = McpRegistry::local();
    let msg = McpMessage::marked_signed("research.summarize", "{}", "v0-token");
    assert!(reg
        .invoke(&msg)
        .unwrap_err()
        .contains("GAIA_SIGNATURE_INVALID"));
    assert_eq!(reg.handler_invocations, 0);
}

#[test]
fn jsonrpc_session_lists_tools_and_resources() {
    let mut reg = McpRegistry::local();
    let signer = IntentSigner::generate();
    let tools = reg.handle(JsonRpcRequest::signed(&signer, 1, "tools/list", "{}"));
    assert!(tools.result.unwrap().contains("research.summarize"));
    let resources = reg.handle(JsonRpcRequest::signed(&signer, 2, "resources/list", "{}"));
    assert!(resources.result.unwrap().contains("agent.researcher"));
}

#[test]
fn jsonrpc_tools_call_requires_signature() {
    let mut reg = McpRegistry::local();
    let unsigned = reg.handle(JsonRpcRequest::unsigned(
        3,
        "tools/call",
        "research.summarize",
    ));
    assert!(unsigned.error.unwrap().contains("unsigned"));
    let ok = reg.handle(JsonRpcRequest::signed(
        &IntentSigner::generate(),
        4,
        "tools/call",
        "research.summarize",
    ));
    assert!(ok.result.unwrap().starts_with("mcp-not-executed:research.summarize"));
}
