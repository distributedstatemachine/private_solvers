use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use bindings_khalani::intents_mempool::IntentsMempool;
use ethers::middleware::Middleware;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Address, U64};
use std::sync::Arc;
use tracing::info;

use crate::strategies::types::Action;
use crate::types::swap_intent::SwapIntent;

pub struct IntentsExecutor<M: Middleware> {
    sender_provider: Arc<M>,
    intents_mempool_address: Address,
}

impl<M: Middleware> IntentsExecutor<M> {
    pub fn new(sender_provider: Arc<M>, intents_mempool_address: Address) -> Self {
        Self {
            sender_provider: sender_provider.clone(),
            intents_mempool_address,
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
        }
    }
}

impl<M> IntentsExecutor<M>
where
    M: Middleware,
    M::Error: 'static,
{
    async fn process_settle_intent(&self, swap_intent: SwapIntent) -> Result<()> {
        info!("Settling intent {:?}", swap_intent);
        let transaction = self.build_settle_intent_tx(&swap_intent).await?;
        // TODO: handle failing and forever pending transactions.
        let result = self
            .sender_provider
            .send_transaction(transaction, None)
            .await?;
        let tx_hash = result.tx_hash();
        info!(
            "Intent {:?} is settled in transaction {:?}",
            &swap_intent.intent_id, tx_hash
        );
        Ok(())
    }

    async fn build_settle_intent_tx(&self, swap_intent: &SwapIntent) -> Result<TypedTransaction> {
        let chain_id: U64 = self.sender_provider.get_chainid().await?.as_u64().into();
        let intents_mempool =
            IntentsMempool::new(self.intents_mempool_address, self.sender_provider.clone());
        let mut call = intents_mempool.settle_intent(swap_intent.intent_id.0);
        Ok(call.tx.set_chain_id(chain_id).clone())
    }
}
