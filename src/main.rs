use std::sync::Arc;

use anyhow::Result;
use artemis_core::engine::Engine;
use clap::Parser;
use ethers::signers::{LocalWallet, Signer};
use tracing::{info, Level};
use tracing_subscriber::filter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use inventory::inventory::Inventory;
use workflow::action::Action;
use workflow::event::Event;

use crate::config::config::Config;
use crate::connectors::connector::Connector;
use crate::workflow::engine::engine::configure_engine;
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;

pub mod config;
pub mod connectors;
pub mod ethereum;
pub mod inventory;
pub mod quote;
pub mod types;
pub mod workflow;

#[derive(Parser, Debug)]
pub struct Args {
    // TODO: move to the config file too.
    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: String,

    #[arg(long)]
    pub config_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    configure_logs();

    let args = Args::parse();

    let config = Config::read_config(args.config_file.as_str()).unwrap();
    info!("Config: {:?}", &config);

    let wallet: LocalWallet = args.private_key.parse::<LocalWallet>().unwrap();
    let address = wallet.address();
    info!("Solver address: {}", address);

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

fn configure_logs() {
    let filter = filter::Targets::new()
        .with_target("artemis_core", Level::INFO)
        .with_target("khalani_solver", Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();
}

async fn run_engine(engine: Engine<Event, Action>) {
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("Result: {:?}", res);
        }
    }
}
