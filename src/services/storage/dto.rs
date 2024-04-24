use anyhow::Error;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::types::{JobStatus, JobType, SupportedDatasource};

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct CreateUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub image_uri: String,
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct EditUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub image_uri: String,
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct CreateDatasourceView {
    pub user_id: Uuid,
    pub view_name: String,
    pub datasource: SupportedDatasource,
    pub description: String,
    pub metadata: Value,
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct EditDatasourceView {
    pub view_name: String,
    pub datasource: SupportedDatasource,
    pub description: String,
    pub metadata: Value,
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct CreateJob {
    pub user_id: Uuid,
    #[builder(setter(into))]
    pub status: JobStatus,
    pub job_type: JobType,
    pub metadata: Value,
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct CreateJobWithDatasource {
    pub user_id: Uuid,
    #[builder(setter(into))]
    pub status: JobStatus,
    pub job_type: JobType,
    pub metadata: Value,
    pub datasource_view_id: Uuid,
}

impl TryFrom<CreateJobWithDatasource> for CreateJob {
    type Error = Error;
    fn try_from(value: CreateJobWithDatasource) -> Result<Self, Self::Error> {
        Ok(CreateJobBuilder::default()
            .user_id(value.user_id)
            .status(value.status)
            .job_type(value.job_type)
            .metadata(value.metadata)
            .build()?)
    }
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct EditJob {
    pub status: JobStatus,
    pub job_type: JobType,
    pub metadata: Value,
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct CreateDatasourceViewJob {
    pub user_id: Uuid,
    pub job_id: Uuid,
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct CreateExportedUser {
    pub job_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub exported_from: SupportedDatasource,
}
