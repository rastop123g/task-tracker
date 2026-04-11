use std::sync::Arc;

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, sleep};
use uuid::Uuid;

use crate::websocket::WsSessionMap;

// TODO: при необходимости распараллелить через NATS queue group:
// spawn N задач с nats_client.queue_subscribe(subject, "local-router")
const RETRY_INTERVAL_SECS: u64 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocalRouterMessage {
    WorkspaceMessage(Uuid, String),
    UserMessage(Uuid, String),
    AddUserToWorkspace { user_id: Uuid, workspace_id: Uuid },
    RemoveUserFromWorkspace { user_id: Uuid, workspace_id: Uuid },
}

pub async fn run_local_router(
    node_id: String,
    nats_client: async_nats::Client,
    session_map: Arc<WsSessionMap>,
) {
    let subject = format!("node.{node_id}.events");

    loop {
        match nats_client.subscribe(subject.clone()).await {
            Ok(mut subscriber) => {
                tracing::info!("Local router subscribed to {subject}");

                while let Some(message) = subscriber.next().await {
                    match serde_json::from_slice::<LocalRouterMessage>(&message.payload) {
                        Ok(m) => match m {
                            LocalRouterMessage::WorkspaceMessage(ws_id, m) => {
                                session_map.send_to_workspace(&ws_id, &m);
                            }
                            LocalRouterMessage::UserMessage(user_id, m) => {
                                session_map.send_to_user(&user_id, &m);
                            }
                            LocalRouterMessage::AddUserToWorkspace { user_id, workspace_id } => {
                                session_map.add_user_to_workspace(&user_id, &workspace_id);
                                // TODO: send message to user and workspace
                            }
                            LocalRouterMessage::RemoveUserFromWorkspace {
                                user_id,
                                workspace_id,
                            } => {
                                session_map.remove_user_from_workspace(&user_id, &workspace_id);
                                // TODO: send message to user and workspace
                            }
                        },
                        Err(e) => {
                            tracing::warn!("Failed to decode routed event: {e}");
                        }
                    }
                }

                tracing::warn!("Local router subscriber ended, reconnecting...");
            }
            Err(e) => {
                tracing::error!("Failed to subscribe to {subject}: {e}, retrying...");
            }
        }

        sleep(Duration::from_secs(RETRY_INTERVAL_SECS)).await;
    }
}
