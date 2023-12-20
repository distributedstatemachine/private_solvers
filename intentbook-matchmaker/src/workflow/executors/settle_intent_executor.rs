use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;

use crate::workflow::action::Action;
use solver_common::types::intent_id::IntentId;

#[derive(Debug, Clone)]
pub struct IntentSettlementData {
    pub intent_id: IntentId,
}

#[async_trait]
pub trait SettleIntentHandler {
    async fn process_settle_intent(&self, settlement_data: IntentSettlementData) -> Result<()>;
}

pub struct SettleIntentExecutor<H: SettleIntentHandler>(H);

impl<H: SettleIntentHandler> SettleIntentExecutor<H> {
    pub fn new(handler: H) -> Self {
        SettleIntentExecutor(handler)
    }
}

#[async_trait]
impl<H: SettleIntentHandler + Send + Sync> Executor<Action> for SettleIntentExecutor<H> {
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::Settle(intent_settlement_data) => {
                self.0.process_settle_intent(intent_settlement_data).await
            }
            _ => Ok(()),
        }
    }
}
