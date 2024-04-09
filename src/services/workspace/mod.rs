use anyhow::Result;
use async_trait::async_trait;

use self::users::WorkspaceUserData;


pub mod service_account;
pub mod users;

#[async_trait]
pub trait WorkspaceClient: Send + Sync {
    async fn list_users(&self, impersonate: &str) -> Result<WorkspaceUserData>;
    async fn create_user(&self, impersonate: &str) -> Result<()>;
}
