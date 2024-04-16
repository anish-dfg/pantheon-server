use anyhow::{bail, Result};
use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    app::errors::AppError,
    services::{
        airtable::{ListRecordsOptions, ListRecordsResponse},
        auth::userdata::UserData,
        storage::{
            entities::{CreateDatasourceView, CreateUser, DatasourceView},
            Cache,
        },
    },
    state::AppState,
};

use super::requests::{CreateDatasourceViewRequest, DatasourceViewRequest};

pub async fn create(
    State(state): State<AppState>,
    Extension(user_info): Extension<UserData>,
    Json(payload): Json<CreateDatasourceViewRequest>,
) -> Result<Response, AppError> {
    let storage = &state.sql;
    let UserData::Auth0(user_info) = user_info;

    let user_id = storage
        .create_or_fetch_user(CreateUser {
            email: user_info.email,
            first_name: user_info.nickname,
            last_name: "".into(),
            image_uri: user_info.picture,
        })
        .await?;

    let _ = storage
        .create_datasource_view(CreateDatasourceView {
            view_name: payload.view_name.clone(),
            datasource_name: payload.datasource_name.clone(),
            metadata: serde_json::to_value(payload.metadata)?,
            description: payload.description.clone(),
            user_id,
        })
        .await?;

    Ok((StatusCode::CREATED).into_response())
}

pub async fn fetch_all(State(state): State<AppState>) -> Result<Response, AppError> {
    let storage = &state.sql;

    let datasource_views = storage.fetch_datasource_views().await?;
    Ok((StatusCode::OK, Json(datasource_views)).into_response())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatasourceViewData {
    Airtable(ListRecordsResponse<Value>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedDatasourceView {
    pub record: DatasourceView,
    pub data: Value,
}

pub async fn fetch_datasource_view_data(
    State(state): State<AppState>,
    Path((datasource, id)): Path<(String, String)>,
    Json(payload): Json<DatasourceViewRequest>,
) -> Result<Response, AppError> {
    dbg!(&payload);
    let sql = &state.sql;
    let cache = &state.cache;
    let airtable = &state.airtable;
    let Some(datasource_view) = sql.fetch_datasource_view_by_id(&id).await? else {
        return Ok((StatusCode::BAD_REQUEST, "requested datasource view does not exist").into_response());
    };

    if datasource.as_str() != "airtable" {
        return Ok((StatusCode::BAD_REQUEST, "unimplemented datasource").into_response());
    };

    let id = datasource_view.id.clone().to_string();
    if let Some(cached) = cache.get_json::<CachedDatasourceView>(&id).await? {
        log::info!("returning cached");
        return Ok((
            StatusCode::OK,
            Json(serde_json::json!({"records": cached.data, "cached": true})),
        )
            .into_response());
    };

    // if let Some(cached) = cached {
    //     if let Some(fields) = cached.record.metadata["fields"]
    //         .as_array()
    //         .and_then(|fields| fields.iter().map(|f| f.as_str()).collect::<Vec<&str>>())
    //     {};
    //     return Ok((StatusCode::OK, Json(cached.data)).into_response());
    // };

    let (Some(base), Some(table), Some(view)) = (
        datasource_view.metadata["base"].as_str(),
        datasource_view.metadata["table"].as_str(),
        datasource_view.metadata["view"].as_str(),
    ) else {
        return Ok((StatusCode::BAD_REQUEST, "missing metadata").into_response());
    };

    match payload {
        DatasourceViewRequest::Airtable { offset } => {
            let records_response = airtable
                .list_all_records::<Value>(
                    base,
                    table,
                    &mut ListRecordsOptions {
                        fields: None,
                        view: view.to_owned().into(),
                        offset,
                    },
                )
                .await?;

            cache
                .set_json(
                    &id,
                    CachedDatasourceView {
                        record: datasource_view.clone(),
                        data: serde_json::to_value(&records_response)?,
                    },
                )
                .await?;

            Ok((
                StatusCode::OK,
                Json(DatasourceViewData::Airtable(ListRecordsResponse {
                    records: records_response,
                    offset: None,
                })),
            )
                .into_response())
        }
        DatasourceViewRequest::Google => Ok((StatusCode::BAD_REQUEST).into_response()),
    }
}
