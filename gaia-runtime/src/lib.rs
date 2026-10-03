//! `gaia-runtime` — WASM/WASI 0.3 sandbox execution engine for GAIA agents.
//!
//! Runtime-integrity scaffold (#932 / #934): grounding, faithfulness,
//! circuit_breaker, quotas, dormancy. Typed stubs with unit tests. No LLM
//! judge, no RSS measurement, no SOS wiring.

pub mod attribute;
pub mod circuit_breaker;
pub mod dormancy;
pub mod faithfulness;
pub mod grounding;
pub mod guest;
pub mod quotas;
pub mod sandbox;

pub use attribute::{attribute_sentences, AttributedSentence};
pub use circuit_breaker::{CircuitBreaker, CircuitOpen, CircuitState};
pub use dormancy::{DormancyGuard, DormancyMode, DormantSystem};
pub use faithfulness::{score_faithfulness, FaithfulnessScore};
pub use grounding::{
    enforce_grounding, ChunkId, GenerationMode, GroundedResponse, GroundingError,
};
pub use quotas::{AgentQuota, RateLimitExceeded, RuntimeAnomaly};
pub use sandbox::{
    error::{SandboxError, GAIA_CAPABILITY_DENIED},
    limits::{GaiaResourceLimiter, ResourceQuota},
    manager::SandboxManager,
    profile::SandboxProfile,
};

#[cfg(test)]
mod tests {
    use super::*;
    use wasmtime::ResourceLimiter;

    #[test]
    fn profile_default_denies_all_capabilities() {
        let p = SandboxProfile::default();
        assert!(!p.scratch_only_writes, "default must not grant scratch writes");
        assert!(!p.network, "default must not grant network access");
        assert!(!p.inherited_env, "default must not forward env vars");
    }

    #[test]
    fn profile_scratch_only_writes_field() {
        let p = SandboxProfile {
            scratch_only_writes: true,
            ..SandboxProfile::default()
        };
        assert!(p.scratch_only_writes);
        assert!(!p.network);
        assert!(!p.inherited_env);
    }

    #[test]
    fn profile_network_field() {
        let p = SandboxProfile {
            network: true,
            ..SandboxProfile::default()
        };
        assert!(p.network);
        assert!(!p.scratch_only_writes);
        assert!(!p.inherited_env);
    }

    #[test]
    fn quota_default_values() {
        let q = ResourceQuota::default();
        assert_eq!(q.max_memory_bytes, 64 * 1024 * 1024, "default memory quota must be 64 MiB");
        assert_eq!(q.max_epochs, 5, "default epoch budget must be 5");
        assert_eq!(q.max_fds, 32, "default fd limit must be 32");
        assert_eq!(q.max_processes, 0, "default must allow zero sub-processes");
    }

    #[test]
    fn resource_limiter_allows_within_quota() {
        let quota = ResourceQuota {
            max_memory_bytes: 64 * 1024 * 1024,
            ..ResourceQuota::default()
        };
        let mut limiter = GaiaResourceLimiter::new(quota);
        let result = limiter.memory_growing(0, 32 * 1024 * 1024, None);
        assert!(result.unwrap(), "request within quota must be approved");
        assert_eq!(limiter.mem_used(), 32 * 1024 * 1024);
    }

    #[test]
    fn resource_limiter_denies_over_quota() {
        let quota = ResourceQuota {
            max_memory_bytes: 64 * 1024 * 1024,
            ..ResourceQuota::default()
        };
        let mut limiter = GaiaResourceLimiter::new(quota);
        let result = limiter.memory_growing(0, 128 * 1024 * 1024, None);
        assert!(!result.unwrap(), "request over quota must be denied via Ok(false)");
    }
}
