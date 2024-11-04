use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::transaction::VersionedTransaction;
use tracing::{debug, info};

pub struct SimulateExecutor {
    connection: Arc<RpcClient>,
}

#[derive(Debug, Clone)]
pub struct SendTransaction {
    pub transaction: VersionedTransaction,
    pub config: RpcSendTransactionConfig,
}

impl SimulateExecutor {
    pub fn new(connection: Arc<RpcClient>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl Executor<SendTransaction> for SimulateExecutor {
    async fn execute(&self, action: SendTransaction) -> Result<()> {
        let res = self
            .connection
            .send_transaction_with_config(&action.transaction, action.config)
            .await;

        match res {
            Ok(res) => {
                info!("tx res: {:#?}", res);
            }
            Err(e) => {
                debug!("error sending tx: {:#?}", e);
            }
        };

        Ok(())
    }
}
