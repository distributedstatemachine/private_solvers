use std::sync::Arc;

use crate::workflow::action::Action;
use crate::workflow::collectors::ethereum::intents_book_source::IntentsMempoolSource;
use crate::workflow::collectors::limit_order_intent_collector::LimitOrderIntentCollector;
use crate::workflow::event::Event;
use crate::workflow::executors::post_limit_order_executor::PostLimitOrderExecutor;
use crate::workflow::executors::settle_intent_executor::SettleIntentExecutor;
use crate::workflow::executors::swap_intent_filler_executor::SwapIntentFillerExecutor;
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;
use crate::workflow::strategies::intents_strategy::IntentsStrategy;
use artemis_core::engine::Engine;
use futures::lock::Mutex;
use solver_common::config::Config;
use solver_common::connectors::Connector;
use solver_common::inventory::Inventory;

pub fn configure_engine(
    config: &Config,
    state_manager: InMemoryStateManager,
    connector: Arc<Connector>,
    inventory: Arc<Inventory>,
) -> Engine<Event, Action> {
    // Set up Ethereum specific clients.
    let intents_mempool_source =
        IntentsMempoolSource::new(connector.clone(), config.addresses.intents_mempool_address);

    // let send_transaction_lock_intent_tokens_handler =
    //     SendTransactionLockIntentTokensHandler::new(config.addresses.clone(), connector.clone());
    // let send_transaction_settle_intent_handler =
    //     SendTransactionSettleIntentHandler::new(config.addresses.clone(), connector.clone());
    // let interchain_liquidity_hub_quoter = InterchainLiquidityHubQuoter::new(
    //     connector.clone(),
    //     inventory.clone(),
    //     config.balancer.clone(),
    // );
    // let swap_intent_filler_handler =
    //     SendTransactionPostLimitOrderHandler::new(config.addresses.clone(), connector.clone());

    let state_manager = Arc::new(Mutex::new(state_manager));

    let mut engine = Engine::<Event, Action>::default();

    // Set up collectors.
    let intents_collector = Box::new(LimitOrderIntentCollector::new(intents_mempool_source));
    engine.add_collector(intents_collector);

    // Set up strategies.
    // let intents_strategy = Box::new(IntentsStrategy::new(
    //     state_manager.clone(),
    //     interchain_liquidity_hub_quoter,
    // ));
    // engine.add_strategy(intents_strategy);

    // Set up executors.
    // let (lock_intent_tokens_executor, lock_intent_tokens_confirmation_collector) =
    //     LockIntentTokensExecutor::new(send_transaction_lock_intent_tokens_handler);
    // engine.add_executor(Box::new(lock_intent_tokens_executor));
    // engine.add_collector(lock_intent_tokens_confirmation_collector);

    // let (swap_intent_filler_executor, swap_intent_filler_confirmation_collector) =
    //     SwapIntentFillerExecutor::new(swap_intent_filler_handler);
    // engine.add_executor(Box::new(swap_intent_filler_executor));
    // engine.add_collector(swap_intent_filler_confirmation_collector);

    // let settle_intent_executor = Box::new(SettleIntentExecutor::new(
    //     send_transaction_settle_intent_handler,
    // ));
    // engine.add_executor(settle_intent_executor);

    engine
}
