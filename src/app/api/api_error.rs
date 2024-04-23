use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
}

impl From<&str> for ApiError {
    fn from(value: &str) -> Self {
        Self {
            error: value.to_owned(),
        }
    }
}
