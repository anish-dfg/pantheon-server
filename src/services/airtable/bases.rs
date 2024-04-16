use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Base {
    pub id: String,
    pub name: String,
    #[serde(rename(deserialize = "permissionLevel", serialize = "permissionLevel"))]
    pub permission_level: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bases {
    pub bases: Vec<Base>,
    pub offset: Option<String>,
}

impl IntoResponse for Bases {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
