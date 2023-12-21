use crate::quote::quoted_intent::QuotedIntent;
use intentbook_matchmaker::types::intent::Intent;
use intentbook_matchmaker::types::swap_intent::SwapIntent;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    MatchIntent(Intent),
    PlaceIntent(Intent),
    LockTokensOnSourceChain(SwapIntent),
    FillIntentOnDestinationChain(QuotedIntent),
    SwapAndBridge(QuotedIntent),
}
