use ai_football_coach_backend::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app()).await?;
    Ok(())
}
