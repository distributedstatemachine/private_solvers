use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use bindings_khalani::intents_mempool::IntentsMempool;
use ethers::contract::ContractCall;
use tracing::info;

use crate::config::addresses::AddressesConfig;
use crate::connectors::connector::{Connector, RpcClient};
use crate::ethereum::transaction::submit_transaction;
use crate::types::swap_intent::SwapIntent;
use crate::workflow::action::Action;

pub struct SettleIntentExecutor {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
}

impl SettleIntentExecutor {
    pub fn new(addresses_config: AddressesConfig, connector: Arc<Connector>) -> Self {
        Self {
            addresses_config,
            connector,
        }
    }
}

#[async_trait]
impl Executor<Action> for SettleIntentExecutor {
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::SettleIntent(quoted_intent) => {
                self.process_settle_intent(quoted_intent.swap_intent).await
            }
            _ => Ok(()),
        }
    }
}

impl SettleIntentExecutor {
    async fn process_settle_intent(&self, swap_intent: SwapIntent) -> Result<()> {
        info!(?swap_intent, "Settling intent");
        let transaction = self.build_settle_intent_tx(&swap_intent);
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(
            ?swap_intent,
            %tx_hash,
            "Intent has been settled"
        );
        Ok(())
    }

    fn build_settle_intent_tx(&self, swap_intent: &SwapIntent) -> ContractCall<RpcClient, ()> {
        let chain_id = swap_intent.source_chain_id.into();
        let rpc_client = self.connector.get_rpc_client(chain_id).unwrap();
        let intents_mempool =
            IntentsMempool::new(self.addresses_config.intents_mempool_address, rpc_client);
        let mut call = intents_mempool.settle_intent(swap_intent.intent_id.0);
        call.tx.set_chain_id(chain_id);
        call
    }
}
