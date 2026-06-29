use std::time::Duration;

use async_nats::jetstream::consumer::pull::Config as PullConfig;
use futures_util::StreamExt;
use tokio::time::sleep;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::{app_resources::AppResources, protocol::websocket::WsIncomingMsg};

const RETRY_INTERVAL_SECS: u64 = 1;

pub fn incoming_task(app: AppResources, cancel: CancellationToken, tracker: TaskTracker) {
    tracker.spawn(async move {
        let js = app.nats.js.clone();

        loop {
            if cancel.is_cancelled() {
                break;
            }
            run_consumer(&js, app.clone(), cancel.clone()).await;
            sleep(Duration::from_secs(RETRY_INTERVAL_SECS)).await;
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
            durable_name: Some("ws-incoming-task".into()),
            filter_subject: "events.wim".into(),
            ..Default::default()
        })
        .await
    else {
        tracing::error!("Failed to create consumer on events.wim");
        return;
    };

    let mut messages = match consumer.messages().await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to start consumer wim stream: {e}");
            return;
        }
    };

    tracing::info!("Started consumer stream on events.wim");

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
                let Ok(msg) = serde_json::from_slice::<WsIncomingMsg>(&message.payload) else {
                    message.ack().await.ok();
                    continue;
                };
                tracing::debug!("Received WS incoming message in jetstream: {msg:?}");
                match msg {
                    WsIncomingMsg::Ping(_) | WsIncomingMsg::Pong(_) => {
                        message.ack().await.ok();
                    }
                }
                message.ack().await.ok();
            }
        }
    }
}
