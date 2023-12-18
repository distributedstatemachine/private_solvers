use crate::types::IntentId;
use ethers::types::{Address, U256};
use solver_common::config::chain::ChainId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpokeChainCall {
    pub intent_id: IntentId,

    pub chain_id: ChainId,
    pub token: Address,
    pub amount: U256,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntentMatch {
    pub intent_id: IntentId,
    pub token: Address,
    pub amount: U256,
}
