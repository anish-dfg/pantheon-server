use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::auth0::UserInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UserData {
    Auth0(UserInfo),
}
