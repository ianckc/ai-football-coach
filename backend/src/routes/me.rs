use crate::accounts::{self, LookupByTokenError};
use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::{
        HeaderMap, StatusCode,
        header::{AUTHORIZATION, COOKIE},
    },
    response::IntoResponse,
};
use serde_json::json;

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let value = headers.get(AUTHORIZATION)?.to_str().ok()?;
    let token = value.strip_prefix("Bearer ")?;
    Some(token)
}

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

fn session_token(headers: &HeaderMap) -> Option<&str> {
    bearer_token(headers).or_else(|| session_cookie_token(headers))
}

fn unauthorized() -> axum::response::Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "invalid token" })),
    )
        .into_response()
}

/// Return `{ id, email }` for the account linked to the session cookie or
/// `Authorization: Bearer <token>`.
pub async fn me(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    let Some(token) = session_token(&headers) else {
        return unauthorized();
    };

    match accounts::lookup_by_token(&state.pool, token).await {
        Ok(account) => (
            StatusCode::OK,
            Json(json!({
                "id": account.id.to_string(),
                "email": account.email,
            })),
        )
            .into_response(),
        Err(LookupByTokenError::Unauthorized) => unauthorized(),
        Err(LookupByTokenError::Internal) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
