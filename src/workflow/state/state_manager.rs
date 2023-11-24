use crate::types::intent_id::IntentId;
use crate::workflow::state::state::IntentState;

pub trait StateManager {
    fn update_state(&mut self, intent_id: IntentId, new_state: IntentState);
    fn get_state(&self, intent_id: IntentId) -> Option<&IntentState>;
    fn get_known_intents(&self) -> Vec<&IntentId>;
}
