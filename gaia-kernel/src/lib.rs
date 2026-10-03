//! GAIA userspace kernel path (Phase 1 + Execution Engine).

pub mod audit;
pub mod broker;
pub mod capability;
pub mod execution;
pub mod executor;
pub mod federation;
pub mod home;
pub mod host;
pub mod hpc;
pub mod identity;
pub mod layers;
pub mod planner;
pub mod ports;
pub mod gate;
pub mod planetary;
pub mod providers;
pub mod reality;
pub mod receipts;
pub mod scheduler;
pub mod si_layer;
pub mod syscall;

pub use capability::{CapabilityRegistration, redundancy_gate};
pub use execution::{
    ExecutionEngine, ExecutionResult, ExecutionError,
    Intent, IntentSignature, Outcome, Task, TaskDAG, TaskResult,
};
pub use federation::{Federation, FederationError, Instance, JointTask, ResidentCube};
pub use host::{KernelError, KernelHost, SignedBlob, TaskHandle};
pub use hpc::{BatchTask, HpcAdapter};
pub use identity::{Principal, PrincipalKind};
pub use ports::{matrix, Arch, Footprint, PortProfile};
pub use syscall::{ABI_VERSION, GaiaSyscall, SyscallRequest, SyscallResult, dispatch};

#[cfg(test)]
mod tests {
    use super::*;
    use super::Task as DagTask;
    use crate::broker::Broker;
    use crate::executor::Executor;
    use crate::execution::error::{
        GAIA_INTENT_SIGNATURE_REQUIRED,
        GAIA_NO_CAPABLE_AGENT,
    };
    use crate::scheduler::select::AgentHandle;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::Uuid;

