use std::str::FromStr;

use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use solana_account_decoder::UiAccount;
use solana_client::{nonblocking::pubsub_client::PubsubClient, rpc_config::RpcAccountInfoConfig};
use solana_sdk::pubkey::Pubkey;
use tokio_stream::StreamExt;

pub struct AccountCollector {
    client: PubsubClient,
    account: String,
    config: Option<RpcAccountInfoConfig>,
}

impl AccountCollector {
    pub fn new(
        client: PubsubClient,
        account: String,
        config: Option<RpcAccountInfoConfig>,
    ) -> Self {
        Self {
            client,
            account,
            config,
        }
    }
}

#[async_trait]
impl Collector<UiAccount> for AccountCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, UiAccount>> {
        let pubkey = Pubkey::from_str(&self.account)?;

        let stream = self
            .client
            .account_subscribe(&pubkey, self.config.clone())
            .await
            .expect("subscribes to account updates");
        let stream = stream.0.filter_map(|evt| Some(evt.value));

        Ok(Box::pin(stream))
    }
}
