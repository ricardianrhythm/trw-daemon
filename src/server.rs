//! HTTP/JSON API server.
//!
//! Serves the latest MachineSnapshot via axum on a configurable port.
//!
//! Endpoints:
//! - GET /status  → latest MachineSnapshot as JSON
//! - GET /health  → {"status": "ok"} liveness check
//! - GET /metrics → Prometheus text format (TODO: stretch goal)

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::MachineSnapshot;

/// Shared application state accessible by all route handlers.
pub type SharedState = Arc<RwLock<Option<MachineSnapshot>>>;

/// Build the axum router with all routes.
pub fn build_router(state: SharedState) -> Router {
    Router::new()
        .route("/status", get(get_status))
        .route("/health", get(get_health))
        .route("/metrics", get(get_metrics))
        .with_state(state)
}

/// GET /status — returns the latest MachineSnapshot.
/// Returns 503 if no snapshot has been collected yet.
async fn get_status(
    State(state): State<SharedState>,
) -> Result<Json<MachineSnapshot>, StatusCode> {
    let snapshot = state.read().await;
    match snapshot.as_ref() {
        Some(s) => Ok(Json(s.clone())),
        None => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

/// GET /health — simple liveness check.
async fn get_health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok"}))
}

/// GET /metrics — Prometheus-compatible text format.
async fn get_metrics(
    State(_state): State<SharedState>,
) -> (StatusCode, String) {
    // TODO: Convert latest MachineSnapshot to Prometheus text format
    //   Example output:
    //   # HELP trw_cpu_percent CPU usage percentage
    //   # TYPE trw_cpu_percent gauge
    //   trw_cpu_percent{hostname="g1-orin-01"} 42.3
    //
    // For now, return 501 Not Implemented
    (StatusCode::NOT_IMPLEMENTED, "# metrics endpoint not yet implemented\n".to_string())
}

/// Start the HTTP server on the given port.
pub async fn serve(state: SharedState, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = build_router(state);
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("HTTP server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
