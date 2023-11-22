use std::sync::Arc;

use anyhow::Result;
use artemis_core::engine::Engine;
use artemis_core::types::CollectorMap;
use clap::Parser;
use ethers::signers::{LocalWallet, Signer};
use inventory::inventory::Inventory;
use tracing::{info, Level};
use tracing_subscriber::filter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use strategies::types::{Action, Event};

use crate::collectors::intents_collector::IntentsCollector;
use crate::config::chain::SEPOLIA_CHAIN_ID;
use crate::config::config::Config;
use crate::connectors::connector::Connector;
use crate::executors::intents_executor::IntentsExecutor;
use crate::strategies::intents_strategy::IntentsStrategy;

pub mod collectors;
pub mod config;
pub mod connectors;
pub mod executors;
pub mod inventory;
pub mod strategies;
pub mod types;

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

    let connector = Connector::new(config.clone(), wallet.clone()).await?;
    let connector = Arc::new(connector);
    let inventory = Inventory::new(config.clone(), connector.clone()).await?;
    let inventory = Arc::new(inventory);

    // Set up engine.
    let engine = configure_engine(&config, connector.clone(), inventory);

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

fn configure_engine(
    config: &Config,
    connector: Arc<Connector>,
    inventory: Arc<Inventory>,
) -> Engine<Event, Action> {
    let mut engine = Engine::<Event, Action>::default();

    let rpc_client = connector.get_rpc_client(SEPOLIA_CHAIN_ID).unwrap();
    let ws_client = connector.get_ws_client(SEPOLIA_CHAIN_ID).unwrap();

    // Set up intents collector.
    let intents_collector = Box::new(IntentsCollector::new(
        ws_client,
        config.addresses.intents_mempool_address.clone(),
    ));
    let intents_collector = CollectorMap::new(intents_collector, Event::NewSwapIntent);
    engine.add_collector(Box::new(intents_collector));

    // Set up intents strategy.
    let strategy = IntentsStrategy::new(
        connector,
        inventory,
        config.addresses.vault_address,
        config.balancer.clone(),
    );
    engine.add_strategy(Box::new(strategy));

    // Set up intents executor.
    engine.add_executor(Box::new(IntentsExecutor::new(
        rpc_client,
        config.addresses.intents_mempool_address.clone(),
    )));

    engine
}

async fn run_engine(engine: Engine<Event, Action>) {
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("Result: {:?}", res);
        }
    }
}
