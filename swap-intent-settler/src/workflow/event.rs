use crate::quote::quoted_intent::QuotedIntent;
use crate::types::swap_intent::SwapIntent;
use crate::workflow::executors::lock_tokens_executor::LockIntentTokensHandlerResult;
use crate::workflow::executors::swap_intent_filler_executor::SwapIntentFillerHandlerResult;
use solver_common::types::intent_id::IntentId;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewSwapIntent(SwapIntent),
    IntentQuoted(QuotedIntent),
    IntentPlaced(),

    TokensLockedOnSourceChain(LockIntentTokensHandlerResult),
    IntentFilledOnDestination(SwapIntentFillerHandlerResult),

    ProvedTokensLockedOnSourceChain(IntentId),
    ProvedSwapIntentFilledOnDestinationChain(IntentId),
}
