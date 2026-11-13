use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

/// Application router. Routes are added as acceptance slices land.
pub fn app() -> Router {
    Router::new().route("/health", get(health))
}
