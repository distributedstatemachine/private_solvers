use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bindings_khalani::swap_intent_filler::{FillFilter, SwapIntentFiller};
use ethers::contract::{parse_log, ContractCall};
use tracing::info;

use crate::quote::quoted_intent::QuotedIntent;
use crate::workflow::executors::swap_intent_filler_executor::{
    SwapIntentFillerHandler, SwapIntentFillerHandlerResult,
};
use solver_common::config::addresses::AddressesConfig;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::error::ConfigError;
use solver_common::ethereum::transaction::submit_transaction;

pub struct SendTransactionSwapIntentFillerHandler {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
}

impl SendTransactionSwapIntentFillerHandler {
    pub fn new(addresses_config: AddressesConfig, connector: Arc<Connector>) -> Self {
        Self {
            addresses_config,
            connector,
        }
    }
}

#[async_trait]
impl SwapIntentFillerHandler for SendTransactionSwapIntentFillerHandler {
    async fn fill_swap_intent(
        &self,
        quoted_intent: QuotedIntent,
    ) -> Result<SwapIntentFillerHandlerResult> {
        info!(
            ?quoted_intent,
            "Filling the swap intent on the destination chain"
        );
        let transaction = self.build_fill_swap_intent_tx(&quoted_intent)?;
        let receipt: ethers::types::TransactionReceipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(?quoted_intent, ?tx_hash, "Swap intent has been filled");
        let fill_event: FillFilter = receipt
            .logs
            .iter()
            .find_map(|log| parse_log(log.clone()).ok())
            .ok_or_else(|| anyhow!("Failed to parse 'Fill' event from receipt {}", tx_hash))?;

        Ok(SwapIntentFillerHandlerResult {
            quoted_intent,
            fill_tx_hash: tx_hash,
            fill_timestamp: fill_event.fill_time_stamp,
            fill_amount: fill_event.fill_amount,
            filler: fill_event.filler,
        })
    }
}

impl SendTransactionSwapIntentFillerHandler {
    fn build_fill_swap_intent_tx(
        &self,
        quoted_intent: &QuotedIntent,
    ) -> Result<ContractCall<RpcClient, ()>> {
        let destination_chain_id = quoted_intent.swap_intent.destination_chain_id;
        let rpc_client = self.connector.get_rpc_client(destination_chain_id)?;
        let swap_intent_filler_address = self
            .addresses_config
            .swap_intent_fillers
            .get(&destination_chain_id)
            .ok_or_else(|| {
                ConfigError::ContractAddressNotFound(
                    String::from("Swap intent filler"),
                    destination_chain_id.into(),
                )
            })?;
        let swap_intent_filler = SwapIntentFiller::new(*swap_intent_filler_address, rpc_client);
        let solver_address = self.connector.get_address();
        let mut call = swap_intent_filler.fill_swap_intent(
            quoted_intent.swap_intent.clone().into(),
            solver_address,
            quoted_intent.destination_amount.base_units,
        );
        call.tx.set_chain_id(destination_chain_id as u64);
        Ok(call)
    }
}
