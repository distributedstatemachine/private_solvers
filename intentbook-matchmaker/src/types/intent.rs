use crate::types::limit_order_intent::LimitOrderIntent;
use crate::types::spoke_chain_call::SpokeChainCall;
use crate::types::swap_intent::SwapIntent;
use solver_common::types::intent_id::IntentId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Intent {
    SpokeChainCall(SpokeChainCall),
    LimitOrder(LimitOrderIntent),
    SwapIntent(SwapIntent),
}

impl Intent {
    pub fn id(&self) -> IntentId {
        match self {
            Intent::SpokeChainCall(spoke_chain_caller) => spoke_chain_caller.intent_id,
            Intent::LimitOrder(limit_order_intent) => limit_order_intent.intent_id,
            Intent::SwapIntent(swap_intent) => swap_intent.intent_id,
        }
    }
}
