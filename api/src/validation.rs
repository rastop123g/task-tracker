use serde::Serialize;
use utoipa::ToSchema;

use crate::error::{ApiError, ApiResult};

#[derive(Debug, Clone, ToSchema, serde::Serialize, ts_rs::TS)]
#[ts(export)]
#[serde(tag = "type", content = "value")]
pub enum FieldValidationErrorKind {
    MinLength(usize),
    MaxLength(usize),
    Length { min: usize, max: usize },
    Email,
    Url,
}

pub trait AppValidateEmail {
    fn validate_email(&self) -> ValidateValueResult;
}

impl AppValidateEmail for String {
    fn validate_email(&self) -> ValidateValueResult {
        if !validator::ValidateEmail::validate_email(&self) {
            return ValidateValueResult::Err(FieldValidationErrorKind::Email);
        }
        ValidateValueResult::Ok
    }
}

pub trait ValidateStringLength {
    fn max_length(&self, max: usize) -> ValidateValueResult;
    fn min_length(&self, min: usize) -> ValidateValueResult;
    fn length(&self, min: usize, max: usize) -> ValidateValueResult;
}

impl ValidateStringLength for String {
    fn max_length(&self, max: usize) -> ValidateValueResult {
        if self.chars().count() > max {
            return ValidateValueResult::Err(FieldValidationErrorKind::MaxLength(max));
        }
        ValidateValueResult::Ok
    }

    fn min_length(&self, min: usize) -> ValidateValueResult {
        if self.chars().count() < min {
            return ValidateValueResult::Err(FieldValidationErrorKind::MinLength(min));
        }
        ValidateValueResult::Ok
    }

    fn length(&self, min: usize, max: usize) -> ValidateValueResult {
        if self.chars().count() < min || self.chars().count() > max {
            return ValidateValueResult::Err(FieldValidationErrorKind::Length { min, max });
        }
        ValidateValueResult::Ok
    }
}

pub trait UrlValidator {
    fn validate_url(&self) -> ValidateValueResult;
}

impl UrlValidator for String {
    fn validate_url(&self) -> ValidateValueResult {
        if !validator::ValidateUrl::validate_url(&self) {
            return ValidateValueResult::Err(FieldValidationErrorKind::Url);
        }
        ValidateValueResult::Ok
    }
}

#[derive(Debug, Clone, Serialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[serde(tag = "type", content = "kind")]
pub enum ValidationErrorKind {
    Field {
        field: String,
        kind: FieldValidationErrorKind,
    },
    Body(String),
}

#[derive(Debug, Clone)]
pub enum ValidateBodyResult {
    None,
    One(ValidationErrorKind),
    Many(Vec<ValidationErrorKind>),
}

#[derive(Debug, Clone)]
pub enum ValidateValueResult {
    Ok,
    Err(FieldValidationErrorKind),
}

impl ValidateValueResult {
    pub fn into_validate_body_result(self, field: &str) -> ValidateBodyResult {
        match self {
            ValidateValueResult::Ok => ValidateBodyResult::None,
            ValidateValueResult::Err(e) => ValidateBodyResult::One(ValidationErrorKind::Field {
                field: field.to_string(),
                kind: e,
            }),
        }
    }
}

pub trait ValidateBody {
    fn validate_body(&self) -> ValidateBodyResult {
        ValidateBodyResult::None
    }
}

impl ValidateBodyResult {
    pub fn values(self) -> Vec<ValidationErrorKind> {
        match self {
            ValidateBodyResult::One(e) => vec![e],
            ValidateBodyResult::Many(e) => e,
            ValidateBodyResult::None => vec![],
        }
    }

    pub fn into_result(self) -> ApiResult<()> {
        self.into()
    }

    pub fn and(self, other: ValidateBodyResult) -> ValidateBodyResult {
        match self {
            ValidateBodyResult::None => other,
            ValidateBodyResult::One(e) => match other {
                ValidateBodyResult::None => ValidateBodyResult::One(e),
                ValidateBodyResult::One(e2) => ValidateBodyResult::Many(vec![e, e2]),
                ValidateBodyResult::Many(mut e2) => {
                    e2.push(e);
                    ValidateBodyResult::Many(e2)
                }
            },
            ValidateBodyResult::Many(mut e) => match other {
                ValidateBodyResult::None => ValidateBodyResult::Many(e),
                ValidateBodyResult::One(e2) => {
                    e.push(e2);
                    ValidateBodyResult::Many(e)
                }
                ValidateBodyResult::Many(e2) => {
                    e.extend(e2);
                    ValidateBodyResult::Many(e)
                }
            },
        }
    }
}

impl From<ValidateBodyResult> for Vec<ValidationErrorKind> {
    fn from(e: ValidateBodyResult) -> Self {
        e.values()
    }
}

impl From<Vec<ValidationErrorKind>> for ValidateBodyResult {
    fn from(e: Vec<ValidationErrorKind>) -> Self {
        if e.is_empty() {
            ValidateBodyResult::None
        } else if e.len() == 1 {
            ValidateBodyResult::One(e[0].clone())
        } else {
            ValidateBodyResult::Many(e)
        }
    }
}

impl From<ValidationErrorKind> for ValidateBodyResult {
    fn from(e: ValidationErrorKind) -> Self {
        ValidateBodyResult::One(e)
    }
}

impl From<ValidateBodyResult> for ApiResult<()> {
    fn from(e: ValidateBodyResult) -> Self {
        match e {
            ValidateBodyResult::None => Ok(()),
            ValidateBodyResult::One(e) => Err(ApiError::Validation(vec![e])),
            ValidateBodyResult::Many(e) => Err(ApiError::Validation(e)),
        }
    }
}

impl<T: ValidateBody> ValidateBody for Option<T> {
    fn validate_body(&self) -> ValidateBodyResult {
        match self {
            Some(v) => v.validate_body(),
            None => ValidateBodyResult::None,
        }
    }
}

impl<T: ValidateBody> ValidateBody for Vec<T> {
    fn validate_body(&self) -> ValidateBodyResult {
        let mut res: ValidateBodyResult = ValidateBodyResult::None;
        for v in self {
            res = res.and(v.validate_body());
        }
        res
    }
}
