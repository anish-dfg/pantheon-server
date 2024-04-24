use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use tokio::task;
use uuid::Uuid;

use crate::{
    app::errors::AppError,
    services::{
        auth::userdata::UserData,
        storage::{
            dto::{CreateJobWithDatasourceBuilder, CreateUserBuilder},
            entities::Job,
            types::{JobStatus, JobType},
        },
    },
    state::AppState,
};

use super::{
    requests::{ExportConflictPolicy, ExportUser, ExportUsersRequest},
    tasks,
};

pub async fn export_users_to_workspace(
    State(state): State<AppState>,
    Extension(user_info): Extension<UserData>,
    Path(id): Path<String>,
    Json(export_data): Json<ExportUsersRequest>,
) -> Result<Response, AppError> {
    let UserData::Auth0(user_info) = user_info;
    let db = &state.storage.db;

    let view_uuid = Uuid::parse_str(&id)?;

    let current_view_jobs = db.fetch_datasource_view_jobs(view_uuid).await?;

    // only one job can be started at a time
    if !current_view_jobs
        .iter()
        .filter(|j| j.status == JobStatus::Pending)
        .collect::<Vec<&Job>>()
        .is_empty()
    {
        return Ok((StatusCode::BAD_REQUEST).into_response());
    };

    let already_exported_users = db
        .fetch_exported_users()
        .await?
        .iter()
        .map(|u| ExportUser {
            first_name: u.first_name.to_owned(),
            last_name: u.last_name.to_owned(),
            email: u.email.to_owned(),
        })
        .collect::<Vec<ExportUser>>();

    let users_to_export = export_data
        .users
        .iter()
        .filter(|email| !already_exported_users.contains(email))
        .cloned()
        .collect::<Vec<ExportUser>>();

    if let ExportConflictPolicy::Reject = export_data.export_conflict_policy {
        if users_to_export.len() != export_data.users.len() {
            return Ok((StatusCode::BAD_REQUEST).into_response());
        }
    };

    let Some(data) = db.fetch_datasource_view(Uuid::parse_str(&id)?).await? else {
        return Ok((StatusCode::NOT_FOUND).into_response());
    };

    let dto = CreateUserBuilder::default()
        .email(user_info.email.clone())
        .first_name(user_info.nickname)
        .last_name("")
        .image_uri(user_info.picture)
        .build()?;

    let user_id = db.create_or_fetch_user(dto).await?;

    let dto = CreateJobWithDatasourceBuilder::default()
        .status(JobStatus::Pending)
        .job_type(JobType::ExportData)
        .user_id(Uuid::parse_str(&user_id)?)
        .metadata(serde_json::json!({"datasource_view_id": &data.id}))
        .datasource_view_id(data.id)
        .build()?;

    let (job_id, _) = db.create_job_with_datasource(dto).await?;
    let job_uuid = Uuid::parse_str(&job_id)?;

    let state = state.clone();

    task::spawn(async move {
        let _ = tasks::create_workspace_users(
            state,
            users_to_export,
            export_data.email_policy,
            export_data.password_policy,
            user_info.email,
            job_uuid,
        )
        .await;
    });

    Ok((StatusCode::OK, "started job").into_response())
}
