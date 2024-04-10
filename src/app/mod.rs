mod api;
mod controllers;
mod errors;
mod middleware;

use axum::{routing, Router};

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    let api = api::routes(state.clone());
    Router::new()
        .fallback(controllers::fallback)
        .with_state(state.clone())
        .nest("/api", api)
        .route("/health", routing::get(controllers::health_check))
        .route_layer(axum::middleware::from_fn_with_state(
            state,
            middleware::auth,
        ))
}
