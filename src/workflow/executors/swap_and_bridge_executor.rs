use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;

use crate::quote::quoted_intent::QuotedIntent;
use crate::workflow::action::Action;

#[async_trait]
pub trait SwapAndBridgeHandler {
    async fn swap_and_bridge(&self, quoted_intent: QuotedIntent) -> Result<()>;
}

pub struct SwapAndBridgeExecutor<H: SwapAndBridgeHandler>(H);

impl<H: SwapAndBridgeHandler> SwapAndBridgeExecutor<H> {
    pub fn new(handler: H) -> Self {
        SwapAndBridgeExecutor(handler)
    }
}

#[async_trait]
impl<H: SwapAndBridgeHandler + Sync + Send> Executor<Action> for SwapAndBridgeExecutor<H> {
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::SwapAndBridge(quoted_intent) => self.0.swap_and_bridge(quoted_intent).await,
            _ => Ok(()),
        }
    }
}
