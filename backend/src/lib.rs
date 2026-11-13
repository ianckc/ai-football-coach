use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    /// Accounts keyed by email. In-memory store is enough for this ticket.
    accounts: Arc<Mutex<HashMap<String, StoredAccount>>>,
}

#[allow(dead_code)] // retained for the account record; login is out of scope
struct StoredAccount {
    id: String,
    email: String,
    password_hash: String,
}

#[derive(Deserialize)]
struct CreateAccountRequest {
    email: Option<String>,
    password: Option<String>,
}

fn is_valid_email(email: &str) -> bool {
    let Some((local, domain)) = email.split_once('@') else {
        return false;
    };
    !local.is_empty() && !domain.is_empty() && domain.contains('.')
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn create_account(
    State(state): State<AppState>,
    Json(body): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    let email = body.email.as_deref().unwrap_or("").trim();
    let password = body.password.as_deref().unwrap_or("");

    if email.is_empty() || !is_valid_email(email) || password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid email or password" })),
        )
            .into_response();
    }

    let Ok(password_hash) = hash_password(password) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "failed to create account" })),
        )
            .into_response();
    };

    let id = Uuid::new_v4().to_string();
    let mut accounts = state.accounts.lock().expect("accounts lock");

    if accounts.contains_key(email) {
        return (
            StatusCode::CONFLICT,
            Json(json!({ "error": "email already exists" })),
        )
            .into_response();
    }

    accounts.insert(
        email.to_string(),
        StoredAccount {
            id: id.clone(),
            email: email.to_string(),
            password_hash,
        },
    );

    (
        StatusCode::CREATED,
        Json(json!({
            "id": id,
            "email": email,
        })),
    )
        .into_response()
}

/// Application router. Routes are added as acceptance slices land.
pub fn app() -> Router {
    let state = AppState {
        accounts: Arc::new(Mutex::new(HashMap::new())),
    };

    Router::new()
        .route("/health", get(health))
        .route("/accounts", post(create_account))
        .with_state(state)
}
