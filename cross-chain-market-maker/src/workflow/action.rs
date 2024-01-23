use solver_common::types::limit_order_intent::LimitOrderIntent;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    PostLimitOderIntent(LimitOrderIntent),
}
