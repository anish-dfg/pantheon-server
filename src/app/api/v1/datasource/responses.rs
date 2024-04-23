use derive_builder::Builder;
use serde::Serialize;
use serde_json::Value;

use crate::services::storage::entities::DatasourceView;

#[derive(Clone, Debug, Serialize, Builder)]
pub struct AirtableViewData {
    data: DatasourceView,
    records: Vec<Value>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum DatasourceViewResponse {
    Airtable(AirtableViewData),
    GoogleWorkspaceAdminDirectory,
}

#[derive(Debug, Serialize)]
pub struct CreateAirtableViewResponse {
    pub job_id: String,
}
