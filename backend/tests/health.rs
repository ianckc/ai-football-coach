use ai_football_coach_backend::app;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

/// Acceptance slice: Create a `/health` endpoint.
#[tokio::test]
async fn health_endpoint_exists() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "GET /health must exist and return 200"
    );
}

/// Acceptance slice: Return `{status: 'ok'}`.
#[tokio::test]
async fn health_returns_status_ok() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("read body")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("GET /health must return JSON");

    assert_eq!(
        json,
        serde_json::json!({ "status": "ok" }),
        "GET /health must return {{status: 'ok'}}"
    );
}
