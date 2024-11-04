use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use solana_client::{nonblocking::pubsub_client::PubsubClient, rpc_response::SlotInfo};
use tokio_stream::StreamExt;

pub struct SlotCollector {
    client: PubsubClient,
}

impl SlotCollector {
    pub fn new(client: PubsubClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Collector<SlotInfo> for SlotCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, SlotInfo>> {
        let stream = self
            .client
            .slot_subscribe()
            .await
            .expect("subscribes to slot stream");
        let stream = stream.0.filter_map(|evt| Some(evt));

        Ok(Box::pin(stream))
    }
}
