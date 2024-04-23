mod controllers;
mod helpers;
mod requests;
mod responses;

use axum::{routing, Router};

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    Router::new()
        .route("/", routing::get(controllers::fetch_all))
        .route("/airtable", routing::post(controllers::create_airtable))
        .route("/:id/jobs", routing::get(controllers::list_datasource_jobs))
        .route("/airtable/:id", routing::post(controllers::fetch_airtable_data))
        .route(
            "/airtable/:id/refresh",
            routing::post(controllers::refresh_airtable_data),
        )
        .with_state(state)
}
