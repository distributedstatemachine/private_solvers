use std::collections::HashMap;

use crate::workflow::state::state_manager::StateManager;
use crate::workflow::state::IntentState;
use intentbook_matchmaker::types::spoke_chain_call::SpokeChainCall;
use solver_common::types::intent_id::IntentId;
use sqlx::migrate::Migrator;
use sqlx::PgPool;
use sqlx::{Encode, Type};
use std::env;
use std::io;
use tokio_postgres::{Client, Error, NoTls};

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

impl Default for DatabaseStateManager {
    fn default() -> Self {
        Self {
            intents: HashMap::new(),
            db_client: None,
        }
    }
}

impl StateManager for DatabaseStateManager {
    async fn update_state(
        &mut self,
        intent_id: IntentId,
        new_state: IntentState,
    ) -> Result<(), sqlx::Error> {
        if let Some(pool) = &self.db_client {
            let mut connection = pool.acquire().await?;
            sqlx::query("UPDATE IntentState SET status = $1, intent_bid_id = $2, spoke_chain_call = $3, block_number = $4 WHERE intent_id = $5")
                .bind(new_state.status)
                .bind(new_state.intent_bid_id)
                .bind(new_state.spoke_chain_call)
                .bind(new_state.block_number)
                .bind(intent_id)
                .execute(&mut connection)
                .await?;
        }
        Ok(())
    }
    async fn get_state(&self, intent_id: IntentId) -> Result<IntentState, sqlx::Error> {
        if let Some(pool) = &self.db_client {
            let mut connection = pool.acquire().await?;
            let intent_state: IntentState =
                sqlx::query_as("SELECT * FROM IntentState WHERE intent_id = $1")
                    .bind(intent_id)
                    .fetch_one(&mut connection)
                    .await?;
            Ok(intent_state)
        } else {
            Err(sqlx::Error::PoolClosed)
        }
    }

    async fn get_all_intents(&self) -> Result<Vec<IntentState>, sqlx::Error> {
        if let Some(pool) = &self.db_client {
            let mut connection = pool.acquire().await?;
            let intents: Vec<IntentState> = sqlx::query_as("SELECT * FROM IntentState")
                .fetch_all(&mut connection)
                .await?;
            Ok(intents)
        } else {
            Err(sqlx::Error::PoolClosed)
        }
    }

    async fn create_intent_state(
        &mut self,
        intent: SpokeChainCall,
    ) -> Result<IntentId, sqlx::Error> {
        if let Some(pool) = &self.db_client {
            let mut connection = pool.acquire().await?;
            let intent_id = intent.intent_id;
            let intent_state = IntentState::new(intent);
            let spoke_chain_call = to_string(&intent_state.spoke_chain_call).unwrap();
            sqlx::query("INSERT INTO IntentState (intent_id, status, intent_bid_id, spoke_chain_call, block_number) VALUES ($1, $2, $3, $4, $5)")
                .bind(intent_id)
                .bind(intent_state.status)
                .bind(intent_state.intent_bid_id)
                .bind(spoke_chain_call)
                .bind(intent_state.block_number)
                .execute(&mut connection)
                .await?;
            Ok(intent_id)
        } else {
            Err(sqlx::Error::PoolClosed)
        }
    }

    async fn update_intent_state<F>(
        &mut self,
        intent_id: IntentId,
        updater: F,
    ) -> Result<Option<IntentState>, sqlx::Error>
    where
        F: FnOnce(&mut IntentState),
    {
        if let Some(pool) = &self.db_client {
            let mut connection = pool.acquire().await?;
            let mut intent_state: IntentState =
                sqlx::query_as("SELECT * FROM IntentState WHERE intent_id = $1")
                    .bind(intent_id)
                    .fetch_one(&mut connection)
                    .await?;
            updater(&mut intent_state);
            let spoke_chain_call = to_string(&intent_state.spoke_chain_call).unwrap();
            sqlx::query("UPDATE IntentState SET status = $1, intent_bid_id = $2, spoke_chain_call = $3, block_number = $4 WHERE intent_id = $5")
                .bind(intent_state.status)
                .bind(intent_state.intent_bid_id)
                .bind(spoke_chain_call)
                .bind(intent_state.block_number)
                .bind(intent_id)
                .execute(&mut connection)
                .await?;
            Ok(Some(intent_state))
        } else {
            Err(sqlx::Error::PoolClosed)
        }
    }

    async fn get_in_progress_intents(&self) -> Result<Vec<IntentState>, sqlx::Error> {
        if let Some(pool) = &self.db_client {
            let mut connection = pool.acquire().await?;
            let intents: Vec<IntentState> =
                sqlx::query_as("SELECT * FROM IntentState WHERE status = 'inprogress'")
                    .fetch_all(&mut connection)
                    .await?;
            Ok(intents)
        } else {
            Err(sqlx::Error::PoolClosed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    fn is_postgres_running() -> bool {
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            let pool = init_pool("postgres://postgres:postgres@localhost:5432/solver");
            Arc::new(pool).get().is_ok()
        }));

        result.unwrap_or(false)
    }

    fn start_postgres() {
        if !is_postgres_running() {
            let status = std::process::Command::new("docker")
                .args([
                    "run",
                    "--rm",
                    "--name",
                    "solver-postgres",
                    "-e",
                    "POSTGRES_PASSWORD=postgres",
                    "-e",
                    "POSTGRES_USER=postgres",
                    "-e",
                    "POSTGRES_DB=solver",
                    "-p",
                    "5432:5432",
                    "-d",
                    "postgres:14",
                ])
                .status()
                .expect("failed to execute process");
            println!("output: {:?}", status);

            // sleep for a tiny bit to allow it to boot
            std::thread::sleep(Duration::from_secs(5));

            println!("Postgres started");
        }

        //   TODO: Run migrations

        println!("output: {:?}", cmd);

        std::thread::sleep(Duration::from_secs(5));

        println!("Running migration");
    }

    #[tokio::test]
    async fn test_create_and_get_intent_state() {
        // Set up the test database
        let database_url = env::var("TEST_DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();

        let mut state_manager = DatabaseStateManager {
            intents: HashMap::new(),
            db_client: Some(pool),
        };

        // Create a new intent state
        let intent = SpokeChainCall {
            intent_id: "some_intent_id".into(),
            signature: Bytes::default(),
            author: "0x0000000000000000000000000000000000000000"
                .parse()
                .unwrap(),
            chain_id: 1.into(),
            contract_to_call: "0x0000000000000000000000000000000000000000"
                .parse()
                .unwrap(),
            call_data: Bytes::default(),
            token: "0x0000000000000000000000000000000000000000"
                .parse()
                .unwrap(), // replace with actual Address
            amount: U256::zero(),
            reward_token: "0x0000000000000000000000000000000000000000"
                .parse()
                .unwrap(), // replace with actual Address
            reward_amount: U256::zero(),
        };
        let intent_id = state_manager.create_intent_state(intent).await.unwrap();

        // Get the intent state
        let intent_state = state_manager.get_state(intent_id).await.unwrap();

        assert_eq!(intent_state.intent_id, intent_id);
    }
}
