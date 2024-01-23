use std::collections::HashMap;
use std::sync::Arc;

use crate::collectors::ethereum::new_intentbook_source::NewIntentbookIntentSource;
use crate::collectors::new_intent_collector::NewIntentSource;

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
use futures::StreamExt;
use serde_json::to_string;
use sqlx::migrate::Migrator;
use sqlx::PgPool;
use std::env;

use super::IntentStatus;

pub struct DatabaseStateManager {
    intents: HashMap<IntentId, IntentState>,
    db_client: Option<PgPool>,
}

impl DatabaseStateManager {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let connection_string = "connection_string";
        let db_client = PgPool::connect(connection_string).await?;
        Ok(Self {
            intents: HashMap::new(),
            db_client: Some(db_client),
        })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        let migrations_dir = env::var("MIGRATIONS_DIR").expect("MIGRATIONS_DIR must be set");
        let migrator = Migrator::new(migrations_dir.as_ref()).await?;
        if let Some(pool) = &self.db_client {
            let mut connection = pool.acquire().await?;
            migrator.run(&mut connection).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl StateManager for DatabaseStateManager {
    async fn update_state(
        &mut self,
        intent_id: IntentId,
        new_state: IntentState,
        _intentbook: &IntentbookType,
    ) -> Result<(), sqlx::Error> {
        if let Some(pool) = &self.db_client {
            let mut connection = pool.acquire().await?;
            let intent = to_string(&new_state.intent).unwrap();
            sqlx::query("UPDATE IntentState SET status = $1, intent_bid_id = $2, spoke_chain_call = $3::jsonb, block_number = $4 WHERE intent_id = $5")
                .bind(new_state.status)
                .bind(new_state.intent_bid_id)
                .bind(intent)
                .bind(new_state.block_number)
                .bind(intent_id.to_string())
                .execute(&mut *connection)
                .await?;
        }
        Ok(())
    }

    async fn get_state(&mut self, intent_id: IntentId) -> Option<IntentState> {
        if let Some(pool) = &self.db_client {
            match pool.acquire().await {
                Ok(mut connection) => {
                    let row: (String, i32, String, String, i64) = sqlx::query_as("SELECT intent_id, status, intent_bid_id, spoke_chain_call, block_number FROM IntentState WHERE intent_id = $1")
                        .bind(intent_id.to_string())
                        .fetch_one(&mut *connection)
                        .await.ok()?;

                    let intent_id: IntentId =
                        IntentId::from(H256::from_slice(&hex::decode(&row.0).unwrap()));
                    let status: IntentStatus = serde_json::from_str(&(row.1).to_string()).ok()?;
                    let intent: Intent = serde_json::from_str(&row.3).ok()?;

                    let intent_state = IntentState {
                        intent_id,
                        status,
                        intent_bid_id: None,
                        intent,
                        block_number: Some(row.4),
                    };

                    self.intents.insert(intent_id, intent_state.clone());
                    Some(intent_state)
                }
                Err(_) => None,
            }
        } else {
            None
        }
    }

    async fn get_all_intents(&self) -> Result<Vec<IntentState>, sqlx::Error> {
        if let Some(pool) = &self.db_client {
            let mut connection = pool.acquire().await?;
            let rows: Vec<(String, i32, String, String, i64)> = sqlx::query_as("SELECT intent_id, status, intent_bid_id, spoke_chain_call, block_number FROM IntentState")
                .fetch_all(&mut *connection)
                .await?;

            let intents: Result<Vec<IntentState>, _> = rows
                .into_iter()
                .map(|row| {
                    let intent_id: IntentId =
                        IntentId::from(H256::from_slice(&hex::decode(&row.0).unwrap()));
                    let status: IntentStatus =
                        serde_json::from_str(&(row.1).to_string()).ok().unwrap();
                    let intent: Intent = serde_json::from_str(&row.3).ok().unwrap();

                    Ok(IntentState {
                        intent_id,
                        status,
                        intent_bid_id: None,
                        intent,
                        block_number: Some(row.4),
                    })
                })
                .collect();

            intents
        } else {
            Err(sqlx::Error::PoolClosed)
        }
    }

    async fn create_intent_state(
        &mut self,
        intent: Intent,
        intent_bid: Option<IntentBid>,
    ) -> Result<IntentId, sqlx::Error> {
        if let Some(pool) = &self.db_client {
            let mut connection = pool.acquire().await?;
            let intent_clone = intent.clone();
            let intent_id = calculate_intent_id(intent_clone.into());
            let intent_state = IntentState {
                intent_id,
                status: IntentStatus::New,
                // TODO: fix this
                intent_bid_id: intent_bid
                    .map(|bid| calculate_intent_bid_id(bid.into()).to_string()),
                intent: intent,
                // TODO: fix this
                block_number: None,
            };
            let intent = to_string(&intent_state.intent).unwrap();
            sqlx::query("INSERT INTO IntentState (intent_id, status, intent_bid_id, spoke_chain_call, block_number) VALUES ($1, $2, $3, $4, $5)")
                .bind(intent_id.to_string())
                .bind(intent_state.status.to_string())
                .bind(intent_state.intent_bid_id)
                .bind(intent)
                .bind(intent_state.block_number)
                .execute(&mut *connection)
                .await?;
            Ok(intent_id)
        } else {
            Err(sqlx::Error::PoolClosed)
        }
    }

    async fn get_in_progress_intents(&self) -> Result<Vec<IntentState>, sqlx::Error> {
        if let Some(pool) = &self.db_client {
            let mut connection = pool.acquire().await?;
            let rows: Vec<(String, i32, Option<String>, String, i64)> = sqlx::query_as("SELECT intent_id, status, intent_bid_id, spoke_chain_call, block_number FROM IntentState WHERE status = 'inprogress'")
                .fetch_all(&mut *connection)
                .await?;

            let intents: Result<Vec<IntentState>, _> = rows
                .into_iter()
                .map(|row| {
                    let intent_id: IntentId =
                        IntentId::from(H256::from_slice(&hex::decode(&row.0).unwrap()));
                    let status: IntentStatus =
                        serde_json::from_str(&(row.1).to_string()).ok().unwrap();
                    let intent: Intent = serde_json::from_str(&row.3).ok().unwrap();

                    Ok(IntentState {
                        intent_id,
                        status,
                        intent_bid_id: row.2,
                        intent,
                        block_number: Some(row.4),
                    })
                })
                .collect();

            intents
        } else {
            Err(sqlx::Error::PoolClosed)
        }
    }

    /// Fetches the state from the database and compares it with the last known block from the rpc
    /// client. If the block number is greater than the last known block, it fetches the new intents
    /// from using get_new_spoke_chain_call_intents_stream and updates the state in the database.
    async fn fetch_state(
        &mut self,
        intentbook: IntentbookType,
        intentbook_source: &NewIntentbookIntentSource,
    ) -> Result<Vec<IntentState>, sqlx::Error> {
        let mut new_intents = vec![];
        let in_progress_intents = self.get_in_progress_intents().await?;
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
                        self.update_state(intent_id, intent_state.clone(), &intentbook)
                            .await?;
                        new_intents.push(intent_state);
                    }
                }
            }
        }
        Ok(new_intents)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ethers::{abi::Bytes, types::U256};
