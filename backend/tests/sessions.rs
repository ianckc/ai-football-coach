mod common;

use axum::{
    Router,
    body::Body,
    http::{Request, Response, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;
use uuid::Uuid;

fn unique_email(prefix: &str) -> String {
    format!("{prefix}-{}@example.com", Uuid::new_v4())
}

async fn response_json(response: Response<Body>) -> serde_json::Value {
    let body = response
        .into_body()
        .collect()
        .await
        .expect("read response body")
        .to_bytes();
    serde_json::from_slice(&body).expect("response must return JSON")
}

async fn create_account(app: Router, email: &str, password: &str) -> String {
    let created = app
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
        "setup: POST /accounts must create the account"
    );
    response_json(created)
        .await
        .get("id")
        .and_then(|v| v.as_str())
        .expect("setup: create must return id")
        .to_owned()
}

async fn login_token(app: Router, email: &str, password: &str) -> String {
    let login = app
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
        login.status(),
        StatusCode::OK,
        "setup: POST /login must succeed"
    );
    response_json(login)
        .await
        .get("token")
        .and_then(|v| v.as_str())
        .expect("setup: login must return token")
        .to_owned()
}

/// Acceptance slice (#59): Add a Postgres migration for session tokens
/// (e.g. a `sessions` table keyed by opaque `token`, linked to `accounts.id`,
/// with a created-at timestamp).
#[tokio::test]
async fn sessions_migration_creates_table_keyed_by_token_linked_to_accounts() {
    let pool = common::test_pool().await;

    let table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_schema = 'public'
              AND table_name = 'sessions'
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("query whether public.sessions exists");

    assert!(
        table_exists,
        "Postgres migration must create a public.sessions table for session tokens"
    );

    let column_names: Vec<String> = sqlx::query_scalar(
        "SELECT column_name
         FROM information_schema.columns
         WHERE table_schema = 'public'
           AND table_name = 'sessions'",
    )
    .fetch_all(&pool)
    .await
    .expect("query sessions columns");

    let has_token = column_names.iter().any(|c| c == "token");
    assert!(
        has_token,
        "sessions must include an opaque token column named token; columns={column_names:?}"
    );

    let has_created_at = column_names.iter().any(|c| c == "created_at");
    assert!(
        has_created_at,
        "sessions must include a created-at timestamp column named created_at; columns={column_names:?}"
    );

    let account_link_columns: Vec<String> = sqlx::query_scalar(
        "SELECT kcu.column_name
         FROM information_schema.table_constraints AS tc
         JOIN information_schema.key_column_usage AS kcu
           ON tc.constraint_name = kcu.constraint_name
          AND tc.table_schema = kcu.table_schema
         JOIN information_schema.constraint_column_usage AS ccu
           ON ccu.constraint_name = tc.constraint_name
          AND ccu.table_schema = tc.table_schema
         WHERE tc.constraint_type = 'FOREIGN KEY'
           AND tc.table_schema = 'public'
           AND tc.table_name = 'sessions'
           AND ccu.table_name = 'accounts'
           AND ccu.column_name = 'id'",
    )
    .fetch_all(&pool)
    .await
    .expect("query sessions → accounts.id foreign keys");

    assert!(
        !account_link_columns.is_empty(),
        "sessions must be linked to accounts.id via a foreign key"
    );

    let token_is_key: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1
            FROM information_schema.table_constraints AS tc
            JOIN information_schema.key_column_usage AS kcu
              ON tc.constraint_name = kcu.constraint_name
             AND tc.table_schema = kcu.table_schema
            WHERE tc.table_schema = 'public'
              AND tc.table_name = 'sessions'
              AND kcu.column_name = 'token'
              AND tc.constraint_type IN ('PRIMARY KEY', 'UNIQUE')
        )",
    )
    .fetch_one(&pool)
    .await
    .expect("query whether token is a primary/unique key");

    assert!(
        token_is_key,
        "sessions.token must be a PRIMARY KEY or UNIQUE key so tokens can be looked up"
    );
}

