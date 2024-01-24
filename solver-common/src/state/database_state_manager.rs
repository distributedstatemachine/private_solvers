use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use crate::collectors::ethereum::new_intentbook_source::NewIntentbookIntentSource;
use crate::collectors::new_intent_collector::NewIntentSource;
use crate::state::database_client::DatabaseClient;

use crate::config::addresses::IntentbookType;
use crate::state::state_manager::StateManager;
use crate::state::IntentState;
use crate::types::intent::{calculate_intent_id, Intent};
use crate::types::intent_bid::{calculate_intent_bid_id, IntentBid};
use crate::types::intent_id::IntentId;
use async_trait::async_trait;
use ethers::types::H256;
use ethers::utils::hex;
use ethers_middleware::Middleware;
use futures::{Future, StreamExt};
use serde_json::to_string;
use sqlx::migrate::Migrator;
use sqlx::PgPool;
use std::env;

use super::IntentStatus;

#[async_trait]
impl DatabaseClient for PgPool {
    async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        // Retrieve the migrations directory from an environment variable
        let migrations_dir = env::var("MIGRATIONS_DIR").expect("MIGRATIONS_DIR must be set");
        let migrator = Migrator::new(std::path::Path::new(&migrations_dir)).await?;
        // Run the migrations using the acquired PgPool (self)
        migrator.run(self).await?;

        Ok(())
    }

    async fn update_intent_state(&self, intent_id: IntentId, new_state: IntentState) -> Result<(), sqlx::Error> {
        let intent = to_string(&new_state.intent).unwrap(); // Serialize the intent
        sqlx::query("UPDATE IntentState SET status = $1, intent_bid_id = $2, spoke_chain_call = $3::jsonb, block_number = $4 WHERE intent_id = $5")
            .bind(new_state.status)
            .bind(new_state.intent_bid_id)
            .bind(&intent)
            .bind(new_state.block_number)
            .bind(intent_id.to_string())
            .execute(self)
            .await?;
        Ok(())
    }

    async fn get_intent_state(
        &self,
        intent_id: IntentId,
    ) -> Result<Option<IntentState>, sqlx::Error> {
        let result = sqlx::query_as::<_, (String, i32, String, String, i64)>("SELECT intent_id, status, intent_bid_id, spoke_chain_call, block_number FROM IntentState WHERE intent_id = $1")
            .bind(intent_id.to_string())
            .fetch_optional(self)
            .await?
            .map(|row| {
                let intent_id = IntentId::from(H256::from_slice(&hex::decode(&row.0).unwrap()));
                let status: IntentStatus = serde_json::from_str(&(row.1).to_string()).unwrap(); 
                let intent: Intent = serde_json::from_str(&row.3).unwrap();

                IntentState {
                    intent_id,
                    status,
                    intent_bid_id: None, 
                    intent,
                    block_number: Some(row.4),
                }
            });
        Ok(result)
    }

    // FIX ME: Still using old schema
    async fn get_all_intents(&self) -> Result<Vec<IntentState>, sqlx::Error> {
        let rows: Vec<(String, i32, Option<String>, String, i64)> = sqlx::query_as(
            "SELECT intent_id, status, intent_bid_id, spoke_chain_call, block_number FROM IntentState")
            .fetch_all(self)
            .await?;

        let intents = rows.into_iter().map(|row| {
            let intent_id = IntentId::from(H256::from_slice(&hex::decode(&row.0).unwrap()));
            let status: IntentStatus = serde_json::from_str(&row.1.to_string()).unwrap();
            let intent: Intent = serde_json::from_str(&row.3).unwrap();

            IntentState {
                intent_id,
                status,
                intent_bid_id: row.2,
                intent,
                block_number: Some(row.4),
            }
        }).collect();

        Ok(intents)
    }

    async fn create_intent_state(&self, intent: Intent, intent_bid: Option<IntentBid>) -> Result<IntentId, sqlx::Error> {
        let intent_clone = intent.clone();
        let intent_id = calculate_intent_id(intent_clone.into());
        let intent_state = IntentState {
            intent_id,
            status: IntentStatus::New,
            intent_bid_id: intent_bid.map(|bid| calculate_intent_bid_id(bid.into()).to_string()),
            intent,
            block_number: None,
        };
        let serialized_intent = to_string(&intent_state.intent).unwrap();
        sqlx::query("INSERT INTO IntentState (intent_id, status, intent_bid_id, spoke_chain_call, block_number) VALUES ($1, $2, $3, $4::jsonb, $5)")
            .bind(intent_id.to_string())
            .bind(intent_state.status.to_string())
            .bind(intent_state.intent_bid_id)
            .bind(&serialized_intent)
            .bind(intent_state.block_number)
            .execute(self)
            .await?;
        Ok(intent_id)
    }

    async fn get_in_progress_intents(&self) -> Result<Vec<IntentState>, sqlx::Error> {
        let rows: Vec<(String, i32, Option<String>, String, i64)> = sqlx::query_as(
            "SELECT intent_id, status, intent_bid_id, spoke_chain_call, block_number FROM IntentState WHERE status = 'InProgress'")
            .fetch_all(self)
            .await?;

        let intents = rows.into_iter().map(|row| {
            let intent_id = IntentId::from(H256::from_slice(&hex::decode(&row.0).unwrap()));
            let status: IntentStatus = serde_json::from_str(&row.1.to_string()).unwrap();
            let intent: Intent = serde_json::from_str(&row.3).unwrap();

            IntentState {
                intent_id,
                status,
                intent_bid_id: row.2,
                intent,
                block_number: Some(row.4),
            }
        }).collect();

        Ok(intents)
    }
}

