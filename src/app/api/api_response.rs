use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use derive_builder::Builder;
use serde::Serialize;

use super::api_error::ApiError;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ApiResponseData<T>
where
    T: Serialize,
{
    Error(ApiError),
    Data(T),
}


#[derive(Debug, Builder)]
pub struct ApiResponse<T>
where
    T: Serialize + Clone,
{
    pub status_code: StatusCode,
    pub data: ApiResponseData<T>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize + Clone,
{
    fn into_response(self) -> Response {
        (self.status_code, Json(self.data)).into_response()
    }
}
