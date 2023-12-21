use crate::quote::quoted_swap_intent::QuotedSwapIntent;
use crate::workflow::executors::fill_spoke_chain_call_intent_creator_executor::FillSpokeChainCallIntentCreatorHandlerResult;
use crate::workflow::executors::lock_tokens_spoke_chain_call_intent_creator_executor::LockTokensSpokeChainCallIntentCreatorHandlerResult;
use intentbook_matchmaker::types::intent::Intent;
use intentbook_matchmaker::workflow::executors::place_intent_executor::PlaceIntentHandlerResult;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewIntent(Intent),
    IntentQuoted(QuotedSwapIntent),
    IntentPlaced(PlaceIntentHandlerResult),

    CreatedSpokeChainCallToLockTokensOnSourceChain(
        LockTokensSpokeChainCallIntentCreatorHandlerResult,
    ),
    CreatedSpokeChainCallIntentToFillSwapIntentOnDestinationChain(
        FillSpokeChainCallIntentCreatorHandlerResult,
    ),
}
