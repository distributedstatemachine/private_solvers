use crate::types::spoke_chain_call::SpokeChainCall;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewSpokeChainCall(SpokeChainCall),
    IntentMatch(SpokeChainCall),
    CallSpokeConfirmed(),
}
