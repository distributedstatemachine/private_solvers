use crate::workflow::action::Action;
use crate::workflow::event::Event;
use anyhow::Result;
use artemis_core::types::{Collector, CollectorMap, Executor};
use async_trait::async_trait;
use ethers::types::TxHash;
use solver_common::types::limit_order_intent::LimitOrderIntent;
use solver_common::workflow::action_confirmation_collector::ActionConfirmationCollector;
use tokio::sync::mpsc::{channel, Sender};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostLimitOrderHandlerResult {
    pub limit_order_intent: LimitOrderIntent,
    pub limit_order_post_tx_hash: TxHash,
}

#[async_trait]
pub trait PostLimitOrderHandler {
    async fn post_limit_order(
        &self,
        limit_order_intent: LimitOrderIntent,
    ) -> Result<PostLimitOrderHandlerResult>;
}

pub struct PostLimitOrderExecutor<H: PostLimitOrderHandler> {
    handler: H,
    confirmation_sender: Sender<PostLimitOrderHandlerResult>,
}

impl<H: PostLimitOrderHandler> PostLimitOrderExecutor<H> {
    pub fn new(handler: H) -> (Self, Box<dyn Collector<Event>>) {
        let (confirmation_sender, confirmation_receiver) =
            channel::<PostLimitOrderHandlerResult>(512);
        let fill_action_confirmation_collector =
            Box::new(ActionConfirmationCollector::new(confirmation_receiver));
        let fill_action_confirmation_collector: Box<dyn Collector<Event>> =
            Box::new(CollectorMap::new(
                fill_action_confirmation_collector,
                |handler_result: PostLimitOrderHandlerResult| {
                    Event::NewLimitOrderIntent(handler_result.limit_order_intent)
                },
            ));
        (
            PostLimitOrderExecutor {
                handler,
                confirmation_sender,
            },
            fill_action_confirmation_collector,
        )
    }
}

#[async_trait]
impl<H: PostLimitOrderHandler + Sync + Send> Executor<Action> for PostLimitOrderExecutor<H> {
    async fn execute(&self, action: Action) -> Result<()> {
        let Action::PostLimitOderIntent(limit_order_intent) = action;
        let post_limit_order_handler_result = self
            .handler
            .post_limit_order(limit_order_intent.clone())
            .await?;
        self.confirmation_sender
            .send(PostLimitOrderHandlerResult {
                limit_order_intent,
                limit_order_post_tx_hash: post_limit_order_handler_result.limit_order_post_tx_hash,
            })
            .await?;
        Ok(())
    }
}
