use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use crate::collectors::ethereum::new_intentbook_source::NewIntentbookIntentSource;
use crate::collectors::new_intent_collector::NewIntentSource;
use crate::state::database_client::DatabaseClient;

use super::IntentStatus;
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

#[async_trait]
impl DatabaseClient for PgPool {
    async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        let migrations_folder = "./solver-common/src/migrations";
        let migrator = Migrator::new(std::path::Path::new(migrations_folder))
            .await
            .expect("Failed to create migrator");

        migrator.run(self).await?;

        Ok(())
    }

    async fn update_intent_state(
        &self,
        intent_id: IntentId,
        new_state: IntentState,
    ) -> Result<(), sqlx::Error> {
        let intent = to_string(&new_state.intent).unwrap();
        sqlx::query("UPDATE IntentState SET status = $1, intent_bid_id = $2, intent = $3::jsonb, block_number = $4 WHERE intent_id = $5")
            .bind(new_state.status)
            .bind(new_state.intent_bid_id.unwrap_or_else(|| "default_bid_id".to_string())) 
            .bind(&intent)
            .bind(new_state.block_number.unwrap_or(0))
            .bind(intent_id.to_string())
            .execute(self)
            .await?;
        Ok(())
    }

    async fn get_intent_state(
        &self,
        intent_id: IntentId,
    ) -> Result<Option<IntentState>, sqlx::Error> {
        let result = sqlx::query_as::<_, (String, String, String, String, i64)>("SELECT intent_id, status, intent_bid_id, intent, block_number FROM IntentState WHERE intent_id = $1")
            .bind(intent_id.to_string())
            .fetch_optional(self)
            .await?
            .map(|row| {
                let intent_id = IntentId::from(H256::from_slice(&hex::decode(&row.0).unwrap()));
                let status: IntentStatus = serde_json::from_str(&row.1).unwrap();
                let intent_bid_id = Some(row.2);
                let intent: Intent = serde_json::from_str(&row.3).unwrap();
                IntentState {
                    intent_id,
                    status,
                    intent_bid_id,
                    intent,
                    block_number: Some(row.4),
                }
            });
        Ok(result)
    }

    async fn get_all_intents(&self) -> Result<Vec<IntentState>, sqlx::Error> {
        let rows: Vec<(String, String, String, String, i64)> = sqlx::query_as(
            "SELECT intent_id, status, intent_bid_id, intent, block_number FROM IntentState",
        )
        .fetch_all(self)
        .await?;

        let intents = rows
            .into_iter()
            .map(|row| {
                let intent_id = IntentId::from(H256::from_slice(&hex::decode(&row.0).unwrap()));
                let status: IntentStatus = serde_json::from_str(&row.1).unwrap();
                let intent_bid_id = Some(row.2); // Since intent_bid_id is now NOT NULL
                let intent: Intent = serde_json::from_str(&row.3).unwrap();

                IntentState {
                    intent_id,
                    status,
                    intent_bid_id,
                    intent,
                    block_number: Some(row.4),
                }
            })
            .collect();

        Ok(intents)
    }

    async fn create_intent_state(
        &self,
        intent: Intent,
        intent_bid: Option<IntentBid>,
    ) -> Result<IntentId, sqlx::Error> {
        let intent_clone = intent.clone();
        let intent_id = calculate_intent_id(intent_clone.into());
        let intent_bid_id = intent_bid
            .map(|bid| calculate_intent_bid_id(bid.into()).to_string())
            .unwrap_or_else(|| "default_bid_id".to_string());
        let intent_state = IntentState {
            intent_id,
            status: IntentStatus::New,
            intent_bid_id: Some(intent_bid_id.clone()),
            intent,
            block_number: None,
        };
        let serialized_intent = to_string(&intent_state.intent).unwrap();
        sqlx::query("INSERT INTO IntentState (intent_id, status, intent_bid_id, intent, block_number) VALUES ($1, $2, $3, $4::jsonb, $5)")
            .bind(intent_id.to_string())
            .bind(intent_state.status.to_string())
            .bind(intent_bid_id) // Adjusted to ensure it's not null
            .bind(&serialized_intent)
            .bind(intent_state.block_number.unwrap_or(0)) // Provide a default value for block_number
            .execute(self)
            .await?;
        Ok(intent_id)
    }
    async fn get_in_progress_intents(&self) -> Result<Vec<IntentState>, sqlx::Error> {
        let rows: Vec<(String, String, String, String, i64)> = sqlx::query_as(
            "SELECT intent_id, status, intent_bid_id, intent, block_number FROM IntentState WHERE status = 'InProgress'")
            .fetch_all(self)
            .await?;

        let intents = rows
            .into_iter()
            .map(|row| {
                let intent_id = IntentId::from(H256::from_slice(&hex::decode(&row.0).unwrap()));
                let status: IntentStatus = serde_json::from_str(&row.1).unwrap();
                let intent_bid_id = Some(row.2); // Since intent_bid_id is now NOT NULL
                let intent: Intent = serde_json::from_str(&row.3).unwrap();

                IntentState {
                    intent_id,
                    status,
                    intent_bid_id,
                    intent,
                    block_number: Some(row.4),
                }
            })
            .collect();

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
        self.db_client
            .update_intent_state(intent_id, new_state)
            .await
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
                        self.db_client
                            .update_intent_state(intent_id, intent_state.clone())
                            .await?;
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
    use sqlx::{postgres::PgPoolOptions, PgPool, Row};
    use std::env;
    use testcontainers_modules::{postgres::Postgres, testcontainers::clients::Cli};

    struct TestDb {
        uri: String,
    }

    impl TestDb {
        async fn new() -> Self {
            // let pool_options = sqlx::postgres::PgPoolOptions::new().max_connections(5);

            let docker = Cli::default();
            let container = docker.run(Postgres::default());



            let uri = format!(
                "postgres://postgres:postgres@127.0.0.1:{}/postgres",
                container.get_host_port_ipv4(5432)
            );

            // Simple delay to wait for the container to be ready, adjust as needed
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;

            env::set_var("DATABASE_URL", &uri);

            // // Create a new instance of PgPool
            // let pool = container
            //     .connect(&uri)
            //     .await
            //     .expect("Failed to connect to the database");

            // Create a new connection pool
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&uri)
                .await
                .expect("Failed to create a connection pool");

            // Run migrations
            Self::run_migrations(&pool)
                .await
                .expect("Failed to run migrations");

            Self { uri }
        }

        async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
            // Specify the path to your migrations folder
            // let migrations_folder = "./solver-common/src/migrations";
            let migrations_folder = "./src/migrations";

            let migrator = sqlx::migrate::Migrator::new(std::path::Path::new(migrations_folder))
                .await
                .expect("Failed to create migrator");

            // Apply migrations
            migrator.run(pool).await?;

            Ok(())
        }
    }

    // impl Drop for TestDb {
    //     fn drop(&mut self) {}
    // }

    #[tokio::test]
    async fn test_update_intent_state() {
        let test_db = TestDb::new().await;
        let pool = PgPool::connect(&test_db.uri).await.unwrap();

        // Insert a test intent into the database
        let insert_query = sqlx::query(
            "INSERT INTO IntentState (intent_id, status, intent_bid_id, spoke_chain_call, block_number) VALUES ($1, $2, $3, $4::jsonb, $5) RETURNING intent_id",
        )
        .bind("test_intent_id")
        .bind("New")
        .bind(None::<String>)
        .bind("{}")
        .bind(0)
        .fetch_one(&pool)
        .await
        .unwrap();
        let intent_id: String = insert_query.get("intent_id");

        // Update the intent state
        let update_query = sqlx::query(
            "UPDATE IntentState SET status = $1, block_number = $2 WHERE intent_id = $3",
        )
        .bind("InProgress")
        .bind(1)
        .bind(&intent_id)
        .execute(&pool)
        .await;
        assert!(update_query.is_ok(), "Failed to update intent state");

        // Fetch the updated intent and assert the changes
        let select_query = sqlx::query_as::<_, (String, i32, Option<String>, String, i64)>(
            "SELECT intent_id, status, intent_bid_id, spoke_chain_call, block_number FROM IntentState WHERE intent_id = $1",
        )
        .bind(&intent_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(select_query.0, intent_id);
        let status = IntentStatus::from_i32(select_query.1).expect("Invalid status value");
        assert_eq!(status, IntentStatus::InProgress);
        assert_eq!(select_query.4, 1);
    }

    // #[tokio::test]
    // async fn test_get_intent_state() {
    //     let test_db = TestDb::new().await;
    //     let pool = PgPool::connect(&test_db.uri).await.unwrap();

    //     // Insert test data
    //     // Fetch intent state using `get_intent_state`
    //     // Assert the fetched data matches expected values
    // }
}
