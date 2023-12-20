use bindings_khalani::shared_types::Intent as ContractIntent;
use bindings_khalani::shared_types::SwapIntent as ContractSwapIntent;
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::types::{Address, Bytes, U256};
use solver_common::config::chain::ChainId;
use solver_common::types::intent_id::{IntentId, WithIntentId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwapIntent {
    pub intent_id: IntentId,
    pub author: Address,
    pub signature: Bytes,
    pub source_chain_id: ChainId,
    pub destination_chain_id: ChainId,
    pub source_token: Address,
    pub destination_token: Address,
    pub source_amount: U256,
    pub source_permit_2: Bytes,
    pub deadline: U256,
    pub nonce: U256,
}

impl TryFrom<WithIntentId<ContractIntent>> for SwapIntent {
    type Error = anyhow::Error;

    fn try_from(value: WithIntentId<ContractIntent>) -> Result<Self, Self::Error> {
        let (intent_id, intent) = value;
        let value: ContractSwapIntent = ContractSwapIntent::decode(intent.intent)?;
        Ok(SwapIntent {
            intent_id,
            author: value.author,
            signature: intent.signature,
            source_chain_id: value.source_chain_id.try_into()?,
            destination_chain_id: value.destination_chain_id.try_into()?,
            source_token: value.source_token,
            destination_token: value.destination_token,
            source_amount: value.source_amount,
            source_permit_2: value.source_permit_2,
            deadline: value.deadline,
            nonce: value.nonce,
        })
    }
}

impl From<SwapIntent> for ContractSwapIntent {
    fn from(value: SwapIntent) -> Self {
        Self {
            author: value.author,
            signature: value.signature,
            source_chain_id: value.source_chain_id.into(),
            destination_chain_id: value.destination_chain_id.into(),
            source_token: value.source_token,
            destination_token: value.destination_token,
            source_amount: value.source_amount,
            source_permit_2: value.source_permit_2,
            deadline: value.deadline,
            nonce: value.nonce,
        }
    }
}

impl From<SwapIntent> for bindings_khalani::base_intent_book::Intent {
    fn from(value: SwapIntent) -> Self {
        let contract_swap_intent: ContractSwapIntent = value.clone().into();
        bindings_khalani::base_intent_book::Intent {
            intent: Bytes::from(AbiEncode::encode(contract_swap_intent)),
            signature: value.signature,
        }
    }
}
