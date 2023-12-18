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
pub struct BidIntentExecutorResult {
    pub spoke_chain_call: SpokeChainCall,
    pub bidding_tx_hash: TxHash,
}

#[async_trait]
pub trait BidIntentHandler {
    async fn bid_intent(&self, spoke_chain_call: SpokeChainCall) -> Result<BidIntentExecutorResult>;
}

pub struct BidIntentExecutor<E: BidIntentHandler> {
    executor: E,
    confirmation_sender: Sender<BidIntentExecutorResult>,
}

impl<E: BidIntentHandler> BidIntentExecutor<E> {
    pub fn new(executor: E) -> (Self, Box<dyn Collector<Event>>) {
        let (confirmation_sender, confirmation_receiver) = channel(512);
        let bid_intent_confirmation_collector =
            Box::new(ActionConfirmationCollector::new(confirmation_receiver));
        let bid_intent_confirmation_collector: Box<dyn Collector<Event>> = Box::new(
            CollectorMap::new(bid_intent_confirmation_collector, |event| {
                Event::BidIntentConfirmed()
            }),
        );
        (
            BidIntentExecutor {
                executor,
                confirmation_sender,
            },
            bid_intent_confirmation_collector,
        )
    }
}

#[async_trait]
impl<E: BidIntentHandler + Sync + Send> Executor<Action> for BidIntentExecutor<E> {
    async fn execute(&self, action: Action) -> Result<()> {
            if let Action::BidIntent(spoke_chain_call) = action {
                let bid_intent_executor_result = self.executor.bid_intent(spoke_chain_call.clone()).await?;
                self.confirmation_sender
                    .send(bid_intent_executor_result)
                    .await?;
            }
            Ok(())
    }
}
