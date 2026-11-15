mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use uuid::Uuid;

fn unique_email(prefix: &str) -> String {
    format!("{prefix}-{}@example.com", Uuid::new_v4())
}

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

/// Acceptance slice (#33): Accept JSON body `{ "email": string, "password": string }`.
#[tokio::test]
async fn login_post_accepts_email_password_json() {
    let app = common::test_app().await;
    let email = unique_email("login-accept");
    let password = "secret123";

    let created = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{"email":"{email}","password":"{password}"}}"#
                )))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        created.status(),
        StatusCode::CREATED,
        "setup: POST /accounts must create the account used for login"
    );

    let accepted = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{"email":"{email}","password":"{password}"}}"#
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(
        accepted.status(),
        StatusCode::UNSUPPORTED_MEDIA_TYPE,
        "POST /login must accept application/json"
    );
    assert_ne!(
        accepted.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "POST /login must accept JSON {{email, password}}"
    );
    assert!(
        accepted.status().is_success(),
        "POST /login must accept JSON {{email, password}}; got {}",
        accepted.status()
    );

    // Proves JSON acceptance is wired: non-JSON content types must not be treated as that body.
    let rejected = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header("content-type", "text/plain")
                .body(Body::from(format!(
                    r#"{{"email":"{email}","password":"{password}"}}"#
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        rejected.status(),
        StatusCode::UNSUPPORTED_MEDIA_TYPE,
        "POST /login must accept application/json bodies (reject non-JSON content types)"
    );
}
