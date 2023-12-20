use crate::types::spoke_chain_call_bid::SpokeChainCallBid;
use anyhow::Result;
use bindings_khalani::spoke_chain_call_intent_book::SpokeChainCall as ContractSpokeChainCall;
use ethers::abi::{encode_packed, AbiDecode, Token as AbiToken};
use ethers::types::{Address, Bytes, U256};
use ethers::utils::keccak256;
use solver_common::config::chain::ChainId;
use solver_common::types::intent_id::{IntentId, WithIntentId};
use solver_common::types::proof_id::ProofId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpokeChainCall {
    pub intent_id: IntentId,
    pub author: Address,
    pub chain_id: ChainId,
    pub contract_to_call: Address,
    pub call_data: Bytes,
    pub token: Address,
    pub amount: U256,
}

impl TryFrom<WithIntentId<bindings_khalani::base_intent_book::Intent>> for SpokeChainCall {
    type Error = anyhow::Error;

    fn try_from(
        value: WithIntentId<bindings_khalani::base_intent_book::Intent>,
    ) -> Result<Self, Self::Error> {
        let (intent_id, value) = value;
        let value: ContractSpokeChainCall = ContractSpokeChainCall::decode(value.intent)?;
        Ok(SpokeChainCall {
            intent_id,
            author: value.author,
            chain_id: value.chain_id.try_into()?,
            contract_to_call: value.contract_to_call,
            call_data: value.call_data,
            token: value.token,
            amount: value.amount,
        })
    }
}

impl SpokeChainCallBid {
    pub fn get_expected_proofs(
        intent_id: IntentId,
        spoke_chain_call: &SpokeChainCall,
        spoke_chain_call_bid: &SpokeChainCallBid,
    ) -> Result<Vec<ProofId>> {
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
