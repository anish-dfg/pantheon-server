mod controllers;
mod helpers;
mod requests;
mod responses;

use axum::{http::StatusCode, routing, Router};

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    Router::new()
        .route("/", routing::get(controllers::fetch_all))
        .route("/", routing::post(controllers::create))
        .route(
            "/:datasource/:id",
            routing::post(controllers::fetch_datasource_view_data),
        )
        .route("/airtable", routing::post(controllers::create_airtable))
        .with_state(state)
}
