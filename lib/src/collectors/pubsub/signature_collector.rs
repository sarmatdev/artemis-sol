use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient, rpc_config::RpcSignatureSubscribeConfig,
    rpc_response::RpcSignatureResult,
};
use solana_sdk::signature::Signature;
use tokio_stream::StreamExt;

pub struct SignatureCollector {
    client: PubsubClient,
    signature: Signature,
    config: Option<RpcSignatureSubscribeConfig>,
}

impl SignatureCollector {
    pub fn new(
        client: PubsubClient,
        signature: Signature,
        config: Option<RpcSignatureSubscribeConfig>,
    ) -> Self {
        Self {
            client,
            signature,
            config,
        }
    }
}

#[async_trait]
impl Collector<RpcSignatureResult> for SignatureCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, RpcSignatureResult>> {
        let stream = self
            .client
            .signature_subscribe(&self.signature, self.config.clone())
            .await
            .expect("subscribes to signature updates");
        let stream = stream.0.filter_map(|evt| Some(evt.value));

        Ok(Box::pin(stream))
    }
}