/// Acceptance slice (#59): On successful `POST /login`, persist the returned
/// `token` server-side before responding `200`.
#[tokio::test]
async fn login_persists_returned_token_in_sessions_before_200() {
    let pool = common::test_pool().await;
    let app = common::test_app().await;
    let email = unique_email("login-persist");
    let password = "secret123";
    let account_id = create_account(app.clone(), &email, password).await;
    let token = login_token(app, &email, password).await;

    assert!(
        !token.is_empty(),
        "token must be a non-empty opaque session token"
    );

    let stored_account_id: Option<String> = sqlx::query_scalar(
        "SELECT account_id::text
         FROM sessions
         WHERE token = $1",
    )
    .bind(&token)
    .fetch_optional(&pool)
    .await
    .expect("query sessions for returned login token");

    assert_eq!(
        stored_account_id.as_deref(),
        Some(account_id.as_str()),
        "successful POST /login must persist the returned token in sessions linked to the account before responding 200"
    );
}

/// Acceptance slice (#59): Given a stored `token`, look up the matching account
/// (`id`, `email`) via a backend helper or route usable by later auth
/// (e.g. `GET /me` with `Authorization: Bearer <token>`).
#[tokio::test]
async fn me_get_with_bearer_token_returns_matching_account() {
    let app = common::test_app().await;
    let email = unique_email("me-lookup");
    let password = "secret123";
    let account_id = create_account(app.clone(), &email, password).await;
    let token = login_token(app.clone(), &email, password).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/me")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "GET /me with Authorization: Bearer <stored token> must return 200"
    );

    let json = response_json(response).await;
    let obj = json
        .as_object()
        .expect("GET /me success body must be a JSON object");

    assert_eq!(
        obj.get("id").and_then(|v| v.as_str()),
        Some(account_id.as_str()),
        "GET /me must return the account id matching the stored token"
    );
    assert_eq!(
        obj.get("email").and_then(|v| v.as_str()),
        Some(email.as_str()),
        "GET /me must return the account email matching the stored token"
    );
}

/// Acceptance slice (#59): Reject missing/unknown/invalid `token` on lookup
/// with `401` and a JSON error body (same shape as other auth errors).
#[tokio::test]
async fn me_get_rejects_missing_unknown_or_invalid_token_with_401_json() {
    let app = common::test_app().await;
    let unknown_authorization = format!("Bearer {}", Uuid::new_v4());

    let cases: [(&str, Option<&str>); 3] = [
        ("missing Authorization header", None),
        ("unknown token", Some(unknown_authorization.as_str())),
        ("invalid Authorization scheme", Some("Token not-a-bearer")),
    ];

    let mut bodies = Vec::new();

    for (label, authorization) in cases {
        let mut builder = Request::builder().method("GET").uri("/me");
        if let Some(value) = authorization {
            builder = builder.header("authorization", value);
        }
        let response = app
            .clone()
            .oneshot(builder.body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "GET /me must return 401 for {label}"
        );

        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("read /me body")
            .to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| panic!("GET /me must return a JSON error body for {label}"));
        assert!(
            json.is_object(),
            "GET /me error body must be a JSON object for {label}"
        );
        assert!(
            json.get("error")
                .and_then(|v| v.as_str())
                .is_some_and(|s| !s.is_empty()),
            "GET /me error body must include a non-empty error string like other auth errors; got {json} for {label}"
        );
        bodies.push(json);
    }

    assert_eq!(
        bodies[0], bodies[1],
        "missing and unknown token must share the same 401 error shape"
    );
    assert_eq!(
        bodies[1], bodies[2],
        "unknown and invalid token must share the same 401 error shape"
    );
}

/// Acceptance slice (#59): Document the session storage shape and how clients
/// present the `token` in `docs/docs/backend-api.mdx`.
#[test]
fn backend_api_docs_document_session_storage_and_bearer_token() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/docs/backend-api.mdx");
    let docs = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("must read {}: {e}", path.display()));
    let lower = docs.to_ascii_lowercase();

    assert!(
        lower.contains("sessions"),
        "backend-api docs must document the `sessions` table used to persist session tokens"
    );
    assert!(
        lower.contains("opaque") && lower.contains("token"),
        "backend-api docs must describe the session `token` as opaque"
    );
    assert!(
        lower.contains("accounts.id"),
        "backend-api docs must document that sessions link to `accounts.id`"
    );
    assert!(
        lower.contains("created_at"),
        "backend-api docs must document the sessions `created_at` timestamp"
    );
    assert!(
        docs.contains("Authorization: Bearer <token>"),
        "backend-api docs must document presenting the token as `Authorization: Bearer <token>`"
    );
    assert!(
        docs.contains("GET /me"),
        "backend-api docs must document the `GET /me` lookup route clients use with the Bearer token"
    );
    assert!(
        docs.contains("401"),
        "backend-api docs must document the 401 response for missing/unknown/invalid tokens on GET /me"
    );
}
