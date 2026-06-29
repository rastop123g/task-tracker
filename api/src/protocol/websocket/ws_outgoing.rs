use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::protocol::websocket::ws_outgoing::user::{AddMemberEvent, RemoveMemberEvent, RevokeInviteEvent, UpdateUserEvent, UserInviteEvent};

pub mod user;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[serde(tag = "event", content = "data")]
pub enum WsOutgoingMsg {
    Ping(()),
    Pong(()),
    Connected(()),
    BadAuth(()),
    UnknownMessageErr(String),
    UpdateUser(UpdateUserEvent),
    UserInvite(UserInviteEvent),
    AddMember(AddMemberEvent),
    RemoveMember(RemoveMemberEvent),
    RevokeInvite(RevokeInviteEvent),
}
