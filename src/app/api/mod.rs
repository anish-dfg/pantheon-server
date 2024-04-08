mod v1;

use std::sync::Arc;

use axum::{routing, Router};

use crate::state::AppState;

pub fn routes(state: AppState) -> Router<()> {
    let v1 = v1::routes(state.clone());

    Router::new().with_state(state).nest("/v1", v1)
}
