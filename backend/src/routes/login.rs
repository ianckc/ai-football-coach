use crate::accounts::{self, LoginError};
use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::{HeaderValue, StatusCode, header::SET_COOKIE},
    response::IntoResponse,
};
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

/// Build `Set-Cookie` for the opaque session token.
///
/// Local Vite (`http://localhost:3000`) → API (`http://127.0.0.1:8000`) is
/// cross-site, so the cookie uses `SameSite=None; Secure` (required for
/// credentialed `fetch`). `HttpOnly` keeps the opaque token out of JS.
fn session_set_cookie(token: &str) -> HeaderValue {
    let value = format!("session={token}; Path=/; HttpOnly; SameSite=None; Secure");
    // Session tokens are UUID strings (ASCII); this cannot fail for valid tokens.
    HeaderValue::from_str(&value).expect("session Set-Cookie must be a valid header value")
}

/// Authenticate with `{ email, password }` and return `{ id, email, token }` on success.
/// Also sets an httpOnly `session` cookie so the browser can stay logged in without
/// reading the token from the JSON body.
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    match accounts::login(&state.pool, body.email.as_ref(), body.password.as_ref()).await {
        Ok(session) => {
            let mut response = (
                StatusCode::OK,
                Json(json!({
                    "id": session.id.to_string(),
                    "email": session.email,
                    "token": session.token,
                })),
            )
                .into_response();
            response
                .headers_mut()
                .insert(SET_COOKIE, session_set_cookie(&session.token));
            response
        }
        Err(LoginError::InvalidInput) => {
            error_json(StatusCode::BAD_REQUEST, "invalid email or password")
        }
        Err(LoginError::Unauthorized) => {
            error_json(StatusCode::UNAUTHORIZED, "invalid email or password")
        }
        Err(LoginError::Internal) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
