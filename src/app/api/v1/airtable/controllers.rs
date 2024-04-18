use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};

use crate::{
    app::errors::AppError,
    services::airtable::{bases::Bases, schema::Schema},
    state::AppState,
};

pub async fn list_bases(State(state): State<AppState>) -> Result<Response, AppError> {
    let airtable = &state.airtable;
    let cache = &state.cache;
    if let Some(bases) = cache.get_json::<Bases>("accessible_bases").await? {
        log::info!("FETCHING CACHED");
        Ok(bases.into_response())
    } else {
        let bases = airtable.list_bases().await?;
        cache.set_json("accessible-bases", bases.clone()).await?;
        Ok(bases.into_response())
    }
}

pub async fn fetch_schema(State(state): State<AppState>, Path(base_id): Path<String>) -> Result<Response, AppError> {
    let airtable = &state.airtable;
    let cache = &state.cache;
    if let Some(schema) = cache.get_json::<Schema>(&format!("{base_id}-schema")).await? {
        log::info!("FETCHING CACHED");
        Ok(schema.into_response())
    } else {
        let schema = airtable.fetch_schema(&base_id).await?;
        cache.set_json(&format!("{base_id}-schema"), schema.clone()).await?;
        Ok(schema.into_response())
    }
}
