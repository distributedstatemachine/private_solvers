use crate::workflow::executors::settle_intent_executor::SettleIntentHandlerResult;
use solver_common::types::intent::Intent;
use solver_common::types::intent_bid::IntentBid;
use solver_common::types::intent_id::IntentId;
use solver_common::types::proof_id::ProofId;
pub use solver_common::workflow::event::Event as Common;
use std::fmt::Debug;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewIntent(Common, Intent),
    NewMatchedIntent(Common, IntentBid),
    NewProofReceived(IntentId, ProofId),
    IntentSettled(SettleIntentHandlerResult),
}
