use crate::types::intent_id::IntentId;
use ethers::types::{Address, Bytes, U256};
use solver_common::inventory::{amount::Amount, token::Token};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LimitOrderIntent {
    pub intent_id: IntentId,
    pub author: Address,
    pub signature: Bytes,
    pub amount: Amount,
    pub token: Token,
}


