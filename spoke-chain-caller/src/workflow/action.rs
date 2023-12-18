use crate::types::spoke_chain_call::SpokeChainCall;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    MatchIntent(SpokeChainCall),
    CallSpoke(SpokeChainCall),
}