//     use sqlx::postgres::PgPoolOptions;
//     use std::{
//         env,
//         panic::{self, AssertUnwindSafe},
//         sync::Arc,
//         time::Duration,
//     };

//     fn is_postgres_running() -> bool {
//         let result = panic::catch_unwind(AssertUnwindSafe(|| {
//             let pool = init_pool("postgres://postgres:postgres@localhost:5432/solver");
//             Arc::new(pool).get().is_ok()
//         }));

//         result.unwrap_or(false)
//     }

//     fn start_postgres() {
//         if !is_postgres_running() {
//             let status = std::process::Command::new("docker")
//                 .args([
//                     "run",
//                     "--rm",
//                     "--name",
//                     "solver-postgres",
//                     "-e",
//                     "POSTGRES_PASSWORD=postgres",
//                     "-e",
//                     "POSTGRES_USER=postgres",
//                     "-e",
//                     "POSTGRES_DB=solver",
//                     "-p",
//                     "5432:5432",
//                     "-d",
//                     "postgres:14",
//                 ])
//                 .status()
//                 .expect("failed to execute process");
//             println!("output: {:?}", status);

//             // sleep for a tiny bit to allow it to boot
//             std::thread::sleep(Duration::from_secs(5));

//             println!("Postgres started");
//         }

//         //   TODO: Run migrations

//         // println!("output: {:?}", cmd);

//         std::thread::sleep(Duration::from_secs(5));

//         println!("Running migration");
//     }

//     #[tokio::test]
//     async fn test_create_and_get_intent_state() {
//         // Set up the test database
//         let database_url = env::var("TEST_DATABASE_URL").unwrap();
//         let pool = PgPoolOptions::new()
//             .max_connections(5)
//             .connect(&database_url)
//             .await
//             .unwrap();

//         let mut state_manager = DatabaseStateManager {
//             intents: HashMap::new(),
//             db_client: Some(pool),
//         };

//         // Create a new intent state
//         let intent = SpokeChainCall {
//             intent_id: "some_intent_id".into(),
//             signature: Bytes::default(),
//             author: "0x0000000000000000000000000000000000000000"
//                 .parse()
//                 .unwrap(),
//             chain_id: 1.into(),
//             contract_to_call: "0x0000000000000000000000000000000000000000"
//                 .parse()
//                 .unwrap(),
//             call_data: Bytes::default(),
//             token: "0x0000000000000000000000000000000000000000"
//                 .parse()
//                 .unwrap(), // replace with actual Address
//             amount: U256::zero(),
//             reward_token: "0x0000000000000000000000000000000000000000"
//                 .parse()
//                 .unwrap(), // replace with actual Address
//             reward_amount: U256::zero(),
//         };
//         let intent_id = state_manager.create_intent_state(intent).await.unwrap();

//         // Get the intent state
//         let intent_state = state_manager.get_state(intent_id).await.unwrap();

//         assert_eq!(intent_state.intent_id, intent_id);
//     }
// }
