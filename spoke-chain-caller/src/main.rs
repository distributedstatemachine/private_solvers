use anyhow::Result;
use artemis_core::engine::Engine;
use solver_common::config::args::Args;
use solver_common::connectors::Connector;
use solver_common::diagnostics::logs::configure_logs;
use solver_common::inventory::Inventory;
use std::sync::Arc;
use tracing::info;

pub mod types;
pub mod workflow;

use crate::workflow::action::Action;
use crate::workflow::event::Event;
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;
use workflow::engine::configure_engine;

#[tokio::main]
async fn main() -> Result<()> {
    configure_logs();
    info!("Starting Spoke Chain Caller");

    let (config, wallet) = Args::get_config_and_wallet()?;

    let connector = Connector::new(config.clone(), wallet.clone()).await?;
    let connector = Arc::new(connector);
    let inventory = Inventory::new(config.clone(), connector.clone()).await?;
    let _inventory = Arc::new(inventory);

    let state_manager = InMemoryStateManager::new();

    // Set up engine.
    let engine: Engine<Event, Action> = configure_engine(&config, connector.clone(), state_manager);

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
