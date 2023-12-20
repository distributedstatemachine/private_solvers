use bindings_khalani::spoke_chain_call_intent_book::SpokeChainCallBid as ContractSpokeChainCallBid;
use ethers::abi::{encode_packed, AbiDecode, Token as AbiToken};
use ethers::types::Address;
use ethers::utils::keccak256;

use crate::types::spoke_chain_call::SpokeChainCall;
use solver_common::types::intent_id::{IntentBidId, IntentId, WithIntentIdAndBidId};
use solver_common::types::proof_id::ProofId;

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
