use anyhow::Result;
use artemis_core::engine::Engine;
use solver_common::config::args::Args;
use solver_common::config::wallet::WalletSigner;
use solver_common::connectors::Connector;
use solver_common::diagnostics::logs::configure_logs;
use solver_common::inventory::Inventory;
use solver_common::workflow::run_engine;
use std::sync::Arc;
use tracing::info;

pub mod workflow;

use crate::workflow::action::Action;
use crate::workflow::event::Event;
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;
use workflow::engine::configure_engine;

#[tokio::main]
async fn main() -> Result<()> {
    configure_logs();
    info!("Starting Spoke Chain Caller");

    let (config, wallet) = Args::get_config_and_wallet().await?;

    let state_manager = InMemoryStateManager::new();

    let connector = match wallet {
        WalletSigner::Local(wallet) => {
            Connector::new(config.clone(), WalletSigner::Local(wallet)).await?
        }
        WalletSigner::Aws(signer) => {
            Connector::new(config.clone(), WalletSigner::Aws(signer)).await?
        }
    };
    let connector = Arc::new(connector);
    let inventory = Inventory::new(config.clone(), connector.clone()).await?;
    let _inventory = Arc::new(inventory);

    // Set up engine.
    let engine: Engine<Event, Action> = configure_engine(&config, connector.clone(), state_manager);

    // Start engine.
    run_engine(engine).await;

    Ok(())
}
