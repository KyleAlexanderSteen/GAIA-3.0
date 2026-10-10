//! GET /health returns a structured JSON health report.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use gaia_gateway::{router, state::AppState};
use http_body_util::BodyExt;
use tower::ServiceExt; // for `.oneshot()`

#[tokio::test]
async fn health_returns_json_report() {
    let app = router(AppState::default());
    let req = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let report: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(report["network"], "UNKNOWN");
    assert_eq!(report["subsystems"][0]["name"], "gateway");
    assert_eq!(report["subsystems"][0]["state"], "READY");
    assert!(report["subsystems"].as_array().unwrap().len() >= 1);
}
