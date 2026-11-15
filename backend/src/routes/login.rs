use crate::accounts::{self, LoginError};
use crate::state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: Option<Value>,
    password: Option<Value>,
}

fn error_json(status: StatusCode, message: &str) -> axum::response::Response {
    (status, Json(json!({ "error": message }))).into_response()
}

/// Authenticate with `{ email, password }` and return `{ id, email, token }` on success.
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    match accounts::login(&state.pool, body.email.as_ref(), body.password.as_ref()).await {
        Ok(session) => (
            StatusCode::OK,
            Json(json!({
                "id": session.id.to_string(),
                "email": session.email,
                "token": session.token,
            })),
        )
            .into_response(),
        Err(LoginError::InvalidInput) => {
            error_json(StatusCode::BAD_REQUEST, "invalid email or password")
        }
        Err(LoginError::Unauthorized) => StatusCode::UNAUTHORIZED.into_response(),
        Err(LoginError::Internal) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
