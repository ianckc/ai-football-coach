use ai_football_coach_backend::{app, connect_pool, run_migrations};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Loads backend/.env when present; existing process env wins.
    let _ = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL").map_err(|_| {
        "DATABASE_URL is required (copy backend/.env.example to backend/.env, or export DATABASE_URL)"
    })?;

    let pool = connect_pool(&database_url).await?;
    run_migrations(&pool).await?;

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app(pool)).await?;
    Ok(())
}
