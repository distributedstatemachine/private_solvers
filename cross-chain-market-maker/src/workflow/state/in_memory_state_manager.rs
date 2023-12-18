use std::collections::HashMap;

use crate::types::intent_id::IntentId;
use crate::types::limit_order_intent::LimitOrderIntent;
use crate::workflow::state::state_manager::StateManager;
use crate::workflow::state::IntentState;

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

impl Default for InMemoryStateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StateManager for InMemoryStateManager {
    fn update_state(&mut self, intent_id: IntentId, new_state: IntentState) {
        self.intents.insert(intent_id, new_state);
    }
    fn get_state(&self, intent_id: IntentId) -> Option<&IntentState> {
        self.intents.get(&intent_id)
    }

    fn get_all_intents(&self) -> Vec<IntentState> {
        self.intents.values().cloned().collect()
    }

    fn create_intent_state(&mut self, limit_order_intent: LimitOrderIntent) -> IntentId {
        let intent_id = limit_order_intent.intent_id;
        let intent_state = IntentState::new(limit_order_intent);
        self.intents.insert(intent_id, intent_state);
        intent_id
    }

    fn update_intent_state<F>(&mut self, intent_id: IntentId, updater: F) -> Option<IntentState>
    where
        F: FnOnce(&mut IntentState),
    {
        if let Some(intent_state) = self.intents.get_mut(&intent_id) {
            updater(intent_state);
            return Some(intent_state.clone());
        }
        None
    }
}