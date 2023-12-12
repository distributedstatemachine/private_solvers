use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::settlement_reactor::SettlementReactor;
use ethers::contract::ContractCall;
use tracing::info;

use crate::config::addresses::AddressesConfig;
use crate::config::chain::ChainId;
use crate::connectors::{Connector, RpcClient};
use crate::error::ChainError;
use crate::ethereum::transaction::submit_transaction;
use crate::workflow::executors::settle_intent_executor::{
    SettleIntentHandler, SwapIntentSettlementData,
};

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
    async fn process_settle_intent(
        &self,
        swap_intent_settlement_data: SwapIntentSettlementData,
    ) -> Result<()> {
        info!(?swap_intent_settlement_data, "Settling intent");
        let transaction = self.build_settle_intent_tx(&swap_intent_settlement_data)?;
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(
            ?swap_intent_settlement_data,
            ?tx_hash,
            "Intent has been settled"
        );
        Ok(())
    }
}

impl SendTransactionSettleIntentHandler {
    fn build_settle_intent_tx(
        &self,
        swap_intent_settlement_data: &SwapIntentSettlementData,
    ) -> Result<ContractCall<RpcClient, ()>, ChainError> {
        let rpc_client = self.connector.get_rpc_client(ChainId::Khalani)?;
        let settlement_reactor =
            SettlementReactor::new(self.addresses_config.settlement_reactor_address, rpc_client);
        let mut call = settlement_reactor.settle(
            swap_intent_settlement_data
                .quoted_intent
                .swap_intent
                .clone()
                .into(),
            swap_intent_settlement_data.filler,
            swap_intent_settlement_data.fill_timestamp,
            swap_intent_settlement_data.fill_amount,
        );
        call.tx.set_chain_id(Into::<u32>::into(ChainId::Khalani));
        Ok(call)
    }
}
