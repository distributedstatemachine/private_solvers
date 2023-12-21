use crate::quote::quoted_swap_intent::QuotedSwapIntent;
use crate::workflow::executors::fill_spoke_chain_call_intent_creator_executor::FillCreatorHandlerResult;
use crate::workflow::executors::lock_tokens_spoke_chain_call_intent_creator_executor::LockTokensSpokeChainCallIntentCreatorHandlerResult;
use intentbook_matchmaker::types::swap_intent::SwapIntent;
use solver_common::types::intent_id::IntentId;

pub mod in_memory_state_manager;
pub mod state_manager;

#[derive(Debug, Clone)]
pub struct IntentState {
    pub intent_id: IntentId,
    pub swap_intent: SwapIntent,
    pub quoted_intent: Option<QuotedSwapIntent>,

    pub lock_intent_tokens_handler_result:
        Option<LockTokensSpokeChainCallIntentCreatorHandlerResult>,
    pub filler_handler_result: Option<FillCreatorHandlerResult>,

    pub is_proved_that_tokens_locked_on_source_chain: bool,
    pub is_proved_that_filled_on_destination_chain: bool,
}

impl IntentState {
    pub fn new(swap_intent: SwapIntent) -> Self {
        IntentState {
            intent_id: swap_intent.intent_id,
            swap_intent,
            quoted_intent: None,
            lock_intent_tokens_handler_result: None,
            filler_handler_result: None,
            is_proved_that_filled_on_destination_chain: false,
            is_proved_that_tokens_locked_on_source_chain: false,
        }
    }

    pub fn get_intent_id(&self) -> IntentId {
        self.intent_id
    }

    pub fn is_ready_to_settle(&self) -> bool {
        self.is_proved_that_tokens_locked_on_source_chain
            && self.is_proved_that_filled_on_destination_chain
    }
}
