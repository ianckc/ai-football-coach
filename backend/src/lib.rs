use axum::{routing::get, Router};

/// Application router. Routes are added as acceptance slices land.
pub fn app() -> Router {
    Router::new().route("/health", get(|| async {}))
}
