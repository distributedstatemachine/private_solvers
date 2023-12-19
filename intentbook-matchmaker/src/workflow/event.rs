use crate::types::intent::Intent;
use solver_common::types::intent_id::IntentId;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewIntent(Intent),
    NewMatchedIntent(IntentId),

    ProvedSpokeChainCall(IntentId),
}
