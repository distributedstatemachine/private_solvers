pub mod in_memory_state_manager;
pub mod state_manager;

use crate::quote::quoted_intent::QuotedIntent;
use crate::types::swap_intent::SwapIntent;

#[derive(Debug)]
pub enum IntentState {
    NewIntent(SwapIntent),
    IntentQuoted(QuotedIntent),
    TokensLocked(QuotedIntent),
}
