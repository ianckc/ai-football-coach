use axum::http::StatusCode;

/// Stub `POST /login` handler — route exists; credential handling comes in later slices.
pub async fn login() -> StatusCode {
    StatusCode::BAD_REQUEST
}
