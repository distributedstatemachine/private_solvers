use crate::types::intent::Intent;

pub mod in_memory_state_manager;
pub mod state_manager;

#[derive(Debug, Clone)]
pub struct IntentState {
    pub intent: Intent,
    pub is_matched: bool,
}

impl IntentState {
    pub fn new(intent: Intent) -> Self {
        IntentState {
            intent,
            is_matched: false,
        }
    }

    pub fn is_ready_to_settle(&self) -> bool {
        true
    }
}
