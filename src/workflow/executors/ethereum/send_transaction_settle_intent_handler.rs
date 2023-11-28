use crate::config::addresses::AddressesConfig;
use crate::connectors::{Connector, RpcClient};
use crate::ethereum::transaction::submit_transaction;
use crate::types::swap_intent::SwapIntent;
use crate::workflow::executors::settle_intent_executor::SettleIntentHandler;
use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::intents_mempool::IntentsMempool;
use ethers::contract::ContractCall;
use std::sync::Arc;
use tracing::info;

pub struct SendTransactionSettleIntentHandler {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
}

impl SendTransactionSettleIntentHandler {
    pub fn new(addresses_config: AddressesConfig, connector: Arc<Connector>) -> Self {
        Self {
            addresses_config,
            connector,
        }
    }
}

#[async_trait]
impl SettleIntentHandler for SendTransactionSettleIntentHandler {
    async fn process_settle_intent(&self, swap_intent: SwapIntent) -> Result<()> {
        info!(?swap_intent, "Settling intent");
        let transaction = self.build_settle_intent_tx(&swap_intent);
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(?swap_intent, ?tx_hash, "Intent has been settled");
        Ok(())
    }
}

impl SendTransactionSettleIntentHandler {
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
