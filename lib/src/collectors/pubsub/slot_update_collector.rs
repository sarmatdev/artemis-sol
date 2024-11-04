use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use solana_client::{nonblocking::pubsub_client::PubsubClient, rpc_response::SlotUpdate};
use tokio_stream::StreamExt;

pub struct SlotUpdateCollector {
    client: PubsubClient,
}

impl SlotUpdateCollector {
    pub fn new(client: PubsubClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Collector<SlotUpdate> for SlotUpdateCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, SlotUpdate>> {
        let stream = self
            .client
            .slot_updates_subscribe()
            .await
            .expect("subscribes to slot update stream");
        let stream = stream.0.filter_map(|evt| Some(evt));

        Ok(Box::pin(stream))
    }
}
