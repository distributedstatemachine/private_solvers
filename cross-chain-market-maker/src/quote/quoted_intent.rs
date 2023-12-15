use crate::types::limit_order_intent::LimitOrderIntent;
use solver_common::inventory::amount::Amount;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedIntent {
    pub limit_order_intent: LimitOrderIntent,
    pub maker_amount: Amount,
    pub taker_amount: Amount,
}
