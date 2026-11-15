mod accounts;
mod health;

use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/accounts", post(accounts::create_account))
}
