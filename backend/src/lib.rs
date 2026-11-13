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
    email: String,
    #[allow(dead_code)] // accepted now; hashing/storage comes in later slices
    password: String,
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn create_account(Json(body): Json<CreateAccountRequest>) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(json!({
            "id": Uuid::new_v4().to_string(),
            "email": body.email,
        })),
    )
}

/// Application router. Routes are added as acceptance slices land.
pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/accounts", post(create_account))
}
