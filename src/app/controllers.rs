use axum::{http::StatusCode, response::IntoResponse};

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "not found")
}
