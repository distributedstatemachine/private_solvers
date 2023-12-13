use crate::quote::quoted_intent::QuotedIntent;
use crate::workflow::action::Action;
use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use ethers::types::{Address, U256};

#[derive(Debug, Clone)]
pub struct SwapIntentSettlementData {
    pub quoted_intent: QuotedIntent,
    pub fill_timestamp: U256,
    pub fill_amount: U256,
    pub filler: Address,
}

#[async_trait]
pub trait SettleIntentHandler {
    async fn process_settle_intent(&self, settlement_data: SwapIntentSettlementData) -> Result<()>;
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
            Action::SettleIntent(settlement_data) => {
                self.0.process_settle_intent(settlement_data).await
            }
            _ => Ok(()),
        }
    }
}
