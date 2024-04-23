use anyhow::{bail, Result};
use axum::{
    extract::{Path, Query, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio::task;
use uuid::Uuid;

use crate::{
    app::{
        api::{
            api_response::{ApiResponseBuilder, ApiResponseData},
            v1::datasource::{
                requests::AirtableDatasourceViewRequestMetadata,
                responses::{AirtableViewData, AirtableViewDataBuilder, DatasourceViewResponse},
            },
        },
        errors::AppError,
        jobs,
    },
    services::{
        airtable::ListRecordsOptionsBuilder,
        auth::userdata::UserData,
        storage::{
            dto::{CreateDatasourceViewBuilder, CreateJobBuilder, CreateJobWithDatasourceBuilder, CreateUserBuilder},
            entities::DatasourceView,
            types::{JobStatus, JobType, SupportedDatasource},
        },
    },
    state::AppState,
};

use super::requests::{CreateDatasourceViewRequest, DatasourceViewRequest};

pub async fn create_airtable(
    State(state): State<AppState>,
    Extension(user_info): Extension<UserData>,
    Json(payload): Json<CreateDatasourceViewRequest>,
) -> Result<Response, AppError> {
    let db = &state.storage.db;
    let tasks = &state.tasks;
    let UserData::Auth0(user_info) = user_info;

    let dto = CreateUserBuilder::default()
        .email(user_info.email)
        .first_name(user_info.nickname)
        .last_name("")
        .image_uri(user_info.picture)
        .build()?;

    let user_id = db.create_or_fetch_user(dto).await?;

    let (base, table, view, fields) = (
        payload.metadata.base.clone(),
        payload.metadata.table.clone(),
        payload.metadata.view.clone(),
        payload.metadata.fields.clone(),
    );

    let dto = CreateDatasourceViewBuilder::default()
        .view_name(payload.name)
        .datasource(SupportedDatasource::Airtable)
        .metadata(serde_json::to_value(payload.metadata)?)
        .description(payload.description)
        .user_id(Uuid::parse_str(&user_id)?)
        .build()?;

    let new_datasource_view_id = db.create_datasource_view(dto).await?;

    let dto = CreateJobWithDatasourceBuilder::default()
        .status(JobStatus::Pending)
        .job_type(JobType::ImportData)
        .user_id(Uuid::parse_str(&user_id)?)
        .metadata(serde_json::json!({"datasource_view_id": new_datasource_view_id}))
        .datasource_view_id(Uuid::parse_str(&new_datasource_view_id)?)
        .build()?;

    let (job_id, _) = db.create_job_with_datasource(dto).await.unwrap();
    let job_uuid = Uuid::parse_str(&job_id)?;

    let state = state.clone();

    let handle = task::spawn(async move {
        let db = &state.clone().storage.db;
        let _ = match jobs::fetch_and_cache_airtable_data(
            state,
            job_uuid,
            new_datasource_view_id,
            base,
            table,
            view,
            fields,
            None,
        )
        .await
        {
            Ok(_) => db.mark_job_complete(job_uuid).await,
            Err(_) => db.mark_job_errored(job_uuid).await,
        };
    });

    let mut guard = tasks.lock().await;

    guard.insert(job_id, Some(handle));

    Ok((StatusCode::CREATED).into_response())
}

pub async fn fetch_all(State(state): State<AppState>) -> Result<Response, AppError> {
    let db = &state.storage.db;

    let datasource_views = db.fetch_datasource_views().await?;
    Ok((StatusCode::OK, Json(datasource_views)).into_response())
}

pub async fn fetch_airtable_data(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user_info): Extension<UserData>,
) -> Result<Response, AppError> {
    let (db, cache) = (&state.storage.db, &state.storage.cache);

    let Some(data) = db.fetch_datasource_view(Uuid::parse_str(&id)?).await? else {
        return Ok((StatusCode::NOT_FOUND).into_response());
    };

    let records = match cache.get_json::<Value>(&data.id.to_string()).await? {
        Some(Value::Array(records)) => records,
        Some(_) => {
            // the fuck happened here
            cache.evict(&data.id.to_string()).await?;
            return Ok((StatusCode::INTERNAL_SERVER_ERROR).into_response());
        }
        None => {
            let state = state.clone();

            let metadata = serde_json::from_value::<AirtableDatasourceViewRequestMetadata>(data.metadata.clone())?;
            let UserData::Auth0(user_info) = user_info;

            let dto = CreateUserBuilder::default()
                .email(user_info.email)
                .first_name(user_info.nickname)
                .last_name("")
                .image_uri(user_info.picture)
                .build()?;

            let user_id = db.create_or_fetch_user(dto).await?;

            let dto = CreateJobWithDatasourceBuilder::default()
                .status(JobStatus::Pending)
                .job_type(JobType::ImportData)
                .user_id(Uuid::parse_str(&user_id)?)
                .metadata(serde_json::json!({"datasource_view_id": &data.id}))
                .datasource_view_id(data.id)
                .build()?;

            let (job_id, _) = db.create_job_with_datasource(dto).await?;
            let job_uuid = Uuid::parse_str(&job_id)?;

            // this task is not cancellable so there is not `let handle = task::spawn(async move
            // {...})`
            task::spawn(async move {
                let db = &state.clone().storage.db;
                let _ = match jobs::fetch_and_cache_airtable_data(
                    state,
                    job_uuid,
                    data.id.to_string(),
                    metadata.base,
                    metadata.table,
                    metadata.view,
                    metadata.fields,
                    None,
                )
                .await
                {
                    Ok(_) => db.mark_job_complete(job_uuid).await,
                    Err(_) => db.mark_job_errored(job_uuid).await,
                };
            });
            vec![]
        }
    };

    let airtable_view_data = AirtableViewDataBuilder::default().data(data).records(records).build()?;
    let res = ApiResponseBuilder::default()
        .status_code(StatusCode::OK)
        .data(ApiResponseData::Data(airtable_view_data))
        .build()?
        .into_response();

    Ok(res)
}

pub async fn refresh_airtable_data(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user_info): Extension<UserData>,
) -> Result<Response, AppError> {
    log::info!("HERE");
    let db = &state.storage.db;

    let Some(data) = db.fetch_datasource_view(Uuid::parse_str(&id)?).await? else {
        return Ok((StatusCode::NOT_FOUND).into_response());
    };

    let UserData::Auth0(user_info) = user_info;

    let dto = CreateUserBuilder::default()
        .email(user_info.email)
        .first_name(user_info.nickname)
        .last_name("")
        .image_uri(user_info.picture)
        .build()?;

    let user_id = db.create_or_fetch_user(dto).await?;

    let dto = CreateJobWithDatasourceBuilder::default()
        .status(JobStatus::Pending)
        .job_type(JobType::ImportData)
        .user_id(Uuid::parse_str(&user_id)?)
        .metadata(serde_json::json!({"datasource_view_id": &data.id}))
        .datasource_view_id(data.id)
        .build()?;
    let metadata = serde_json::from_value::<AirtableDatasourceViewRequestMetadata>(data.metadata.clone())?;

    let (job_id, _) = db.create_job_with_datasource(dto).await?;
    let job_uuid = Uuid::parse_str(&job_id)?;

    task::spawn(async move {
        let db = &state.clone().storage.db;
        let _ = match jobs::fetch_and_cache_airtable_data(
            state,
            job_uuid,
            data.id.to_string(),
            metadata.base,
            metadata.table,
            metadata.view,
            metadata.fields,
            None,
        )
        .await
        {
            Ok(_) => db.mark_job_complete(job_uuid).await,
            Err(_) => db.mark_job_errored(job_uuid).await,
        };
    });

    Ok((StatusCode::OK).into_response())
}

pub async fn list_datasource_jobs(State(state): State<AppState>, Path(id): Path<String>) -> Result<Response, AppError> {
    let db = &state.storage.db;
    let jobs = db.fetch_datasource_view_jobs(Uuid::parse_str(&id)?).await?;

    let res = ApiResponseBuilder::default()
        .data(ApiResponseData::Data(jobs))
        .status_code(StatusCode::OK)
        .build()?
        .into_response();
    Ok(res)
}
