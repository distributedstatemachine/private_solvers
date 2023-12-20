use intentbook_matchmaker::types::intent::Intent;
use solver_common::types::intent_id::IntentId;

use crate::quote::quoted_intent::QuotedIntent;
use crate::workflow::executors::lock_tokens_executor::LockIntentTokensHandlerResult;
use crate::workflow::executors::swap_intent_filler_executor::SwapIntentFillerHandlerResult;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewIntent(Intent),
    IntentQuoted(QuotedIntent),
    IntentPlaced(),

    TokensLockedOnSourceChain(LockIntentTokensHandlerResult),
    IntentFilledOnDestination(SwapIntentFillerHandlerResult),

    ProvedTokensLockedOnSourceChain(IntentId),
    ProvedSwapIntentFilledOnDestinationChain(IntentId),
}
