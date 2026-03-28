use utoipa::ToSchema;

use crate::{
    error::{
        bad_request::BadRequestError, forbidden::ForbiddenError, unauthotized::UnauthotizedError,
    },
    validation::ValidationErrorKind,
};

#[derive(serde::Serialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[schema(description = "Unauthotized Error")]
#[ts(export)]
pub struct UnauthotizedErrorResponse {
    pub reason: UnauthotizedError,
}

#[derive(serde::Serialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
pub struct ForbiddenErrorResponse {
    pub reason: ForbiddenError,
}

#[derive(serde::Serialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[serde(tag = "type", content = "reason")]
pub enum BadRequestErrorResponse {
    Validation(Vec<ValidationErrorKind>),
    Other(BadRequestError),
}
