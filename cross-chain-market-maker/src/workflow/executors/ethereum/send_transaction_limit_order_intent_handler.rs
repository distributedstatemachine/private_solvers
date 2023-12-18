use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bindings_khalani::swap_intent_filler::{FillFilter, SwapIntentFiller};
use ethers::contract::{parse_log, ContractCall};
use tracing::info;

use crate::quote::quoted_intent::QuotedIntent;
use crate::workflow::executors::post_limit_order_executor::{
    PostLimitOrderExecutor, PostLimitOrderHandler, PostLimitOrderHandlerResult,
};
use solver_common::config::addresses::AddressesConfig;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::error::ConfigError;
use solver_common::ethereum::transaction::submit_transaction;

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
impl PostLimtOrderHandler for SendTransactionPostLimitOrderHandler {
    async fn post_limit_order_intent(
        &self,
        quoted_intent: QuotedIntent,
    ) -> Result<PostLimitOrderHandlerResult> {
        info!(
            ?quoted_intent,
            "Filling the swap intent on the destination chain"
        );
        let transaction = self.build_limit_order_intent_tx(&quoted_intent)?;
        let receipt: ethers::types::TransactionReceipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(?quoted_intent, ?tx_hash, "Limit Order has been posted");
        let limit_order_event: LimitOrderFilter = receipt
            .logs
            .iter()
            .find_map(|log| parse_log(log.clone()).ok())
            .ok_or_else(|| anyhow!("Failed to parse 'Fill' event from receipt {}", tx_hash))?;

        Ok(PostLimitOrderHandlerResult {
            limit_order_intent: quoted_intent.limit_order_intent,
            limit_order_post_tx_hash: tx_hash,
        })
    }
}

impl SendTransactionPostLimitOrderHandler {
    fn build_post_limit_order_intent_tx(
        &self,
        quoted_intent: &QuotedIntent,
    ) -> Result<ContractCall<RpcClient, ()>> {
        let destination_chain_id = quoted_intent.limit_order_intent.destination_chain_id;
        let rpc_client = self.connector.get_rpc_client(destination_chain_id)?;
        let intent_book_address = self
            .addresses_config
            .swap_intent_fillers
            .get(&destination_chain_id)
            .ok_or_else(|| {
                ConfigError::ContractAddressNotFound(
                    String::from("IntentBook not found"),
                    destination_chain_id.into(),
                )
            })?;
        let intent_book = IntentBook::new(*intent_book_address, rpc_client);
        let solver_address = self.connector.get_address();
        let mut call = swap_intent_filler.post_limit_order_intent(
            quoted_intent.limit_order_intent.clone().into(),
            solver_address,
            quoted_intent.destination_amount.base_units,
        );
        call.tx.set_chain_id(destination_chain_id as u64);
        Ok(call)
    }
}
