use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[serde(tag = "type", content = "data")]
pub enum WsIncomingMsg {
    Ping(()),
    Pong(()),
}
