use crate::quote::quoted_intent::QuotedIntent;
use crate::types::intent_id::IntentId;
use crate::types::limit_order_intent::LimitOrderIntent;
use crate::workflow::executors::lock_tokens_executor::LockIntentTokensHandlerResult;
use crate::workflow::executors::swap_intent_filler_executor::SwapIntentFillerHandlerResult;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewLimitOrderIntent(LimitOrderIntent),
    IntentQuoted(QuotedIntent),

    TokensLockedOnSourceChain(LockIntentTokensHandlerResult),
    IntentFilledOnDestination(SwapIntentFillerHandlerResult),

    ProvedTokensLockedOnSourceChain(IntentId),
    ProvedSwapIntentFilledOnDestinationChain(IntentId),
}
