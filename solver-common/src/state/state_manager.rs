use crate::collectors::ethereum::new_intentbook_source::NewIntentbookIntentSource;
use crate::config::addresses::IntentbookType;
use crate::state::IntentState;
use crate::types::intent::Intent;
use crate::types::intent_bid::IntentBid;
use crate::types::intent_id::IntentId;

use async_trait::async_trait;

// TODO: Move sync states here
#[async_trait]
pub trait StateManager {
    // TODO: Make error generic
    async fn update_state(
        &mut self,
        intent_id: IntentId,
        new_state: IntentState,
        intentbook: &IntentbookType,
    ) -> Result<(), sqlx::Error>;

    async fn get_state(&mut self, intent_id: IntentId) -> Option<IntentState>;
    async fn get_all_intents(&self) -> Result<Vec<IntentState>, sqlx::Error>;

    async fn create_intent_state(
        &mut self,
        intent: Intent,
        intent_bid: Option<IntentBid>,
    ) -> Result<IntentId, sqlx::Error>;

    async fn get_in_progress_intents(&self) -> Result<Vec<IntentState>, sqlx::Error>;

    /// Fetches the state from the database and compares it with the last known block from the rpc
    /// client. If the block number is greater than the last known block, it fetches the new intents
    /// from the rpc client and updates the state in the database.
    async fn fetch_state(
        &mut self,
        intentbook: IntentbookType,
        intentbook_source: &NewIntentbookIntentSource,
    ) -> Result<Vec<IntentState>, sqlx::Error>;
}
