use crate::accounts::{self, LookupByTokenError};
use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    response::IntoResponse,
};
use serde_json::json;

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let value = headers.get(AUTHORIZATION)?.to_str().ok()?;
    let token = value.strip_prefix("Bearer ")?;
    Some(token)
}

/// Return `{ id, email }` for the account linked to `Authorization: Bearer <token>`.
pub async fn me(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    let Some(token) = bearer_token(&headers) else {
        return StatusCode::UNAUTHORIZED.into_response();
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
        Err(LookupByTokenError::Unauthorized) => StatusCode::UNAUTHORIZED.into_response(),
        Err(LookupByTokenError::Internal) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
