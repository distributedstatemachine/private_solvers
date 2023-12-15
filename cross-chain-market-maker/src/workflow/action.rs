use crate::quote::quoted_intent::QuotedIntent;
use crate::types::limit_order_intent::LimitOrderIntent;
use crate::workflow::executors::settle_intent_executor::SwapIntentSettlementData;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    LockTokensOnSourceChain(LimitOrderIntent),
    FillIntentOnDestinationChain(QuotedIntent),
    SwapAndBridge(QuotedIntent),
    ApproveTokens(QuotedIntent), // TODO: is it necessary?
    SettleIntent(SwapIntentSettlementData),
}
