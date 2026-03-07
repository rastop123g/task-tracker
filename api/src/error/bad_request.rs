use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum BadRequestError {
    UserNotConfirmed,
    UserAlreadyConfirmed,
    UserDeleted,
    EmailAlreadyUsed,
    OldPasswordWrong,
}
