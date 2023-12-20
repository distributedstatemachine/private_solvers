use crate::types::intent::Intent;
use crate::types::intent_bid::IntentBid;
use crate::workflow::executors::settle_intent_executor::IntentSettlementData;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    MatchIntent(Intent, IntentBid),
    Settle(IntentSettlementData),
}
