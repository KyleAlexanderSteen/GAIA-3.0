//! [`ExecutionEngine`] — the 12-stage GAIA intent execution pipeline.
//!
//! Same-tier tasks run stages 4–10 on `tokio::spawn`. Audit and MemOS writes
//! apply on the engine task after the join, in `task_id` order (#993 / #937).

use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;
use tokio::time::timeout;
use uuid::Uuid;

use crate::{
    audit::AuditLog,
    execution::{
        dag::Task,
        error::ExecutionError,
        metrics::IntentSpan,
    },
    identity::{sha256_hex, verify, Principal},
    planner::{decompose::Planner, replan::Replanner},
    scheduler::{allocate::Scheduler, select::AgentRegistry},
};

use gaia_memos::{CubeType, MemCube, MemOs};

const TASK_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub schema_version: String,
    pub id: Uuid,
    pub user_did: String,
    pub intent_type: String,
    pub slots: HashMap<String, String>,
    pub timestamp_ms: u64,
    pub ttl_ms: u64,
    pub signature: Option<IntentSignature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentSignature {
    pub alg: String,
    pub sig_hex: String,
    pub pub_hex: String,
}

impl Intent {
    pub fn signing_bytes(&self) -> Vec<u8> {
        let canon = format!(
            "{}|{}|{}|{}",
            self.id, self.user_did, self.intent_type, self.timestamp_ms
        );
        sha256_hex(canon.as_bytes()).into_bytes()
    }

    pub fn verify_signature(&self) -> Result<(), ExecutionError> {
        let sig = self.signature.as_ref().ok_or_else(|| {
            ExecutionError::SignatureRequired("intent.signature field is missing".into())
        })?;
        if sig.alg != "EdDSA" {
            return Err(ExecutionError::SignatureRequired(format!(
                "unsupported algorithm: {}",
                sig.alg
            )));
        }
        let sig_bytes = hex::decode(&sig.sig_hex).map_err(|_| {
            ExecutionError::SignatureRequired("signature hex invalid".into())
        })?;
        let payload = self.signing_bytes();
        if !verify(&sig.pub_hex, &payload, &sig_bytes) {
            return Err(ExecutionError::SignatureRequired(
                "Ed25519 signature verification failed".into(),
            ));
        }
        Ok(())
    }

