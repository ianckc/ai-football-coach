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

/// Build `Set-Cookie` for the opaque session token (HttpOnly + SameSite).
/// Secure is omitted unless `APP_ENV` is set to a non-local value so
/// http://localhost can store the cookie during local/test runs.
fn session_set_cookie(token: &str) -> HeaderValue {
    let secure = std::env::var("APP_ENV")
        .is_ok_and(|env| !env.eq_ignore_ascii_case("local") && !env.eq_ignore_ascii_case("test"));
    let mut value = format!("session={token}; Path=/; HttpOnly; SameSite=Lax");
    if secure {
        value.push_str("; Secure");
    }
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
