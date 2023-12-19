use crate::types::intent_id::IntentId;
use bindings_khalani::limit_order_intent_book::Intent;
use ethers::types::{Address, Bytes};
use solver_common::inventory::{amount::Amount, token::Token};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LimitOrderIntent {
    pub intent_id: IntentId,
    pub author: Address,
    pub signature: Bytes,
    pub amount: Amount,
    pub token: Token,
}

impl From<Intent> for LimitOrderIntent {
    fn from(_value: Intent) -> Self {
        // Implement conversion to limit order intent.
        todo!()
    }
}

impl From<LimitOrderIntent> for Intent {
    fn from(value: LimitOrderIntent) -> Self {
        Self {
            // TODO: encode LimitOrderIntent properly.
            order: value.signature.clone(),
            sig: value.signature,
        }
    }
}
