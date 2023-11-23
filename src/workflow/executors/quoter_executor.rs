use crate::quote::interchain_liquidity_hub_quoter::InterchainLiquidityHubQuoter;
use crate::quote::quoted_intent::QuotedIntent;
use crate::types::swap_intent::SwapIntent;
use crate::workflow::action::Action;
use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use tracing::info;

pub struct QuoterExecutor {
    interchain_liquidity_hub_quoter: InterchainLiquidityHubQuoter,
    quoted_intents_sender: Sender<QuotedIntent>,
}

impl QuoterExecutor {
    pub fn new(
        interchain_liquidity_hub_quoter: InterchainLiquidityHubQuoter,
        quoted_intents_sender: Sender<QuotedIntent>,
    ) -> Self {
        Self {
            interchain_liquidity_hub_quoter,
            quoted_intents_sender,
        }
    }
}

#[async_trait]
impl Executor<Action> for QuoterExecutor {
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::QuoteIntent(swap_intent) => self.quote_intent(swap_intent).await,
            _ => Ok(()),
        }
    }
}

impl QuoterExecutor {
    async fn quote_intent(&self, swap_intent: SwapIntent) -> Result<()> {
        info!(?swap_intent, "Quoting intent");
        let quoted_intent = self
            .interchain_liquidity_hub_quoter
            .quote_intent(swap_intent)
            .await?;
        self.quoted_intents_sender.send(quoted_intent).await?;
        Ok(())
    }
}
