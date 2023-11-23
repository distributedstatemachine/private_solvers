use crate::quote::quoted_intent::QuotedIntent;
use crate::types::swap_intent::SwapIntent;
use ethers::types::H256;

/// Core Event enum.
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub enum Event {
    NewSwapIntent(SwapIntent),
    IntentQuoted(QuotedIntent),
    TokensLocked { intent_id: H256 },
}

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    QuoteIntent(SwapIntent),
    LockTokens(QuotedIntent),
    SettleIntent(SwapIntent),
}
