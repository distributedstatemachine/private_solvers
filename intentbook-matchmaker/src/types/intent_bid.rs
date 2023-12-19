use cross_chain_market_maker::types::limit_order_intent_bid::LimitOrderIntentBid;
use solver_common::types::intent_id::{IntentBidId, IntentId};
use spoke_chain_caller::types::spoke_chain_call_bid::SpokeChainCallBid;
use swap_intent_settler::types::swap_intent_bid::SwapIntentBid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntentBid {
    SpokeChainCallBid(SpokeChainCallBid),
    LimitOrderBid(LimitOrderIntentBid),
    SwapIntentBid(SwapIntentBid),
}

impl IntentBid {
    pub fn intent_id(&self) -> IntentId {
        match self {
            IntentBid::SpokeChainCallBid(spoke_chain_caller) => spoke_chain_caller.intent_id,
            IntentBid::LimitOrderBid(limit_order_intent) => limit_order_intent.intent_id,
            IntentBid::SwapIntentBid(swap_intent) => swap_intent.intent_id,
        }
    }

    pub fn intent_bid_id(&self) -> IntentBidId {
        match self {
            IntentBid::SpokeChainCallBid(spoke_chain_caller) => spoke_chain_caller.intent_bid_id,
            IntentBid::LimitOrderBid(limit_order_intent) => limit_order_intent.intent_bid_id,
            IntentBid::SwapIntentBid(swap_intent) => swap_intent.intent_bid_id,
        }
    }
}
