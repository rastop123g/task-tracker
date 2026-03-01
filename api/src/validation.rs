use crate::error::{ApiError, ApiResult};

pub trait AppValidateEmail {
    fn validate_email(&self) -> ApiResult<()>;
}

impl AppValidateEmail for String {
    fn validate_email(&self) -> ApiResult<()> {
        if !validator::ValidateEmail::validate_email(&self) {
            return Err(ApiError::BadRequest("invalid email".to_string()));
        }
        Ok(())
    }
}

pub trait ValidateStringLength {
    fn max_length(&self, field: &str, max: usize) -> ApiResult<()>;
    fn min_length(&self, field: &str, min: usize) -> ApiResult<()>;
    fn length(&self, field: &str, min: usize, max: usize) -> ApiResult<()>;
}

impl ValidateStringLength for String {
    fn max_length(&self, field: &str, max: usize) -> ApiResult<()> {
        if self.chars().count() > max {
            return Err(ApiError::BadRequest(format!("field {} - length must be at most {} characters", field, max)));
        }
        Ok(())
    }

    fn min_length(&self, field: &str, min: usize) -> ApiResult<()> {
        if self.chars().count() < min {
            return Err(ApiError::BadRequest(format!("field {} - length must be at least {} characters", field, min)));
        }
        Ok(())
    }

    fn length(&self, field: &str, min: usize, max: usize) -> ApiResult<()> {
        if self.chars().count() < min || self.chars().count() > max {
            return Err(ApiError::BadRequest(format!("field {} length must be between {} and {} characters", field, min, max)));
        }
        Ok(())
    }
}

pub trait UrlValidator {
    fn validate_url(&self, field: &str) -> ApiResult<()>;
}

impl UrlValidator for String {
    fn validate_url(&self, field: &str) -> ApiResult<()> {
        if !validator::ValidateUrl::validate_url(&self) {
            return Err(ApiError::BadRequest(format!("field {} - invalid url", field)));
        }
        Ok(())
    }
}
