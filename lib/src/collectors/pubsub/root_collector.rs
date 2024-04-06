use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_program::clock::Slot;
use tokio_stream::StreamExt;

pub struct RootCollector {
    client: PubsubClient,
}

impl RootCollector {
    pub fn new(client: PubsubClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Collector<Slot> for RootCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, Slot>> {
        let stream = self
            .client
            .root_subscribe()
            .await
            .expect("subscribes to root stream");
        let stream = stream.0.filter_map(|evt| Some(evt));

        Ok(Box::pin(stream))
    }
}
