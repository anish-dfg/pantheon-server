use std::env;
use std::fs::File;

use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use sendgrid::Mail;
use serde::{Deserialize, Serialize};
// use tokio::fs::File;
use tokio::task;
use uuid::Uuid;

use crate::{
    app::errors::AppError,
    services::{
        auth::userdata::UserData,
        storage::{
            dto::{CreateJobWithDatasourceBuilder, CreateUserBuilder},
            entities::{ExportedUser, Job},
            types::{JobStatus, JobType},
        },
    },
    state::AppState,
};

use super::{
    requests::{DownloadUsersRequest, ExportConflictPolicy, ExportUser, ExportUsersRequest},
    tasks,
};

pub async fn undo_export_job(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user_info): Extension<UserData>,
) -> Result<Response, AppError> {
    let db = &state.storage.db;
    let UserData::Auth0(user_info) = user_info;

    let users_to_delete = db
        .fetch_exported_users_by_job(Uuid::parse_str(&id)?)
        .await?
        .into_iter()
        .map(|u| u.generated_email)
        .collect::<Vec<String>>();

    let job_uuid = Uuid::parse_str(&id)?;
    let admin_email = user_info.email;

    let state = state.clone();
    task::spawn(async move {
        let _ = tasks::delete_workspace_users(state, users_to_delete, admin_email, job_uuid)
            .await
            .unwrap();
    });

    Ok((StatusCode::OK).into_response())
}

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
            email: u.personal_email.to_owned(),
            generated_email: Some(u.generated_email.to_owned()),
        })
        .collect::<Vec<ExportUser>>();

    let users_to_export = export_data
        .users
        .iter()
        .filter(|user| !already_exported_users.contains(user))
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

#[derive(Debug, Serialize, Deserialize)]
struct ExportedUserCsvRecord {
    first_name: String,
    last_name: String,
    project_name: String,
    email: String,
    dfg_email: String,
}

pub async fn download_exported_users_as_csv(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user_info): Extension<UserData>,
    Json(payload): Json<DownloadUsersRequest>,
) -> Result<Response, AppError> {
    let (db, mail) = (&state.storage.db, &state.mail);

    log::info!("WE ARE HERE");
    dbg!(&payload);

    let UserData::Auth0(user_info) = user_info;
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
        .metadata(serde_json::json!({}))
        .datasource_view_id(Uuid::parse_str(&id)?)
        .build()?;

    let (job_id, _) = db.create_job_with_datasource(dto).await?;
    // let job_uuid = Uuid::parse_str(&job_id)?;
    //
    // let state = state.clone();
    //

    let all_exported_users = db.fetch_exported_users_by_view(Uuid::parse_str(&id)?).await?;

    let csv_users = payload
        .user_data
        .into_iter()
        .filter(|u| u.email.is_some())
        .filter_map(|u| {
            let email = u.email.unwrap();
            return match all_exported_users
                .iter()
                .find(|u| u.personal_email == email)
                .map(|u| u.generated_email.clone())
            {
                Some(dfg_email) => Some(ExportedUserCsvRecord {
                    first_name: u.first_name.clone(),
                    last_name: u.last_name.clone(),
                    project_name: u.project.clone().unwrap_or_default(),
                    email,
                    dfg_email,
                }),
                None => None,
            };
        })
        .collect::<Vec<ExportedUserCsvRecord>>();

    dbg!(&csv_users);

    let file_id = Uuid::new_v4().to_string();
    let file = File::create(format!("export-{file_id}.csv"))?;

    let mut writer = csv::Writer::from_writer(file);

    // Write header row
    // writer.write_record(["first_name", "last_name", "project_name", "email", "dfg_email"])?;

    log::info!("WE ARE HERE");
    // Write data rows
    for record in csv_users {
        writer.serialize(record)?;
    }

    tokio::task::spawn_blocking(move || {
        writer.flush().unwrap();
    })
    .await?;

    let dir = env::current_dir()?.to_string_lossy().to_string();

    let path = format!("{dir}/export-{file_id}.csv");
    dbg!(&path);

    let m = Mail::new()
        .add_attachment(path)?
        .add_from("pantheon@developforgood.org")
        .add_to((payload.send_to.as_str(), payload.send_to.as_str()).into())
        .add_subject("Export User Data")
        .add_text("The requested data is attached below as a CSV.");
    //
    //
    dbg!(&m);

    log::info!("HERE");
    let x = mail.send(m).await.context("FUCK THIS")?;
    dbg!(&x.text().await?);
    // .into_iter()
    // .map(|u| u.into())
    // .collect::<Vec<ExportedUserCsvRecord>>();

    // let jobs = db
    //     .fetch_datasource_view_jobs(Uuid::parse_str(&id)?)
    //     .await?
    //     .into_iter()
    //     .filter(|job| job.status == JobStatus::Complete && job.job_type == JobType::ExportData)
    //     .collect::<Vec<Job>>();
    //
    // dbg!(&jobs);

    // task::spawn(async move {
    //     let _ = tasks::create_and_send_csv(state, payload.send_to, payload.user_data, payload.columns, job_uuid).await;
    // });

    Ok((StatusCode::OK).into_response())
}
