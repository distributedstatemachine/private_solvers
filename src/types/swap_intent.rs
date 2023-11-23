use ethers::types::{Address, Bytes, H256, U256};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SwapIntent {
    pub intent_id: H256,

    pub author: Address,
    pub signature: Bytes,
    // TODO: use ChainId type.
    pub source_chain_id: u32,
    pub destination_chain_id: u32,
    pub source_token: Address,
    pub destination_token: Address,
    pub source_amount: U256,
    pub source_permit_2: Bytes,
}

impl From<bindings_khalani::shared_types::SwapIntent> for SwapIntent {
    fn from(value: bindings_khalani::abstract_request_processor::SwapIntent) -> Self {
        SwapIntent {
            // TODO: create a function to purely calculate the intent ID.
            intent_id: Default::default(),

            author: value.author,
            signature: value.signature,
            source_chain_id: value.source_chain_id,
            destination_chain_id: value.destination_chain_id,
            source_token: value.source_token,
            destination_token: value.destination_token,
            source_amount: value.source_amount,
            source_permit_2: value.source_permit_2,
        }
    }
}

// TODO: consider using a macros that would reduce this boilerplate.
impl From<SwapIntent> for bindings_khalani::shared_types::SwapIntent {
    fn from(value: SwapIntent) -> Self {
        Self {
            author: value.author,
            signature: value.signature,
            source_chain_id: value.source_chain_id,
            destination_chain_id: value.destination_chain_id,
            source_token: value.source_token,
            destination_token: value.destination_token,
            source_amount: value.source_amount,
            source_permit_2: value.source_permit_2,
        }
    }
}
