use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};

use crate::{app::errors::AppError, services::airtable::bases::Bases, state::AppState};

pub async fn list_bases(State(state): State<AppState>) -> Result<Response, AppError> {
    let airtable = &state.airtable;
    let bases = airtable.list_bases().await?;
    Ok(bases.into_response())
}

pub async fn fetch_schema(State(state): State<AppState>, Path(base_id): Path<String>) -> Result<Response, AppError> {
    let airtable = &state.airtable;
    let schema = airtable.fetch_schema(&base_id).await?;
    Ok(schema.into_response())
}
