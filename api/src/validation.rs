use crate::error::{ApiError, ApiResult, validation::ValidationErrorKind};

type ValidationResult = Result<(), ValidationErrorKind>;

pub trait AppValidateEmail {
    fn validate_email(&self) -> ValidationResult;
}

impl AppValidateEmail for String {
    fn validate_email(&self) -> ValidationResult {
        if !validator::ValidateEmail::validate_email(&self) {
            return Err(ValidationErrorKind::Email);
        }
        Ok(())
    }
}

pub trait ValidateStringLength {
    fn max_length(&self, max: usize) -> ValidationResult;
    fn min_length(&self, min: usize) -> ValidationResult;
    fn length(&self, min: usize, max: usize) -> ValidationResult;
}

impl ValidateStringLength for String {
    fn max_length(&self, max: usize) -> ValidationResult {
        if self.chars().count() > max {
            return Err(ValidationErrorKind::MaxLength(max));
        }
        Ok(())
    }

    fn min_length(&self, min: usize) -> ValidationResult {
        if self.chars().count() < min {
            return Err(ValidationErrorKind::MinLength(min));
        }
        Ok(())
    }

    fn length(&self, min: usize, max: usize) -> ValidationResult {
        if self.chars().count() < min || self.chars().count() > max {
            return Err(ValidationErrorKind::Length{min, max});
        }
        Ok(())
    }
}

pub trait UrlValidator {
    fn validate_url(&self) -> ValidationResult;
}

impl UrlValidator for String {
    fn validate_url(&self) -> ValidationResult {
        if !validator::ValidateUrl::validate_url(&self) {
            return Err(ValidationErrorKind::Url);
        }
        Ok(())
    }
}
