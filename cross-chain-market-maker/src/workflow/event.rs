use crate::workflow::executors::post_limit_order_executor::PostLimitOrderHandlerResult;
use intentbook_matchmaker::types::limit_order_intent::LimitOrderIntent;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewLimitOrderIntent(LimitOrderIntent),
    LimitOrderPosted(PostLimitOrderHandlerResult),
}
