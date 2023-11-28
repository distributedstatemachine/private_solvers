use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bindings_khalani::swap_intent_filler::{FillFilter, SwapIntentFiller};
use ethers::contract::{parse_log, ContractCall};
use tracing::info;

use crate::config::addresses::AddressesConfig;
use crate::connectors::{Connector, RpcClient};
use crate::ethereum::transaction::submit_transaction;
use crate::quote::quoted_intent::QuotedIntent;
use crate::workflow::executors::swap_intent_filler_executor::{
    SwapIntentFillerHandler, SwapIntentFillerHandlerResult,
};

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
        let transaction = self.build_fill_swap_intent_tx(&quoted_intent);
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(?quoted_intent, ?tx_hash, "Swap intent has been filled");
        let fill_event: Option<FillFilter> = receipt
            .logs
            .iter()
            .find_map(|log| parse_log(log.clone()).ok());

        let fill_event = fill_event
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
    ) -> ContractCall<RpcClient, ()> {
        let destination_chain_id = quoted_intent.swap_intent.destination_chain_id.into();
        let rpc_client = self.connector.get_rpc_client(destination_chain_id).unwrap();
        let swap_intent_filler =
            SwapIntentFiller::new(self.addresses_config.swap_intent_filler_address, rpc_client);
        let solver_address = self.connector.get_address();
        let mut call = swap_intent_filler.fill_swap_intent(
            quoted_intent.swap_intent.clone().into(),
            solver_address,
            quoted_intent.destination_amount.base_units,
        );
        call.tx.set_chain_id(destination_chain_id);
        info!(?call.tx, "Prepared transaction");
        call
    }
}
