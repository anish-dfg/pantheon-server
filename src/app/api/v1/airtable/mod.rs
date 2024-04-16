mod controllers;

use axum::{http::StatusCode, routing, Router};

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    Router::new()
        .route("/base/:base_id/schema", routing::get(controllers::fetch_schema))
        .route("/bases", routing::get(controllers::list_bases))
        .with_state(state)
}
