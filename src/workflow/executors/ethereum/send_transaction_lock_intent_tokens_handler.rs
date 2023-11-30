use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::escrow::Escrow;
use ethers::contract::ContractCall;
use tracing::info;

use crate::config::addresses::AddressesConfig;
use crate::connectors::{Connector, RpcClient};
use crate::ethereum::transaction::submit_transaction;
use crate::types::swap_intent::SwapIntent;
use crate::workflow::executors::lock_tokens_executor::{
    LockIntentTokensHandler, LockIntentTokensHandlerResult,
};

pub struct SendTransactionLockIntentTokensHandler {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
}

impl SendTransactionLockIntentTokensHandler {
    pub fn new(addresses_config: AddressesConfig, connector: Arc<Connector>) -> Self {
        Self {
            addresses_config,
            connector,
        }
    }
}

#[async_trait]
impl LockIntentTokensHandler for SendTransactionLockIntentTokensHandler {
    async fn lock_tokens(&self, swap_intent: SwapIntent) -> Result<LockIntentTokensHandlerResult> {
        info!(?swap_intent, "Locking source tokens of the intent");
        let transaction = self.build_lock_tokens_tx(&swap_intent);
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(?swap_intent, ?tx_hash, "Source tokens have been locked");
        Ok(LockIntentTokensHandlerResult {
            swap_intent,
            locking_tx_hash: tx_hash,
        })
    }
}

impl SendTransactionLockIntentTokensHandler {
    fn build_lock_tokens_tx(&self, swap_intent: &SwapIntent) -> ContractCall<RpcClient, ()> {
        let source_chain_id = swap_intent.source_chain_id.into();
        let rpc_client = self.connector.get_rpc_client(source_chain_id).unwrap();
        let escrow_address = self.addresses_config.escrows.get(&source_chain_id).unwrap();
        let escrow = Escrow::new(*escrow_address, rpc_client);
        let mut call = escrow.lock_tokens(swap_intent.clone().into());
        call.tx.set_chain_id(source_chain_id);
        call
    }
}
