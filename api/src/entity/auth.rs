
use crate::{db, error::{ApiError, validation::{ValidationError, ValidationErrorNamed}}, validation::{AppValidateEmail, ValidateStringLength}};

#[derive(Debug, Clone)]
pub struct RegisterUserEntity {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl TryFrom<crate::protocol::auth::RegisterRequest> for RegisterUserEntity {
    type Error = crate::error::ApiError;
    fn try_from(req: crate::protocol::auth::RegisterRequest) -> Result<Self, Self::Error> {
        let mut errs = Vec::new();
        if let Err(e) = req.name.length(3, 128) {
            errs.push(ValidationError("name", e));
        }
        if let Err(e) = req.email.validate_email() {
            errs.push(ValidationError("email", e));
        }
        if let Err(e) = req.password.length(8, 128) {
            errs.push(ValidationError("password", e));
        }
        if errs.len() > 0 {
            return Err(ApiError::Validation(errs));
        }
        Ok(Self {
            name: req.name,
            email: req.email,
            password: req.password,
        })
    }
}

impl From<RegisterUserEntity> for db::user::DBNewUser {
    fn from(req: RegisterUserEntity) -> Self {
        Self {
            name: req.name,
            email: req.email,
            password: req.password,
            avatar: None,
            avatar_preview: None,
        }
    }
}
