use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::intentbook::Intentbook;
use ethers::contract::ContractCall;
use tracing::info;

use crate::types::spoke_chain_call::SpokeChainCall;
use crate::workflow::executors::bid_intent_executor::{
    BidIntentHandler, BidIntentExecutorResult,
};
use solver_common::config::addresses::AddressesConfig;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::ethereum::transaction::submit_transaction;

pub struct SendTransactionBidIntentHandler {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
}

impl SendTransactionBidIntentHandler {
    pub fn new(addresses_config: AddressesConfig, connector: Arc<Connector>) -> Self {
        Self {
            addresses_config,
            connector,
        }
    }
}

#[async_trait]
impl BidIntentHandler for SendTransactionBidIntentHandler {
    async fn bid_intent(&self, spoke_chain_call: SpokeChainCall) -> Result<BidIntentExecutorResult> {
        info!(?spoke_chain_call, "Bidding intent");
        let transaction = self.build_bid_intent_tx(&spoke_chain_call)?;
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(?spoke_chain_call, ?tx_hash, "Intent have been bidded");
        Ok(BidIntentExecutorResult {
            spoke_chain_call,
            bidding_tx_hash: tx_hash,
        })
    }
}

impl SendTransactionBidIntentHandler {
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
        let mut call = intentbook.lock_tokens(spoke_chain_call.clone().into());
        call.tx.set_chain_id(Into::<u32>::into(source_chain_id));
        Ok(call)
    }
}
