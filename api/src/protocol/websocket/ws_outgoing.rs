use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[serde(tag = "type", content = "data")]
pub enum WsOutgoingMsg {
    Ping(()),
    Pong(()),
    BadAuth(()),
}
