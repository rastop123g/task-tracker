use utoipa::ToSchema;

use crate::error::{
    bad_request::BadRequestError, forbidden::ForbiddenError, unauthotized::UnauthotizedError,
    validation::ValidationErrorNamed,
};

#[derive(serde::Serialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[schema(description = "Unauthotized Error")]
#[ts(export)]
pub struct UnauthotizedErrorResponse {
    pub reason: UnauthotizedError,
}

#[derive(serde::Serialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[schema(description = "Validation Error")]
#[ts(export)]
pub struct ValidationErrorResponse {
    pub errors: Vec<ValidationErrorNamed>,
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
    Validation(Vec<ValidationErrorNamed>),
    Other(BadRequestError),
}
