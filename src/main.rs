use std::sync::Arc;

use anyhow::Result;
use artemis_core::engine::Engine;
use tracing::info;

use crate::connectors::Connector;
use crate::diagnostics::logs::configure_logs;
use crate::workflow::engine::configure_engine;
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;
use config::args::Args;
use inventory::Inventory;
use workflow::action::Action;
use workflow::event::Event;

pub mod config;
pub mod connectors;
pub mod error;
pub mod ethereum;
pub mod inventory;
pub mod quote;
pub mod types;
pub mod workflow;

pub mod diagnostics;

#[tokio::main]
async fn main() -> Result<()> {
    configure_logs();

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
