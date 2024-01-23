use solver_common::types::intent::Intent;
use solver_common::types::swap_intent::SwapIntent;
use solver_common::types::swap_intent_bid::SwapIntentBid;

use crate::quote::quoted_swap_intent::QuotedSwapIntent;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    CreateMatchedBid(QuotedSwapIntent),
    MatchSwapIntent(SwapIntent, SwapIntentBid),

    PlaceIntent(Intent),
    CreateSpokeChainCallIntentToLockSwapIntentTokensOnSourceChain(SwapIntent),
    CreateSpokeChainCallIntentToFillSwapIntentOnDestinationChain(QuotedSwapIntent),
}
