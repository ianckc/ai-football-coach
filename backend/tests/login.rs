mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

/// Acceptance slice (#33): Add `POST /login`.
#[tokio::test]
async fn login_post_endpoint_exists() {
    let response = common::test_app()
        .await
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(
        response.status(),
        StatusCode::NOT_FOUND,
        "POST /login must exist (must not return 404)"
    );
    assert_ne!(
        response.status(),
        StatusCode::METHOD_NOT_ALLOWED,
        "POST /login must accept POST (must not return 405)"
    );
}
