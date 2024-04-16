mod api;
mod controllers;
mod errors;
mod middleware;

use axum::{routing, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

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
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
