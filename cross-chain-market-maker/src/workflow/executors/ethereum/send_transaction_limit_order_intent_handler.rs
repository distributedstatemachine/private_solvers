use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::limit_order_intent_book::LimitOrderIntentBook;
use ethers::contract::ContractCall;
use tracing::info;

use solver_common::config::addresses::AddressesConfig;
use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::ethereum::transaction::submit_transaction;

use crate::types::limit_order_intent::LimitOrderIntent;
use crate::workflow::executors::post_limit_order_executor::{
    PostLimitOrderHandler, PostLimitOrderHandlerResult,
};

pub struct SendTransactionPostLimitOrderHandler {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
}

impl SendTransactionPostLimitOrderHandler {
    pub fn new(addresses_config: AddressesConfig, connector: Arc<Connector>) -> Self {
        Self {
            addresses_config,
            connector,
        }
    }
}

#[async_trait]
impl PostLimitOrderHandler for SendTransactionPostLimitOrderHandler {
    async fn post_limit_order(
        &self,
        limit_order_intent: LimitOrderIntent,
    ) -> Result<PostLimitOrderHandlerResult> {
        info!(
            ?limit_order_intent,
            "Filling the swap intent on the destination chain"
        );
        let transaction = self.build_post_limit_order_intent_tx(&limit_order_intent)?;
        let receipt: ethers::types::TransactionReceipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(?limit_order_intent, ?tx_hash, "Limit Order has been posted");

        Ok(PostLimitOrderHandlerResult {
            limit_order_intent,
            limit_order_post_tx_hash: tx_hash,
        })
    }
}

impl SendTransactionPostLimitOrderHandler {
    fn build_post_limit_order_intent_tx(
        &self,
        limit_order_intent: &LimitOrderIntent,
    ) -> Result<ContractCall<RpcClient, [u8; 32]>> {
        let rpc_client = self.connector.get_rpc_client(ChainId::Khalani)?;
        let intent_book_address = self
            .addresses_config
            .intentbook_addresses
            .limit_order_intentbook;
        let intent_book = LimitOrderIntentBook::new(intent_book_address, rpc_client);
        let mut call = intent_book.place_intent(limit_order_intent.clone().into());
        call.tx.set_chain_id(Into::<u32>::into(ChainId::Khalani));
        Ok(call)
    }
}
