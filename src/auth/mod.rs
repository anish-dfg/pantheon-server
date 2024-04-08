pub mod auth0;
pub mod userdata;

use anyhow::Result;
use async_trait::async_trait;

use self::userdata::UserData;

#[async_trait]
pub trait TokenAuthenticator {
    async fn authenticate(&self, token: &str) -> Result<UserData>;
}

pub enum Authenticator {
    TokenAuthenticator(Box<dyn TokenAuthenticator>),
}