pub struct DatabaseStateManager {
    _intents: HashMap<IntentId, IntentState>,
    db_client: Arc<dyn DatabaseClient + Send + Sync>,
}

impl DatabaseStateManager {
    pub async fn new(db_client: Arc<dyn DatabaseClient + Send + Sync>) -> Self {
        Self {
            _intents: HashMap::new(),
            db_client,
        }
    }
}

#[async_trait]
impl StateManager for DatabaseStateManager {
    async fn update_intent_state(
        &mut self,
        intent_id: IntentId,
        new_state: IntentState,
        _intentbook: &IntentbookType,
    ) -> Result<(), sqlx::Error> {
        self.db_client.update_intent_state(intent_id, new_state).await
    }

    async fn get_intent_state(
        &self,
        intent_id: IntentId,
    ) -> Pin<Box<dyn Future<Output = Option<IntentState>> + Send>> {
        let db_client = Arc::clone(&self.db_client);
        Box::pin(async move {
            match db_client.get_intent_state(intent_id).await {
                Ok(intent_state_option) => intent_state_option,
                Err(_) => None,
            }
        })
    }
    

    async fn get_all_intents(&self) -> Result<Vec<IntentState>, sqlx::Error> {
        self.db_client.get_all_intents().await
    }

    async fn create_intent_state(
        &mut self,
        intent: Intent,
        intent_bid: Option<IntentBid>,
    ) -> Result<IntentId, sqlx::Error> {
        self.db_client.create_intent_state(intent, intent_bid).await
    }

    async fn get_in_progress_intents(&self) -> Result<Vec<IntentState>, sqlx::Error> {
        self.db_client.get_in_progress_intents().await
    }

