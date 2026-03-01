use std::fmt;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::error::unauthotized::UnauthotizedError;
pub mod unauthotized;

#[derive(Debug, Clone)]
pub enum ApiError {
    NotFound(String), // model
    Unauthorized(UnauthotizedError),
    Forbidden,
    BadRequest(String), // reason
    InternalServerError,
    CustomHttp(StatusCode,String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ApiError {}

pub type ApiResult<T> = Result<T, ApiError>;

impl From<sqlx::Error> for ApiError {
    fn from(_: sqlx::Error) -> Self {
        ApiError::InternalServerError
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(e: validator::ValidationErrors) -> Self {
        ApiError::BadRequest(e.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, reason) = match self {
            ApiError::NotFound(model) => (StatusCode::NOT_FOUND, format!("{} not found", model)),
            ApiError::Unauthorized(reason) => (StatusCode::UNAUTHORIZED, reason.to_string()),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "forbidden".to_string()),
            ApiError::BadRequest(reason) => (StatusCode::BAD_REQUEST, reason),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
            ApiError::CustomHttp(status, reason) => (status, reason),
        };
        (status, Json(json!({ "error": reason }))).into_response()
    }
}
