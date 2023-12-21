use anyhow::Result;
use bindings_khalani::spoke_chain_call_intent_book::SpokeChainCall as ContractSpokeChainCall;
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::types::{Address, Bytes, H256, U256};
use std::sync::Arc;

use crate::types::intent::calculate_intent_id;
use solver_common::config::chain::ChainId;
use solver_common::connectors::Connector;
use solver_common::types::intent_id::{IntentId, WithIntentId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpokeChainCallStub {
    pub chain_id: ChainId,
    pub contract_to_call: Address,
    pub call_data: Bytes,
    pub token: Address,
    pub amount: U256,
    pub reward_token: Address,
    pub reward_amount: U256,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpokeChainCall {
    pub intent_id: IntentId,
    pub signature: Bytes,
    pub author: Address,
    pub chain_id: ChainId,
    pub contract_to_call: Address,
    pub call_data: Bytes,
    pub token: Address,
    pub amount: U256,
    pub reward_token: Address,
    pub reward_amount: U256,
}

impl SpokeChainCall {
    pub fn create_signed(
        connector: Arc<Connector>,
        spoke_chain_call_stub: SpokeChainCallStub,
    ) -> Result<SpokeChainCall> {
        let spoke_chain_call = Self {
            intent_id: Default::default(),
            signature: Default::default(),
            author: connector.get_address(),
            chain_id: spoke_chain_call_stub.chain_id,
            contract_to_call: spoke_chain_call_stub.contract_to_call,
            call_data: spoke_chain_call_stub.call_data,
            token: spoke_chain_call_stub.token,
            amount: spoke_chain_call_stub.amount,
            reward_token: spoke_chain_call_stub.reward_token,
            reward_amount: spoke_chain_call_stub.reward_amount,
        };
        let contract_intent: bindings_khalani::base_intent_book::Intent =
            spoke_chain_call.clone().into();
        let intent_id = calculate_intent_id(contract_intent);
        let signature = Self::sign(connector.clone(), spoke_chain_call.clone())?;
        Ok(Self {
            intent_id,
            signature,
            ..spoke_chain_call
        })
    }

    fn get_eip712_hash(_spoke_chain_call: SpokeChainCall) -> H256 {
        todo!()
    }

    fn sign(_connector: Arc<Connector>, spoke_chain_call: SpokeChainCall) -> Result<Bytes> {
        let _eip712_hash = Self::get_eip712_hash(spoke_chain_call);
        todo!()
    }
}

impl TryFrom<WithIntentId<bindings_khalani::base_intent_book::Intent>> for SpokeChainCall {
    type Error = anyhow::Error;

    fn try_from(
        value: WithIntentId<bindings_khalani::base_intent_book::Intent>,
    ) -> Result<Self, Self::Error> {
        let (intent_id, value) = value;
        let contract_spoke_chain_call: ContractSpokeChainCall =
            ContractSpokeChainCall::decode(value.intent)?;
        Ok(SpokeChainCall {
            intent_id,
            signature: value.signature,
            author: contract_spoke_chain_call.author,
            chain_id: contract_spoke_chain_call.chain_id.try_into()?,
            contract_to_call: contract_spoke_chain_call.contract_to_call,
            call_data: contract_spoke_chain_call.call_data,
            token: contract_spoke_chain_call.token,
            amount: contract_spoke_chain_call.amount,
            reward_amount: contract_spoke_chain_call.reward_amount,
            reward_token: contract_spoke_chain_call.reward_token,
        })
    }
}

impl From<SpokeChainCall> for bindings_khalani::base_intent_book::Intent {
    fn from(value: SpokeChainCall) -> Self {
        let contract_spoke_chain_call: ContractSpokeChainCall = ContractSpokeChainCall {
            chain_id: value.chain_id.into(),
            author: value.author,
            call_data: value.call_data,
            contract_to_call: value.contract_to_call,
            token: value.token,
            amount: value.amount,
            reward_token: value.reward_token,
            reward_amount: value.reward_amount,
        };
        bindings_khalani::base_intent_book::Intent {
            intent: Bytes::from(contract_spoke_chain_call.encode()),
            signature: value.signature,
        }
    }
}
