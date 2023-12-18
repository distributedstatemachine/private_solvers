use crate::workflow::executors::settle_intent_executor::IntentSettlementData;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    Settle(IntentSettlementData),
}
