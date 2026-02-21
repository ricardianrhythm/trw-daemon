//! Tests for the /health endpoint.

use axum::http::StatusCode;
use axum::body::Body;
use axum::http::Request;
use tower::ServiceExt;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn health_returns_ok() {
    let state = Arc::new(RwLock::new(None));
    let app = trw_daemon::server::build_router(state);

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn status_returns_503_when_no_snapshot() {
    let state = Arc::new(RwLock::new(None));
    let app = trw_daemon::server::build_router(state);

    let response = app
        .oneshot(Request::builder().uri("/status").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}
