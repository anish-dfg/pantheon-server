use axum::{routing, Router};

use crate::state::AppState;

mod controllers;
mod requests;
mod responses;
mod tasks;

pub fn routes(state: AppState) -> Router<()> {
    Router::new()
        .route("/:id/export", routing::post(controllers::export_users_to_workspace))
        .with_state(state)
}
