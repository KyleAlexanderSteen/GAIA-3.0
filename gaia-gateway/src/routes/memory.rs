use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
}

pub async fn list_memory(State(_state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "status": "not-implemented",
            "detail": "memory list is not wired to MemOS"
        })),
    )
}

pub async fn search_memory(
    State(_state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "status": "not-implemented",
            "detail": format!("memory search is not wired: {:?}", params.q)
        })),
    )
}
