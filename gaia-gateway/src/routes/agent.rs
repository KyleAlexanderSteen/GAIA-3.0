use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    pub manifest: Option<serde_json::Value>,
}


pub async fn create_agent(
    State(_state): State<AppState>,
    Json(req): Json<CreateAgentRequest>,
) -> impl IntoResponse {
    let _ = req.manifest;
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "status": "not-implemented",
            "detail": format!("agent create is not wired: {}", req.name)
        })),
    )
}

pub async fn deploy_agent(
    State(_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "status": "not-implemented",
            "detail": format!("agent deploy is not wired: {id}")
        })),
    )
}

/// DELETE /agents/{id}/revoke — cancels a handle this process actually holds.
pub async fn revoke_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut inner = state._inner.write().await;
    if let Some(handle) = inner.active_agents.remove(&id) {
        handle.abort();
        tracing::info!(%id, "agent revoked");
        StatusCode::NO_CONTENT.into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "status": "not-found",
                "detail": format!("no active agent: {id}")
            })),
        )
            .into_response()
    }
}
