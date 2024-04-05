use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use jito_geyser_client::interceptor::GrpcInterceptor;
use jito_geyser_protos::solana::geyser::{
    geyser_client::GeyserClient, SubscribeSlotUpdateRequest, TimestampedSlotUpdate,
};
use tokio_stream::StreamExt;
use tonic::{codegen::InterceptedService, transport::Channel, IntoRequest};

pub struct SlotCollector {
    geyser_client: GeyserClient<InterceptedService<Channel, GrpcInterceptor>>,
}

impl SlotCollector {
    pub fn new(geyser_client: GeyserClient<InterceptedService<Channel, GrpcInterceptor>>) -> Self {
        Self { geyser_client }
    }
}

#[async_trait]
impl Collector<TimestampedSlotUpdate> for SlotCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, TimestampedSlotUpdate>> {
        let stream = self
            .geyser_client
            .subscribe_slot_updates(IntoRequest::into_request(SubscribeSlotUpdateRequest {}))
            .await
            .expect("subscribes to slot stream");

        let stream = stream.into_inner().filter_map(|event| match event {
            Ok(evt) => Some(evt),
            Err(_) => None,
        });

        Ok(Box::pin(stream))
    }
}
