use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

/// Application router. Routes are added as acceptance slices land.
pub fn app() -> Router {
    Router::new().route("/health", get(health))
}
