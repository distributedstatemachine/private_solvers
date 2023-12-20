use solver_common::types::intent_id::IntentId;
use solver_common::types::proof_id::ProofId;

use crate::types::intent::Intent;
use crate::types::intent_bid::IntentBid;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewIntent(Intent),
    NewMatchedIntent(IntentBid),
    NewProofReceived(IntentId, ProofId),
}
