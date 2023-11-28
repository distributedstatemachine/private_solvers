use ethers::types::U256;

use crate::quote::quoted_intent::QuotedIntent;
use crate::types::intent_id::IntentId;
use crate::types::swap_intent::SwapIntent;

pub mod in_memory_state_manager;
pub mod state_manager;

#[derive(Debug, Clone, Default)]
pub struct IntentState {
    pub intent_id: IntentId,
    pub swap_intent: SwapIntent,
    pub quoted_intent: Option<QuotedIntent>,

    pub is_tokens_locked_on_source_chain: bool,
    pub is_filled_on_destination: bool,
    pub fill_timestamp: Option<U256>,

    pub is_proved_that_tokens_locked_on_source_chain: bool,
    pub is_proved_that_filled_on_destination_chain: bool,
}

impl IntentState {
    pub fn get_intent_id(&self) -> IntentId {
        self.intent_id
    }

    pub fn is_ready_to_settle(&self) -> bool {
        self.is_proved_that_tokens_locked_on_source_chain
            && self.is_proved_that_filled_on_destination_chain
    }
}
