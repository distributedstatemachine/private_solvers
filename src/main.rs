use std::sync::Arc;

use anyhow::Result;
use artemis_core::engine::Engine;
use clap::Parser;
use ethers::signers::{LocalWallet, Signer};
use inventory::inventory::Inventory;
use tokio::sync::mpsc::channel;
use tracing::{info, Level};
use tracing_subscriber::filter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use strategies::types::{Action, Event};

use crate::collectors::intents_collector::IntentsCollector;
use crate::collectors::locked_tokens_collector::LockedTokensCollector;
use crate::collectors::quoted_intents_collector::QuotedIntentsCollector;
use crate::config::config::Config;
use crate::connectors::connector::Connector;
use crate::executors::intents_executor::IntentsExecutor;
use crate::executors::quoter_executor::QuoterExecutor;
use crate::quote::interchain_liquidity_hub_quoter::InterchainLiquidityHubQuoter;
use crate::quote::quoted_intent::QuotedIntent;
use crate::strategies::intents_strategy::IntentsStrategy;

pub mod collectors;
pub mod config;
pub mod connectors;
pub mod ethereum;
pub mod executors;
pub mod inventory;
pub mod quote;
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

    let (quoted_intents_sender, quoted_intents_receiver) = channel::<QuotedIntent>(512);

    // Set up collectors.
    let intents_collector = Box::new(IntentsCollector::new(
        connector.clone(),
        config.addresses.intents_mempool_address.clone(),
    ));
    engine.add_collector(intents_collector);

    let quoted_intents_collector = Box::new(QuotedIntentsCollector::new(quoted_intents_receiver));
    engine.add_collector(quoted_intents_collector);

    let locked_tokens_collector = Box::new(LockedTokensCollector::new(
        connector.clone(),
        config.addresses.clone(),
    ));
    engine.add_collector(locked_tokens_collector);

    let interchain_liquidity_hub_quoter = InterchainLiquidityHubQuoter::new(
        connector.clone(),
        inventory.clone(),
        config.addresses.clone(),
        config.balancer.clone(),
    );

    // Set up strategies.
    let strategy = IntentsStrategy::new();
    engine.add_strategy(Box::new(strategy));

    // Set up executors.
    engine.add_executor(Box::new(IntentsExecutor::new(
        config.addresses.clone(),
        connector.clone(),
    )));

    engine.add_executor(Box::new(QuoterExecutor::new(
        interchain_liquidity_hub_quoter,
        quoted_intents_sender,
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
