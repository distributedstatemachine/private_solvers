use std::sync::Arc;

use anyhow::Result;
use artemis_core::engine::Engine;
use solver_common::config::args::Args;
use solver_common::connectors::Connector;
use solver_common::diagnostics::logs::configure_logs;
use solver_common::inventory::Inventory;
use solver_common::workflow::run_engine;
use spoke_chain_caller::workflow::action::Action;
use spoke_chain_caller::workflow::collectors::ethereum::spoke_chain_call_intents_book_source::SpokeChainCallIntentbookSource;
use spoke_chain_caller::workflow::engine::configure_engine;
use spoke_chain_caller::workflow::event::Event;
use spoke_chain_caller::workflow::state::database_state_manager::DatabaseStateManager;
use spoke_chain_caller::workflow::state::state_manager::StateManager;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    configure_logs();
    info!("Starting Spoke Chain Caller");

    let (config, wallet) = Args::get_config_and_wallet().await?;

    let connector = Connector::new(config.clone(), wallet).await?;
    let connector = Arc::new(connector);
    let mut state_manager = DatabaseStateManager::new().await?;

    let inventory = Arc::new(Inventory::new(config.clone(), connector.clone()).await?);
    let spoke_chain_caller_intent_source = &SpokeChainCallIntentbookSource::new(
        connector.clone(),
        inventory.clone(),
        config.addresses.intentbook_addresses.clone(),
    );

    // Sync state.
    state_manager
        .fetch_state(spoke_chain_caller_intent_source)
        .await?;

    // TODO: Retry/ Continue InProgress intents.

    // Set up engine.
    let engine: Engine<Event, Action> = configure_engine(&config, connector.clone(), state_manager);

    // Start engine.
    run_engine(engine).await;

    Ok(())
}
