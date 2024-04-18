use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub options: Option<Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct View {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    pub visible_field_ids: Option<Vec<String>>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    pub id: String,
    #[serde(rename = "primaryFieldId")]
    pub primary_field_id: String,
    pub name: String,
    pub description: Option<String>,
    // #[serde(skip_serializing)]
    pub fields: Vec<Field>,
    pub views: Vec<View>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schema {
    pub tables: Vec<Table>,
}

impl IntoResponse for Schema {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
