use std::fmt;

use crate::{
    error::{
        bad_request::BadRequestError, forbidden::ForbiddenError, unauthotized::UnauthotizedError,
        validation::ValidationError,
    },
    protocol::error::{
        BadRequestErrorResponse, ForbiddenErrorResponse, UnauthotizedErrorResponse,
        ValidationErrorResponse,
    },
};
use axum::{Json, http::StatusCode, response::IntoResponse};
use bb8::RunError;
use redis::RedisError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod bad_request;
pub mod forbidden;
pub mod unauthotized;
pub mod validation;

#[derive(Debug, Clone)]
pub enum ApiError {
    NotFound(String), // model
    Unauthorized(UnauthotizedError),
    Forbidden(ForbiddenError),
    BadRequest(BadRequestError), // reason
    InternalServerError,
    CustomHttp(StatusCode, String),
    Validation(Vec<ValidationError>),
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

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, reason) = match self {
            ApiError::NotFound(model) => (StatusCode::NOT_FOUND, format!("{} not found", model)),
            ApiError::Unauthorized(reason) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(UnauthotizedErrorResponse { reason }),
                )
                    .into_response();
            }
            ApiError::Forbidden(reason) => {
                return (
                    StatusCode::FORBIDDEN,
                    Json(ForbiddenErrorResponse { reason }),
                )
                    .into_response();
            }
            ApiError::BadRequest(reason) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(BadRequestErrorResponse { reason }),
                )
                    .into_response();
            }
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
            ApiError::CustomHttp(status, reason) => (status, reason),
            ApiError::Validation(errors) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ValidationErrorResponse {
                        errors: errors.into_iter().map(|e| e.into()).collect(),
                    }),
                )
                    .into_response();
            }
        };
        (status, Json(ApiErrorResponse { error: reason })).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(description = "Api Error")]
pub struct ApiErrorResponse {
    /// Error reason
    pub error: String,
}

impl From<RunError<RedisError>> for ApiError {
    fn from(_: RunError<RedisError>) -> Self {
        ApiError::InternalServerError
    }
}

impl From<RedisError> for ApiError {
    fn from(_: RedisError) -> Self {
        ApiError::InternalServerError
    }
}
