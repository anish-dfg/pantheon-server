use anyhow::Result;
use std::time::Duration;

use rand::{distributions::Alphanumeric, Rng};
use uuid::Uuid;

use crate::{
    services::{
        storage::{
            dto::{CreateExportedUser, CreateExportedUserBuilder},
            types::SupportedDatasource,
        },
        workspace::users::{CreateWorkspaceUserBuilder, NameBuilder},
    },
    state::AppState,
};

use super::requests::{EmailPolicy, ExportUser, ExportUsersRequest, PasswordPolicy};

pub async fn create_workspace_users(
    state: AppState,
    users_to_export: Vec<ExportUser>,
    email_policy: EmailPolicy,
    password_policy: PasswordPolicy,
    admin_email: String,
    job_uuid: Uuid,
) -> Result<()> {
    let (db, workspace) = (&state.storage.db, &state.workspace_client);

    let num = rand::thread_rng().gen_range(10..100);

    let mut created_users: Vec<&ExportUser> = vec![];

    for (i, user) in users_to_export.iter().enumerate() {
        if i % 8 == 0 {
            tokio::time::sleep(Duration::from_secs(3)).await;
        };

        let new_email = format!(
            "{}{}{}{}@developforgood.org",
            user.first_name.trim().to_lowercase(),
            email_policy.separator,
            user.last_name.trim().to_lowercase(),
            num
        );

        let password = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(password_policy.generated_password_length as usize)
            .map(char::from)
            .collect::<String>();

        let workspace_user_data = CreateWorkspaceUserBuilder::default()
            .name(
                NameBuilder::default()
                    .family_name(user.last_name.trim().to_owned())
                    .given_name(user.first_name.trim().to_owned())
                    .full_name(None)
                    .build()?,
            )
            .primary_email(new_email)
            .password(password)
            .change_password_at_next_login(true)
            .build()?;

        match workspace.create_user(&admin_email, workspace_user_data.clone()).await {
            Ok(_) => {
                log::info!("successfully created new user");
                created_users.push(user);
                continue;
            }
            Err(_) => {
                db.mark_job_errored(job_uuid).await?;
                break;
            }
        };
    }

    let users_to_export = created_users
        .iter()
        .filter_map(|u| {
            let Ok(e) = CreateExportedUserBuilder::default()
                .first_name(u.first_name.to_owned())
                .last_name(u.last_name.to_owned())
                .email(u.email.to_owned())
                .exported_from(SupportedDatasource::Airtable)
                .job_id(job_uuid)
                .build()
            else {
                return None;
            };
            Some(e)
        })
        .collect::<Vec<CreateExportedUser>>();

    db.save_exported_users(users_to_export).await?;
    db.mark_job_complete(job_uuid).await?;

    Ok(())
}
