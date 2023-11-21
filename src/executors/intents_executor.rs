use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use bindings_khalani::escrow::Escrow;
use bindings_khalani::intents_mempool::IntentsMempool;
use ethers::middleware::Middleware;
use ethers::prelude::H256;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::U64;
use tracing::info;

use crate::config::addresses::AddressesConfig;
use crate::strategies::types::Action;
use crate::types::swap_intent::SwapIntent;

pub struct IntentsExecutor<M: Middleware> {
    sender_provider: Arc<M>,
    addresses_config: AddressesConfig,
}

impl<M: Middleware> IntentsExecutor<M> {
    pub fn new(sender_provider: Arc<M>, addresses_config: AddressesConfig) -> Self {
        Self {
            sender_provider,
            addresses_config,
        }
    }
}

#[async_trait]
impl<M> Executor<Action> for IntentsExecutor<M>
where
    M: Middleware,
    M::Error: 'static,
{
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::SettleIntent(swap_intent) => self.process_settle_intent(swap_intent).await,
            Action::LockTokens(swap_intent) => self.lock_tokens(swap_intent).await,
        }
    }
}

impl<M> IntentsExecutor<M>
where
    M: Middleware,
    M::Error: 'static,
{
    async fn lock_tokens(&self, swap_intent: SwapIntent) -> Result<()> {
        info!(%swap_intent, "Locking source tokens of the intent");
        let transaction = self.build_settle_intent_tx(&swap_intent).await?;
        let tx_hash = self.submit_tx(transaction).await?;
        info!(
            %swap_intent,
            %tx_hash,
            "Source tokens have been locked"
        );
        Ok(())
    }

    async fn process_settle_intent(&self, swap_intent: SwapIntent) -> Result<()> {
        info!(%swap_intent, "Settling intent");
        let transaction = self.build_settle_intent_tx(&swap_intent).await?;
        let tx_hash = self.submit_tx(transaction).await?;
        info!(
            %swap_intent,
            %tx_hash,
            "Intent has been settled"
        );
        Ok(())
    }

    async fn build_lock_tokens_tx(&self, swap_intent: &SwapIntent) -> Result<TypedTransaction> {
        let chain_id: U64 = self.sender_provider.get_chainid().await?.as_u64().into();
        let escrow = Escrow::new(self.addresses_config.escrow, self.sender_provider.clone());
        let mut call = escrow.lock_tokens(swap_intent.into());
        Ok(call.tx.set_chain_id(chain_id).clone())
    }

    async fn build_settle_intent_tx(&self, swap_intent: &SwapIntent) -> Result<TypedTransaction> {
        let chain_id: U64 = self.sender_provider.get_chainid().await?.as_u64().into();
        let intents_mempool = IntentsMempool::new(
            self.addresses_config.intents_mempool_address,
            self.sender_provider.clone(),
        );
        let mut call = intents_mempool.settle_intent(swap_intent.intent_id.0);
        Ok(call.tx.set_chain_id(chain_id).clone())
    }

    async fn submit_tx(&self, transaction: TypedTransaction) -> Result<H256> {
        // TODO: handle failing and forever pending transactions.
        let result = self
            .sender_provider
            .send_transaction(transaction, None)
            .await?;
        let tx_hash = result.tx_hash();
        Ok(tx_hash)
    }
}
