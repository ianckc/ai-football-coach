use crate::accounts::{self, DeleteSessionError};
use crate::state::AppState;
use axum::{
    extract::State,
    http::{
        HeaderMap, HeaderValue, StatusCode,
        header::{COOKIE, SET_COOKIE},
    },
    response::IntoResponse,
};

/// Read the opaque session token from a `Cookie: session=<token>` header.
fn session_cookie_token(headers: &HeaderMap) -> Option<&str> {
    let value = headers.get(COOKIE)?.to_str().ok()?;
    value.split(';').find_map(|part| {
        let part = part.trim();
        part.split_once('=').and_then(|(name, token)| {
            name.trim()
                .eq_ignore_ascii_case("session")
                .then_some(token.trim())
        })
    })
}

/// Clear the httpOnly `session` cookie (empty value + Max-Age=0).
fn clear_session_cookie() -> HeaderValue {
    let secure = std::env::var("APP_ENV").is_ok_and(|env| {
        !env.eq_ignore_ascii_case("local") && !env.eq_ignore_ascii_case("test")
    });
    let mut value = String::from("session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0");
    if secure {
        value.push_str("; Secure");
    }
    HeaderValue::from_str(&value).expect("clear session Set-Cookie must be a valid header value")
}

/// Clear the session cookie and delete the matching `sessions` row.
pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    if let Some(token) = session_cookie_token(&headers)
        && let Err(DeleteSessionError::Internal) =
            accounts::delete_session_by_token(&state.pool, token).await
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let mut response = StatusCode::OK.into_response();
    response
        .headers_mut()
        .insert(SET_COOKIE, clear_session_cookie());
    response
}
