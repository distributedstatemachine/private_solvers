use solver_common::types::intent::Intent;
use solver_common::types::intent_bid::IntentBid;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    MatchIntent(Intent, IntentBid),
    Settle(Intent),
}
