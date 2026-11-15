use crate::accounts::{self, CreateAccountError};
use crate::state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    email: Option<Value>,
    password: Option<Value>,
}

fn error_json(status: StatusCode, message: &str) -> axum::response::Response {
    (status, Json(json!({ "error": message }))).into_response()
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(body): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    match accounts::create(&state.pool, body.email.as_ref(), body.password.as_ref()).await {
        Ok(created) => (
            StatusCode::CREATED,
            Json(json!({ "id": created.id.to_string(), "email": created.email })),
        )
            .into_response(),
        Err(CreateAccountError::InvalidInput) => {
            error_json(StatusCode::BAD_REQUEST, "invalid email or password")
        }
        Err(CreateAccountError::Conflict) => {
            error_json(StatusCode::CONFLICT, "email already exists")
        }
        Err(CreateAccountError::Internal) => error_json(
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to create account",
        ),
    }
}
