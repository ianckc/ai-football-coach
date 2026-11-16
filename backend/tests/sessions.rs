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
        .expect("read login body")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("POST /login must return JSON");
    let token = json
        .get("token")
        .and_then(|v| v.as_str())
        .expect("success body must include token")
        .to_owned();
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
