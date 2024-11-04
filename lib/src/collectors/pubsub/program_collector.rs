use std::str::FromStr;

use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient, rpc_config::RpcProgramAccountsConfig,
    rpc_response::RpcKeyedAccount,
};
use solana_sdk::pubkey::Pubkey;
use tokio_stream::StreamExt;

pub struct ProgramCollector {
    client: PubsubClient,
    program: String,
    config: Option<RpcProgramAccountsConfig>,
}

impl ProgramCollector {
    pub fn new(
        client: PubsubClient,
        program: String,
        config: Option<RpcProgramAccountsConfig>,
    ) -> Self {
        Self {
            client,
            program,
            config,
        }
    }
}

#[async_trait]
impl Collector<RpcKeyedAccount> for ProgramCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, RpcKeyedAccount>> {
        let pubkey = Pubkey::from_str(&self.program)?;
        let stream = self
            .client
            .program_subscribe(&pubkey, self.config.clone())
            .await
            .expect("subscribes to program updates");
        let stream = stream.0.filter_map(|evt| Some(evt.value));

        Ok(Box::pin(stream))
    }
}
