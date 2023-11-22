use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use bindings_khalani::escrow::Escrow;
use bindings_khalani::intents_mempool::IntentsMempool;
use ethers::contract::ContractCall;
use tracing::info;

use crate::config::addresses::AddressesConfig;
use crate::connectors::connector::{Connector, RpcClient};
use crate::ethereum::transaction::submit_transaction;
use crate::quote::quoted_intent::QuotedIntent;
use crate::strategies::types::Action;
use crate::types::swap_intent::SwapIntent;

pub struct IntentsExecutor {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
}

impl IntentsExecutor {
    pub fn new(addresses_config: AddressesConfig, connector: Arc<Connector>) -> Self {
        Self {
            addresses_config,
            connector,
        }
    }
}

#[async_trait]
impl Executor<Action> for IntentsExecutor {
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::SettleIntent(swap_intent) => self.process_settle_intent(swap_intent).await,
            Action::LockTokens(quoted_intent) => self.lock_tokens(quoted_intent).await,
            Action::QuoteIntent(_) => Ok(()),
        }
    }
}

impl IntentsExecutor {
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

    async fn process_settle_intent(&self, swap_intent: SwapIntent) -> Result<()> {
        info!(?swap_intent, "Settling intent");
        let transaction = self.build_settle_intent_tx(&swap_intent);
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(
            ?swap_intent,
            %tx_hash,
            "Intent has been settled"
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

    fn build_settle_intent_tx(&self, swap_intent: &SwapIntent) -> ContractCall<RpcClient, ()> {
        let chain_id = swap_intent.source_chain_id.into();
        let rpc_client = self.connector.get_rpc_client(chain_id).unwrap();
        let intents_mempool =
            IntentsMempool::new(self.addresses_config.intents_mempool_address, rpc_client);
        let mut call = intents_mempool.settle_intent(swap_intent.intent_id.0);
        call.tx.set_chain_id(chain_id);
        call
    }
}
