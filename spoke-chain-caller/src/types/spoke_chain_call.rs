use ethers::types::{Address, U256, H256};

pub type IntentId = H256;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpokeChainCall {
    pub intent_id: IntentId,
    pub token: Address,
    pub amount: U256,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntentMatch {
    pub intent_id: IntentId,
    pub token: Address,
    pub amount: U256,
}