use ai_football_coach_backend::{app, connect_pool, run_migrations};
use axum::Router;
use sqlx::PgPool;
use std::sync::OnceLock;
use testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner};
use testcontainers_modules::postgres::Postgres;
use tokio::runtime::Runtime;

/// Long-lived runtime so the Postgres container outlives individual `#[tokio::test]` runtimes.
static SHARED_RT: OnceLock<Runtime> = OnceLock::new();
static DATABASE_URL: OnceLock<String> = OnceLock::new();
// Keep the container alive for the process; dropping it stops Postgres.
static CONTAINER: OnceLock<ContainerAsync<Postgres>> = OnceLock::new();

fn shared_runtime() -> &'static Runtime {
    SHARED_RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("build shared test runtime")
    })
}

fn database_url() -> &'static str {
    DATABASE_URL.get_or_init(|| {
        // `block_on` must not run on a thread already inside a tokio test runtime.
        std::thread::spawn(|| {
            shared_runtime().block_on(async {
                // Match local Compose major (postgres:18); module default is still 11-alpine.
                let container = Postgres::default()
                    .with_tag("18-alpine")
                    .start()
                    .await
                    .expect("start postgres testcontainer");
                let host = container.get_host().await.expect("postgres host");
                let port = container
                    .get_host_port_ipv4(5432)
                    .await
                    .expect("postgres port");
                let url = format!("postgres://postgres:postgres@{host}:{port}/postgres");
                let pool = connect_pool(&url).await.expect("connect to test postgres");
                run_migrations(&pool)
                    .await
                    .expect("run migrations on test postgres");
                // Disconnect bootstrap pool; tests open their own pools on their runtime.
                pool.close().await;
                CONTAINER
                    .set(container)
                    .unwrap_or_else(|_| panic!("testcontainer already set"));
                url
            })
        })
        .join()
        .expect("shared postgres init thread")
    })
}

/// Pool on the current test tokio runtime, against the shared hermetic Postgres.
pub async fn test_pool() -> PgPool {
    let _ = database_url();
    connect_pool(database_url())
        .await
        .expect("connect pool for test")
}

/// Router bound to a pool on the current test runtime.
pub async fn test_app() -> Router {
    app(test_pool().await)
}
