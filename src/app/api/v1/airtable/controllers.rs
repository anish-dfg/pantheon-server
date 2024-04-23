use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    app::{
        api::api_response::{ApiResponseBuilder, ApiResponseData},
        errors::AppError,
    },
    services::airtable::{bases::Bases, schema::Schema},
    state::AppState,
};

pub async fn list_bases(State(state): State<AppState>) -> Result<Response, AppError> {
    let airtable = &state.airtable;
    let cache = &state.storage.cache;

    let bases = match cache.get_json::<Bases>("accessible-bases").await? {
        Some(bases) => bases,
        None => {
            let bases = airtable.list_bases().await?;
            cache.set_json("accessible-bases", bases.clone()).await?;
            bases
        }
    };
    let res = ApiResponseBuilder::default()
        .status_code(StatusCode::OK)
        .data(ApiResponseData::Data(bases))
        .build()?
        .into_response();
    Ok(res)
}

pub async fn fetch_schema(State(state): State<AppState>, Path(base_id): Path<String>) -> Result<Response, AppError> {
    let airtable = &state.airtable;
    let cache = &state.storage.cache;
    let schema = match cache.get_json::<Schema>(&format!("{base_id}-schema")).await? {
        Some(schema) => schema,
        None => {
            let schema = airtable.fetch_schema(&base_id).await?;
            cache.set_json(&format!("{base_id}-schema"), schema.clone()).await?;
            schema
        }
    };
    let res = ApiResponseBuilder::default()
        .status_code(StatusCode::OK)
        .data(ApiResponseData::Data(schema))
        .build()?
        .into_response();
    Ok(res)
}
