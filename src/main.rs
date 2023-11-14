use std::sync::Arc;

use crate::collectors::intents_collector::IntentsCollector;
use crate::executors::intents_executor::IntentsExecutor;
use crate::strategies::intents_strategy::IntentsStrategy;
use anyhow::Result;
use artemis_core::engine::Engine;
use artemis_core::types::CollectorMap;
use clap::Parser;
use ethers::prelude::MiddlewareBuilder;
use ethers::providers::{Provider, Ws};
use ethers::signers::{LocalWallet, Signer};
use strategies::types::{Action, Event};
use tracing::{info, Level};
use tracing_subscriber::filter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod collectors;
pub mod executors;
pub mod strategies;
pub mod types;

#[derive(Parser, Debug)]
pub struct Args {
    /// Ethereum node WS endpoint.
    #[arg(long)]
    pub wss: String,

    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter = filter::Targets::new()
        .with_target("artemis_core", Level::INFO)
        .with_target("khalani_solver", Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let args = Args::parse();

    let wallet: LocalWallet = args.private_key.parse::<LocalWallet>().unwrap();

    let address = wallet.address();
    info!("Solver address: {}", address);

    // Set up engine.
    let mut engine = Engine::<Event, Action>::default();

    // Set up ethers provider.
    let ws = Ws::connect(args.wss).await?;
    let provider = Provider::new(ws);
    let address = wallet.address();
    let provider = Arc::new(provider.nonce_manager(address).with_signer(wallet.clone()));

    // Set up intents collector.
    let intents_collector = Box::new(IntentsCollector::new(provider.clone()));
    let intents_collector = CollectorMap::new(intents_collector, Event::NewSwapIntent);
    engine.add_collector(Box::new(intents_collector));

    // Set up intents strategy.
    let strategy = IntentsStrategy::new();
    engine.add_strategy(Box::new(strategy));

    // Set up intents executor.
    engine.add_executor(Box::new(IntentsExecutor::new()));

    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("Result: {:?}", res);
        }
    }
    Ok(())
}
