use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use jito_geyser_client::interceptor::GrpcInterceptor;
use jito_geyser_protos::solana::geyser::{
    geyser_client::GeyserClient, MaybePartialAccountUpdate, SubscribePartialAccountUpdatesRequest,
};
use tokio_stream::StreamExt;
use tonic::{codegen::InterceptedService, transport::Channel};

pub struct PartialAccountCollector {
    geyser_client: GeyserClient<InterceptedService<Channel, GrpcInterceptor>>,
    skip_votes: bool,
}

impl PartialAccountCollector {
    pub fn new(
        geyser_client: GeyserClient<InterceptedService<Channel, GrpcInterceptor>>,
        skip_votes: bool,
    ) -> Self {
        Self {
            geyser_client,
            skip_votes,
        }
    }
}

#[async_trait]
impl Collector<MaybePartialAccountUpdate> for PartialAccountCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, MaybePartialAccountUpdate>> {
        let stream = self
            .geyser_client
            .subscribe_partial_account_updates(SubscribePartialAccountUpdatesRequest {
                skip_vote_accounts: self.skip_votes,
            })
            .await
            .expect("subscribes to partial updates");

        let stream = stream.into_inner().filter_map(|evt| match evt {
            Ok(evt) => Some(evt),
            Err(_) => None,
        });

        Ok(Box::pin(stream))
    }
}
