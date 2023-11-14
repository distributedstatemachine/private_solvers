use crate::strategies::types::Action;
use crate::types::swap_intent::SwapIntent;
use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use tracing::info;

#[derive(Debug)]
pub struct IntentsExecutor {}

impl IntentsExecutor {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Executor<Action> for IntentsExecutor {
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::LogSwapIntent(swap_intent) => self.process_log_swap_intent(swap_intent).await,
        }
    }
}

impl IntentsExecutor {
    // Process new orders as they come in.
    async fn process_log_swap_intent(&self, swap_intent: SwapIntent) -> Result<()> {
        info!("New swap intent {:?}", swap_intent);
        Ok(())
    }
}
