use utoipa::ToSchema;

use crate::error::{
    bad_request::BadRequestError, forbidden::ForbiddenError, unauthotized::UnauthotizedError,
    validation::ValidationErrorNamed,
};

#[derive(serde::Serialize, ToSchema, Clone, Debug)]
#[schema(description = "Unauthotized Error")]
pub struct UnauthotizedErrorResponse {
    pub reason: UnauthotizedError,
}

#[derive(serde::Serialize, ToSchema, Clone, Debug)]
#[schema(description = "Validation Error")]
pub struct ValidationErrorResponse {
    pub errors: Vec<ValidationErrorNamed>,
}

#[derive(serde::Serialize, ToSchema, Clone, Debug)]
pub struct ForbiddenErrorResponse {
    pub reason: ForbiddenError,
}

#[derive(serde::Serialize, ToSchema, Clone, Debug)]
pub struct BadRequestErrorResponse {
    pub reason: BadRequestError,
}
