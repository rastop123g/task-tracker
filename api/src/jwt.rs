use chrono::{DateTime, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{ApiResult, unauthotized::UnauthotizedError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    sub: Uuid,
    exp: usize,
}

pub fn create(
    user_id: &Uuid,
    exp: DateTime<Utc>,
    config: &crate::config::Config,
) -> ApiResult<String> {
    let exp = exp.timestamp() as usize;
    let claims = Claims {
        sub: *user_id,
        exp,
    };
    let encoded = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    );
    encoded.map_err(|_| crate::error::ApiError::InternalServerError)
}

pub fn verify(token: &str, config: &crate::config::Config) -> ApiResult<Uuid> {
    let decoded = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    decoded
        .map(|data| data.claims.sub)
        .map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => {
                crate::error::ApiError::Unauthorized(UnauthotizedError::TokenExpired)
            }
            _ => crate::error::ApiError::Unauthorized(UnauthotizedError::InvalidToken),
        })
}
