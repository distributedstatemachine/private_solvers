use crate::types::limit_order_intent::LimitOrderIntent;
use crate::types::spoke_chain_call::SpokeChainCall;
use crate::types::swap_intent::SwapIntent;
use ethers::abi::{encode_packed, Token};
use ethers::utils::keccak256;
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

impl From<Intent> for bindings_khalani::base_intent_book::Intent {
    fn from(value: Intent) -> Self {
        match value {
            Intent::SpokeChainCall(spoke_chain_call) => spoke_chain_call.into(),
            Intent::LimitOrder(limit_order_intent) => limit_order_intent.into(),
            Intent::SwapIntent(swap_intent) => swap_intent.into(),
        }
    }
}

pub fn calculate_intent_id(intent: bindings_khalani::base_intent_book::Intent) -> IntentId {
    keccak256(
        // TODO[solidity]: ensure this encoding is exactly what Solidity returns (write a test).
        encode_packed(&[
            Token::Bytes(intent.intent.to_vec()),
            Token::Bytes(intent.signature.to_vec()),
        ])
        .unwrap(),
    )
    .into()
}
