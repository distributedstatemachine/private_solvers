use bindings_khalani::shared_types::IntentBid as ContractIntent;
use bindings_khalani::swap_intent_book::SwapIntentBid as ContractSwapIntentBid;
use ethers::abi::AbiDecode;
use ethers::types::{Address, U256};

use solver_common::types::intent_id::{IntentBidId, IntentId, WithIntentIdAndBidId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwapIntentBid {
    pub intent_id: IntentId,
    pub intent_bid_id: IntentBidId,
    pub filler: Address,
    pub fill_timestamp: U256,
    pub fill_amount: U256,
}

impl TryFrom<WithIntentIdAndBidId<ContractIntent>> for SwapIntentBid {
    type Error = anyhow::Error;

    fn try_from(value: WithIntentIdAndBidId<ContractIntent>) -> Result<Self, Self::Error> {
        let (intent_id, intent_bid_id, intent_bid) = value;
        let value: ContractSwapIntentBid = ContractSwapIntentBid::decode(intent_bid.bid)?;
        Ok(SwapIntentBid {
            intent_id,
            intent_bid_id,
            filler: value.filler,
            fill_amount: value.fill_amount,
            fill_timestamp: value.fill_timestamp,
        })
    }
}

impl From<SwapIntentBid> for ContractSwapIntentBid {
    fn from(value: SwapIntentBid) -> Self {
        Self {
            filler: value.filler,
            fill_amount: value.fill_amount,
            fill_timestamp: value.fill_timestamp,
        }
    }
}
