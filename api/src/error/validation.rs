use utoipa::ToSchema;

#[derive(Debug, Clone, ToSchema, serde::Serialize)]
#[schema(description = "Validation Error")]
pub struct ValidationErrorNamed {
    pub field: &'static str,
    pub reason: ValidationErrorKind,
}

#[derive(Debug, Clone)]
pub struct ValidationError(pub &'static str, pub ValidationErrorKind);

impl From<ValidationError> for ValidationErrorNamed {
    fn from(e: ValidationError) -> Self {
        Self {
            field: e.0,
            reason: e.1,
        }
    }
}

#[derive(Debug, Clone, ToSchema, serde::Serialize)]
#[serde(tag = "type", content = "value")]
pub enum ValidationErrorKind {
    MinLength(usize),
    MaxLength(usize),
    Length{min: usize, max: usize},
    Email,
    Url,
}
