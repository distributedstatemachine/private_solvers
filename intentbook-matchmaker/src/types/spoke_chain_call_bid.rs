use bindings_khalani::spoke_chain_call_intent_book::SpokeChainCallBid as ContractSpokeChainCallBid;
use ethers::abi::{encode_packed, Token as AbiToken};
use ethers::types::{Address, Bytes};
use ethers::utils::keccak256;

use crate::types::intent_bid::calculate_intent_bid_id;
use crate::types::spoke_chain_call::SpokeChainCall;
use crate::types::swap_intent::{abi_decode_with_prefix, abi_encode_with_prefix};
use solver_common::types::intent_id::{IntentBidId, IntentId, WithIntentIdAndBidId};
use solver_common::types::proof_id::ProofId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpokeChainCallBid {
    pub intent_bid_id: IntentBidId,
    pub intent_id: IntentId,
    pub caller: Address,
}

impl SpokeChainCallBid {
    pub fn new(intent_id: IntentId, caller: Address) -> Self {
        let bid = Self {
            intent_bid_id: Default::default(),
            intent_id,
            caller,
        };
        let intent_bid_id = calculate_intent_bid_id(bid.clone().into());
        Self {
            intent_bid_id,
            ..bid
        }
    }
}

impl TryFrom<WithIntentIdAndBidId<bindings_khalani::base_intent_book::IntentBid>>
    for SpokeChainCallBid
{
    type Error = anyhow::Error;

    fn try_from(
        value: WithIntentIdAndBidId<bindings_khalani::base_intent_book::IntentBid>,
    ) -> Result<Self, Self::Error> {
        let (intent_id, intent_bid_id, value) = value;
        let value: ContractSpokeChainCallBid = abi_decode_with_prefix(value.bid)?;
        Ok(SpokeChainCallBid {
            intent_id,
            intent_bid_id,
            caller: value.caller,
        })
    }
}

impl SpokeChainCallBid {
    pub fn get_expected_proofs(
        intent_id: IntentId,
        spoke_chain_call: &SpokeChainCall,
        spoke_chain_call_bid: &SpokeChainCallBid,
    ) -> anyhow::Result<Vec<ProofId>> {
        let encoded_spoke_chain_call = encode_packed(&[
            AbiToken::String(String::from("SpokeCalled")),
            AbiToken::Address(spoke_chain_call_bid.caller),
            AbiToken::FixedBytes(Vec::from(intent_id.as_bytes())),
            AbiToken::Address(spoke_chain_call.contract_to_call),
            AbiToken::Bytes(spoke_chain_call.call_data.to_vec()),
            AbiToken::Address(spoke_chain_call.token),
            AbiToken::Uint(spoke_chain_call.amount),
        ])
        .unwrap();
        Ok(vec![keccak256(encoded_spoke_chain_call.clone()).into()])
    }
}

impl From<SpokeChainCallBid> for bindings_khalani::base_intent_book::IntentBid {
    fn from(value: SpokeChainCallBid) -> Self {
        let bid: ContractSpokeChainCallBid = ContractSpokeChainCallBid {
            caller: value.caller,
        };
        Self {
            intent_id: value.intent_id.into(),
            bid: Bytes::from(abi_encode_with_prefix(bid)),
        }
    }
}
