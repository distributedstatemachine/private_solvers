use intentbook_matchmaker::types::intent::Intent;
use solver_common::types::intent_id::IntentId;

use crate::quote::quoted_swap_intent::QuotedSwapIntent;
use crate::workflow::executors::fill_spoke_chain_call_intent_creator_executor::FillCreatorHandlerResult;
use crate::workflow::executors::lock_tokens_executor::LockIntentTokensHandlerResult;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewIntent(Intent),
    IntentQuoted(QuotedSwapIntent),

    TokensLockedOnSourceChain(LockIntentTokensHandlerResult),
    CreatedSpokeChainCallIntentToFillSwapIntentOnDestinationChain(FillCreatorHandlerResult),

    ProvedTokensLockedOnSourceChain(IntentId),
    ProvedSwapIntentFilledOnDestinationChain(IntentId),
}
