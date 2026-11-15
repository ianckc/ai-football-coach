mod common;

use ai_football_coach_backend::app_with_cors_origins;
use axum::{
    body::Body,
    http::{HeaderValue, Method, Request, StatusCode, header},
};
use tower::ServiceExt;

#[tokio::test]
async fn cors_reflects_configured_origin_on_preflight() {
    let pool = common::test_pool().await;
    let app = app_with_cors_origins(
        pool,
        vec![HeaderValue::from_static("http://localhost:5173")],
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/health")
                .header(header::ORIGIN, "http://localhost:5173")
                .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN),
        Some(&HeaderValue::from_static("http://localhost:5173")),
        "preflight must allow the configured CORS origin"
    );
}

#[tokio::test]
async fn cors_does_not_reflect_unlisted_origin() {
    let pool = common::test_pool().await;
    let app = app_with_cors_origins(
        pool,
        vec![HeaderValue::from_static("http://localhost:3000")],
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/health")
                .header(header::ORIGIN, "http://localhost:5173")
                .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .is_none(),
        "unlisted origins must not receive Access-Control-Allow-Origin"
    );
}
