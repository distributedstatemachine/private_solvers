use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use bindings_khalani::escrow::Escrow;
use ethers::contract::ContractCall;
use tracing::info;

use crate::config::addresses::AddressesConfig;
use crate::connectors::connector::{Connector, RpcClient};
use crate::ethereum::transaction::submit_transaction;
use crate::quote::quoted_intent::QuotedIntent;
use crate::types::swap_intent::SwapIntent;
use crate::workflow::strategies::types::Action;

pub struct LockIntentTokensExecutor {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
}

impl LockIntentTokensExecutor {
    pub fn new(addresses_config: AddressesConfig, connector: Arc<Connector>) -> Self {
        Self {
            addresses_config,
            connector,
        }
    }
}

#[async_trait]
impl Executor<Action> for LockIntentTokensExecutor {
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::LockTokens(quoted_intent) => self.lock_tokens(quoted_intent).await,
            _ => Ok(()),
        }
    }
}

impl LockIntentTokensExecutor {
    async fn lock_tokens(&self, quoted_intent: QuotedIntent) -> Result<()> {
        info!(?quoted_intent, "Locking source tokens of the intent");
        let transaction = self.build_lock_tokens_tx(&quoted_intent.swap_intent);
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(
            ?quoted_intent,
            %tx_hash,
            "Source tokens have been locked"
        );
        Ok(())
    }

    fn build_lock_tokens_tx(&self, swap_intent: &SwapIntent) -> ContractCall<RpcClient, ()> {
        let chain_id = swap_intent.source_chain_id.into();
        let rpc_client = self.connector.get_rpc_client(chain_id).unwrap();
        let escrow = Escrow::new(self.addresses_config.escrow_address, rpc_client);
        let mut call = escrow.lock_tokens(swap_intent.clone().into());
        call.tx.set_chain_id(chain_id);
        call
    }
}
