use axum::{http::StatusCode, routing, Router};

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    Router::new()
        .route("/users", routing::get(|| async { StatusCode::OK }))
        .route("/users", routing::post(|| async { StatusCode::OK }))
        .with_state(state)
}
