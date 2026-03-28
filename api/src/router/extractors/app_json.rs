use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;

use crate::{error::ApiError, utils::AppTrim, validation::ValidateBody};

pub struct AppJson<T: DeserializeOwned + Clone + AppTrim + ValidateBody>(pub T);

impl<S, T> FromRequest<S> for AppJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Clone + AppTrim + ValidateBody,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(mut data): Json<T> = Json::from_request(req, state)
            .await
            .map_err(|_| ApiError::InternalServerError)?;
        data.app_trim();
        data.validate_body().into_result()?;
        Ok(Self(data))
    }
}
