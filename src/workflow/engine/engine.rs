use crate::config::config::Config;
use crate::connectors::connector::Connector;
use crate::inventory::inventory::Inventory;
use crate::quote::interchain_liquidity_hub_quoter::InterchainLiquidityHubQuoter;
use crate::workflow::action::Action;
use crate::workflow::collectors::locked_tokens_collector::LockedTokensCollector;
use crate::workflow::collectors::mempool_intents_collector::MempoolIntentsCollector;
use crate::workflow::collectors::quoted_intents_collector::QuotedIntentsCollector;
use crate::workflow::event::Event;
use crate::workflow::executors::lock_tokens_executor::LockIntentTokensExecutor;
use crate::workflow::executors::quoter_executor::QuoterExecutor;
use crate::workflow::executors::settle_intent_executor::SettleIntentExecutor;
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;
use crate::workflow::strategies::intents_strategy::IntentsStrategy;
use artemis_core::engine::Engine;
use std::sync::Arc;

pub fn configure_engine(
    config: &Config,
    state_manager: InMemoryStateManager,
    connector: Arc<Connector>,
    inventory: Arc<Inventory>,
) -> Engine<Event, Action> {
    let mut engine = Engine::<Event, Action>::default();

    // Set up collectors.
    let intents_collector = Box::new(MempoolIntentsCollector::new(
        connector.clone(),
        config.addresses.intents_mempool_address.clone(),
    ));
    engine.add_collector(intents_collector);

    let (quoted_intents_collector, quoted_intents_sender) = QuotedIntentsCollector::new();
    engine.add_collector(Box::new(quoted_intents_collector));

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
    let strategy = IntentsStrategy::new(state_manager);
    engine.add_strategy(Box::new(strategy));

    // Set up executors.
    engine.add_executor(Box::new(SettleIntentExecutor::new(
        config.addresses.clone(),
        connector.clone(),
    )));

    engine.add_executor(Box::new(LockIntentTokensExecutor::new(
        config.addresses.clone(),
        connector.clone(),
    )));

    engine.add_executor(Box::new(QuoterExecutor::new(
        interchain_liquidity_hub_quoter,
        quoted_intents_sender,
    )));

    engine
}
