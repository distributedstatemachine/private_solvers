use crate::workflow::state::IntentState;
use intentbook_matchmaker::types::spoke_chain_call::SpokeChainCall;
use solver_common::types::intent_id::IntentId;

// TODO: Move sync states here
pub trait StateManager {
    // TODO: Make error generic
    async fn update_state(
        &mut self,
        intent_id: IntentId,
        new_state: IntentState,
    ) -> Result<(), sqlx::Error>;

    async fn get_state(&mut self, intent_id: IntentId) -> Option<IntentState>;
    async fn get_all_intents(&self) -> Result<Vec<IntentState>, sqlx::Error>;

    async fn create_intent_state(&mut self, intent: SpokeChainCall) ->  Result<IntentId, sqlx::Error>;



    async fn get_in_progress_intents(&self) -> Result<Vec<IntentState>, sqlx::Error> ;
}
