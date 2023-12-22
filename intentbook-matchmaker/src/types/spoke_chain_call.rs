use std::sync::Arc;

use anyhow::Result;
use bindings_khalani::spoke_chain_call_intent_book::SpokeChainCall as ContractSpokeChainCall;
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::contract::{Eip712, EthAbiType};
use ethers::types::{Address, Bytes, U256};

use solver_common::config::chain::ChainId;
use solver_common::connectors::Connector;
use solver_common::types::intent_id::{IntentId, WithIntentId};

use crate::types::intent::calculate_intent_id;

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

#[derive(Debug, Clone, Eip712, EthAbiType)]
#[eip712(name = "SpokeChainCall", version = "1.0.0")]
// TODO: this has to be fixed to correspond to:
//   https://github.com/tvl-labs/khalani-protocol/blob/master/src/Intents/intentbook/lib/SpokeChainCallIntentLib.sol#L33
struct SpokeChainCall712 {
    author: Address,
    chain_id: u32,
    call_data: Bytes,
    contract_to_call: Address,
    token: Address,
    amount: U256,
}

impl SpokeChainCall {
    pub async fn create_signed(
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
        let signature = Self::sign(connector.clone(), spoke_chain_call.clone()).await?;
        Ok(Self {
            intent_id,
            signature,
            ..spoke_chain_call
        })
    }

    async fn sign(_connector: Arc<Connector>, _spoke_chain_call: SpokeChainCall) -> Result<Bytes> {
        // TODO: implement EIP712 signing. Currently, the SpokeChainCall intentbook does not validate signature.
        Ok(Bytes::default())
        /* let spoke_chain_call_712: SpokeChainCall712 = spoke_chain_call.clone().into();
        let rpc_client = connector.get_rpc_client(spoke_chain_call.chain_id)?;
        let signature = rpc_client
            .signer()
            .sign_typed_data(&spoke_chain_call_712)
            .await?;
        Ok(Bytes::from(signature.to_vec()))*/
    }
}

impl From<SpokeChainCall> for SpokeChainCall712 {
    fn from(value: SpokeChainCall) -> Self {
        Self {
            author: value.author,
            chain_id: value.chain_id.into(),
            contract_to_call: value.contract_to_call,
            call_data: value.call_data,
            token: value.token,
            amount: value.amount,
        }
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