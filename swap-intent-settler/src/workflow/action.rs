use crate::quote::quoted_intent::QuotedIntent;
use crate::workflow::executors::settle_intent_executor::SwapIntentSettlementData;
use bindings_khalani::shared_types::Intent;
use intentbook_matchmaker::types::swap_intent::SwapIntent;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    PlaceIntent(Intent),
    LockTokensOnSourceChain(SwapIntent),
    FillIntentOnDestinationChain(QuotedIntent),
    SwapAndBridge(QuotedIntent),
    ApproveTokens(QuotedIntent), // TODO: is it necessary?
    SettleIntent(SwapIntentSettlementData),
}
