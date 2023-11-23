use crate::inventory::amount::Amount;
use crate::types::swap_intent::SwapIntent;

#[derive(Debug, Clone, Default, PartialOrd, PartialEq, Eq)]
pub struct QuotedIntent {
    pub swap_intent: SwapIntent,
    pub kai_amount: Amount,
    pub destination_amount: Amount,
}
