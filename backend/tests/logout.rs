mod common;

use axum::{
    body::Body,
    http::{HeaderMap, Request, Response, StatusCode, header::SET_COOKIE},
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

fn set_cookie_headers(headers: &HeaderMap) -> Vec<String> {
    headers
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok().map(str::to_owned))
        .collect()
}

fn cookie_attribute_present(set_cookie: &str, name: &str) -> bool {
    set_cookie.split(';').any(|part| {
        let part = part.trim();
        part.eq_ignore_ascii_case(name)
            || part
                .split_once('=')
                .is_some_and(|(key, _)| key.trim().eq_ignore_ascii_case(name))
    })
}

fn cookie_pair_name_value(set_cookie: &str) -> Option<(&str, &str)> {
    set_cookie
        .split(';')
        .next()
        .and_then(|pair| pair.split_once('='))
        .map(|(name, value)| (name.trim(), value.trim()))
}

fn session_cookie_cleared(set_cookie: &str) -> bool {
    let Some((name, value)) = cookie_pair_name_value(set_cookie) else {
        return false;
    };
    if !name.eq_ignore_ascii_case("session") {
        return false;
    }
    let max_age_zero = cookie_attribute_present(set_cookie, "Max-Age")
        && set_cookie.split(';').any(|part| {
            part.trim()
                .split_once('=')
                .is_some_and(|(key, val)| {
                    key.trim().eq_ignore_ascii_case("Max-Age") && val.trim() == "0"
                })
        });
    let expires_past = cookie_attribute_present(set_cookie, "Expires");
    value.is_empty() || max_age_zero || expires_past
}

async fn create_account_and_login_token(
    app: axum::Router,
    email: &str,
    password: &str,
) -> String {
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
        "setup: POST /accounts must create the account"
    );

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

/// Acceptance slice (#71): Add logout that clears the session cookie and
/// invalidates (deletes) the matching `sessions` row.
#[tokio::test]
async fn logout_post_clears_session_cookie_and_deletes_sessions_row() {
    let pool = common::test_pool().await;
    let app = common::test_app().await;
    let email = unique_email("logout");
    let password = "secret123";
    let token = create_account_and_login_token(app.clone(), &email, password).await;

    let stored_before: Option<String> = sqlx::query_scalar(
        "SELECT token
         FROM sessions
         WHERE token = $1",
    )
    .bind(&token)
    .fetch_optional(&pool)
    .await
    .expect("setup: query sessions for login token");
    assert_eq!(
        stored_before.as_deref(),
        Some(token.as_str()),
        "setup: login must have persisted the session token"
    );

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/logout")
                .header("cookie", format!("session={token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(
        response.status(),
        StatusCode::NOT_FOUND,
        "POST /logout must exist (must not return 404)"
    );
    assert_ne!(
        response.status(),
        StatusCode::METHOD_NOT_ALLOWED,
        "POST /logout must accept POST (must not return 405)"
    );
    assert!(
        response.status().is_success(),
        "POST /logout must succeed when a session cookie is present; got {}",
        response.status()
    );

    let cookies = set_cookie_headers(response.headers());
    let cleared = cookies.iter().find(|header| session_cookie_cleared(header));
    assert!(
        cleared.is_some(),
        "POST /logout must clear the session cookie via Set-Cookie (empty value and/or Max-Age=0); got Set-Cookie={cookies:?}"
    );
    let cleared = cleared.expect("cleared session cookie");
    assert!(
        cookie_attribute_present(cleared, "HttpOnly"),
        "cleared session cookie must remain HttpOnly; got {cleared}"
    );

    let stored_after: Option<String> = sqlx::query_scalar(
        "SELECT token
         FROM sessions
         WHERE token = $1",
    )
    .bind(&token)
    .fetch_optional(&pool)
    .await
    .expect("query sessions after logout");
    assert_eq!(
        stored_after, None,
        "POST /logout must invalidate (delete) the matching sessions row"
    );

    let me = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/me")
                .header("cookie", format!("session={token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        me.status(),
        StatusCode::UNAUTHORIZED,
        "after logout, GET /me with the old session cookie must return 401"
    );
}

/// Acceptance slice (#71): document the logout flow.
#[test]
fn backend_api_docs_document_logout_flow() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/docs/backend-api.mdx");
    let docs = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("must read {}: {e}", path.display()));
    let lower = docs.to_ascii_lowercase();

    assert!(
        docs.contains("POST /logout") || lower.contains("post `/logout`"),
        "backend-api docs must document POST /logout"
    );
    assert!(
        lower.contains("cookie") && lower.contains("session"),
        "backend-api docs must document that logout clears the session cookie"
    );
    assert!(
        lower.contains("sessions")
            && (lower.contains("delete")
                || lower.contains("invalidat")
                || lower.contains("remov")),
        "backend-api docs must document that logout invalidates/deletes the matching sessions row"
    );
}
