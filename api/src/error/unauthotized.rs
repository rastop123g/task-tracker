use enum_stringify::EnumStringify;
use serde::Serialize;

#[derive(Debug, Clone, Copy, EnumStringify)]
pub enum UnauthotizedError {
    BadCredentials,
    EmailNotConfirmed,
    MissingToken,
    InvalidToken,
    TokenExpired,
    UserDeleted,
}
