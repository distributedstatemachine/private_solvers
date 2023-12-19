use std::sync::Arc;

use crate::workflow::executors::place_intent_executor::{
    PlaceIntentHandler, PlaceIntentHandlerResult,
};
use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::shared_types::Intent;
use bindings_khalani::spoke_chain_call_intent_book::SpokeChainCallIntentBook;
use ethers::contract::ContractCall;
use solver_common::config::addresses::AddressesConfig;
use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::ethereum::transaction::submit_transaction;

pub struct SendTransactionPlaceIntentHandler {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
}

impl SendTransactionPlaceIntentHandler {
    pub fn new(addresses_config: AddressesConfig, connector: Arc<Connector>) -> Self {
        Self {
            addresses_config,
            connector,
        }
    }
}

#[async_trait]
impl PlaceIntentHandler for SendTransactionPlaceIntentHandler {
    async fn post_intent(&self, intent: Intent) -> Result<PlaceIntentHandlerResult> {
        let transaction = self.build_place_intent_tx(intent)?;
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        Ok(PlaceIntentHandlerResult { tx_hash })
    }
}

impl SendTransactionPlaceIntentHandler {
    fn build_place_intent_tx(&self, intent: Intent) -> Result<ContractCall<RpcClient, [u8; 32]>> {
        let rpc_client = self.connector.get_rpc_client(ChainId::Sepolia)?;
        let intentbook_address = self
            .addresses_config
            .intentbook_addresses
            .spoke_chain_call_intentbook;
        let intentbook = SpokeChainCallIntentBook::new(intentbook_address, rpc_client);

        let mut call = intentbook.place_intent(intent);
        call.tx.set_chain_id(Into::<u32>::into(ChainId::Sepolia));
        Ok(call)
    }
}
