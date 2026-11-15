use ai_football_coach_backend::{app, connect_pool, run_migrations};
use axum::Router;
use sqlx::PgPool;
use std::sync::OnceLock;
use tokio::sync::Mutex;

/// Same URL as `docker-compose.yml` / `backend/.env.example`.
const DEFAULT_DATABASE_URL: &str =
    "postgres://ai_football_coach:ai_football_coach@127.0.0.1:5432/ai_football_coach";

static MIGRATIONS: OnceLock<Mutex<()>> = OnceLock::new();

fn database_url() -> String {
    // Prefer process env, then backend/.env, then Compose local default.
    let _ = dotenvy::dotenv();
    std::env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_owned())
}

/// Pool against local Compose (or `DATABASE_URL`) — no ephemeral testcontainers.
pub async fn test_pool() -> PgPool {
    let url = database_url();
    let pool = connect_pool(&url).await.unwrap_or_else(|err| {
        panic!(
            "connect to test postgres failed ({err}). Start local DB with \
             `docker compose up -d` from the repo root, or set DATABASE_URL."
        )
    });

    let lock = MIGRATIONS.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().await;
    run_migrations(&pool)
        .await
        .expect("run migrations on test postgres");

    pool
}

/// Router bound to a pool on the current test runtime.
///
/// Not every integration binary uses this helper (e.g. CORS tests build a
/// custom allowlist), so allow unused within those crates.
#[allow(dead_code)]
pub async fn test_app() -> Router {
    app(test_pool().await)
}
