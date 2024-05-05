use anyhow::Result;
use sendgrid::Mail;
use serde_json::Value;
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

pub async fn create_and_send_csv(
    state: AppState,
    send_to: String,
    user_data: Vec<Value>,
    columns: Vec<String>,
    job_uuid: Uuid,
) -> Result<()> {
    Ok(())
}

pub async fn delete_workspace_users(
    state: AppState,
    users_to_delete: Vec<String>,
    admin_email: String,
    job_uuid: Uuid,
) -> Result<()> {
    let (db, workspace) = (&state.storage.db, &state.workspace_client);

    for user in users_to_delete {
        match workspace.delete_user(&admin_email, &user).await {
            Ok(_) => {
                log::info!("successfully deleted user");
                let _ = db.delete_exported_user_by_email(&user).await;
            }
            Err(_) => {
                let _ = db.mark_job_errored(job_uuid).await;
                break;
            }
        };
    }

    let _ = db.mark_job_complete(job_uuid).await;

    Ok(())
}

pub async fn create_workspace_users(
    state: AppState,
    users_to_export: Vec<ExportUser>,
    email_policy: EmailPolicy,
    password_policy: PasswordPolicy,
    admin_email: String,
    job_uuid: Uuid,
) -> Result<()> {
    let (db, workspace, mail) = (&state.storage.db, &state.workspace_client, &state.mail);

    let mut created_users: Vec<ExportUser> = vec![];

    for (i, mut user) in users_to_export.into_iter().enumerate() {
        if i % 8 == 0 {
            tokio::time::sleep(Duration::from_secs(3)).await;
        };

        let num = rand::thread_rng().gen_range(10..100);

        let new_email = format!(
            "{}{}{}{}@developforgood.org",
            user.first_name.trim().to_lowercase(),
            email_policy.separator,
            user.last_name.trim().to_lowercase(),
            num
        );

        user.generated_email = Some(new_email.clone());

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
            .primary_email(new_email.clone())
            .password(password.clone())
            .change_password_at_next_login(true)
            .build()?;

        match workspace.create_user(&admin_email, workspace_user_data.clone()).await {
            Ok(_) => {
                log::info!("successfully created new user");
                let res = mail
                    .send(
                        Mail::new()
                            .add_from("pantheon@developforgood.org")
                            .add_to(("anish@developforgood.org", "Anish Sinha").into())
                            .add_subject("Welcome Aboard! Your Login Instructions Inside")
                            .add_text(&format!(
                                "You have been issued a Develop for Good email.
                             Your handle is: {new_email} and your temporary password is {password}. 

                             You will need to change it at your next login",
                            )),
                    )
                    .await?;

                dbg!(&res.text().await?);
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
                .personal_email(u.email.to_owned())
                .generated_email(u.generated_email.clone()?)
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
