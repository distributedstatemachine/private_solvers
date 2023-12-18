use crate::quote::quoted_intent::QuotedIntent;
use crate::types::limit_order_intent::LimitOrderIntent;
use crate::workflow::executors::post_limit_order_executor::PostLimitOrderHandlerResult;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewLimitOrderIntent(LimitOrderIntent),
    IntentQuoted(QuotedIntent),
    LimitOrderPosted(PostLimitOrderHandlerResult),
}
