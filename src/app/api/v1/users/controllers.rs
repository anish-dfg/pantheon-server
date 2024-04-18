use std::thread;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use rand::{distributions::Alphanumeric, Rng};
use std::time::Duration;
use tokio::{runtime, task};

use crate::{
    app::errors::AppError,
    services::{
        auth::userdata::UserData,
        storage::entities::{CreateJob, CreateUser, MarkJobErrored},
        workspace::users::{CreateWorkspaceUser, Name},
    },
    state::AppState,
};

use super::requests::{ExportUser, ExportUsersRequest};

pub async fn export_users_to_workspace(
    State(state): State<AppState>,
    Extension(user_info): Extension<UserData>,
    Json(data): Json<ExportUsersRequest>,
) -> Result<Response, AppError> {
    let sql = &state.sql;
    let UserData::Auth0(user_info) = user_info;

    let user_id = sql
        .create_or_fetch_user(CreateUser {
            email: user_info.email.clone(),
            first_name: user_info.nickname,
            last_name: "".into(),
            image_uri: user_info.picture,
        })
        .await?;

    let s = state.clone();

    let create_job_data = CreateJob {
        user_id,
        description: "export_users_to_workspace".to_owned(),
    };

    let job_id = sql.create_job(create_job_data).await?;

    dbg!(&data);

    thread::spawn(move || {
        if let Ok(rt) = runtime::Builder::new_current_thread().enable_all().build() {
            rt.block_on(async move {
                thread::sleep(Duration::from_secs(5));
                let sql = &s.sql;
                let workspace = &s.workspace_client;
                let admin_email = &user_info.email;

                let num = rand::thread_rng().gen_range(0..100);

                for (i, user) in data.users.iter().enumerate() {
                    if i % 8 == 0 {
                        thread::sleep(Duration::from_secs(3));
                    };
                    thread::sleep(Duration::from_secs(10));

                    let new_email = format!(
                        "{}{}{}{}@developforgood.org",
                        user.first_name.trim().to_lowercase(),
                        data.email_policy.separator,
                        user.last_name.trim().to_lowercase(),
                        num
                    );

                    let password = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(8)
                        .map(char::from)
                        .collect();

                    let workspace_user_data = CreateWorkspaceUser {
                        primary_email: new_email,
                        name: Name {
                            given_name: user.first_name.trim().to_owned(),
                            family_name: user.last_name.trim().to_owned(),
                            full_name: None,
                        },
                        password,
                        change_password_at_next_login: data.password_policy.change_password_at_next_login,
                    };

                    dbg!(&workspace_user_data);
                    match workspace.create_user(admin_email, workspace_user_data.clone()).await {
                        Ok(_) => {
                            log::info!("successfully created new user");
                            continue;
                        }
                        Err(e) => {
                            let Ok(_) = sql
                                .mark_job_errored(MarkJobErrored {
                                    job_id: job_id.clone(),
                                    error: serde_json::json!({
                                        "error": e.to_string(),
                                        "job_description": "export_users_to_workspace",
                                        "failed_upload": workspace_user_data
                                    }),
                                })
                                .await
                            else {
                                return;
                            };
                        }
                    };
                }
                let Ok(_) = sql.mark_job_complete(&job_id).await else {
                    return;
                };
            })
        };
    });

    Ok((StatusCode::OK, "started job").into_response())
}
