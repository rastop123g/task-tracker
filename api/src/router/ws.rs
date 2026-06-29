use std::time::Duration;

use axum::{
    extract::{Query, State, WebSocketUpgrade},
    response::Response,
};
use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio_util::sync::CancellationToken;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::{
    app_resources::AppResources,
    jwt,
    protocol::websocket::{WsIncomingMsg, ws_outgoing::WsOutgoingMsg},
    router::extractors::req_ctx::Ctx,
    websocket::{WsSessionMap, ws_session::WsOutChannelMessage},
};

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct WsParams {
    /// JWT token
    pub token: String,
}

#[utoipa::path(
    get,
    path = "/ws",
    tag = "websocket",
    responses(
        (status = 101, description = "Switching Protocols"),
    ),
    params(
        WsParams,
    )
)]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(app): State<AppResources>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, params.token, app))
}

async fn handle_socket(socket: axum::extract::ws::WebSocket, token: String, app: AppResources) {
    let (mut sender, mut receiver) = socket.split();

    let user_id = match jwt::verify(&token, &app.config) {
        Ok(id) => id,
        Err(_) => {
            let msg = serde_json::to_string(&WsOutgoingMsg::BadAuth(())).unwrap();
            let _ = sender
                .send(axum::extract::ws::Message::Text(msg.into()))
                .await;
            return;
        }
    };
    tracing::info!("User connected: {user_id}");

    let ctx = app.ctx();
    //NOTE: check user
    match ctx.user_service().get(&user_id).await {
        Ok(user) => user,
        Err(_) => {
            let msg = serde_json::to_string(&WsOutgoingMsg::BadAuth(())).unwrap();
            let _ = sender
                .send(axum::extract::ws::Message::Text(msg.into()))
                .await;
            return;
        }
    };

    //NOTE: get user workspaces
    let workspaces = match ctx
        .ws_registry_service()
        .get_user_workspaces(&user_id)
        .await
    {
        Ok(workspaces) => workspaces,
        Err(_) => {
            return;
        }
    };

    let (send_tx, mut send_rx) =
        tokio::sync::mpsc::channel::<WsOutChannelMessage>(WsSessionMap::channel_capacity());
    let cancel = ctx.app.cancel.child_token();

    //NOTE: add session to local and global registry
    let session = ctx.app.ws_sessions.add_session(
        user_id,
        workspaces.clone(),
        send_tx.clone(),
        cancel.clone(),
    );
    tracing::info!("Session created: {session}");
    let mut cleanup = CleanupSession::new(session.clone());
    let Ok(_) = ctx.ws_registry_service().add_node_to_user(&user_id).await else {
        cleanup.cleanup(&ctx).await;
        return;
    };
    cleanup.user_setted = true;
    let Ok(_) = ctx
        .ws_registry_service()
        .add_node_to_workspaces(&workspaces)
        .await
    else {
        cleanup.cleanup(&ctx).await;
        return;
    };
    cleanup.workspace_setted = true;

    let sender_task = ctx.app.tracker.spawn({
        let cancel = cancel.clone();
        async move {
            loop {
                tokio::select! {
                    _ = cancel.cancelled() => {
                        break;
                    },
                    msg = send_rx.recv() => {
                        let Some(msg) = msg else {
                            break;
                        };
                        match msg {
                            WsOutChannelMessage::Send(msg) => match sender.send(axum::extract::ws::Message::Text(msg.into())).await {
                                Ok(_) => {}
                                Err(e) => {
                                    tracing::warn!("Failed to send message: {e}");
                                    break;
                                }
                            }
                            WsOutChannelMessage::Pong => {
                                match sender.send(axum::extract::ws::Message::Ping(Bytes::new())).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        tracing::warn!("Failed to send message: {e}");
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    let (hb_tx, mut hb_rx) = tokio::sync::mpsc::channel::<HeartBitMessage>(4);

    let receiver_task = ctx.app.tracker.spawn({
        let cancel = cancel.clone();
        let send_tx = send_tx.clone();
        let js = ctx.app.nats.js.clone();
        async move {
            loop {
                tokio::select! {
                    _ = cancel.cancelled() => {
                        break;
                    },
                    msg = receiver.next() => {
                        let Some(Ok(msg)) = msg else {
                            break;
                        };
                        match msg {
                            axum::extract::ws::Message::Ping(_) => {
                                send_tx.send(WsOutChannelMessage::Pong).await.ok();
                            }
                            axum::extract::ws::Message::Pong(_) => {}
                            axum::extract::ws::Message::Text(msg) => {
                                hb_tx.try_send(HeartBitMessage::AnyMessage).ok();
                                // TODO: Может имеет смысл не парсить сообщение а попытаться более
                                // оптимально определить ping/pong исключить ping/pong WsIncomingMsg
                                let parsed = serde_json::from_str::<WsIncomingMsg>(&msg);
                                let Ok(parsed) = parsed else {
                                    let unknown_msg = WsOutgoingMsg::UnknownMessageErr(format!("{msg}"));
                                    if let Ok(json) = serde_json::to_string(&unknown_msg) {
                                        send_tx.send(WsOutChannelMessage::Send(json)).await.ok();
                                    }
                                    continue;
                                };
                                if let WsIncomingMsg::Ping(()) = parsed {
                                    hb_tx.try_send(HeartBitMessage::Ping).ok();
                                    continue;
                                }
                                if let WsIncomingMsg::Pong(()) = parsed {
                                    hb_tx.try_send(HeartBitMessage::Pong).ok();
                                    continue;
                                }
                                // wim - ws incoming msg
                                // TODO: ack to ws
                                let Ok(ack) = js.publish("events.wim", msg.into()).await else {
                                    break;
                                };
                                let Ok(_) = ack.await else {
                                    break;
                                };

                            }
                            axum::extract::ws::Message::Binary(_) => {
                                let unknown_msg = WsOutgoingMsg::UnknownMessageErr(format!("Unknown binary message"));
                                if let Ok(json) = serde_json::to_string(&unknown_msg) {
                                    send_tx.send(WsOutChannelMessage::Send(json)).await.ok();
                                }
                            }
                            axum::extract::ws::Message::Close(_) => {
                                break;
                            },
                        };
                    }
                }
            }
        }
    });

    let heartbeat_task = ctx.app.tracker.spawn({
        let cancel = cancel.clone();
        let send_tx = send_tx.clone();
        async move {
            let ping_msg = format!("{{\"event\":\"Ping\",\"data\":null}}");
            let pong_msg = format!("{{\"event\":\"Pong\",\"data\":null}}");
            let mut last_active = std::time::Instant::now();
            loop {
                tokio::select! {
                    _ = cancel.cancelled() => {
                        break;
                    },
                    _ = tokio::time::sleep(Duration::from_secs(10)) => {
                        //NOTE: Падаем если нет активности в последние 30 секунд
                        if last_active.elapsed() > Duration::from_secs(30) {
                            break;
                        } 
                        // NOTE: сюда мы попали если активности небыло от 10 секунд до 30 секунд
                        send_tx.send(WsOutChannelMessage::Send(ping_msg.clone())).await.ok();
                    }
                    msg = hb_rx.recv() => {
                        let Some(msg) = msg else {
                            break;
                        };
                        match msg {
                            HeartBitMessage::Ping => {
                                last_active = std::time::Instant::now();
                                send_tx.send(WsOutChannelMessage::Send(pong_msg.clone())).await.ok();
                            }
                            HeartBitMessage::Pong | HeartBitMessage::AnyMessage => {
                                last_active = std::time::Instant::now();
                            }
                        }
                    }
                }
            }
        }
    });
    if let Ok(json) = serde_json::to_string(&WsOutgoingMsg::Connected(())) {
        send_tx.send(WsOutChannelMessage::Send(json)).await.ok();
    } else {
        cancel.cancel();
    }

    //Чистим все при штатном завершении сессии или приложения
    ctx.app.tracker.spawn({
        let ctx = ctx.clone();
        let cancel = cancel.clone();
        async move {
            cancel.cancelled().await;
            cleanup.cleanup(&ctx).await;
            tracing::info!("Session closed: {session}");
        }
    });

    tokio::select! {
        _ = sender_task => {}
        _ = receiver_task => {}
        _ = heartbeat_task => {}
    }
    cancel.cancel();

}

struct CleanupSession {
    session_id: Uuid,
    user_setted: bool,
    workspace_setted: bool,
}

impl CleanupSession {
    fn new(session_id: Uuid) -> Self {
        Self {
            session_id,
            user_setted: false,
            workspace_setted: false,
        }
    }

    async fn cleanup(self, ctx: &Ctx) {
        let Some((user_id, workspace_ids)) = ctx.app.ws_sessions.remove_session(&self.session_id)
        else {
            return;
        };
        if self.user_setted {
            let has_another_session = ctx.app.ws_sessions.has_user_sessions(&user_id);
            if !has_another_session {
                ctx.ws_registry_service()
                    .remove_node_from_user(&user_id)
                    .await
                    .ok();
            }
        }
        if self.workspace_setted {
            let to_remove = workspace_ids
                .into_iter()
                .filter(|ws_id| !ctx.app.ws_sessions.has_workspace_sessions(ws_id))
                .collect::<Vec<Uuid>>();
            if !to_remove.is_empty() {
                ctx.ws_registry_service()
                    .remove_node_from_workspaces(&to_remove)
                    .await
                    .ok();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum HeartBitMessage {
    Ping,
    Pong,
    AnyMessage,
}
