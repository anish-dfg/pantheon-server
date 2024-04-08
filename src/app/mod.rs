mod api;
mod controllers;

use axum::{routing, Router};

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    let api = api::routes(state.clone());
    Router::new()
        .route("/health", routing::get(controllers::health_check))
        .with_state(state)
        .nest("/api", api)
}
