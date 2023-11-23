use crate::quote::intent_quoter::IntentQuoter;
use crate::quote::quoted_intent::QuotedIntent;
use crate::types::swap_intent::SwapIntent;
use crate::workflow::action::Action;
use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use tracing::info;

pub struct QuoterExecutor<Q: IntentQuoter> {
    quoter: Q,
    quoted_intents_sender: Sender<QuotedIntent>,
}

impl<Q: IntentQuoter> QuoterExecutor<Q> {
    pub fn new(quoter: Q, quoted_intents_sender: Sender<QuotedIntent>) -> Self {
        Self {
            quoter,
            quoted_intents_sender,
        }
    }
}

#[async_trait]
impl<Q: IntentQuoter + Send + Sync> Executor<Action> for QuoterExecutor<Q> {
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::QuoteIntent(swap_intent) => self.quote_intent(swap_intent).await,
            _ => Ok(()),
        }
    }
}

impl<Q: IntentQuoter> QuoterExecutor<Q> {
    async fn quote_intent(&self, swap_intent: SwapIntent) -> Result<()> {
        info!(?swap_intent, "Quoting intent");
        let quoted_intent = self.quoter.quote_intent(swap_intent).await?;
        self.quoted_intents_sender.send(quoted_intent).await?;
        Ok(())
    }
}
