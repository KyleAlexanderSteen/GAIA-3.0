use std::sync::Arc;

use crate::broker::{Broker, Capabilities, ExecutorInfo, Task};
use crate::identity::Principal;

pub struct Executor {
    pub node_id: String,
    pub caps: Capabilities,
    broker: Arc<Broker>,
}

impl Executor {
    pub fn new(principal: &Principal, caps: Capabilities, broker: Arc<Broker>) -> Self {
        let node_id = principal.did();
        broker.register(ExecutorInfo {
            node_id: node_id.clone(),
            caps: caps.clone(),
        });
        Self {
            node_id,
            caps,
            broker,
        }
    }

    pub fn pull_one(&self) -> Option<Task> {
        self.broker.pull(&self.node_id)
    }

    /// `noop` is an admitted no-work kind. Any other kind is unsupported, not ok.
    pub fn run_task(&self, task: &Task) -> String {
        match task.kind.as_str() {
            "noop" => format!("noop-ok:{}", task.id),
            other => format!("unsupported:{other}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntentReceipt {
    pub id: String,
    pub status: &'static str,
    pub detail: String,
}

impl Executor {
    /// One job. A super-power claim without evidence is refused.
    /// A plain string is not executed. Admission is not `done`.
    pub fn run_intent(&self, id: &str, payload: &str) -> IntentReceipt {
        let trimmed = payload.trim();
        if trimmed.is_empty() {
            return IntentReceipt {
                id: id.into(),
                status: "refused",
                detail: "empty intent".into(),
            };
        }
        let lower = trimmed.to_ascii_lowercase();
        let claims_power = lower.contains("super power") || lower.contains("super-power");
        let has_evidence = lower.contains("evidence:");
        if claims_power && !has_evidence {
            return IntentReceipt {
                id: id.into(),
                status: "refused",
                detail: "power claim has no evidence".into(),
            };
        }
        IntentReceipt {
            id: id.into(),
            status: "not-executed",
            detail: "no syscall dispatched; admission is not execution".into(),
        }
    }
}

#[cfg(test)]
mod intent_tests {
    use super::*;
    use crate::broker::Broker;
    use crate::identity::{Principal, PrincipalKind};
    use std::sync::Arc;

    fn executor() -> Executor {
        let principal = Principal::generate(PrincipalKind::Node);
        Executor::new(&principal, Capabilities::default(), Arc::new(Broker::new()))
    }

    #[test]
    fn plain_intent_is_not_done() {
        let receipt = executor().run_intent("job-1", "record mineral row");
        assert_eq!(receipt.status, "not-executed");
        assert_eq!(receipt.id, "job-1");
        assert_ne!(receipt.status, "done");
    }

    #[test]
    fn empty_intent_is_refused() {
        let receipt = executor().run_intent("job-0", "   ");
        assert_eq!(receipt.status, "refused");
    }

    #[test]
    fn power_claim_without_evidence_is_refused() {
        let receipt = executor().run_intent("job-2", "grant super power");
        assert_eq!(receipt.status, "refused");
    }

    #[test]
    fn unsupported_task_is_not_ok() {
        let exec = executor();
        let task = Task {
            id: uuid::Uuid::new_v4(),
            kind: "forecast-earth".into(),
            payload: "{}".into(),
        };
        let out = exec.run_task(&task);
        assert!(out.starts_with("unsupported:"));
        assert!(!out.contains("ok"));
    }
}
