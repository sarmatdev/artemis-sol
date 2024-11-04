use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use jito_geyser_client::interceptor::GrpcInterceptor;
use jito_geyser_protos::solana::geyser::{
    geyser_client::GeyserClient, SubscribeBlockUpdatesRequest, TimestampedBlockUpdate,
};
use tokio_stream::StreamExt;
use tonic::{codegen::InterceptedService, transport::Channel, IntoRequest};

pub struct BlockCollector {
    geyser_client: GeyserClient<InterceptedService<Channel, GrpcInterceptor>>,
}

impl BlockCollector {
    pub fn new(geyser_client: GeyserClient<InterceptedService<Channel, GrpcInterceptor>>) -> Self {
        Self { geyser_client }
    }
}

#[async_trait]
impl Collector<TimestampedBlockUpdate> for BlockCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, TimestampedBlockUpdate>> {
        let stream = self
            .geyser_client
            .subscribe_block_updates(IntoRequest::into_request(SubscribeBlockUpdatesRequest {}))
            .await
            .expect("subscribes to block updates");

        let stream = stream.into_inner().filter_map(|event| match event {
            Ok(evt) => Some(evt),
            Err(_) => None,
        });

        Ok(Box::pin(stream))
    }
}
