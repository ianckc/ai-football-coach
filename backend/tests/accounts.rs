use ai_football_coach_backend::app;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

/// Acceptance slice: Add `POST /accounts`.
#[tokio::test]
async fn accounts_post_endpoint_exists() {
    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(
        response.status(),
        StatusCode::NOT_FOUND,
        "POST /accounts must exist (must not return 404)"
    );
    assert_ne!(
        response.status(),
        StatusCode::METHOD_NOT_ALLOWED,
        "POST /accounts must accept POST (must not return 405)"
    );
}
