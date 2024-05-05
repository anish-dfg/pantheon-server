use anyhow::{bail, Error};
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone)]
#[sqlx(type_name = "supported_datasource", rename_all = "snake_case")]
pub enum SupportedDatasource {
    Airtable,
    GoogleWorkspaceAdminDirectory,
}

impl TryInto<SupportedDatasource> for String {
    type Error = Error;

    fn try_into(self) -> Result<SupportedDatasource, Self::Error> {
        match self.to_lowercase().as_str() {
            "airtable" => Ok(SupportedDatasource::Airtable),
            "google" => Ok(SupportedDatasource::GoogleWorkspaceAdminDirectory),
            _ => bail!("unsupported datasource"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "job_status", rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Error,
    Complete,
}

impl TryInto<JobStatus> for &str {
    type Error = Error;

    fn try_into(self) -> Result<JobStatus, Self::Error> {
        match self {
            "pending" => Ok(JobStatus::Pending),
            "error" => Ok(JobStatus::Error),
            "complete" => Ok(JobStatus::Complete),
            _ => bail!("unsupported value"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Copy, Clone, PartialEq, Eq)]
#[sqlx(type_name = "job_type", rename_all = "snake_case")]
pub enum JobType {
    ExportData,
    ImportData,
    UndoExport,
}
