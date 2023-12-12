use crate::config::chain::ChainId;
use crate::types::intent_id::IntentId;
use ethers::types::{Address, Bytes, U256};

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

impl TryFrom<bindings_khalani::shared_types::SwapIntent> for SwapIntent {
    type Error = anyhow::Error;

    fn try_from(
        value: bindings_khalani::abstract_request_processor::SwapIntent,
    ) -> Result<Self, Self::Error> {
        Ok(SwapIntent {
            // TODO: create a function to purely calculate the intent ID.
            intent_id: Default::default(),

            author: value.author,
            signature: value.signature,
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

// TODO: consider using a macros that would reduce this boilerplate.
impl From<SwapIntent> for bindings_khalani::shared_types::SwapIntent {
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
