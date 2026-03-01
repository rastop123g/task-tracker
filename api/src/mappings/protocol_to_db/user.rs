use sha2::Digest;

use crate::{db, error::{ApiError, ApiResult}, validation::{AppValidateEmail, ValidateStringLength}};

impl TryFrom<protocol::auth::RegisterRequest> for db::user::NewUser {
    type Error = ApiError;
    /// validate and hash password
    fn try_from(req: protocol::auth::RegisterRequest) -> ApiResult<Self> {
        req.password.length("password", 8, 128)?;
        req.email.validate_email()?;
        req.name.length("name", 3, 128)?;
        let sha2 = sha2::Sha256::digest(req.password.as_bytes());
        let password = hex::encode(sha2);
        Ok(Self {
            name: req.name,
            email: req.email,
            password,
            avatar: req.avatar_id.map(|id| id.to_string()),
        })
    }
}
