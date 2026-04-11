use axum::{
    extract::{Query, State, WebSocketUpgrade},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    app_resources::AppResources,
    jwt,
    protocol::websocket::ws_outgoing::WsOutgoingMsg, websocket::WsSessionMap,
};

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct WsParams {
    /// JWT token
    pub token: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/ws",
    tag = "WebSocket Gateway",
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

async fn handle_socket(
    socket: axum::extract::ws::WebSocket,
    token: String,
    app: AppResources,
) {
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

    let (send_tx, mut send_rx) = tokio::sync::mpsc::channel::<String>(WsSessionMap::channel_capacity());

}
