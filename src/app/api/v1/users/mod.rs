use axum::{routing, Router};

use crate::state::AppState;

mod controllers;
mod requests;
mod responses;

pub fn routes(state: AppState) -> Router<()> {
    Router::new()
        .route("/export", routing::post(controllers::export_users_to_workspace))
        .with_state(state)
}
