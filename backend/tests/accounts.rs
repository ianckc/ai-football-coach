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

/// Acceptance slice: Map wrong JSON field types for `email` / `password` (e.g.
/// `"email": 1`) to `400` with a JSON error body — not Axum's default 422.
#[tokio::test]
async fn accounts_post_rejects_wrong_json_field_types_with_400() {
    let cases = [
        (r#"{"email":1,"password":"secret123"}"#, "email as number"),
        (
            r#"{"email":"coach@example.com","password":1}"#,
            "password as number",
        ),
        (
            r#"{"email":true,"password":"secret123"}"#,
            "email as boolean",
        ),
        (
            r#"{"email":"coach@example.com","password":false}"#,
            "password as boolean",
        ),
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
            "POST /accounts must return 400 for {label} (not 422)"
        );
        assert_ne!(
            response.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "POST /accounts must not return Axum default 422 for {label}"
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

/// Acceptance slice: Reject whitespace-only passwords (trim before the non-empty
/// check) with `400` and a JSON error body.
#[tokio::test]
async fn accounts_post_rejects_whitespace_only_password() {
    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"email":"coach@example.com","password":"   "}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "POST /accounts must return 400 for a whitespace-only password"
    );

    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("read body")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&bytes)
        .expect("POST /accounts must return a JSON error body for whitespace-only password");
    assert!(
        json.is_object(),
        "POST /accounts whitespace-only password error body must be a JSON object"
    );
}

/// Acceptance slice (#16): Reject passwords shorter than 8 characters with `400`
/// and a JSON error body.
#[tokio::test]
async fn accounts_post_rejects_password_shorter_than_8_characters() {
    let cases = [
        (
            r#"{"email":"coach@example.com","password":"short12"}"#,
            "7-char password",
        ),
        (
            r#"{"email":"coach@example.com","password":"1234567"}"#,
            "7-digit password",
        ),
        (
            r#"{"email":"coach@example.com","password":"a"}"#,
            "1-char password",
        ),
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
            "POST /accounts must return 400 for password shorter than 8 characters ({label})"
        );

        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("read body")
            .to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or_else(|_| {
            panic!(
                "POST /accounts must return a JSON error body for password shorter than 8 ({label})"
            )
        });
        assert!(
            json.is_object(),
            "POST /accounts short-password error body must be a JSON object for {label}"
        );
    }
}

/// Acceptance slice (#16): Run password hashing off the async runtime (e.g.
/// `spawn_blocking`) so Argon2 does not block Axum workers.
#[test]
fn create_account_runs_password_hashing_off_async_runtime() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let src = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("must read {}: {e}", path.display()));

    let handler_start = src
        .find("async fn create_account")
        .expect("create_account handler must exist");
    let handler = &src[handler_start..];
    let handler_end = handler[1..]
        .find("\nasync fn ")
        .or_else(|| handler[1..].find("\nfn "))
        .map(|i| i + 1)
        .unwrap_or(handler.len());
    let handler = &handler[..handler_end];

    assert!(
        handler.contains("spawn_blocking"),
        "create_account must run password hashing via spawn_blocking (or equivalent) so Argon2 does not block Axum workers"
    );
    assert!(
        handler.contains("hash_password"),
        "create_account must still hash passwords (via hash_password) off the async runtime"
    );
}

/// Acceptance slice: Normalize email for uniqueness (e.g. lowercase) so
/// `Coach@example.com` and `coach@example.com` collide as the same account (`409`).
#[tokio::test]
async fn accounts_post_rejects_duplicate_email_differing_only_by_case() {
    let app = app();

    let first = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"email":"Coach@example.com","password":"secret123"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        first.status(),
        StatusCode::CREATED,
        "first POST /accounts with Coach@example.com must succeed"
    );

    let second = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accounts")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"email":"coach@example.com","password":"other-secret"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        second.status(),
        StatusCode::CONFLICT,
        "POST /accounts must return 409 when email differs only by case"
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
        "POST /accounts case-duplicate email error body must be a JSON object"
    );
}

/// Acceptance slice: Document current email validation rules (or tighten to a
/// clearly defined product rule) so clients know what `POST /accounts` accepts.
#[test]
fn backend_api_docs_document_email_validation_rules() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/docs/backend-api.mdx");
    let docs = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("must read {}: {e}", path.display()));
    let lower = docs.to_ascii_lowercase();

    assert!(
        !lower.contains("case-sensitive"),
        "backend-api docs must not claim case-sensitive email uniqueness; emails are normalized"
    );
    assert!(
        lower.contains("lowercase")
            || lower.contains("case-insensitive")
            || lower.contains("normaliz"),
        "backend-api docs must document that emails are normalized (e.g. lowercase) for uniqueness"
    );
    assert!(
        lower.contains("local@domain")
            || (lower.contains("local") && lower.contains("domain") && docs.contains('@')),
        "backend-api docs must document the email shape POST /accounts accepts (local@domain…)"
    );
    assert!(
        lower.contains("trim") && lower.contains("non-empty"),
        "backend-api docs must document trim and non-empty email requirements"
    );
    // Product rule clients can rely on: domain must include a '.' (not full RFC 5322).
    assert!(
        lower.contains("domain")
            && (docs.contains(".tld")
                || lower.contains("contains a '.'")
                || lower.contains("contains a `.`")
                || lower.contains("dot in the domain")
                || lower.contains("'.' in the domain")),
        "backend-api docs must state the domain-dot product rule for accepted emails"
    );
}
