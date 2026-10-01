//! L5 agent ecosystem (#5).
//! #24 Wasmtime guest, #25 pack, #26 A2A handoff and signed install.
//! #722 Full DISCOVER→ARCHIVE lifecycle state machine.
//! This is not a marketplace transport or containerd sandbox.

mod a2a;
mod host;
mod lifecycle;
mod pack;
mod runtime;
pub mod session;
pub mod surface;
pub mod tool_registry;
mod wasm;

pub use a2a::{
    registry_layout, ContextBundle, Handoff, MarketError, Package, PackageMarket, PrivacyMode,
};
pub use host::{AgentHost, HostError, Review};
pub use lifecycle::{
    AgentCheckpoint, AgentDid, CapabilityToken, LifecycleError, LifecycleEvent, LifecycleManager,
    LifecycleStage, ResourceSnapshot,
};
pub use pack::{catalog, find as find_pack, AgentKind, PackEntry};
pub use runtime::{
    AgentManifest, AgentOutcome, AgentRuntime, Capability, ResourceLimits, RuntimeError,
};
pub use session::{
    HumanReauthorization, SessionContext, SessionError, SessionEvent, SessionRecorder,
    SessionToken,
};
pub use surface::{
    rank_resources, surface_for_session, ConfidenceBand, ResourceCandidate, ResourceKind,
    SurfaceError, SurfaceEvent,
};
pub use tool_registry::{
    AgentId as ToolAgentId, ToolId, ToolPermissionTier, ToolRegistration, ToolRegistry,
};
pub use wasm::{WasiGrant, WasmOutcome, WasmRuntime};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_stage_ordering() {
        assert!(LifecycleStage::Discover < LifecycleStage::Register);
        assert!(LifecycleStage::Register < LifecycleStage::Initialize);
        assert!(LifecycleStage::Initialize < LifecycleStage::Run);
        assert!(LifecycleStage::Run < LifecycleStage::Pause);
        assert!(LifecycleStage::Pause < LifecycleStage::Archive);
    }

    #[test]
    fn agent_manifest_default_capabilities_empty() {
        let manifest = AgentManifest {
            name: String::new(),
            version: String::new(),
            declared_capabilities: vec![],
            limits: ResourceLimits::default(),
        };
        assert!(manifest.declared_capabilities.is_empty());
    }

    #[test]
    fn resource_limits_default_values() {
        let limits = ResourceLimits::default();
        assert!(limits.memory_mib > 0);
        assert!(limits.cpu_millis > 0);
    }

    #[test]
    fn pack_catalog_returns_slice() {
        assert!(!catalog().is_empty());
    }

    #[test]
    fn wasi_grant_debug_is_non_empty() {
        let grant = WasiGrant {
            filesystem: false,
            network: false,
        };
        let s = format!("{grant:?}");
        assert!(!s.is_empty());
    }
}
