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
pub struct CallSpokeHandlerResult {
    pub spoke_chain_call: SpokeChainCall,
    pub calling_tx_hash: TxHash,
}

#[async_trait]
pub trait CallSpokeHandler {
    async fn call_spoke(&self, spoke_chain_call: SpokeChainCall) -> Result<CallSpokeHandlerResult>;
}

pub struct CallSpokeExecutor<E: CallSpokeHandler> {
    executor: E,
    confirmation_sender: Sender<CallSpokeHandlerResult>,
}

impl<E: CallSpokeHandler> CallSpokeExecutor<E> {
    pub fn new(executor: E) -> (Self, Box<dyn Collector<Event>>) {
        let (confirmation_sender, confirmation_receiver) = channel(512);
        let call_spoke_confirmation_collector =
            Box::new(ActionConfirmationCollector::new(confirmation_receiver));
        let call_spoke_confirmation_collector: Box<dyn Collector<Event>> = Box::new(
            CollectorMap::new(call_spoke_confirmation_collector, |event| {
                Event::CallSpokeConfirmed()
            }),
        );
        (
            CallSpokeExecutor {
                executor,
                confirmation_sender,
            },
            call_spoke_confirmation_collector,
        )
    }
}

#[async_trait]
impl<E: CallSpokeHandler + Sync + Send> Executor<Action> for CallSpokeExecutor<E> {
    async fn execute(&self, action: Action) -> Result<()> {
            if let Action::CallSpoke(spoke_chain_call) = action {
                let call_spoke_executor_result = self.executor.call_spoke(spoke_chain_call.clone()).await?;
                self.confirmation_sender
                    .send(call_spoke_executor_result)
                    .await?;
            }
            Ok(())
    }
}
