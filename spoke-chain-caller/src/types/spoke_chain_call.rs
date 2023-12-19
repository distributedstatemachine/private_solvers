use bindings_khalani::spoke_chain_call_intent_book::Intent as ContractIntent;
use bindings_khalani::spoke_chain_call_intent_book::SpokeChainCall as ContractSpokeChainCall;
use ethers::abi::AbiDecode;
use ethers::types::{Address, Bytes, U256};
use solver_common::config::chain::ChainId;
use solver_common::types::intent_id::IntentId;

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

impl TryFrom<ContractSpokeChainCall> for SpokeChainCall {
    type Error = anyhow::Error;

    fn try_from(value: ContractSpokeChainCall) -> Result<Self, Self::Error> {
        Ok(SpokeChainCall {
            intent_id: Default::default(),

            author: value.author,
            chain_id: value.chain_id.try_into()?,
            contract_to_call: value.contract_to_call,
            call_data: value.call_data,
            token: value.token,
            amount: value.amount,
        })
    }
}

impl TryFrom<ContractIntent> for SpokeChainCall {
    type Error = anyhow::Error;

    fn try_from(value: ContractIntent) -> Result<Self, Self::Error> {
        let spoke_chain_call = ContractIntent::decode(value.intent)?;
        spoke_chain_call.try_into()
    }
}
