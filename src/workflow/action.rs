use crate::quote::quoted_intent::QuotedIntent;
use crate::types::swap_intent::SwapIntent;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    QuoteIntent(SwapIntent),
    LockTokens(QuotedIntent),
    SettleIntent(SwapIntent),
}
