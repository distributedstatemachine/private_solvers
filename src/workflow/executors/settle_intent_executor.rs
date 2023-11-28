use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;

use crate::types::swap_intent::SwapIntent;
use crate::workflow::action::Action;

#[async_trait]
pub trait SettleIntentHandler {
    async fn process_settle_intent(&self, swap_intent: SwapIntent) -> Result<()>;
}

pub struct SettleIntentExecutor<H: SettleIntentHandler>(H);

impl<H: SettleIntentHandler> SettleIntentExecutor<H> {
    pub fn new(handler: H) -> Self {
        SettleIntentExecutor(handler)
    }
}

#[async_trait]
impl<H: SettleIntentHandler + Send + Sync> Executor<Action> for SettleIntentExecutor<H> {
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::SettleIntent(swap_intent) => self.0.process_settle_intent(swap_intent).await,
            _ => Ok(()),
        }
    }
}
