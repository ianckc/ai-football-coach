use axum::{Json, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

/// Accepts JSON `{ email, password }`. Credential checks / token response come later.
pub async fn login(Json(body): Json<LoginRequest>) -> StatusCode {
    let _ = (body.email, body.password);
    StatusCode::OK
}
