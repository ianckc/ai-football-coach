mod accounts;
mod health;
mod login;
mod logout;
mod me;

use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/accounts", post(accounts::create_account))
        .route("/login", post(login::login))
        .route("/logout", post(logout::logout))
        .route("/me", get(me::me))
}
