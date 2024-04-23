use anyhow::Result;
use async_trait::async_trait;

use self::users::{CreateWorkspaceUser, WorkspaceUserData};

pub mod service_account;
pub mod users;

#[async_trait]
pub trait WorkspaceClient: Send + Sync {
    async fn list_users(&self, impersonate: &str) -> Result<reqwest::StatusCode>;
    async fn create_user(&self, impersonate: &str, user: CreateWorkspaceUser) -> Result<()>;
}
