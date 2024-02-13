use crate::state::IntentState;
use crate::types::intent::Intent;
use crate::types::intent_bid::IntentBid;
use crate::types::intent_id::IntentId;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait DatabaseClient {
    async fn run_migrations(&self) -> Result<(), sqlx::Error>;
    async fn update_intent_state(
        &self,
        intent_id: IntentId,
        new_state: IntentState,
    ) -> Result<(), sqlx::Error>;
    async fn get_intent_state(
        &self,
        intent_id: IntentId,
    ) -> Result<Option<IntentState>, sqlx::Error>;
    async fn get_all_intents(&self) -> Result<Vec<IntentState>, sqlx::Error>;
    async fn create_intent_state(
        &self,
        intent: Intent,
        intent_bid: Option<IntentBid>,
    ) -> Result<IntentId, sqlx::Error>;
    async fn get_in_progress_intents(&self) -> Result<Vec<IntentState>, sqlx::Error>;
}
