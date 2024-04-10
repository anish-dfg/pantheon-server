use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;

pub async fn list_records(State(state): State<AppState>) -> impl IntoResponse {
    ""
}
