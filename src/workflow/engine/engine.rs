use crate::config::config::Config;
use crate::connectors::connector::Connector;
use crate::inventory::inventory::Inventory;
use crate::quote::interchain_liquidity_hub::interchain_liquidity_hub_quoter::InterchainLiquidityHubQuoter;
use crate::workflow::action::Action;
use crate::workflow::collectors::ethereum::escrow_events_locked_tokens_proof_source::EscrowEventsLockedTokensProofSource;
use crate::workflow::collectors::ethereum::intents_mempool_source::IntentsMempoolSource;
use crate::workflow::collectors::locked_tokens_proofs_collector::LockedTokensProofCollector;
use crate::workflow::collectors::quoted_intents_collector::QuotedIntentsCollector;
use crate::workflow::collectors::swap_intent_collector::SwapIntentCollector;
use crate::workflow::event::Event;
use crate::workflow::executors::ethereum::send_transaction_lock_intent_tokens_handler::SendTransactionLockIntentTokensHandler;
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

    // Set up Ethereum specific clients.
    let intents_mempool_source = IntentsMempoolSource::new(
        connector.clone(),
        config.addresses.intents_mempool_address.clone(),
    );
    let escrow_events_locked_tokens_proof_source =
        EscrowEventsLockedTokensProofSource::new(connector.clone(), config.addresses.clone());
    let send_transaction_lock_intent_tokens_handler =
        SendTransactionLockIntentTokensHandler::new(config.addresses.clone(), connector.clone());

    // Set up collectors.
    let swap_intent_collector = SwapIntentCollector::new(intents_mempool_source);
    let intents_collector = Box::new(swap_intent_collector);
    engine.add_collector(intents_collector);

    let (quoted_intents_collector, quoted_intents_sender) = QuotedIntentsCollector::new();
    engine.add_collector(Box::new(quoted_intents_collector));

    let locked_tokens_proofs_collector =
        LockedTokensProofCollector::new(escrow_events_locked_tokens_proof_source);
    let locked_tokens_collector = Box::new(locked_tokens_proofs_collector);
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
        send_transaction_lock_intent_tokens_handler,
    )));

    engine.add_executor(Box::new(QuoterExecutor::new(
        interchain_liquidity_hub_quoter,
        quoted_intents_sender,
    )));

    engine
}
