mod accounts;
mod routes;
mod state;

use axum::{
    Router,
    http::{HeaderValue, Method},
};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tower_http::cors::{AllowOrigin, CorsLayer};

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

/// Parse comma-separated CORS origins. Empty/unset/invalid input falls back to
/// Vite defaults on port 3000 (`localhost` and `127.0.0.1`).
pub fn parse_cors_origins(raw: Option<&str>) -> Vec<HeaderValue> {
    const DEFAULTS: &[&str] = &["http://localhost:3000", "http://127.0.0.1:3000"];

    let parsed: Vec<HeaderValue> = raw
        .into_iter()
        .flat_map(|value| value.split(','))
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .filter_map(|origin| HeaderValue::from_str(origin).ok())
        .collect();

    if parsed.is_empty() {
        DEFAULTS
            .iter()
            .map(|origin| HeaderValue::from_static(origin))
            .collect()
    } else {
        parsed
    }
}

fn cors_origins_from_env() -> Vec<HeaderValue> {
    parse_cors_origins(std::env::var("CORS_ALLOWED_ORIGINS").ok().as_deref())
}

fn local_dev_cors(origins: Vec<HeaderValue>) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::ACCEPT])
}

/// Build the HTTP router using `CORS_ALLOWED_ORIGINS` when set.
pub fn app(pool: PgPool) -> Router {
    app_with_cors_origins(pool, cors_origins_from_env())
}

/// Build the HTTP router with an explicit CORS allowlist (for tests).
pub fn app_with_cors_origins(pool: PgPool, origins: Vec<HeaderValue>) -> Router {
    let state = AppState { pool };

    routes::router()
        .layer(local_dev_cors(origins))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::parse_cors_origins;
    use axum::http::HeaderValue;

    #[test]
    fn parse_cors_origins_defaults_when_unset() {
        let origins = parse_cors_origins(None);
        assert_eq!(
            origins,
            vec![
                HeaderValue::from_static("http://localhost:3000"),
                HeaderValue::from_static("http://127.0.0.1:3000"),
            ]
        );
    }

    #[test]
    fn parse_cors_origins_defaults_when_blank() {
        let origins = parse_cors_origins(Some("  ,  "));
        assert_eq!(origins.len(), 2);
    }

    #[test]
    fn parse_cors_origins_reads_comma_separated_list() {
        let origins = parse_cors_origins(Some("http://localhost:5173, http://127.0.0.1:5173"));
        assert_eq!(
            origins,
            vec![
                HeaderValue::from_static("http://localhost:5173"),
                HeaderValue::from_static("http://127.0.0.1:5173"),
            ]
        );
    }
}
