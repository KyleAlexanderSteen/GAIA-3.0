use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::state::AppState;
use gaia_dispatch::dispatch_intent;

#[derive(Deserialize)]
pub struct IntentRequest {
    pub text: String,
    pub profile: Option<String>,
}

#[derive(Serialize)]
pub struct IntentResponse {
    pub id: String,
    pub status: String,
    pub detail: String,
}

/// POST /intent — uses the same shared dispatch implementation as the CLI.
pub async fn submit_intent(
    State(_state): State<AppState>,
    Json(req): Json<IntentRequest>,
) -> impl IntoResponse {
    let _ = req.profile;
    let id = uuid::Uuid::new_v4().to_string();
    let receipt = dispatch_intent(&id, &req.text);
    let mut detail = receipt.detail;
    if receipt.status == "recorded" {
        if let Err(err) = gaia_kernel::receipts::append_from("gateway", &receipt.id, &detail) {
            detail = format!("{detail}; receipt-not-written: {err}");
        }
    }
    let body = Json(IntentResponse {
        id: receipt.id,
        status: receipt.status.to_string(),
        detail,
    });
    let code = match receipt.status {
        "recorded" => StatusCode::OK,
        "refused" => StatusCode::FORBIDDEN,
        _ => StatusCode::NOT_IMPLEMENTED,
    };
    (code, body)
}

/// GET /intent/stream — not wired. Must not emit a stub success event.
pub async fn stream_intent(State(_state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "status": "not-implemented",
            "detail": "intent stream is not wired"
        })),
    )
}
