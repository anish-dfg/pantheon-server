mod controllers;

use axum::{routing, Router};

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    Router::new()
        .route("/", routing::get(controllers::list_jobs))
        .with_state(state)
}
