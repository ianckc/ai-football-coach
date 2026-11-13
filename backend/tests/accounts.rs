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

/// Acceptance slice: Accept JSON body `{ "email": string, "password": string }`.
#[tokio::test]
async fn accounts_post_accepts_email_password_json() {
    let accepted = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"email":"coach@example.com","password":"secret123"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(
        accepted.status(),
        StatusCode::UNSUPPORTED_MEDIA_TYPE,
        "POST /accounts must accept application/json"
    );
    assert_ne!(
        accepted.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "POST /accounts must accept JSON {{email, password}}"
    );
    assert!(
        accepted.status().is_success(),
        "POST /accounts must accept JSON {{email, password}}; got {}",
        accepted.status()
    );

    // Proves JSON acceptance is wired: non-JSON content types must not be treated as that body.
    let rejected = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "text/plain")
                .body(Body::from(
                    r#"{"email":"coach@example.com","password":"secret123"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        rejected.status(),
        StatusCode::UNSUPPORTED_MEDIA_TYPE,
        "POST /accounts must accept application/json bodies (reject non-JSON content types)"
    );
}