    pub fn validate_schema(&self) -> Result<(), ExecutionError> {
        if self.schema_version != "1.0" {
            return Err(ExecutionError::SchemaInvalid(format!(
                "unsupported schema_version: {}",
                self.schema_version
            )));
        }
        if self.user_did.is_empty() {
            return Err(ExecutionError::SchemaInvalid("user_did must not be empty".into()));
        }
        let valid_types = ["device_control", "query", "plan", "alert", "memory_write"];
        if !valid_types.contains(&self.intent_type.as_str()) {
            return Err(ExecutionError::SchemaInvalid(format!(
                "unknown intent_type: {}",
                self.intent_type
            )));
        }
        if self.ttl_ms == 0 {
            return Err(ExecutionError::SchemaInvalid("ttl_ms must be > 0".into()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outcome {
    Success { output: String },
    Failed { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub outcome: Outcome,
    pub stage_ms: [u64; 12],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub intent_id: Uuid,
    pub task_results: Vec<TaskResult>,
    pub total_ms: u64,
    /// W3C `traceparent`. Flags are `00`. Not exported.
    #[serde(default)]
    pub traceparent: String,
    /// Why export did not happen. Never "ok".
    #[serde(default)]
    pub export_status: String,
}

struct PreparedTask {
    result: TaskResult,
    audit_detail: String,
    cube_body: Option<String>,
}

pub struct ExecutionEngine {
    pub planner: Planner,
    pub replanner: Replanner,
    pub registry: AgentRegistry,
    pub scheduler: Scheduler,
    pub audit: AuditLog,
    pub memory: MemOs,
    pub principal: Principal,
}

impl ExecutionEngine {
    pub fn new(principal: Principal) -> Self {
        Self {
            planner: Planner::new(),
            replanner: Replanner::new(),
            registry: AgentRegistry::new(),
            scheduler: Scheduler::new(),
            audit: AuditLog::default(),
            memory: MemOs::new(),
            principal,
        }
    }

    pub async fn execute(&mut self, intent: Intent) -> Result<ExecutionResult, ExecutionError> {
        let mut span = IntentSpan::new();

        let s1 = crate::execution::metrics::StageSpan::begin(1);
        intent.verify_signature()?;
        intent.validate_schema()?;
        span.record_stage(1, s1.finish());

        let s2 = crate::execution::metrics::StageSpan::begin(2);
        let subgoals = self.planner.decompose(&intent);
        span.record_stage(2, s2.finish());

        let s3 = crate::execution::metrics::StageSpan::begin(3);
        let dag = self.planner.plan(&subgoals);
        span.record_stage(3, s3.finish());

        let tiers = dag
            .topological_tiers()
            .map_err(|e| ExecutionError::SchemaInvalid(format!("DAG cycle: {e}")))?;

        let mut task_results: Vec<TaskResult> = Vec::new();

        for tier in tiers {
            let mut prepared = self.spawn_tier(&intent, tier).await;
            prepared.sort_by_key(|p| p.result.task_id);
            for item in prepared {
                self.audit.append(
                    &self.principal,
                    "execution",
                    &format!(
                        "intent:{}:task:{}:{}",
                        intent.id, item.result.task_id, item.audit_detail
                    ),
                );
                if let Some(body) = item.cube_body {
                    self.memory.put(MemCube::new(
                        CubeType::Episodic,
                        body,
                        "execution-engine",
                    ));
                }
                for (i, &ms) in item.result.stage_ms.iter().enumerate() {
                    span.record_stage(i as u8 + 1, ms);
                }
                task_results.push(item.result);
            }
        }

        let outcomes: Vec<bool> = task_results
            .iter()
            .map(|task| matches!(task.outcome, Outcome::Success { .. }))
            .collect();
        let status_ok = crate::execution::trace::status_ok(&outcomes);
        let total_ms = span.finish(status_ok);
        let trace = crate::execution::trace::LocalTrace::new(status_ok);
        let export_status = match trace.export_otlp() {
            Ok(()) => "export-refused-ok-is-a-bug".into(),
            Err(reason) => reason.to_string(),
        };
        Ok(ExecutionResult {
            intent_id: intent.id,
            task_results,
            total_ms,
            traceparent: trace.traceparent(),
            export_status,
        })
    }

    async fn spawn_tier(&self, intent: &Intent, tier: Vec<Task>) -> Vec<PreparedTask> {
        let mut set: JoinSet<PreparedTask> = JoinSet::new();
        for task in tier {
            let intent = intent.clone();
            let registry = self.registry.clone();
            let task_id = task.id;
            set.spawn(async move {
                match timeout(TASK_TIMEOUT, async {
                    execute_task_body(&registry, &intent, task)
                })
                .await
                {
                    Ok(prepared) => prepared,
                    Err(_) => PreparedTask {
                        result: TaskResult {
                            task_id,
                            outcome: Outcome::Failed {
                                reason: "GAIA_EXECUTION_TIMEOUT: task exceeded 30s".into(),
                            },
                            stage_ms: [0; 12],
                        },
                        audit_detail: "timeout".into(),
                        cube_body: None,
                    },
                }
            });
        }

        let mut out = Vec::new();
        while let Some(joined) = set.join_next().await {
            match joined {
                Ok(item) => out.push(item),
                Err(join_err) => out.push(PreparedTask {
                    result: TaskResult {
                        task_id: Uuid::nil(),
                        outcome: Outcome::Failed {
                            reason: format!("GAIA_EXECUTION_TIMEOUT: join {join_err}"),
                        },
                        stage_ms: [0; 12],
                    },
                    audit_detail: "join-error".into(),
                    cube_body: None,
                }),
            }
        }
        out
    }
}

fn execute_task_body(registry: &AgentRegistry, intent: &Intent, task: Task) -> PreparedTask {
    let task_id = task.id;
    let mut stage_ms = [0u64; 12];
    let scheduler = Scheduler::new();
    let replanner = Replanner::new();

    let s4 = crate::execution::metrics::StageSpan::begin(4);
    let candidates = registry.find(&task.capability);
    stage_ms[3] = s4.finish();

    if candidates.is_empty() {
        return PreparedTask {
            result: TaskResult {
                task_id,
                outcome: Outcome::Failed {
                    reason: format!("GAIA_NO_CAPABLE_AGENT: {}", task.capability),
                },
                stage_ms,
            },
            audit_detail: "no_capable_agent".into(),
            cube_body: None,
        };
    }

    let s5 = crate::execution::metrics::StageSpan::begin(5);
    let agent = registry.select(candidates);
    stage_ms[4] = s5.finish();

    let s6 = crate::execution::metrics::StageSpan::begin(6);
    let policy_ok = agent.capabilities.contains(&task.capability);
    stage_ms[5] = s6.finish();

    if !policy_ok {
        return PreparedTask {
            result: TaskResult {
                task_id,
                outcome: Outcome::Failed {
                    reason: format!(
                        "GAIA_CAPABILITY_DENIED: {} denied to {}",
                        task.capability, agent.id
                    ),
                },
                stage_ms,
            },
            audit_detail: "capability_denied".into(),
            cube_body: None,
        };
    }

    let s7 = crate::execution::metrics::StageSpan::begin(7);
    let _quota = scheduler.allocate(&task);
    stage_ms[6] = s7.finish();

    let s8 = crate::execution::metrics::StageSpan::begin(8);
    let mut outcome = Outcome::Success {
        output: format!("executed:{}:{}", agent.id, task.capability),
    };
    stage_ms[7] = s8.finish();

    let s9 = crate::execution::metrics::StageSpan::begin(9);
    stage_ms[8] = s9.finish();

    let s10 = crate::execution::metrics::StageSpan::begin(10);
    if let Outcome::Failed { ref reason } = outcome {
        match replanner.fallback(&task, reason) {
            Some(fallback) => {
                outcome = Outcome::Success {
                    output: format!("fallback:{}:{}", agent.id, fallback.capability),
                };
            }
            None => {
                stage_ms[9] = s10.finish();
                return PreparedTask {
                    result: TaskResult {
                        task_id,
                        outcome: Outcome::Failed {
                            reason: format!("GAIA_REPLAN_EXHAUSTED: {}", task.capability),
                        },
                        stage_ms,
                    },
                    audit_detail: "replan_exhausted".into(),
                    cube_body: None,
                };
            }
        }
    }
    stage_ms[9] = s10.finish();

    let s11 = crate::execution::metrics::StageSpan::begin(11);
    stage_ms[10] = s11.finish();

    let s12 = crate::execution::metrics::StageSpan::begin(12);
    let output_str = match &outcome {
        Outcome::Success { output } => output.clone(),
        Outcome::Failed { reason } => reason.clone(),
    };
    let cube_body = Some(format!(
        "intent:{}:task:{}:{}",
        intent.id, task_id, output_str
    ));
    stage_ms[11] = s12.finish();

    PreparedTask {
        result: TaskResult {
            task_id,
            outcome,
            stage_ms,
        },
        audit_detail: "success".into(),
        cube_body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::PrincipalKind;
    use crate::scheduler::select::AgentHandle;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn signed_intent(principal: &Principal, intent_type: &str, slots: HashMap<String, String>) -> Intent {
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
            slots,
            timestamp_ms: now,
            ttl_ms: 30_000,
            signature: Some(IntentSignature {
                alg: "EdDSA".into(),
                sig_hex: hex::encode(&sig_bytes),
                pub_hex: principal.public_hex(),
            }),
        }
    }

    #[tokio::test]
    async fn two_same_tier_tasks_both_succeed() {
        let node = Principal::generate(PrincipalKind::Node);
        let user = Principal::generate(PrincipalKind::Human);
        let mut engine = ExecutionEngine::new(node);
        engine.registry.register(AgentHandle::new("agent-a", vec!["query".into()]));
        engine.registry.register(AgentHandle::new("agent-b", vec!["plan".into()]));

        let mut slots = HashMap::new();
        slots.insert("query".into(), "q".into());
        slots.insert("plan".into(), "p".into());
        let intent = signed_intent(&user, "query", slots);
        let result = engine.execute(intent).await.expect("pipeline");
        assert_eq!(result.task_results.len(), 2);
        assert!(result
            .task_results
            .iter()
            .all(|t| matches!(t.outcome, Outcome::Success { .. })));
        assert!(engine.audit.chain_ok());
        assert!(engine.audit.len() >= 2);
    }
}
