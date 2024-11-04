use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    rpc_response::RpcBlockUpdate,
};
use tokio_stream::StreamExt;

pub struct BlockCollector {
    client: PubsubClient,
    filter: RpcBlockSubscribeFilter,
    config: Option<RpcBlockSubscribeConfig>,
}

impl BlockCollector {
    pub fn new(
        client: PubsubClient,
        filter: RpcBlockSubscribeFilter,
        config: Option<RpcBlockSubscribeConfig>,
    ) -> Self {
        Self {
            client,
            filter,
            config,
        }
    }
}

#[async_trait]
impl Collector<RpcBlockUpdate> for BlockCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, RpcBlockUpdate>> {
        let stream = self
            .client
            .block_subscribe(self.filter.clone(), self.config.clone())
            .await
            .expect("subscribes to block updates");
        let stream = stream.0.filter_map(|evt| Some(evt.value));

        Ok(Box::pin(stream))
    }
}
