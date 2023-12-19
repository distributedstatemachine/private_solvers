use crate::workflow::action::Action;
use crate::workflow::event::Event;
use anyhow::Result;
use artemis_core::types::{Collector, CollectorMap, Executor};
use async_trait::async_trait;
use bindings_khalani::shared_types::Intent;
use ethers::types::TxHash;
use solver_common::workflow::action_confirmation_collector::ActionConfirmationCollector;
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceIntentHandlerResult {
    pub tx_hash: TxHash,
}

#[async_trait]
pub trait PlaceIntentHandler {
    async fn post_intent(&self, intent: Intent) -> Result<PlaceIntentHandlerResult>;
}

pub struct PlaceIntentExecutor<H: PlaceIntentHandler> {
    handler: H,
    confirmation_sender: Sender<PlaceIntentHandlerResult>,
}

impl<H: PlaceIntentHandler> PlaceIntentExecutor<H> {
    pub fn new(handler: H) -> (Self, Box<dyn Collector<Event>>) {
        let (confirmation_sender, confirmation_receiver): (
            Sender<PlaceIntentHandlerResult>,
            Receiver<PlaceIntentHandlerResult>,
        ) = channel(512);
        let action_confirmation_collector =
            Box::new(ActionConfirmationCollector::new(confirmation_receiver));
        let action_confirmation_collector: Box<dyn Collector<Event>> = Box::new(CollectorMap::new(
            action_confirmation_collector,
            |_handler_result| Event::IntentPlaced(),
        ));
        (
            PlaceIntentExecutor {
                handler,
                confirmation_sender,
            },
            action_confirmation_collector,
        )
    }
}

#[async_trait]
impl<H: PlaceIntentHandler + Sync + Send> Executor<Action> for PlaceIntentExecutor<H> {
    async fn execute(&self, action: Action) -> Result<()> {
        if let Action::PlaceIntent(intent) = action {
            let match_intent_handler_result = self.handler.post_intent(intent).await?;
            self.confirmation_sender
                .send(match_intent_handler_result)
                .await?;
        }
        Ok(())
    }
}
