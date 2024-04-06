use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    rpc_response::RpcLogsResponse,
};
use tokio_stream::StreamExt;

pub struct LogsCollector {
    client: PubsubClient,
    filter: RpcTransactionLogsFilter,
    config: RpcTransactionLogsConfig,
}

impl LogsCollector {
    pub fn new(
        client: PubsubClient,
        filter: RpcTransactionLogsFilter,
        config: RpcTransactionLogsConfig,
    ) -> Self {
        Self {
            client,
            filter,
            config,
        }
    }
}

#[async_trait]
impl Collector<RpcLogsResponse> for LogsCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, RpcLogsResponse>> {
        let stream = self
            .client
            .logs_subscribe(self.filter.clone(), self.config.clone())
            .await
            .expect("subscribes to logs stream");
        let stream = stream.0.filter_map(|evt| Some(evt.value));

        Ok(Box::pin(stream))
    }
}
