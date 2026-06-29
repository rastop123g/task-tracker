use std::sync::Arc;

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, sleep};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    protocol::websocket::{
        WsOutgoingMsg,
        ws_outgoing::user::{AddMemberEvent, RemoveMemberEvent},
    },
    websocket::WsSessionMap,
};

// TODO: при необходимости распараллелить через NATS queue group:
// spawn N задач с nats_client.queue_subscribe(subject, "local-router")
const RETRY_INTERVAL_SECS: u64 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocalFanOutMessage {
    WorkspaceMessage(Uuid, String),
    UserMessage(Uuid, String),
    UsersMessage(Vec<Uuid>, String),
    AddUserToWorkspace(AddMemberEvent),
    RemoveUserFromWorkspace(RemoveMemberEvent),
}

pub async fn run_local_fan_out(
    node_id: String,
    nats_client: async_nats::Client,
    session_map: Arc<WsSessionMap>,
    cancel: CancellationToken,
) {
    let subject = format!("node.{node_id}.events");

    'glob: loop {
        let Ok(mut subscriber) = nats_client.subscribe(subject.clone()).await else {
            tracing::error!("Failed to subscribe to {subject}");
            sleep(Duration::from_secs(RETRY_INTERVAL_SECS)).await;
            continue;
        };

        'read: loop {
            tokio::select! {
                _ = cancel.cancelled() => {
                    break 'glob;
                }
                msg = subscriber.next() => {
                    let Some(msg) = msg else {
                        tracing::warn!("Local router subscriber ended, reconnecting...");
                        break 'read;
                    };
                    let Ok(msg) = serde_json::from_slice::<LocalFanOutMessage>(&msg.payload) else {
                        tracing::warn!("Failed to decode routed event {:?}", msg);
                        continue;
                    };
                    match msg {
                        LocalFanOutMessage::WorkspaceMessage(ws_id, m) => {
                            session_map.send_to_workspace(&ws_id, &m);
                        }
                        LocalFanOutMessage::UserMessage(user_id, m) => {
                            session_map.send_to_user(&user_id, &m);
                        }
                        LocalFanOutMessage::UsersMessage(user_ids, m) => {
                            for user_id in user_ids {
                                session_map.send_to_user(&user_id, &m);
                            }
                        }
                        LocalFanOutMessage::AddUserToWorkspace(ev) => {
                            session_map.add_user_to_workspace(&ev.user_id, &ev.workspace_id);
                            let str_ev =
                                serde_json::to_string(&WsOutgoingMsg::AddMember(ev.clone()));
                            if let Ok(m) = str_ev {
                                session_map.send_to_workspace(&ev.workspace_id, &m);
                            }
                        }
                        LocalFanOutMessage::RemoveUserFromWorkspace(ev) => {
                            session_map
                                .remove_user_from_workspace(&ev.user_id, &ev.workspace_id);
                            let str_ev =
                                serde_json::to_string(&WsOutgoingMsg::RemoveMember(ev.clone()));
                            if let Ok(m) = str_ev {
                                session_map.send_to_workspace(&ev.workspace_id, &m);
                                session_map.send_to_user(&ev.user_id, &m);
                            }
                        }
                    }
                }
            }
        }

        sleep(Duration::from_secs(RETRY_INTERVAL_SECS)).await;
    }
}
