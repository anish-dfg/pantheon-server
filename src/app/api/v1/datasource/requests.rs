use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatasourceView {
    Airtable { base: String, table: String, view: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatasourceViewRequest1 {
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
//
//
//

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirtableDatasourceViewRequestMetadata {
    pub is_user_table: bool,
    pub user_first_name_column: Option<String>,
    pub user_last_name_column: Option<String>,
    pub user_email_column: Option<String>,
    pub base: String,
    pub table: String,
    pub view: String,
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatasourceViewRequestMetadata {
    Airtable(AirtableDatasourceViewRequestMetadata),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatasourceViewRequest {
    pub name: String,
    pub description: String,
    pub metadata: AirtableDatasourceViewRequestMetadata,
}
