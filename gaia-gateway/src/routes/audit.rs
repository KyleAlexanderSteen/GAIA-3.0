use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use crate::state::AppState;

pub async fn get_audit(State(_state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "status": "not-implemented",
            "detail": "gateway audit store is not wired; use the local CLI ledger"
        })),
    )
}

pub async fn stream_audit(State(_state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "status": "not-implemented",
            "detail": "audit stream is not wired"
        })),
    )
}
