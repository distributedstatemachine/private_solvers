use crate::workflow::action::Action;
use crate::workflow::event::Event;
use anyhow::Result;
use artemis_core::types::{Collector, CollectorMap, Executor};
use async_trait::async_trait;
use ethers::types::TxHash;
use intentbook_matchmaker::types::swap_intent::SwapIntent;
use solver_common::workflow::action_confirmation_collector::ActionConfirmationCollector;
use tokio::sync::mpsc::{channel, Sender};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockIntentTokensHandlerResult {
    pub swap_intent: SwapIntent,
    pub locking_tx_hash: TxHash,
}

#[async_trait]
pub trait LockIntentTokensHandler {
    async fn lock_tokens(&self, swap_intent: SwapIntent) -> Result<LockIntentTokensHandlerResult>;
}

pub struct LockIntentTokensExecutor<H: LockIntentTokensHandler> {
    handler: H,
    confirmation_sender: Sender<LockIntentTokensHandlerResult>,
}

impl<H: LockIntentTokensHandler> LockIntentTokensExecutor<H> {
    pub fn new(handler: H) -> (Self, Box<dyn Collector<Event>>) {
        let (confirmation_sender, confirmation_receiver) = channel(512);
        let fill_action_confirmation_collector =
            Box::new(ActionConfirmationCollector::new(confirmation_receiver));
        let fill_action_confirmation_collector: Box<dyn Collector<Event>> = Box::new(
            CollectorMap::new(fill_action_confirmation_collector, |intent| {
                Event::TokensLockedOnSourceChain(intent)
            }),
        );
        (
            LockIntentTokensExecutor {
                handler,
                confirmation_sender,
            },
            fill_action_confirmation_collector,
        )
    }
}

#[async_trait]
impl<H: LockIntentTokensHandler + Sync + Send> Executor<Action> for LockIntentTokensExecutor<H> {
    async fn execute(&self, action: Action) -> Result<()> {
        if let Action::LockTokensOnSourceChain(swap_intent) = action {
            let lock_intent_tokens_handler_result = self.handler.lock_tokens(swap_intent).await?;
            self.confirmation_sender
                .send(lock_intent_tokens_handler_result)
                .await?;
        }
        Ok(())
    }
}
