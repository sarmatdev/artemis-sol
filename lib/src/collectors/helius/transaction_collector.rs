use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use helius::{
    types::{RpcTransactionsConfig, TransactionNotification},
    websocket::EnhancedWebsocket,
};

#[derive(Clone)]
pub struct TransactionCollector {
    client: Arc<EnhancedWebsocket>,
    config: RpcTransactionsConfig,
}

impl TransactionCollector {
    pub fn new(client: Arc<EnhancedWebsocket>, config: RpcTransactionsConfig) -> Self {
        Self { client, config }
    }
}

#[async_trait]
impl Collector<TransactionNotification> for TransactionCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, TransactionNotification>> {
        let (stream, _unsub) = self
            .client
            .transaction_subscribe(self.config.clone())
            .await?
            .into();

        Ok(Box::pin(stream))
    }
}
