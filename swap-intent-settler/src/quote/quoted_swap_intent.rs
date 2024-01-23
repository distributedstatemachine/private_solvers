use solver_common::inventory::amount::Amount;
use solver_common::types::swap_intent::SwapIntent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedSwapIntent {
    pub swap_intent: SwapIntent,
    pub destination_amount: Amount,
}
