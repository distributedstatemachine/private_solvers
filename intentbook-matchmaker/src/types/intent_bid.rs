use crate::types::limit_order_intent_bid::LimitOrderIntentBid;
use crate::types::spoke_chain_call_bid::SpokeChainCallBid;
use crate::types::swap_intent_bid::SwapIntentBid;
use ethers::abi::{encode_packed, Token};
use ethers::utils::keccak256;
use solver_common::types::intent_id::{IntentBidId, IntentId};

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

impl From<IntentBid> for bindings_khalani::base_intent_book::IntentBid {
    fn from(value: IntentBid) -> Self {
        match value {
            IntentBid::SpokeChainCallBid(bid) => bid.into(),
            IntentBid::LimitOrderBid(bid) => bid.into(),
            IntentBid::SwapIntentBid(bid) => bid.into(),
        }
    }
}

pub fn calculate_intent_bid_id(
    intent_bid: bindings_khalani::base_intent_book::IntentBid,
) -> IntentBidId {
    keccak256(
        // TODO[solidity]: ensure this encoding is exactly what Solidity returns (write a test).
        encode_packed(&[
            Token::FixedBytes(Vec::from(intent_bid.intent_id)),
            Token::Bytes(intent_bid.bid.to_vec()),
        ])
        .unwrap(),
    )
    .into()
}
