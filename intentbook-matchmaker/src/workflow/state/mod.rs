use crate::types::intent::Intent;

pub mod in_memory_state_manager;
pub mod state_manager;

#[derive(Debug, Clone)]
pub struct IntentState {
    pub intent: Intent,
    pub is_matched: bool,

    // TODO: this flag only applies to SpokeChainCall intent. Refactor structs and move it there.
    pub is_spoke_chain_called: bool,
}

impl IntentState {
    pub fn new(intent: Intent) -> Self {
        IntentState {
            intent,
            is_matched: false,
            is_spoke_chain_called: false,
        }
    }

    pub fn is_ready_to_settle(&self) -> bool {
        match &self.intent {
            &Intent::SpokeChainCall(..) => self.is_spoke_chain_called,
            _ => false,
        }
    }
}
