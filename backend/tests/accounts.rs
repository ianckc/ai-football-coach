use ai_football_coach_backend::app;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn is_uuid_v4(value: &str) -> bool {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() != 5
        || parts[0].len() != 8
        || parts[1].len() != 4
        || parts[2].len() != 4
        || parts[3].len() != 4
        || parts[4].len() != 12
    {
        return false;
    }
    if !parts
        .iter()
        .all(|part| part.chars().all(|c| c.is_ascii_hexdigit()))
    {
        return false;
    }
    // Version nibble must be 4; RFC 4122 variant nibble must be 8, 9, a, or b.
    parts[2].starts_with('4')
        && matches!(
            parts[3].chars().next(),
            Some('8' | '9' | 'a' | 'b' | 'A' | 'B')
        )
}

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

/// Acceptance slice: On success, return `201` with `{ "id": string, "email": string }`
/// where `id` is a UUID v4 string (never return the password).
#[tokio::test]
async fn accounts_post_returns_201_with_id_and_email() {
    let email = "coach@example.com";
    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{"email":"{email}","password":"secret123"}}"#
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "POST /accounts must return 201 on success"
    );

    let body = response
        .into_body()
        .collect()
        .await
        .expect("read body")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("POST /accounts must return JSON");

    let obj = json
        .as_object()
        .expect("POST /accounts success body must be a JSON object");
    assert_eq!(
        obj.get("email").and_then(|v| v.as_str()),
        Some(email),
        "success body must include the account email"
    );
    let id = obj
        .get("id")
        .and_then(|v| v.as_str())
        .expect("success body must include id as a string");
    assert!(is_uuid_v4(id), "id must be a UUID v4 string, got {id:?}");
    assert!(
        !obj.contains_key("password"),
        "success body must never return the password"
    );
}

/// Acceptance slice: Reject missing/invalid email or password with `400` and a JSON error body.
#[tokio::test]
async fn accounts_post_rejects_missing_or_invalid_email_or_password() {
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
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/accounts")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "POST /accounts must return 400 for {label}"
        );

        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("read body")
            .to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| panic!("POST /accounts must return a JSON error body for {label}"));
        assert!(
            json.is_object(),
            "POST /accounts error body must be a JSON object for {label}"
        );
    }
}

/// Acceptance slice: Reject a duplicate email with `409` and a JSON error body.
#[tokio::test]
async fn accounts_post_rejects_duplicate_email() {
    let app = app();
    let body = r#"{"email":"duplicate@example.com","password":"secret123"}"#;

    let first = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        first.status(),
        StatusCode::CREATED,
        "first POST /accounts with a new email must succeed"
    );

    let second = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        second.status(),
        StatusCode::CONFLICT,
        "POST /accounts must return 409 for a duplicate email"
    );

    let bytes = second
        .into_body()
        .collect()
        .await
        .expect("read body")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&bytes).expect("POST /accounts must return a JSON error body");
    assert!(
        json.is_object(),
        "POST /accounts duplicate-email error body must be a JSON object"
    );
}
