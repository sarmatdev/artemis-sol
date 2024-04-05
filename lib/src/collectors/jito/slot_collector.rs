use anyhow::Result;
use async_trait::async_trait;
use jito_geyser_client::interceptor::GrpcInterceptor;
use jito_geyser_protos::solana::geyser::{geyser_client::GeyserClient, SubscribeSlotUpdateRequest};
use tokio_stream::StreamExt;
use tonic::{codegen::InterceptedService, transport::Channel, IntoRequest};

use artemis_core::types::{Collector, CollectorStream};

pub struct SlotCollector {
    geyser_client: GeyserClient<InterceptedService<Channel, GrpcInterceptor>>,
}

#[derive(Debug, Clone, Default)]
pub struct NewSlot {
    pub slot: u64,
    pub parent_slot: Option<u64>,
    pub status: i32,
}

impl SlotCollector {
    pub fn new(geyser_client: GeyserClient<InterceptedService<Channel, GrpcInterceptor>>) -> Self {
        Self { geyser_client }
    }
}

#[async_trait]
impl Collector<NewSlot> for SlotCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, NewSlot>> {
        let stream = self
            .geyser_client
            .subscribe_slot_updates(IntoRequest::into_request(SubscribeSlotUpdateRequest {}))
            .await
            .expect("subscribes to slot stream");

        let stream = stream.into_inner().map(|msg| match msg {
            Ok(update) => match update.slot_update {
                Some(slot) => NewSlot {
                    slot: slot.slot,
                    parent_slot: slot.parent_slot,
                    status: slot.status,
                },
                None => NewSlot::default(),
            },
            Err(_) => todo!(),
        });

        Ok(Box::pin(stream))
    }
}
