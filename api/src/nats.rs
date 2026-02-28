use std::time::Duration;

use anyhow::Result;
use async_nats::{
    jetstream::{
        self,
        stream::{self},
        Context,
    },
    Client, ConnectOptions,
};
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct NatsClients {
    pub js: Context,
    pub client: Client,
}

pub type NatsClient = NatsClients;

pub async fn init_nats_client(config: &crate::config::Config) -> Result<NatsClient> {
    let mut backoff = 1;
    let client = loop {
        let opts = ConnectOptions::new()
            .connection_timeout(Duration::from_secs(5))
            .max_reconnects(None)
            .ping_interval(Duration::from_secs(5));

        match opts.connect(&config.nats_url).await {
            Ok(client) => {
                log::info!("Connected to NATS at {}", &config.nats_url);
                break Ok(client);
            }
            Err(e) if backoff <= 32 => {
                log::warn!("Connect failed, retry in {}s: {}", backoff, e);
                sleep(Duration::from_secs(backoff)).await;
                backoff *= 2;
            }
            Err(e) => {
                log::error!("Failed to connect after retries: {}", e);
                break Err(e);
            }
        }
    };
    let client = client?;
    let js = async_nats::jetstream::new(client.clone());
    // create stream if need
    Ok(NatsClients { js, client })
}

async fn create_stream(js: &jetstream::Context, config: stream::Config) -> Result<()> {
    let s = js.create_stream(config.clone()).await;
    if let Err(e) = s {
        tracing::error!("Failed to create stream: {:?}, try update", e);
        js.update_stream(config).await?;
    }
    Ok(())
}
