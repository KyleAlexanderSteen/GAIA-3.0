use std::sync::Arc;
use std::time::Duration;

use gaia_kernel::broker::{Broker, Capabilities};
use gaia_kernel::executor::Executor;
use gaia_kernel::{Principal, PrincipalKind};

/// Pull-based continuum node. Opens no inbound port; it only calls broker.pull.
#[tokio::main]
async fn main() {
    let broker = Arc::new(Broker::new());
    broker.enqueue("noop", "{}");
    let principal = Principal::generate(PrincipalKind::Node);
    let exec = Executor::new(&principal, Capabilities::default(), broker);
    println!("gaia-executor node={}", exec.node_id);
    println!(
        "caps cpu={} gpu={} cuda={} rocm={} opencl={}",
        exec.caps.cpu_cores, exec.caps.gpu, exec.caps.cuda, exec.caps.rocm, exec.caps.opencl
    );
    if let Some(goal) = std::env::args().nth(1) {
        let dir = std::env::temp_dir().join("gaia-os-boot");
        std::fs::create_dir_all(&dir).expect("boot dir");
        let mut host = gaia_kernel::KernelHost::new(dir).expect("kernel host");
        let receipt = host.executor.run_intent("cli", &goal);
        // `recorded` is the only admitted local act (echo:). `done` would mean a syscall.
        if receipt.status != "recorded" {
            eprintln!("{} {}", receipt.status, receipt.detail);
            std::process::exit(1);
        }
        println!(
            "recorded id={} detail={}",
            receipt.id, receipt.detail
        );
    } else if let Some(task) = exec.pull_one() {
        let out = exec.run_task(&task);
        if out.starts_with("unsupported:") {
            eprintln!("{out}");
            std::process::exit(1);
        }
        println!("{out}");
    } else {
        println!("idle");
    }
    tokio::time::sleep(Duration::from_millis(1)).await;
}
