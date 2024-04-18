use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub image_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub image_uri: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub image_uri: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDatasourceView {
    pub view_name: String,
    pub description: String,
    pub datasource_name: String,
    pub metadata: Value,
    pub user_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DatasourceView {
    pub id: Uuid,
    pub user_id: Uuid,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub view_name: String,
    pub datasource_name: String,
    pub metadata: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: Uuid,
    pub user_id: Uuid,
    pub description: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct CreateJob {
    pub user_id: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct JobError {
    pub id: Uuid,
    pub job_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub error_data: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarkJobErrored {
    pub job_id: String,
    pub error: Value,
}
