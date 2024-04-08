mod airtable;
mod gsuite;

use axum::Router;

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    let workspace_routes = gsuite::routes(state.clone());
    let airtable_routes = airtable::routes(state.clone());
    Router::new()
        .with_state(state)
        .nest("/workspace", workspace_routes)
        .nest("/airtable", airtable_routes)
}