use axum::{http::StatusCode, routing, Router};

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    Router::new()
        .route(
            "/volunteers/base/:base/table/:table/view/:view",
            routing::get(|| async { StatusCode::OK }),
        )
        .route("/bases", routing::get(|| async { StatusCode::OK }))
        .route(
            "/base/:base/schema",
            routing::get(|| async { StatusCode::OK }),
        )
        .with_state(state)
}
