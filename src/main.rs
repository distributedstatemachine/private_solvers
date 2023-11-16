use std::str::FromStr;
use std::sync::Arc;

use crate::collectors::intents_collector::IntentsCollector;
use crate::config::config::Config;
use crate::executors::intents_executor::IntentsExecutor;
use crate::strategies::intents_strategy::IntentsStrategy;
use anyhow::Result;
use artemis_core::engine::Engine;
use artemis_core::types::CollectorMap;
use clap::Parser;
use ethers::middleware::Middleware;
use ethers::prelude::MiddlewareBuilder;
use ethers::providers::{Http, Provider, Ws};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Address;
use strategies::types::{Action, Event};
use tracing::{info, Level};
use tracing_subscriber::filter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod collectors;
pub mod config;
pub mod executors;
pub mod inventory;
pub mod strategies;
pub mod types;

#[derive(Parser, Debug)]
pub struct Args {
    /// Ethereum node HTTPS endpoint.
    #[arg(long)]
    pub rpc: String,

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

    let sender_provider =
        Provider::<Http>::try_from(args.rpc).expect("Failed to instantiate HTTP Provider");
    let chain_id = sender_provider.get_chainid().await?.as_u64();

    let wallet: LocalWallet = args
        .private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);

    let address = wallet.address();
    info!("Solver address: {}", address);

    // Set up engine.
    let mut engine = Engine::<Event, Action>::default();

    // Set up ethers provider.
    let ws_client = Ws::connect(args.wss).await?;
    let ws_provider = Provider::new(ws_client);
    let ws_provider = Arc::new(
        ws_provider
            .nonce_manager(address)
            .with_signer(wallet.clone()),
    );

    let sender_provider = Arc::new(sender_provider.nonce_manager(address).with_signer(wallet));

    let config = Config {
        intents_mempool_address: Address::from_str("0xec60021da4f7d482f020bbf0aa8b3d6ea73345a2")
            .unwrap(),
    };

    // Set up intents collector.
    let intents_collector = Box::new(IntentsCollector::new(ws_provider.clone(), config.clone()));
    let intents_collector = CollectorMap::new(intents_collector, Event::NewSwapIntent);
    engine.add_collector(Box::new(intents_collector));

    // Set up intents strategy.
    let strategy = IntentsStrategy::new();
    engine.add_strategy(Box::new(strategy));

    // Set up intents executor.
    engine.add_executor(Box::new(IntentsExecutor::new(
        sender_provider,
        config.clone(),
    )));

    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("Result: {:?}", res);
        }
    }
    Ok(())
}
