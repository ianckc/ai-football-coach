use ai_football_coach_backend::{app, connect_pool, run_migrations};
use axum::Router;
use sqlx::PgPool;
use std::sync::OnceLock;
use tokio::sync::Mutex;

/// Dedicated test Compose service (`postgres-test` on host port 5433).
/// Not the manual-dev `postgres` on 5432 / `backend/.env` `DATABASE_URL`.
const DEFAULT_TEST_DATABASE_URL: &str =
    "postgres://ai_football_coach:ai_football_coach@127.0.0.1:5433/ai_football_coach";

static MIGRATIONS: OnceLock<Mutex<()>> = OnceLock::new();

fn database_url() -> String {
    // Prefer explicit TEST_DATABASE_URL (CI / quality-tests). Do not load
    // DATABASE_URL from backend/.env — that targets manual-dev Postgres.
    std::env::var("TEST_DATABASE_URL")
        .ok()
        .filter(|url| !url.is_empty())
        .unwrap_or_else(|| DEFAULT_TEST_DATABASE_URL.to_owned())
}

/// Pool against test Compose (or `TEST_DATABASE_URL`) — no ephemeral testcontainers.
pub async fn test_pool() -> PgPool {
    let url = database_url();
    let pool = connect_pool(&url).await.unwrap_or_else(|err| {
        panic!(
            "connect to test postgres failed ({err}). Start the test DB with \
             `docker compose up -d postgres-test` from the repo root, or set \
             TEST_DATABASE_URL."
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
