use crate::quote::quoted_intent::QuotedIntent;
use crate::types::intent_id::IntentId;
use crate::types::swap_intent::SwapIntent;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewSwapIntent(SwapIntent),
    IntentQuoted(QuotedIntent),
    TokensLocked { intent_id: IntentId },
}
