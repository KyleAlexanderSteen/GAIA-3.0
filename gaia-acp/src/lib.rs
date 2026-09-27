//! Local-first agent control plane. Fake adapters remain. MCP stdio is fixture-only. HTTP off.

mod adapter;
mod approval;
mod audit;
mod autonomy;
mod claim;
mod compaction;
mod config;
mod covert;
mod eval_awareness;
mod eval_pin;
mod failure_harness;
mod gateway;
mod grounding;
mod halt;
mod incident_scope;
mod live_trace;
mod manifest;
mod mcp_budget;
mod mcp_http_gate;
mod mcp_profile;
mod mcp_stdio;
mod oversight;
mod paste_egress;
mod policy;
mod predict;
mod quota;
mod registry;
mod sandbox;
mod timeout_lock;
pub mod tool_audit;
pub mod tool_auth;
mod trace;
mod types;

pub use adapter::{FakeAdapter, RecordingAdapter};
pub use approval::{ApprovalDecision, HumanApprovalReceipt};
pub use audit::{ActionReceipt, AuditChain, AuditPushInput, PlaneEvent, PlaneState};
pub use autonomy::{gate as autonomy_gate, AutonomyLevel, ConfirmDomain, PeerEnvelope};
pub use claim::{claim_gate, Claim};
pub use compaction::{compaction_is_untrusted, compaction_may_raise};
pub use config::{description_is_untrusted, digest_pinned, lint_mcp_config, McpServerConfig};
pub use covert::{classify_name_service, covert_invoke_allowed, resolver_is_egress};
pub use eval_awareness::{scenario_gate, AwarenessScenario, ScenarioStatus};
pub use eval_pin::{eval_target_allowed, eval_target_class};
pub use failure_harness::{run_fixture, FixtureKind, HarnessOutcome};
pub use gateway::{ControlPlane, InvokeResult};
pub use grounding::GroundingClaim;
pub use halt::{actor_resume, human_resume, HumanHaltReceipt};
pub use incident_scope::{
    description_may_invoke, payload_secret_shaped, reject_secret_payload, shared_board_allowed,
};
pub use live_trace::{
    credential_is_absent, map_row, try_forward, LiveSendError, LiveTraceConfig, LiveTraceMode,
    LiveTraceRole, LiveTraceRow, LiveTraceTransport, RecordingLiveTransport,
};
pub use manifest::{CapabilityManifest, IdentityKind, PrincipalId, RevocationList};
pub use mcp_budget::{discover_tools, Descriptor, DiscoveryCaps};
pub use mcp_http_gate::{
    audience_ok, http_enabled, redirect_exact, ssrf_block, STREAMABLE_HTTP_ENABLED,
};
pub use mcp_profile::{classify_method, MethodClass, MCP_SPEC, SUPPORTED};
pub use mcp_stdio::{parse_frame, StdioError, MAX_FRAME};
pub use oversight::{halt_wash, record_verdict, Verdict};
pub use paste_egress::{impossible_task_next, paste_dest_class, paste_invoke_allowed};
pub use policy::{PolicyDecision, PolicyEngine, PolicyEvaluationContext, POLICY_VERSION};
pub use predict::{honesty, ForecastKind, HonestyBand};
pub use quota::{resource_quota_gate, ResourceQuota, ResourceUsage};
pub use registry::{LocalAip, LocalRegistry};
pub use sandbox::{
    classify_destination, classify_rebinding_host, classify_redirect, EgressClass, SandboxProfile,
};
pub use timeout_lock::actor_set_timeout;
pub use tool_audit::{ToolAuditEntry, ToolAuditLog, ToolOutcome};
pub use tool_auth::{
    authorize as authorize_tool, AgentId as ToolAgentId, ApprovalToken, AuthError, ToolCall,
    ToolId as AuthToolId, ToolPermissionTier as AuthToolPermissionTier,
};
pub use trace::{
    execution_allowed, from_invoke, refuse_live_supabase, ClaimClass, GapLock, GateMode,
    InvokeTraceInput, MemoryTraceSink, TraceEvent, TraceKind, TraceSink,
};
pub use types::{
    ActionClass, ProposedAction, ReasonCode, RiskTier, SignedIntent, TrustedPolicy,
    UntrustedContent, UntrustedToolOutput,
};

pub const CRATE_SCOPE: &str = "local-fake-mcp-only";
