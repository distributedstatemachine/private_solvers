use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::escrow::Escrow;
use ethers::contract::ContractCall;
use tracing::info;

use crate::types::spoke_chain_call::SpokeChainCall;
use crate::workflow::executors::match_intent_executor::{
    MatchIntentHandler, MatchIntentHandlerResult,
};
use solver_common::config::addresses::AddressesConfig;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::error::ConfigError;
use solver_common::ethereum::transaction::submit_transaction;

pub struct SendTransactionMatchIntentHandler {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
}

impl SendTransactionMatchIntentHandler {
    pub fn new(addresses_config: AddressesConfig, connector: Arc<Connector>) -> Self {
        Self {
            addresses_config,
            connector,
        }
    }
}

#[async_trait]
impl MatchIntentHandler for SendTransactionMatchIntentHandler {
    async fn match_intent(&self, spoke_chain_call: SpokeChainCall) -> Result<MatchIntentHandlerResult> {
        info!(?spoke_chain_call, "Matching intent");
        let transaction = self.build_bid_intent_tx(&spoke_chain_call)?;
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(?spoke_chain_call, ?tx_hash, "Intent have been matched");
        Ok(MatchIntentHandlerResult {
            spoke_chain_call,
            matching_tx_hash: tx_hash,
        })
    }
}

impl SendTransactionMatchIntentHandler {
    fn build_bid_intent_tx(
        &self,
        spoke_chain_call: &SpokeChainCall,
    ) -> Result<ContractCall<RpcClient, ()>> {
        let source_chain_id = spoke_chain_call.source_chain_id;
        let rpc_client = self.connector.get_rpc_client(source_chain_id)?;
        let intentbook_address = self
            .addresses_config
            .intentbook_address;
        let intentbook = Intentbook::new(intentbook_address, rpc_client);
        let mut call = intentbook.match_intent(spoke_chain_call.clone().into());
        call.tx.set_chain_id(Into::<u32>::into(source_chain_id));
        Ok(call)
    }
}
