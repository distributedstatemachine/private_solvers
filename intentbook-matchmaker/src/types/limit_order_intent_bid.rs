use bindings_khalani::limit_order_intent_book::{
    IntentBid as ContractIntentBid, LimitOrderBid as ContractLimitOrderBid,
};
use ethers::types::{Address, Bytes, U256};

use crate::types::swap_intent::{abi_decode_with_prefix, abi_encode_with_prefix};
use solver_common::types::intent_id::{IntentBidId, IntentId, WithIntentIdAndBidId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LimitOrderIntentBid {
    pub intent_id: IntentId,
    pub intent_bid_id: IntentBidId,
    pub filler: Address,
    pub volume: U256,
}

impl TryFrom<WithIntentIdAndBidId<ContractIntentBid>> for LimitOrderIntentBid {
    type Error = anyhow::Error;

    fn try_from(value: WithIntentIdAndBidId<ContractIntentBid>) -> Result<Self, Self::Error> {
        let (intent_id, intent_bid_id, intent) = value;
        let limit_order: ContractLimitOrderBid = abi_decode_with_prefix(intent.bid)?;
        Ok(Self {
            intent_id,
            intent_bid_id,
            filler: limit_order.filler,
            volume: limit_order.volume,
        })
    }
}

impl From<LimitOrderIntentBid> for ContractLimitOrderBid {
    fn from(value: LimitOrderIntentBid) -> Self {
        Self {
            filler: value.filler,
            volume: value.volume,
        }
    }
}

impl From<LimitOrderIntentBid> for ContractIntentBid {
    fn from(value: LimitOrderIntentBid) -> Self {
        let limit_order: ContractLimitOrderBid = value.clone().into();
        Self {
            intent_id: value.intent_id.into(),
            bid: Bytes::from(abi_encode_with_prefix(limit_order)),
        }
    }
}
