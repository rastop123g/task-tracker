use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ts_rs::TS)]
pub enum BadRequestError {
    UserNotConfirmed,
    UserAlreadyConfirmed,
    UserDeleted,
    EmailAlreadyUsed,
    OldPasswordWrong,
    MissingStatusOnCreateWorkspace,
    AvatarMissing,
    UserAlreadyInvited,
    UserIsMember,
    BadPathParams,
}
