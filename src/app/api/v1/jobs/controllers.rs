use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{app::errors::AppError, state::AppState};

pub async fn list_jobs(State(state): State<AppState>) -> Result<Response, AppError> {
    let sql = &state.sql;
    let jobs = sql.fetch_jobs().await?;
    Ok((StatusCode::OK, Json(jobs)).into_response())
}
