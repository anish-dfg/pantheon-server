use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatasourceView {
    Airtable { base: String, table: String, view: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatasourceViewRequest {
    pub view_name: String,
    pub datasource_name: String,
    pub description: String,
    pub metadata: DatasourceView,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatasourceViewRequest {
    Airtable { offset: Option<String> },
    Google,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub enum DatasourceViewData {
//     Airtable(ListRecordsResponse<Value>),
// }
