use argon2::{Argon2, password_hash::PasswordHasher};
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
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
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
}

fn error_json(status: StatusCode, message: &str) -> axum::response::Response {
    (status, Json(json!({ "error": message }))).into_response()
}

/// Connect to Postgres using `DATABASE_URL` (or any Postgres URL).
///
/// # Errors
///
/// Returns an error if the URL is invalid or the server is unreachable.
pub async fn connect_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(database_url)
        .await
}

/// Apply SQL migrations from `backend/migrations`.
///
/// # Errors
///
/// Returns an error if a migration fails to apply.
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
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

    let id = Uuid::new_v4();
    let inserted = sqlx::query_scalar::<_, Uuid>(
        r"
        INSERT INTO accounts (id, email, password_hash)
        VALUES ($1, $2, $3)
        ON CONFLICT (email) DO NOTHING
        RETURNING id
        ",
    )
    .bind(id)
    .bind(&email)
    .bind(&password_hash)
    .fetch_optional(&state.pool)
    .await;

    match inserted {
        Ok(Some(created_id)) => (
            StatusCode::CREATED,
            Json(json!({ "id": created_id.to_string(), "email": email })),
        )
            .into_response(),
        Ok(None) => error_json(StatusCode::CONFLICT, "email already exists"),
        Err(_) => error_json(
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to create account",
        ),
    }
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

pub fn app(pool: PgPool) -> Router {
    let state = AppState { pool };

    Router::new()
        .route("/health", get(health))
        .route("/accounts", post(create_account))
        .layer(local_dev_cors())
        .with_state(state)
}
