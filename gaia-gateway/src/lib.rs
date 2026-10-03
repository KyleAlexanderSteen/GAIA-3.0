pub mod routes;
pub mod state;

use axum::{
    Router,
    routing::{delete, get, post},
};
use state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        // Intent
        .route("/intent", post(routes::intent::submit_intent))
        .route("/intent/stream", get(routes::intent::stream_intent))
        // Agents — Axum 0.8 captures use `{id}`, not `:id`.
        .route("/agents", post(routes::agent::create_agent))
        .route("/agents/{id}/deploy", post(routes::agent::deploy_agent))
        .route("/agents/{id}/revoke", delete(routes::agent::revoke_agent))
        // Memory
        .route("/memory", get(routes::memory::list_memory))
        .route("/memory/search", get(routes::memory::search_memory))
        // Audit
        .route("/audit", get(routes::audit::get_audit))
        .route("/audit/stream", get(routes::audit::stream_audit))
        // Health
        .route("/health", get(routes::health::health))
        .route("/v1/earth", get(routes::earth::earth))
        .route("/v1/intents", post(routes::earth::submit_v1))
        .with_state(state)
}

/// Listed routes. Tests assert this table; no socket bind.
pub fn listed_paths() -> &'static [&'static str] {
    &["/intent", "/agents", "/memory", "/audit", "/health", "/v1/earth", "/v1/intents"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_builds_without_bind() {
        let _ = router(AppState::default());
    }

    #[test]
    fn listed_paths_cover_health() {
        assert!(listed_paths().contains(&"/health"));
        assert_eq!(listed_paths().len(), 7);
    }

    #[test]
    fn app_state_default_has_no_agents() {
        let state = AppState::default();
        assert!(state._inner.try_read().expect("lock").active_agents.is_empty());
    }
}
