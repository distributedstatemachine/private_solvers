use crate::types::spoke_chain_call::SpokeChainCall;
use crate::types::IntentId;

pub mod in_memory_state_manager;
pub mod state_manager;

#[derive(Debug, Clone)]
pub struct IntentState {
    pub intent_id: IntentId,
    pub spoke_chain_call: SpokeChainCall,
}

impl IntentState {
    pub fn new(intent: SpokeChainCall) -> Self {
        IntentState {
            intent_id: intent.intent_id,
            spoke_chain_call: intent,
        }
    }

    pub fn get_intent_id(&self) -> IntentId {
        self.intent_id
    }

    pub fn is_ready_to_settle(&self) -> bool {
        true
    }
}
