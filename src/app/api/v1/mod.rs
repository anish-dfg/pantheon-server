mod airtable;
mod datasource;
mod gsuite;
mod jobs;
mod users;

use axum::Router;

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    let workspace_routes = gsuite::routes(state.clone());
    let airtable_routes = airtable::routes(state.clone());
    let datasource_routes = datasource::routes(state.clone());
    let user_routes = users::routes(state.clone());
    let job_routes = jobs::routes(state.clone());

    Router::new()
        .with_state(state)
        .nest("/workspace", workspace_routes)
        .nest("/airtable", airtable_routes)
        .nest("/datasource", datasource_routes)
        .nest("/users", user_routes)
        .nest("/jobs", job_routes)
}
