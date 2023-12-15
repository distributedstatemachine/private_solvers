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
pub trait BidIntentExecutor {
    async fn bid_intent(&self, spoke_chain_call: SpokeChainCall) -> Result<BidIntentExecutorResult>;
}

pub struct BidIntentExecutorImpl<E: BidIntentExecutor> {
    executor: E,
    confirmation_sender: Sender<BidIntentExecutorResult>,
}

impl<E: BidIntentExecutor> BidIntentExecutorImpl<E> {
    pub fn new(executor: E) -> (Self, Box<dyn Collector<Event>>) {
        let (confirmation_sender, confirmation_receiver) = channel(512);
        let bid_intent_confirmation_collector =
            Box::new(ActionConfirmationCollector::new(confirmation_receiver));
        let bid_intent_confirmation_collector: Box<dyn Collector<Event>> = Box::new(
            CollectorMap::new(bid_intent_confirmation_collector, |event| {
                Event::BidIntentConfirmed(event)
            }),
        );
        (
            BidIntentExecutorImpl {
                executor,
                confirmation_sender,
            },
            bid_intent_confirmation_collector,
        )
    }
}

#[async_trait]
impl<E: BidIntentExecutor + Sync + Send> Executor<Event, Action> for BidIntentExecutorImpl<E> {
    async fn execute(&self, event: &Event) -> Result<Action> {
        match event {
            Event::SpokeChainCall(spoke_chain_call) => {
                if some_condition {
                    let bid_intent_executor_result = self.executor.bid_intent(spoke_chain_call.clone()).await?;
                    self.confirmation_sender
                        .send(bid_intent_executor_result)
                        .await?;
                    Ok(Action::BidIntentConfirmed(bid_intent_executor_result.spoke_chain_call))
                } else {
                    Ok(Action::NoAction)
                }
            }
            _ => {
                // Handle other event types or return an appropriate action
                Ok(Action::NoAction)
            }
        }
    }
}
