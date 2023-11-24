use crate::types::intent_id::IntentId;
use crate::workflow::state::state::IntentState;
use crate::workflow::state::state_manager::StateManager;
use std::collections::HashMap;

pub struct InMemoryStateManager {
    intents: HashMap<IntentId, IntentState>,
}

impl InMemoryStateManager {
    pub fn new() -> Self {
        Self {
            intents: HashMap::new(),
        }
    }
}

impl StateManager for InMemoryStateManager {
    fn update_state(&mut self, intent_id: IntentId, new_state: IntentState) {
        self.intents.insert(intent_id, new_state);
    }
    fn get_state(&self, intent_id: IntentId) -> Option<&IntentState> {
        self.intents.get(&intent_id)
    }

    fn get_known_intents(&self) -> Vec<&IntentId> {
        return self.intents.keys().collect();
    }
}
