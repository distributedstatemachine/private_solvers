use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::base_intent_book::BaseIntentBook;
use ethers::contract::ContractCall;
use ethers::types::Address;
use tracing::info;

use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::error::ChainError;
use solver_common::ethereum::transaction::submit_transaction;

use crate::workflow::executors::settle_intent_executor::{
    IntentSettlementData, SettleIntentHandler,
};

pub struct SendTransactionSettleIntentHandler {
    intentbook_address: Address,
    connector: Arc<Connector>,
}

impl SendTransactionSettleIntentHandler {
    pub fn new(intentbook_address: Address, connector: Arc<Connector>) -> Self {
        Self {
            intentbook_address,
            connector,
        }
    }
}

#[async_trait]
impl SettleIntentHandler for SendTransactionSettleIntentHandler {
    async fn process_settle_intent(
        &self,
        intent_settlement_data: IntentSettlementData,
    ) -> Result<()> {
        info!(?intent_settlement_data, "Settling intent");
        let transaction = self.build_settle_intent_tx(&intent_settlement_data)?;
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(?intent_settlement_data, ?tx_hash, "Intent has been settled");
        Ok(())
    }
}

impl SendTransactionSettleIntentHandler {
    fn build_settle_intent_tx(
        &self,
        intent_settlement_data: &IntentSettlementData,
    ) -> Result<ContractCall<RpcClient, ()>, ChainError> {
        let rpc_client = self.connector.get_rpc_client(ChainId::Khalani)?;
        let intentbook = BaseIntentBook::new(self.intentbook_address, rpc_client);
        let mut call = intentbook.settle_intent(intent_settlement_data.intent_id.into());
        call.tx.set_chain_id(Into::<u32>::into(ChainId::Khalani));
        Ok(call)
    }
}
