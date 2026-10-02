//! #1304: gateway must not report queued/created for work it did not do.

use axum::{body::Body, http::{Request, StatusCode}};
use gaia_gateway::{router, state::AppState};
use tower::ServiceExt;

async fn post(uri: &str, body: &str) -> (StatusCode, String) {
    let app = router(AppState::default());
    let req = Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = axum::body::to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
    (status, String::from_utf8_lossy(&bytes).to_string())
}

#[tokio::test]
async fn echo_is_recorded_not_queued() {
    let (status, body) = post("/intent", r#"{"text":"echo: gateway row"}"#).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(body.contains("recorded"), "{body}");
    assert!(!body.contains("queued"), "{body}");
}

#[tokio::test]
async fn plain_intent_is_not_implemented() {
    let (status, body) = post("/intent", r#"{"text":"hello"}"#).await;
    assert_eq!(status, StatusCode::NOT_IMPLEMENTED, "{body}");
    assert!(body.contains("not-executed") || body.contains("not-implemented"), "{body}");
    assert!(!body.contains("queued"), "{body}");
}

#[tokio::test]
async fn agent_create_is_not_implemented() {
    let (status, body) = post("/agents", r#"{"name":"a"}"#).await;
    assert_eq!(status, StatusCode::NOT_IMPLEMENTED, "{body}");
    assert!(!body.contains("created"), "{body}");
}
