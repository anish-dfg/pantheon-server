use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record<T> {
    pub id: String,
    pub fields: T,
    pub created_time: String,
}
