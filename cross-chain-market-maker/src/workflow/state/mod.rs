use crate::types::limit_order_intent::LimitOrderIntent;
use solver_common::types::intent_id::IntentId;

pub mod in_memory_state_manager;
pub mod state_manager;

#[derive(Debug, Clone)]
pub struct IntentState {
    pub intent_id: IntentId,
    pub limit_order_intent: LimitOrderIntent,
}

impl IntentState {
    pub fn new(limit_order_intent: LimitOrderIntent) -> Self {
        IntentState {
            intent_id: limit_order_intent.intent_id,
            limit_order_intent,
        }
    }
}
