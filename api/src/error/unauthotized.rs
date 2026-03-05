use enum_stringify::EnumStringify;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, ToSchema, EnumStringify, Serialize)]
pub enum UnauthotizedError {
    BadCredentials,
    EmailNotConfirmed,
    MissingToken,
    InvalidToken,
    TokenExpired,
    UserDeleted,
}
