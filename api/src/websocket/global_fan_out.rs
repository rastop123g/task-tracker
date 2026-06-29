use async_nats::jetstream::consumer::pull::Config as PullConfig;
use bytes::Bytes;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use uuid::Uuid;
use std::time::Duration;

use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::{app_resources::AppResources, protocol::websocket::ws_outgoing::user::{AddMemberEvent, RemoveMemberEvent, RevokeInviteEvent, UpdateUserEvent, UserInviteEvent}, router::extractors::req_ctx::Ctx, websocket::LocalFanOutMessage};

pub mod common;
mod user;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GlobalFanOutMessage {
    UpdateUser(UpdateUserEvent),
    AddMember(AddMemberEvent),
    RemoveMember(RemoveMemberEvent),
    UserInvite(UserInviteEvent),
    RevokeInvite(RevokeInviteEvent),
}

pub trait GlobalFanOutSender: Sized {
    async fn send_event(self, client: &async_nats::jetstream::Context);
}

impl GlobalFanOutMessage {
    async fn send(self, client: &async_nats::jetstream::Context) {
        let Ok(json) = serde_json::to_string(&self) else {
            return;
        };
        // wom - websocket outgoing message
        let Ok(_) = client.publish("events.wom", json.into()).await else {
            return;
        };
    }
}

impl<T> GlobalFanOutSender for T
where
    GlobalFanOutMessage: From<T>,
{
    async fn send_event(self, client: &async_nats::jetstream::Context) {
        let msg: GlobalFanOutMessage = self.into();
        msg.send(client).await;
    }
}

const RETRY_INTERVAL_SECS: u64 = 1;

pub fn run_global_fan_out_task(app: AppResources, cancel: CancellationToken, tracker: TaskTracker) {
    tracker.spawn(async move {
        let js = app.nats.js.clone();

        loop {
            if cancel.is_cancelled() {
                break;
            }
            run_consumer(&js, app.clone(), cancel.clone()).await;
            tokio::time::sleep(Duration::from_secs(RETRY_INTERVAL_SECS)).await;
        }
    });
}

async fn run_consumer(js: &async_nats::jetstream::Context, app: AppResources, cancel: CancellationToken) {
    let Ok(stream) = js.get_stream("EVENTS").await else {
        tracing::error!("Failed to get stream: EVENTS");
        return;
    };

    let Ok(consumer) = stream
        .create_consumer(PullConfig {
            durable_name: Some("ws-outgoing-task".into()),
            filter_subject: "events.wom".into(),
            ..Default::default()
        })
        .await
    else {
        tracing::error!("Failed to create consumer on events.wom");
        return;
    };

    let mut messages = match consumer.messages().await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to start consumer wom stream: {e}");
            return;
        }
    };

    tracing::info!("Started consumer stream on events.wom");

    let ctx = app.ctx();

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                break;
            }
            msg = messages.next() => {
                let Some(Ok(message)) = msg else {
                    break;
                };
                // Если 3 раза не обработали то дальше не будем
                let delivered = message.info().map(|i| i.delivered).unwrap_or(0);
                if delivered > 3 {
                    tracing::error!(
                        "Too many deliveries for message: {:?}",
                        String::from_utf8_lossy(&message.payload)
                    );
                    message.ack().await.ok();
                    continue;
                }

                let Ok(msg) = serde_json::from_slice::<GlobalFanOutMessage>(&message.payload) else {
                    message.ack().await.ok();
                    continue;
                };
                tracing::debug!("global fan out message: {msg:?}");

                let res = match msg {
                    GlobalFanOutMessage::UpdateUser(event) => user::upd_user(event, &ctx).await,
                    GlobalFanOutMessage::AddMember(event) => user::add_member(event, &ctx).await,
                    GlobalFanOutMessage::RemoveMember(event) => user::remove_member(event, &ctx).await,
                    GlobalFanOutMessage::UserInvite(event) => user::invite(event, &ctx).await,
                    GlobalFanOutMessage::RevokeInvite(event) => user::revoke_invite(event, &ctx).await,
                };

                if let Ok(_) = res {
                    message.ack().await.ok();
                }
            }
        }
    }
}

pub async fn send_to_user(user_id: &Uuid, msg: &str, ctx: &Ctx) -> anyhow::Result<()> {
    let user_nodes = ctx.ws_registry_service().get_user_nodes(user_id).await?;
    let msg = serde_json::to_string(&LocalFanOutMessage::UserMessage(*user_id, msg.to_owned()))?;
    let payload = Bytes::from(msg);
    for node_id in user_nodes {
        ctx.app.nats.client.publish(format!("node.{node_id}.events"), payload.clone()).await?;
    }
    Ok(())
}

pub async fn send_to_workspace(workspace_id: &Uuid, msg: &str, ctx: &Ctx) -> anyhow::Result<()> {
    let workspace_nodes = ctx.ws_registry_service().get_workspace_nodes(workspace_id).await?;
    let msg = serde_json::to_string(&LocalFanOutMessage::WorkspaceMessage(*workspace_id, msg.to_owned()))?;
    let payload = Bytes::from(msg);
    for node_id in workspace_nodes {
        ctx.app.nats.client.publish(format!("node.{node_id}.events"), payload.clone()).await?;
    }
    Ok(())
}
