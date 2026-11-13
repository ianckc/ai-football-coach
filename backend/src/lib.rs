use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde_json::{Value, json};

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn create_account() -> StatusCode {
    StatusCode::OK
}

/// Application router. Routes are added as acceptance slices land.
pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/accounts", post(create_account))
}
