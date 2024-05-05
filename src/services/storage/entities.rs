use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use super::types::{JobStatus, JobType, SupportedDatasource};

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub image_uri: String,
}

pub type Users = Vec<User>;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DatasourceView {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub view_name: String,
    pub datasource: SupportedDatasource,
    pub description: String,
    pub metadata: Value,
}

pub type DatasourceViews = Vec<DatasourceView>;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub status: JobStatus,
    pub job_type: JobType,
    pub metadata: Value,
}

pub type Jobs = Vec<Job>;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DatasourceViewJob {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub job_id: Uuid,
}

pub type DatasourceViewJobs = Vec<DatasourceViewJob>;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ExportedUser {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub first_name: String,
    pub last_name: String,
    pub personal_email: String,
    pub generated_email: String,
    pub exported_from: SupportedDatasource,
}

pub type ExportedUsers = Vec<ExportedUser>;
