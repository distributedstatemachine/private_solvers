use anyhow::Result;
use artemis_core::engine::Engine;
use solver_common::config::args::Args;
use solver_common::connectors::Connector;
use solver_common::diagnostics::logs::configure_logs;
use solver_common::inventory::Inventory;
use std::sync::Arc;
use tracing::info;

pub mod quote;
pub mod types;
pub mod workflow;

use workflow::action::Action;
use workflow::event::Event;

use workflow::engine::configure_engine;
use workflow::state::in_memory_state_manager::InMemoryStateManager;

#[tokio::main]
async fn main() -> Result<()> {
    configure_logs();
    info!("Starting Cross Chain Market Maker");

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

async fn run_engine(engine: Engine<Event, Action>) {
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("Result: {:?}", res);
        }
    }
}