    /// Fetches the state from the database and compares it with the last known block from the rpc
    /// client. If the block number is greater than the last known block, it fetches the new intents
    /// from using get_new_spoke_chain_call_intents_stream and updates the state in the database.
    async fn fetch_state(
        &mut self,
        _intentbook: IntentbookType,
        intentbook_source: &NewIntentbookIntentSource,
    ) -> Result<Vec<IntentState>, sqlx::Error> {
        let mut new_intents = vec![];
        let in_progress_intents = self.db_client.get_in_progress_intents().await?;
        for intent in in_progress_intents {
            let block_number = intent.block_number.unwrap();
    
            let rpc_client = Arc::clone(&intentbook_source.rpc_client);
            let latest_block_number_result = rpc_client.get_block_number().await;
            if let Ok(latest_block_number) = latest_block_number_result {
                if block_number < latest_block_number.as_u64() as i64 {
                    let new_spoke_chain_call_intents_stream_result =
                        intentbook_source.get_new_intent_source().await;
                    let mut new_spoke_chain_call_intents_stream =
                        match new_spoke_chain_call_intents_stream_result {
                            Ok(stream) => stream,
                            Err(e) => return Err(sqlx::Error::Protocol(e.to_string())),
                        };
                    while let Some(new_intent) = new_spoke_chain_call_intents_stream.next().await {
                        let new_intent_clone = new_intent.clone();
                        let intent_id = calculate_intent_id(new_intent_clone.into());
                        let intent_state = IntentState {
                            intent_id,
                            status: IntentStatus::InProgress,
                            intent_bid_id: None,
                            intent: new_intent,
                            block_number: Some(latest_block_number.as_u64() as i64),
                        };
                        self.db_client.update_intent_state(intent_id, intent_state.clone()).await?;
                        new_intents.push(intent_state);
                    }
                }
            }
        }
        Ok(new_intents)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use crate::state::database_client::MockDatabaseClient;

    // TODO: Make this test more robust by creating actual states and not default.
    // it might also make sense to combine this with create_intent_state
     #[tokio::test]
    async fn test_update_intent_state() {
        let mut mock_db_client = MockDatabaseClient::new();

        // Set up the mock to return Ok(()) when update_intent_state is called
        let expected_intent_id = IntentId::default();
        let expected_new_state = IntentState::default();
        let expected_new_state_clone = expected_new_state.clone(); // Clone before moving
        mock_db_client.expect_update_intent_state()
            .with(eq(expected_intent_id), eq(expected_new_state_clone))
            .return_once(|_, _| Ok(()));

        // Create a DatabaseStateManager with the mock client
        let mut state_manager = DatabaseStateManager::new(Arc::new(mock_db_client)).await;

        // Call update_intent_state and check the result
        let result = state_manager.update_intent_state(expected_intent_id, expected_new_state, &IntentbookType::SwapIntentIntentBook).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_intent_state() {
        let mut mock_db_client = MockDatabaseClient::new();

        // Set up the mock to return a specific value when get_intent_state is called
        let expected_intent_id = IntentId::default();
        let expected_intent_state = Some(IntentState::default());
        let expected_intent_state_clone = expected_intent_state.clone(); // Clone before moving
        mock_db_client.expect_get_intent_state()
            .with(eq(expected_intent_id))
            .return_once(move |_| Ok(expected_intent_state_clone));

        // Create a DatabaseStateManager with the mock client
        let state_manager = DatabaseStateManager::new(Arc::new(mock_db_client)).await;

        // Call get_intent_state and check the result
        let result = state_manager.get_intent_state(expected_intent_id).await.await;
        assert_eq!(result, expected_intent_state);
    }

    #[tokio::test]
    async fn test_get_all_intents() {
        let mut mock_db_client = MockDatabaseClient::new();

        // Set up the mock to return a specific value when get_all_intents is called
        let expected_intents = vec![IntentState::default()];
        let expected_intents_clone = expected_intents.clone(); // Clone for the assertion later
        mock_db_client.expect_get_all_intents()
            .return_once(move || Ok(expected_intents_clone));

        // Create a DatabaseStateManager with the mock client
        let state_manager = DatabaseStateManager::new(Arc::new(mock_db_client)).await;

        // Call get_all_intents and check the result
        let result = state_manager.get_all_intents().await;
        assert_eq!(result.unwrap(), expected_intents);
    }

    #[tokio::test]
    async fn test_create_intent_state() {
        let mut mock_db_client = MockDatabaseClient::new();

        // Set up the mock to return a specific value when create_intent_state is called
        let expected_intent = Intent::default();
        let expected_intent_bid = Some(IntentBid::default());
        let expected_intent_id = IntentId::default();
        mock_db_client.expect_create_intent_state()
            .with(eq(expected_intent.clone()), eq(expected_intent_bid.clone()))
            .return_once(move |_, _| Ok(expected_intent_id));

        // Create a DatabaseStateManager with the mock client
        let mut state_manager = DatabaseStateManager::new(Arc::new(mock_db_client)).await;

        // Call create_intent_state and check the result
        let result = state_manager.create_intent_state(expected_intent, expected_intent_bid).await;
        assert_eq!(result.unwrap(), expected_intent_id);
    }
}
