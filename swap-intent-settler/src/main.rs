use anyhow::Result;
use solver_common::config::args::Args;
use solver_common::connectors::Connector;
use solver_common::diagnostics::logs::configure_logs;
use solver_common::inventory::Inventory;
use solver_common::workflow::run_engine;
use std::sync::Arc;
use tracing::info;

pub mod quote;
pub mod types;
pub mod workflow;

use workflow::engine::configure_engine;
use workflow::state::in_memory_state_manager::InMemoryStateManager;

#[tokio::main]
async fn main() -> Result<()> {
    configure_logs();
    info!("Starting Swap Intent Filler");

    let (config, wallet) = Args::get_config_and_wallet()?;

    let state_manager = InMemoryStateManager::new();

    let connector = Connector::new(config.clone(), wallet.clone()).await?;
    let connector = Arc::new(connector);
    let inventory = Inventory::new(config.clone(), connector.clone()).await?;
    let inventory = Arc::new(inventory);

    // Set up engine.
    let engine = configure_engine(&config, state_manager, connector.clone(), inventory);

    // Start engine.
    run_engine(engine).await;

    Ok(())
}
