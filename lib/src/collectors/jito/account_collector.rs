use std::str::FromStr;

use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use jito_geyser_client::interceptor::GrpcInterceptor;
use jito_geyser_protos::solana::geyser::{
    geyser_client::GeyserClient, SubscribeAccountUpdatesRequest, TimestampedAccountUpdate,
};
use solana_sdk::pubkey::Pubkey;
use tokio_stream::StreamExt;
use tonic::{codegen::InterceptedService, transport::Channel};

pub struct AccountCollector {
    geyser_client: GeyserClient<InterceptedService<Channel, GrpcInterceptor>>,
    accounts: Vec<String>,
}

impl AccountCollector {
    pub fn new(
        geyser_client: GeyserClient<InterceptedService<Channel, GrpcInterceptor>>,
        accounts: &Vec<String>,
    ) -> Self {
        Self {
            geyser_client,
            accounts: accounts.clone(),
        }
    }
}

#[async_trait]
impl Collector<TimestampedAccountUpdate> for AccountCollector {
    async fn get_event_stream(&mut self) -> Result<CollectorStream<'_, TimestampedAccountUpdate>> {
        let stream = self
            .geyser_client
            .subscribe_account_updates(SubscribeAccountUpdatesRequest {
                accounts: self
                    .accounts
                    .iter()
                    .map(|a| Pubkey::from_str(a).unwrap().to_bytes().to_vec())
                    .collect(),
            })
            .await
            .expect("subscribe to geyser");

        let stream = stream.into_inner().filter_map(|evt| match evt {
            Ok(evt) => Some(evt),
            Err(_) => None,
        });

        Ok(Box::pin(stream))
    }
}
