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
    /// One job. A super-power claim without evidence is refused. A plain intent returns a receipt.
    pub fn run_intent(&self, id: &str, payload: &str) -> IntentReceipt {
        let lower = payload.to_ascii_lowercase();
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
            status: "done",
            detail: format!("ran:{payload}"),
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
    fn plain_intent_returns_a_receipt() {
        let receipt = executor().run_intent("job-1", "record mineral row");
        assert_eq!(receipt.status, "done");
        assert_eq!(receipt.id, "job-1");
    }

    #[test]
    fn power_claim_without_evidence_is_refused() {
        let receipt = executor().run_intent("job-2", "grant super power");
        assert_eq!(receipt.status, "refused");
    }
}
