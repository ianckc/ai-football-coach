use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
#[allow(dead_code)] // fields are validated by deserialization; used in later slices
struct CreateAccountRequest {
    email: String,
    password: String,
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn create_account(Json(_body): Json<CreateAccountRequest>) -> StatusCode {
    StatusCode::OK
}

/// Application router. Routes are added as acceptance slices land.
pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/accounts", post(create_account))
}
