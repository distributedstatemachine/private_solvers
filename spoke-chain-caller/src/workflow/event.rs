use crate::types::spoke_chain_call::SpokeChainCall;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    IntentMatch(SpokeChainCall),
    IntentMatched(),
    BidIntentConfirmed()
}
