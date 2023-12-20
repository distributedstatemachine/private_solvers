use bindings_khalani::spoke_chain_call_intent_book::SpokeChainCallBid as ContractSpokeChainCallBid;
use ethers::abi::AbiDecode;
use ethers::types::Address;

use solver_common::types::intent_id::{IntentBidId, IntentId, WithIntentIdAndBidId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpokeChainCallBid {
    pub intent_id: IntentId,
    pub intent_bid_id: IntentBidId,
    pub caller: Address,
}

impl TryFrom<WithIntentIdAndBidId<bindings_khalani::base_intent_book::IntentBid>>
    for SpokeChainCallBid
{
    type Error = anyhow::Error;

    fn try_from(
        value: WithIntentIdAndBidId<bindings_khalani::base_intent_book::IntentBid>,
    ) -> Result<Self, Self::Error> {
        let (intent_id, intent_bid_id, value) = value;
        let value: ContractSpokeChainCallBid = ContractSpokeChainCallBid::decode(value.bid)?;
        Ok(SpokeChainCallBid {
            intent_id,
            intent_bid_id,
            caller: value.caller,
        })
    }
}
