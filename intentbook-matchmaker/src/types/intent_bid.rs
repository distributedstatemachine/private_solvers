use cross_chain_market_maker::types::limit_order_intent::LimitOrderIntent;
use solver_common::types::intent_id::IntentId;
use spoke_chain_caller::types::spoke_chain_call::SpokeChainCall;
use swap_intent_settler::types::swap_intent::SwapIntent;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntentBid {
    SpokeChainCall(SpokeChainCall),
    LimitOrder(LimitOrderIntent),
    SwapIntent(SwapIntent),
}

impl IntentBid {
    pub fn id(&self) -> IntentId {
        match self {
            IntentBid::SpokeChainCall(spoke_chain_caller) => spoke_chain_caller.intent_id,
            IntentBid::LimitOrder(limit_order_intent) => limit_order_intent.intent_id,
            IntentBid::SwapIntent(swap_intent) => swap_intent.intent_id,
        }
    }
}
