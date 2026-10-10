//! Integration tests that compile and execute real WebAssembly components
//! through the production Wasmtime sandbox manager.

use gaia_runtime::{SandboxManager, SandboxProfile};
use std::time::Duration;

const SUCCESS_COMPONENT: &str = r#"
(component
  (core module $m
    (func (export "run"))
  )
  (core instance $i (instantiate $m))
  (func (export "run") (canon lift (core func $i "run")))
)
"#;

const TRAPPING_COMPONENT: &str = r#"
(component
  (core module $m
    (func (export "run")
      unreachable
    )
  )
  (core instance $i (instantiate $m))
  (func (export "run") (canon lift (core func $i "run")))
)
"#;

#[tokio::test]
async fn real_component_executes_in_deny_by_default_sandbox_and_returns_success() {
    let bytes = wat::parse_str(SUCCESS_COMPONENT).expect("valid WAT component fixture");
    let sandbox = SandboxManager::new(SandboxProfile::default())
        .expect("initialize Wasmtime sandbox");
    let component = sandbox
        .compile(&bytes)
        .expect("compile actual WebAssembly component");

    let (result, telemetry) = sandbox
        .execute_component_with_load(&component, Duration::from_secs(5), 5_000)
        .await;

    assert!(result.is_ok(), "component execution failed: {result:?}");
    assert_eq!(telemetry.operations, 1);
    assert_eq!(telemetry.failures, 0);
    assert!(telemetry.elapsed < Duration::from_secs(5));
    assert_eq!(telemetry.memory_bytes_budget, 64 * 1024 * 1024);
    assert_eq!(telemetry.interruptions, 0);
}

#[tokio::test]
async fn real_component_trap_is_returned_as_execution_failure() {
    let bytes = wat::parse_str(TRAPPING_COMPONENT).expect("valid trapping WAT component fixture");
    let sandbox = SandboxManager::new(SandboxProfile::default())
        .expect("initialize Wasmtime sandbox");
    let component = sandbox
        .compile(&bytes)
        .expect("compile actual trapping WebAssembly component");

    let (result, telemetry) = sandbox
        .execute_component_with_load(&component, Duration::from_secs(5), 5_000)
        .await;

    assert!(result.is_err(), "a trapping component must not return success");
    assert_eq!(telemetry.operations, 1);
    assert_eq!(telemetry.failures, 1);
    assert_eq!(telemetry.interruptions, 0);
}
