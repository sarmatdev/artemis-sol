use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::transaction::VersionedTransaction;
use tracing::{debug, info};

pub struct SimulateExecutor {
    connection: Arc<RpcClient>,
}

#[derive(Debug, Clone)]
pub struct SimTransaction {
    pub transaction: VersionedTransaction,
}

impl SimulateExecutor {
    pub fn new(connection: Arc<RpcClient>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl Executor<SimTransaction> for SimulateExecutor {
    async fn execute(&self, action: SimTransaction) -> Result<()> {
        let res = self
            .connection
            .simulate_transaction(&action.transaction)
            .await;

        match res {
            Ok(res) => {
                info!("simulating res: {:#?}", res);
            }
            Err(e) => {
                debug!("error simulating tx: {:#?}", e);
            }
        };

        Ok(())
    }
}
