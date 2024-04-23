use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatasourceViewRequest {
    Airtable { offset: Option<String> },
    Google,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirtableDatasourceViewRequestMetadata {
    pub is_user_table: bool,
    pub first_name_column: Option<String>,
    pub last_name_column: Option<String>,
    pub email_column: Option<String>,
    pub base: String,
    pub table: String,
    pub view: String,
    pub fields: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
