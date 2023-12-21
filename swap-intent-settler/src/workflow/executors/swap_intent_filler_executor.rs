use anyhow::Result;
use artemis_core::types::{Collector, CollectorMap, Executor};
use async_trait::async_trait;
use ethers::types::{Address, TxHash, U256};
use solver_common::workflow::action_confirmation_collector::ActionConfirmationCollector;
use tokio::sync::mpsc::{channel, Sender};

use crate::quote::quoted_swap_intent::QuotedSwapIntent;
use crate::workflow::action::Action;
use crate::workflow::event::Event;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwapIntentFillerHandlerResult {
    pub quoted_intent: QuotedSwapIntent,
    pub fill_tx_hash: TxHash,
    pub fill_timestamp: U256,
    pub fill_amount: U256,
    pub filler: Address,
}

#[async_trait]
pub trait SwapIntentFillerHandler {
    async fn fill_swap_intent(
        &self,
        quoted_intent: QuotedSwapIntent,
    ) -> Result<SwapIntentFillerHandlerResult>;
}

pub struct SwapIntentFillerExecutor<H: SwapIntentFillerHandler> {
    handler: H,
    confirmation_sender: Sender<SwapIntentFillerHandlerResult>,
}

impl<H: SwapIntentFillerHandler> SwapIntentFillerExecutor<H> {
    pub fn new(handler: H) -> (Self, Box<dyn Collector<Event>>) {
        let (confirmation_sender, confirmation_receiver) = channel(512);
        let fill_action_confirmation_collector =
            Box::new(ActionConfirmationCollector::new(confirmation_receiver));
        let fill_action_confirmation_collector: Box<dyn Collector<Event>> =
            Box::new(CollectorMap::new(fill_action_confirmation_collector, |e| {
                Event::IntentFilledOnDestination(e)
            }));
        (
            SwapIntentFillerExecutor {
                handler,
                confirmation_sender,
            },
            fill_action_confirmation_collector,
        )
    }
}

#[async_trait]
impl<H: SwapIntentFillerHandler + Sync + Send> Executor<Action> for SwapIntentFillerExecutor<H> {
    async fn execute(&self, action: Action) -> Result<()> {
        if let Action::FillIntentOnDestinationChain(quoted_intent) = action {
            let filled_intent = self.handler.fill_swap_intent(quoted_intent).await?;
            self.confirmation_sender.send(filled_intent).await?;
        }
        return Ok(());
    }
}
