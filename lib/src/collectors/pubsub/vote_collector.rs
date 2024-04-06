use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use solana_client::{nonblocking::pubsub_client::PubsubClient, rpc_response::RpcVote};
use tokio_stream::StreamExt;

pub struct VoteCollector {
    client: PubsubClient,
}

impl VoteCollector {
    pub fn new(client: PubsubClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Collector<RpcVote> for VoteCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, RpcVote>> {
        let stream = self
            .client
            .vote_subscribe()
            .await
            .expect("subscribes to vote stream");
        let stream = stream.0.filter_map(|evt| Some(evt));

        Ok(Box::pin(stream))
    }
}
