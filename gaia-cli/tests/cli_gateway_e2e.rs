//! End-to-end test for the real CLI -> HTTP gateway -> shared intent dispatch path.

use assert_cmd::Command;
use gaia_gateway::{router, state::AppState};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn start_gateway() -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind ephemeral gateway port");
    let address: SocketAddr = listener.local_addr().expect("read gateway address");
    let server = tokio::spawn(async move {
        axum::serve(listener, router(AppState::default()))
            .await
            .expect("serve test gateway");
    });
    (format!("http://{address}"), server)
}

#[tokio::test]
async fn cli_intent_reaches_gateway_and_returns_dispatch_result() {
    let (gateway, server) = start_gateway().await;

    let output = Command::cargo_bin("gaia-cli")
        .expect("gaia-cli binary should be built")
        .args(["intent", "echo: cli gateway e2e", "--gateway", &gateway])
        .output()
        .expect("run CLI against live test gateway");

    server.abort();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "CLI failed. stdout={stdout}; stderr={stderr}"
    );
    assert!(
        stdout.contains("recorded"),
        "expected a recorded gateway receipt: {stdout}"
    );
    assert!(
        stdout.contains("cli gateway e2e"),
        "expected gateway dispatch detail: {stdout}"
    );
    assert!(
        !stdout.contains("not implemented"),
        "CLI fell back to a stub: {stdout}"
    );
}

#[tokio::test]
async fn cli_surfaces_gateway_denial_as_failure() {
    let (gateway, server) = start_gateway().await;

    let output = Command::cargo_bin("gaia-cli")
        .expect("gaia-cli binary should be built")
        .args([
            "intent",
            "perform an unsupported action",
            "--gateway",
            &gateway,
        ])
        .output()
        .expect("run CLI against live test gateway");

    server.abort();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "denied intent must fail. stdout={stdout}; stderr={stderr}"
    );
    assert!(
        stderr.contains("gateway intent failed"),
        "expected explicit gateway failure: {stderr}"
    );
    assert!(
        !stdout.contains("recorded"),
        "denied intent must not print success: {stdout}"
    );
}
