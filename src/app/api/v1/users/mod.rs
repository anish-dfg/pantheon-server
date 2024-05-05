use axum::{routing, Router};

use crate::state::AppState;

mod controllers;
mod requests;
mod responses;
mod tasks;

pub fn routes(state: AppState) -> Router<()> {
    Router::new()
        .route("/:id/export", routing::post(controllers::export_users_to_workspace))
        .route("/jobs/:id/undo", routing::delete(controllers::undo_export_job))
        .route(
            "/download/:id",
            routing::post(controllers::download_exported_users_as_csv),
        )
        .with_state(state)
}
