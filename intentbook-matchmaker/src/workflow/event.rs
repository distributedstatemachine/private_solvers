use crate::types::intent::Intent;
use crate::types::intent_bid::IntentBid;
use crate::types::proof_id::ProofId;
use solver_common::types::intent_id::IntentId;
use spoke_chain_caller::types::spoke_chain_call::SpokeChainCall;
use spoke_chain_caller::types::spoke_chain_call_bid::SpokeChainCallBid;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewIntent(Intent),
    NewMatchedIntent(IntentBid),

    ProvedSpokeChainCall(IntentId, ProofId, SpokeChainCall, SpokeChainCallBid),
}
