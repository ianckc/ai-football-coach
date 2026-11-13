use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use uuid::Uuid;

#[derive(Deserialize)]
struct CreateAccountRequest {
    email: Option<String>,
    #[allow(dead_code)] // accepted now; hashing/storage comes in later slices
    password: Option<String>,
}

fn is_valid_email(email: &str) -> bool {
    let Some((local, domain)) = email.split_once('@') else {
        return false;
    };
    !local.is_empty() && !domain.is_empty() && domain.contains('.')
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn create_account(Json(body): Json<CreateAccountRequest>) -> impl IntoResponse {
    let email = body.email.as_deref().unwrap_or("").trim();
    let password = body.password.as_deref().unwrap_or("");

    if email.is_empty() || !is_valid_email(email) || password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid email or password" })),
        )
            .into_response();
    }

    (
        StatusCode::CREATED,
        Json(json!({
            "id": Uuid::new_v4().to_string(),
            "email": email,
        })),
    )
        .into_response()
}

/// Application router. Routes are added as acceptance slices land.
pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/accounts", post(create_account))
}
