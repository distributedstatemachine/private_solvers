use crate::workflow::{
    collectors::ethereum::spoke_chain_call_intents_book_source::SpokeChainCallIntentbookSource,
    state::IntentState,
};
use async_trait::async_trait;
use intentbook_matchmaker::types::spoke_chain_call::SpokeChainCall;
use solver_common::types::intent_id::IntentId;

// TODO: Move sync states here
#[async_trait]
pub trait StateManager {
    // TODO: Make error generic
    async fn update_state(
        &mut self,
        intent_id: IntentId,
        new_state: IntentState,
    ) -> Result<(), sqlx::Error>;

    async fn get_state(&mut self, intent_id: IntentId) -> Option<IntentState>;
    async fn get_all_intents(&self) -> Result<Vec<IntentState>, sqlx::Error>;

    async fn create_intent_state(
        &mut self,
        intent: SpokeChainCall,
    ) -> Result<IntentId, sqlx::Error>;

    async fn get_in_progress_intents(&self) -> Result<Vec<IntentState>, sqlx::Error>;

    /// Fetches the state from the database and compares it with the last known block from the rpc
    /// client. If the block number is greater than the last known block, it fetches the new intents
    /// from the rpc client and updates the state in the database.
    async fn fetch_state(
        &mut self,
        // rpc_client: &RpcClient,
        spoke_chain_call_intentbook_source: &SpokeChainCallIntentbookSource,
    ) -> Result<Vec<IntentState>, sqlx::Error>;
}
