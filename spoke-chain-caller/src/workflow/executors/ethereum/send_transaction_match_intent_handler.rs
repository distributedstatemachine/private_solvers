use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::shared_types::IntentBid;
use bindings_khalani::spoke_chain_call_intent_book::{SpokeChainCallBid, SpokeChainCallIntentBook};
use ethers::abi::AbiEncode;
use ethers::contract::ContractCall;
use ethers::types::Bytes;
use tracing::info;

use crate::workflow::executors::match_intent_executor::{
    MatchIntentHandler, MatchIntentHandlerResult,
};
use intentbook_matchmaker::types::spoke_chain_call::SpokeChainCall;
use solver_common::config::addresses::AddressesConfig;
use solver_common::connectors::{Connector, RpcClient};
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
    async fn match_intent(
        &self,
        spoke_chain_call: SpokeChainCall,
    ) -> Result<MatchIntentHandlerResult> {
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
        let rpc_client = self.connector.get_rpc_client(spoke_chain_call.chain_id)?;
        let intentbook_address = self.addresses_config.intentbook_addresses.clone();
        let intentbook = SpokeChainCallIntentBook::new(
            intentbook_address.spoke_chain_call_intentbook,
            rpc_client,
        );
        let spoke_chain_call_bid = SpokeChainCallBid {
            caller: self.connector.get_address(),
        };
        let intent_bid = IntentBid {
            intent_id: spoke_chain_call.intent_id.into(),
            bid: Bytes::from(spoke_chain_call_bid.encode()),
        };
        let mut call = intentbook.match_intent(intent_bid);
        call.tx
            .set_chain_id(Into::<u32>::into(spoke_chain_call.chain_id));
        Ok(call)
    }
}
