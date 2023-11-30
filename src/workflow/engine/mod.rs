use std::sync::Arc;

use crate::config::Config;
use crate::connectors::Connector;
use crate::inventory::Inventory;
use crate::quote::interchain_liquidity_hub::interchain_liquidity_hub_quoter::InterchainLiquidityHubQuoter;
use crate::workflow::action::Action;
use crate::workflow::collectors::ethereum::intents_mempool_source::IntentsMempoolSource;
use crate::workflow::collectors::proofs::gmp_verifier_proof_source::GmpEventVerifierProofSource;
use crate::workflow::collectors::proofs::proofs_collector::ProofsCollector;
use crate::workflow::collectors::swap_intent_collector::SwapIntentCollector;
use crate::workflow::event::Event;
use crate::workflow::executors::ethereum::send_transaction_lock_intent_tokens_handler::SendTransactionLockIntentTokensHandler;
use crate::workflow::executors::ethereum::send_transaction_settle_intent_handler::SendTransactionSettleIntentHandler;
use crate::workflow::executors::ethereum::send_transaction_swap_intent_filler_handler::SendTransactionSwapIntentFillerHandler;
use crate::workflow::executors::lock_tokens_executor::LockIntentTokensExecutor;
use crate::workflow::executors::settle_intent_executor::SettleIntentExecutor;
use crate::workflow::executors::swap_intent_filler_executor::SwapIntentFillerExecutor;
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;
use crate::workflow::strategies::intents_strategy::IntentsStrategy;
use artemis_core::engine::Engine;
use futures::lock::Mutex;

pub fn configure_engine(
    config: &Config,
    state_manager: InMemoryStateManager,
    connector: Arc<Connector>,
    inventory: Arc<Inventory>,
) -> Engine<Event, Action> {
    // Set up Ethereum specific clients.
    let intents_mempool_source =
        IntentsMempoolSource::new(connector.clone(), config.addresses.intents_mempool_address);
    let gmp_event_verifier_sources: Vec<GmpEventVerifierProofSource> = config
        .addresses
        .verifiers
        .iter()
        .map(|verifier_config| {
            GmpEventVerifierProofSource::new(connector.clone(), verifier_config.clone())
        })
        .collect();

    let send_transaction_lock_intent_tokens_handler =
        SendTransactionLockIntentTokensHandler::new(config.addresses.clone(), connector.clone());
    let send_transaction_settle_intent_handler =
        SendTransactionSettleIntentHandler::new(config.addresses.clone(), connector.clone());
    let interchain_liquidity_hub_quoter = InterchainLiquidityHubQuoter::new(
        connector.clone(),
        inventory.clone(),
        config.balancer.clone(),
    );
    let swap_intent_filler_handler =
        SendTransactionSwapIntentFillerHandler::new(config.addresses.clone(), connector.clone());

    let state_manager = Arc::new(Mutex::new(state_manager));

    let mut engine = Engine::<Event, Action>::default();

    // Set up collectors.
    let intents_collector = Box::new(SwapIntentCollector::new(intents_mempool_source));
    engine.add_collector(intents_collector);

    for gmp_event_verifier_source in gmp_event_verifier_sources {
        let proof_collector = Box::new(ProofsCollector::new(
            gmp_event_verifier_source,
            state_manager.clone(),
            connector.clone(),
        ));
        engine.add_collector(proof_collector);
    }

    // Set up strategies.
    let intents_strategy = Box::new(IntentsStrategy::new(
        state_manager.clone(),
        interchain_liquidity_hub_quoter,
    ));
    engine.add_strategy(intents_strategy);

    // Set up executors.
    let (lock_intent_tokens_executor, lock_intent_tokens_confirmation_collector) =
        LockIntentTokensExecutor::new(send_transaction_lock_intent_tokens_handler);
    engine.add_executor(Box::new(lock_intent_tokens_executor));
    engine.add_collector(lock_intent_tokens_confirmation_collector);

    let (swap_intent_filler_executor, swap_intent_filler_confirmation_collector) =
        SwapIntentFillerExecutor::new(swap_intent_filler_handler);
    engine.add_executor(Box::new(swap_intent_filler_executor));
    engine.add_collector(swap_intent_filler_confirmation_collector);

    let settle_intent_executor = Box::new(SettleIntentExecutor::new(
        send_transaction_settle_intent_handler,
    ));
    engine.add_executor(settle_intent_executor);

    engine
}
