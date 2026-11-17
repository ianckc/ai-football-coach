mod common;

use axum::{
    body::Body,
    http::{HeaderMap, Request, StatusCode, header::SET_COOKIE},
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

fn cookie_attribute_value<'a>(set_cookie: &'a str, name: &str) -> Option<&'a str> {
    set_cookie.split(';').find_map(|part| {
        let part = part.trim();
        part.split_once('=').and_then(|(key, value)| {
            key.trim()
                .eq_ignore_ascii_case(name)
                .then_some(value.trim())
        })
    })
}

/// Acceptance slice (#71): On successful `POST /login`, set a session cookie
/// (httpOnly, Secure in non-local env, SameSite appropriate for the local +
/// deployed topology) whose value is the opaque session `token` already stored
/// in Postgres `sessions`; do not require the browser to read the token from
/// the JSON body to stay logged in.
#[tokio::test]
async fn login_post_sets_httponly_session_cookie_with_opaque_token() {
    let pool = common::test_pool().await;
    let app = common::test_app().await;
    let email = unique_email("login-cookie");
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

    let cookies = set_cookie_headers(response.headers());
    assert!(
        !cookies.is_empty(),
        "successful POST /login must set a session cookie via Set-Cookie"
    );

    let body = response
        .into_body()
        .collect()
        .await
        .expect("read body")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("POST /login must return JSON");
    let token = json
        .get("token")
        .and_then(|v| v.as_str())
        .expect("success body must include token as a string");
    assert!(
        !token.is_empty(),
        "token must be a non-empty opaque session token"
    );

    let session_cookie = cookies.iter().find(|header| {
        header
            .split(';')
            .next()
            .and_then(|pair| pair.split_once('='))
            .is_some_and(|(name, value)| {
                name.trim().eq_ignore_ascii_case("session") && value.trim() == token
            })
    });
    let session_cookie = session_cookie.unwrap_or_else(|| {
        panic!(
            "successful POST /login must Set-Cookie session=<opaque token>; got Set-Cookie={cookies:?}"
        )
    });

    assert!(
        cookie_attribute_present(session_cookie, "HttpOnly"),
        "session cookie must be HttpOnly so the browser cannot read the token from JS; got {session_cookie}"
    );

    let same_site = cookie_attribute_value(session_cookie, "SameSite")
        .unwrap_or_else(|| panic!("session cookie must set SameSite; got {session_cookie}"))
        .to_ascii_lowercase();
    assert!(
        same_site == "lax" || same_site == "none" || same_site == "strict",
        "session cookie SameSite must be Lax, None, or Strict for the local + deployed topology; got {session_cookie}"
    );

    let stored: Option<String> = sqlx::query_scalar(
        "SELECT token
         FROM sessions
         WHERE token = $1",
    )
    .bind(token)
    .fetch_optional(&pool)
    .await
    .expect("query sessions for login token");
    assert_eq!(
        stored.as_deref(),
        Some(token),
        "session cookie value must be the opaque token already stored in Postgres sessions"
    );
}

/// Acceptance slice (#71): cookie attributes required for local Vite
/// (`localhost:3000`) → backend (`127.0.0.1:8000`) cross-origin credentialed
/// requests (different sites, so SameSite=Lax is not sent on XHR/fetch).
#[tokio::test]
async fn login_session_cookie_uses_samesite_none_for_cross_origin_local_vite() {
    let app = common::test_app().await;
    let email = unique_email("login-cookie-cross");
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

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/login")
                .header("content-type", "application/json")
                .header("origin", "http://localhost:3000")
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

    let cookies = set_cookie_headers(response.headers());
    let session_cookie = cookies.iter().find(|header| {
        header
            .split(';')
            .next()
            .and_then(|pair| pair.split_once('='))
            .is_some_and(|(name, _)| name.trim().eq_ignore_ascii_case("session"))
    });
    let session_cookie = session_cookie.unwrap_or_else(|| {
        panic!("successful POST /login must Set-Cookie session=…; got Set-Cookie={cookies:?}")
    });

    let same_site = cookie_attribute_value(session_cookie, "SameSite")
        .unwrap_or_else(|| panic!("session cookie must set SameSite; got {session_cookie}"))
        .to_ascii_lowercase();
    assert_eq!(
        same_site, "none",
        "local Vite on localhost:3000 calling API on 127.0.0.1:8000 is cross-site; session cookie must use SameSite=None so credentials:include can send it; got {session_cookie}"
    );
    assert!(
        cookie_attribute_present(session_cookie, "Secure"),
        "SameSite=None requires Secure for modern browsers (including local http://127.0.0.1); got {session_cookie}"
    );
}
