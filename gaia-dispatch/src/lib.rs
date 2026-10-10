use std::sync::Arc;

use gaia_kernel::broker::{Broker, Capabilities};
use gaia_kernel::executor::{Executor, IntentReceipt};
use gaia_kernel::{Principal, PrincipalKind};

/// Dispatch one intent through the canonical local kernel execution path.
///
/// This preserves the kernel current semantics and does not imply persistence,
/// authorization, sandboxing, or general system-call execution.
pub fn dispatch_intent(id: &str, payload: &str) -> IntentReceipt {
    let broker = Arc::new(Broker::new());
    let principal = Principal::generate(PrincipalKind::Node);
    let executor = Executor::new(&principal, Capabilities::default(), broker);
    executor.run_intent(id, payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recorded_receipt_preserves_caller_id_and_detail() {
        let receipt = dispatch_intent("shared-1", "echo: same path");
        assert_eq!(receipt.id, "shared-1");
        assert_eq!(receipt.status, "recorded");
        assert_eq!(receipt.detail, "same path");
    }

    #[test]
    fn unsupported_intent_is_not_reported_as_success() {
        let receipt = dispatch_intent("shared-2", "perform an unsupported action");
        assert_eq!(receipt.status, "not-executed");
        assert_ne!(receipt.status, "done");
    }

    #[test]
    fn empty_intent_is_refused() {
        assert_eq!(dispatch_intent("shared-3", " ").status, "refused");
    }
}
