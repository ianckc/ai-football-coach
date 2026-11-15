use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower_http::cors::CorsLayer;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    accounts: Arc<Mutex<HashMap<String, StoredAccount>>>,
}

#[expect(dead_code)] // written for the account record; not read until login exists
struct StoredAccount {
    id: String,
    email: String,
    password_hash: String,
}

#[derive(Deserialize)]
struct CreateAccountRequest {
    email: Option<Value>,
    password: Option<Value>,
}

fn is_valid_email(email: &str) -> bool {
    let Some((local, domain)) = email.split_once('@') else {
        return false;
    };
    !local.is_empty() && !domain.is_empty() && domain.contains('.')
}

/// String fields only; wrong JSON types, null, and missing are treated as absent.
fn json_string_field(value: Option<&Value>) -> Option<&str> {
    match value {
        Some(Value::String(s)) => Some(s.as_str()),
        _ => None,
    }
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

fn error_json(status: StatusCode, message: &str) -> axum::response::Response {
    (status, Json(json!({ "error": message }))).into_response()
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn create_account(
    State(state): State<AppState>,
    Json(body): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    let email = json_string_field(body.email.as_ref())
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    let password = json_string_field(body.password.as_ref())
        .unwrap_or("")
        .trim();

    if email.is_empty() || !is_valid_email(&email) || password.chars().count() < 8 {
        return error_json(StatusCode::BAD_REQUEST, "invalid email or password");
    }

    let password = password.to_owned();
    let hash_result = tokio::task::spawn_blocking(move || hash_password(&password)).await;
    let Ok(Ok(password_hash)) = hash_result else {
        return error_json(
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to create account",
        );
    };

    let id = Uuid::new_v4().to_string();
    let mut accounts = match state.accounts.lock() {
        Ok(guard) => guard,
        Err(_) => {
            return error_json(
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to create account",
            );
        }
    };

    if accounts.contains_key(&email) {
        return error_json(StatusCode::CONFLICT, "email already exists");
    }

    let created = json!({ "id": id, "email": email });
    accounts.insert(
        email.clone(),
        StoredAccount {
            id,
            email,
            password_hash,
        },
    );

    (StatusCode::CREATED, Json(created)).into_response()
}

fn local_dev_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("http://127.0.0.1:3000"),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::ACCEPT])
}

pub fn app() -> Router {
    let state = AppState {
        accounts: Arc::new(Mutex::new(HashMap::new())),
    };

    Router::new()
        .route("/health", get(health))
        .route("/accounts", post(create_account))
        .layer(local_dev_cors())
        .with_state(state)
}