    fn tmp() -> std::path::PathBuf {
        let n = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("gaia-kernel-{n}"))
    }

    /// Build a valid signed Intent using the given Principal.
    fn signed_intent(principal: &Principal, intent_type: &str) -> Intent {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let id = Uuid::new_v4();
        let user_did = principal.did();
        let canon = format!("{id}|{user_did}|{intent_type}|{now}");
        let hash = crate::identity::sha256_hex(canon.as_bytes());
        let sig_bytes = principal.sign(hash.as_bytes());
        Intent {
            schema_version: "1.0".into(),
            id,
            user_did,
            intent_type: intent_type.into(),
            slots: HashMap::new(),
            timestamp_ms: now,
            ttl_ms: 30_000,
            signature: Some(IntentSignature {
                alg:     "EdDSA".into(),
                sig_hex: hex::encode(&sig_bytes),
                pub_hex: principal.public_hex(),
            }),
        }
    }

    // ── Legacy tests (must stay green) ──────────────────────────────────────────

    #[test]
    fn executor_registers_and_pulls_noop() {
        let broker = Arc::new(Broker::new());
        broker.enqueue("noop", "{}");
        let p = Principal::generate(PrincipalKind::Node);
        let exec = Executor::new(&p, Default::default(), broker.clone());
        let task = exec.pull_one().expect("task");
        assert_eq!(task.kind, "noop");
        assert!(exec.run_task(&task).starts_with("noop-ok"));
        assert_eq!(broker.node_count(), 1);
        assert!(exec.pull_one().is_none());
    }

    #[test]
    fn syscalls_sfs_memos_signed_audit() {
        let mut host = KernelHost::new(tmp()).unwrap();
        let intent = host.intent("register this node").unwrap();
        assert_eq!(intent.state, "admitted");

        let obj = host
            .store_file(
                "docs/note.txt",
                b"gaia semantic file about texas weather",
                "keep notes",
            )
            .unwrap();
        assert_eq!(obj.cid.len(), 64);
        let hits = host.sfs.search("texas weather", 2);
        assert!(!hits.is_empty());

        let cube = host
            .write_cube("episodic: talked about texas weather")
            .unwrap();
        let recalled = host.context("texas weather").unwrap();
        assert!(!recalled.content.is_empty());
        let _ = cube;

        let blob = host.sign(b"intent-proof").unwrap();
        assert!(host.verify(b"intent-proof", &blob).unwrap());
        assert!(!host.verify(b"other", &blob).unwrap());

        let proof = host
            .last_audit("sfs.put", "docs/note.txt")
            .expect("audit record");
        assert!(proof.signature_hex.len() > 20);
        assert!(host.audit.chain_ok());

        let err = host.reject_unsigned(None, b"x").unwrap_err();
        assert!(matches!(err, KernelError::Denied(_)));

        let invoke_err = host.invoke("analyst", "{}").unwrap_err();
        assert!(matches!(invoke_err, KernelError::NotImplemented(_)));
        let observe_err = host.observe("imu").unwrap_err();
        assert!(matches!(observe_err, KernelError::NotImplemented(_)));
        assert_eq!(host.declare("gpu0", "device").unwrap().name, "gpu0");

        let stranger = Principal::generate(PrincipalKind::Agent);
        let foreign = SignedBlob {
            algorithm: "ed25519".into(),
            public_hex: stranger.public_hex(),
            signature_hex: hex::encode(stranger.sign(b"x")),
        };
        let err = host.reject_unsigned(Some(&foreign), b"x").unwrap_err();
        assert!(matches!(err, KernelError::Denied(_)));
    }

    // ── Execution Engine tests ───────────────────────────────────────────────

    #[tokio::test]
    async fn stage1_rejects_unsigned_intent() {
        let principal = Principal::generate(PrincipalKind::Human);
        let engine = ExecutionEngine::new(
            Principal::generate(PrincipalKind::Node)
        );
        let mut intent = signed_intent(&principal, "query");
        intent.signature = None;

        let err = intent.verify_signature().unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains(GAIA_INTENT_SIGNATURE_REQUIRED),
            "expected GAIA_INTENT_SIGNATURE_REQUIRED in: {msg}"
        );
        let _ = engine;
    }

    #[tokio::test]
    async fn stage1_accepts_valid_signed_intent() {
        let principal = Principal::generate(PrincipalKind::Human);
        let intent = signed_intent(&principal, "query");
        assert!(intent.verify_signature().is_ok());
        assert!(intent.validate_schema().is_ok());
    }

    #[tokio::test]
    async fn full_pipeline_success_with_registered_agent() {
        let node_principal = Principal::generate(PrincipalKind::Node);
        let user_principal = Principal::generate(PrincipalKind::Human);
        let mut engine = ExecutionEngine::new(node_principal);

        engine.registry.register(AgentHandle::new(
            "agent-alpha",
            vec!["query".into()],
        ));

        let intent = signed_intent(&user_principal, "query");
        let result = engine.execute(intent).await.expect("pipeline must succeed");

        assert_eq!(result.task_results.len(), 1);
        assert!(
            matches!(result.task_results[0].outcome, Outcome::Success { .. }),
            "expected Success outcome"
        );
    }

    #[tokio::test]
    async fn stage4_no_capable_agent_returns_failed_outcome() {
        let node_principal = Principal::generate(PrincipalKind::Node);
        let user_principal = Principal::generate(PrincipalKind::Human);
        let mut engine = ExecutionEngine::new(node_principal);

        let intent = signed_intent(&user_principal, "query");
        let result = engine.execute(intent).await.expect("pipeline returns result even on task failure");

        assert_eq!(result.task_results.len(), 1);
        let outcome = &result.task_results[0].outcome;
        assert!(
            matches!(outcome, Outcome::Failed { reason } if reason.contains(GAIA_NO_CAPABLE_AGENT)),
            "expected GAIA_NO_CAPABLE_AGENT in outcome: {:?}", outcome
        );
    }

    #[tokio::test]
    async fn dag_topological_tiers_parallel_grouping() {
        let mut dag = TaskDAG::new();
        let t1 = DagTask::new("search",    "{}");
        let t2 = DagTask::new("summarise", "{}");
        let t3 = DagTask::new("alert",     "{}");
        let id1 = t1.id;
        let id2 = t2.id;
        dag.add_task(t1);
        dag.add_task(t2);
        dag.add_task(t3);
        dag.add_edge(id1, id3_placeholder(id1, id2, &dag));
        dag.add_edge(id2, id3_placeholder(id1, id2, &dag));
        let tiers = dag.topological_tiers().unwrap();
        assert_eq!(tiers[0].len(), 2, "tier 0 should contain t1 and t2");
    }

    fn id3_placeholder(id1: Uuid, id2: Uuid, dag: &TaskDAG) -> Uuid {
        *dag.tasks.keys().find(|&&id| id != id1 && id != id2).unwrap()
    }
}
