use crate::types::intent::Intent;
use crate::types::IntentId;
use crate::workflow::state::IntentState;

pub trait StateManager {
    fn update_state(&mut self, intent_id: IntentId, new_state: IntentState);
    fn get_state(&self, intent_id: IntentId) -> Option<&IntentState>;
    fn get_all_intents(&self) -> Vec<IntentState>;

    fn create_intent_state(&mut self, intent: Intent) -> IntentId;

    fn update_intent_state<F>(&mut self, intent_id: IntentId, updater: F) -> Option<IntentState>
    where
        F: FnOnce(&mut IntentState);
}
