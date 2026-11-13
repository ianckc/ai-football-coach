use ai_football_coach_backend::app;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
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
