use ai_football_coach_backend::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind address");

    axum::serve(listener, app())
        .await
        .expect("server error");
}
