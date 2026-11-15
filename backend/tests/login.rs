mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
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

/// Acceptance slice (#33): On success (valid credentials), return `200` with
/// `{ "id": string, "email": string, "token": string }` where `id` and `email`
/// match the account and `token` is a non-empty opaque session token (never
/// return the password).
#[tokio::test]
async fn login_post_returns_200_with_id_email_and_token() {
    let app = common::test_app().await;
    let email = unique_email("login-success");
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

    let created_body = created
        .into_body()
        .collect()
        .await
        .expect("read create body")
        .to_bytes();
    let created_json: serde_json::Value =
        serde_json::from_slice(&created_body).expect("setup: create must return JSON");
    let account_id = created_json
        .get("id")
        .and_then(|v| v.as_str())
        .expect("setup: create must return id")
        .to_owned();

    let response = app
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

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "POST /login must return 200 on success"
    );

    let body = response
        .into_body()
        .collect()
        .await
        .expect("read body")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("POST /login must return JSON");

    let obj = json
        .as_object()
        .expect("POST /login success body must be a JSON object");
    assert_eq!(
        obj.get("id").and_then(|v| v.as_str()),
        Some(account_id.as_str()),
        "success body id must match the account"
    );
    assert_eq!(
        obj.get("email").and_then(|v| v.as_str()),
        Some(email.as_str()),
        "success body email must match the account"
    );
    let token = obj
        .get("token")
        .and_then(|v| v.as_str())
        .expect("success body must include token as a string");
    assert!(
        !token.is_empty(),
        "token must be a non-empty opaque session token"
    );
    assert!(
        !obj.contains_key("password"),
        "success body must never return the password"
    );
}

/// Acceptance slice (#33): Reject missing/invalid email or password shape with
/// `400` and a JSON error body.
#[tokio::test]
async fn login_post_rejects_missing_or_invalid_email_or_password() {
    let cases = [
        (r#"{"email":"","password":"secret123"}"#, "empty email"),
        (
            r#"{"email":"not-an-email","password":"secret123"}"#,
            "invalid email",
        ),
        (
            r#"{"email":"coach@example.com","password":""}"#,
            "empty password",
        ),
        (r#"{"email":"coach@example.com"}"#, "missing password"),
        (r#"{"password":"secret123"}"#, "missing email"),
    ];

    for (body, label) in cases {
        let response = common::test_app()
            .await
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "POST /login must return 400 for {label}"
        );

        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("read body")
            .to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| panic!("POST /login must return a JSON error body for {label}"));
        assert!(
            json.is_object(),
            "POST /login error body must be a JSON object for {label}"
        );
    }
}

/// Acceptance slice (#33): Reject unknown email or wrong password with `401`
/// and a JSON error body (same error shape for both — do not reveal which failed).
#[tokio::test]
async fn login_post_rejects_unknown_email_or_wrong_password_with_same_401_shape() {
    let app = common::test_app().await;
    let email = unique_email("login-auth");
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
        "setup: POST /accounts must create the account used for wrong-password case"
    );

    let cases = [
        (
            format!(
                r#"{{"email":"unknown-{}@example.com","password":"{password}"}}"#,
                Uuid::new_v4()
            ),
            "unknown email",
        ),
        (
            format!(r#"{{"email":"{email}","password":"wrong-password"}}"#),
            "wrong password",
        ),
    ];

    let mut bodies = Vec::new();

    for (body, label) in &cases {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header("content-type", "application/json")
                    .body(Body::from(body.clone()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "POST /login must return 401 for {label}"
        );

        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("read body")
            .to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| panic!("POST /login must return a JSON error body for {label}"));
        assert!(
            json.is_object(),
            "POST /login error body must be a JSON object for {label}"
        );
        bodies.push(json);
    }

    assert_eq!(
        bodies[0], bodies[1],
        "unknown email and wrong password must share the same 401 error shape (do not reveal which failed)"
    );
}
