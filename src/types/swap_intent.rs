use ethers::types::{Address, Bytes, H256, U256};

#[derive(Clone, Debug)]
pub struct SwapIntent {
    pub intent_id: H256,

    pub author: Address,
    pub signature: Bytes,
    pub source_chain_id: u32,
    pub destination_chain_id: u32,
    pub source_token: Address,
    pub destination_token: Address,
    pub source_amount: U256,
    pub source_permit_2: Bytes,
}
