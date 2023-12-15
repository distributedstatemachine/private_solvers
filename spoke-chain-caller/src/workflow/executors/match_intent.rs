use crate::types::spoke_chain_call::SpokeChainCall;
use crate::workflow::action::Action;
use crate::workflow::collectors::action_confirmation_collector::ActionConfirmationCollector;
use crate::workflow::event::Event;
use anyhow::Result;
use artemis_core::types::{Collector, CollectorMap, Executor};
use async_trait::async_trait;
use ethers::types::TxHash;
use tokio::sync::mpsc::{channel, Sender};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchIntentHandlerResult {
    pub spoke_chain_call: SpokeChainCall,
    pub matching_tx_hash: TxHash,
}

#[async_trait]
pub trait MatchIntentHandler {
    async fn match_intent(&self, spoke_chain_call: SpokeChainCall) -> Result<MatchIntentHandlerResult>;
}

pub struct MatchIntentExecutor<H: MatchIntentHandler> {
    handler: H,
    confirmation_sender: Sender<MatchIntentHandlerResult>,
}

impl<H: MatchIntentHandler> MatchIntentExecutor<H> {
    pub fn new(handler: H) -> (Self, Box<dyn Collector<Event>>) {
        let (confirmation_sender, confirmation_receiver) = channel(512);
        let action_confirmation_collector =
            Box::new(ActionConfirmationCollector::new(confirmation_receiver));
        let action_confirmation_collector: Box<dyn Collector<Event>> = Box::new(
            CollectorMap::new(action_confirmation_collector, |intent| {
                Event::IntentMatched(intent)
            }),
        );
        (
            MatchIntentExecutor {
                handler,
                confirmation_sender,
            },
            action_confirmation_collector,
        )
    }
}

#[async_trait]
impl<H: MatchIntentHandler + Sync + Send> Executor<Action> for MatchIntentExecutor<H> {
    async fn execute(&self, action: Action) -> Result<()> {
        if let Action::MatchIntentOnSpokeChain(spoke_chain_call) = action {
            let match_intent_handler_result = self.handler.match_intent(spoke_chain_call).await?;
            self.confirmation_sender
                .send(match_intent_handler_result)
                .await?;
        }
        Ok(())
    }
}
