use utoipa::ToSchema;

use crate::error::{unauthotized::UnauthotizedError, validation::ValidationErrorNamed};

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
