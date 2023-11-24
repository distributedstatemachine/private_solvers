use std::sync::Arc;

use crate::config::chain::KHALANI_CHAIN_ID;
use artemis_core::engine::Engine;

use crate::config::config::Config;
use crate::connectors::connector::Connector;
use crate::inventory::inventory::Inventory;
use crate::quote::intent_quoter::IntentQuoter;
use crate::quote::interchain_liquidity_hub::interchain_liquidity_hub_quoter::InterchainLiquidityHubQuoter;
use crate::workflow::action::Action;
use crate::workflow::collectors::ethereum::gmp_verifier_proof_source::GmpEventVerifierProofSource;
use crate::workflow::collectors::ethereum::intents_mempool_source::IntentsMempoolSource;
use crate::workflow::collectors::proofs_collector::{ProofSource, ProofsCollector};
use crate::workflow::collectors::swap_intent_collector::{SwapIntentCollector, SwapIntentSource};
use crate::workflow::event::Event;
use crate::workflow::executors::ethereum::send_transaction_lock_intent_tokens_handler::SendTransactionLockIntentTokensHandler;
use crate::workflow::executors::ethereum::send_transaction_settle_intent_handler::SendTransactionSettleIntentHandler;
use crate::workflow::executors::lock_tokens_executor::{
    LockIntentTokensExecutor, LockIntentTokensHandler,
};
use crate::workflow::executors::settle_intent_executor::{
    SettleIntentExecutor, SettleIntentHandler,
};
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;
use crate::workflow::state::state_manager::StateManager;
use crate::workflow::strategies::intents_strategy::IntentsStrategy;

pub fn configure_engine(
    config: &Config,
    state_manager: InMemoryStateManager,
    connector: Arc<Connector>,
    inventory: Arc<Inventory>,
) -> Engine<Event, Action> {
    // Set up Ethereum specific clients.
    let intents_mempool_source = IntentsMempoolSource::new(
        connector.clone(),
        config.addresses.intents_mempool_address.clone(),
    );
    let khalani_gmp_verifier_proof_source = GmpEventVerifierProofSource::new(
        connector.clone(),
        KHALANI_CHAIN_ID,
        config.addresses.clone(),
    );
    let send_transaction_lock_intent_tokens_handler =
        SendTransactionLockIntentTokensHandler::new(config.addresses.clone(), connector.clone());
    let send_transaction_settle_intent_handler =
        SendTransactionSettleIntentHandler::new(config.addresses.clone(), connector.clone());
    let interchain_liquidity_hub_quoter = InterchainLiquidityHubQuoter::new(
        connector.clone(),
        inventory.clone(),
        config.addresses.clone(),
        config.balancer.clone(),
    );

    register_engine(
        state_manager,
        intents_mempool_source,
        khalani_gmp_verifier_proof_source,
        send_transaction_lock_intent_tokens_handler,
        send_transaction_settle_intent_handler,
        interchain_liquidity_hub_quoter,
    )
}

fn register_engine<
    'lifetime,
    _StateManager,
    _SwapIntentsSource,
    _ProofSource,
    _LockIntentTokensHandler,
    _SettleIntentHandler,
    _IntentQuoter,
>(
    state_manager: _StateManager,
    swap_intents_source: _SwapIntentsSource,
    proof_source: _ProofSource,
    lock_intent_tokens_handler: _LockIntentTokensHandler,
    settle_intent_handler: _SettleIntentHandler,
    intent_quoter: _IntentQuoter,
) -> Engine<Event, Action>
where
    _StateManager: StateManager + Send + Sync + 'static,
    _SwapIntentsSource: SwapIntentSource + Send + Sync + 'static,
    _ProofSource: ProofSource + Send + Sync + 'static,
    _LockIntentTokensHandler: LockIntentTokensHandler + Send + Sync + 'static,
    _SettleIntentHandler: SettleIntentHandler + Send + Sync + 'static,
    _IntentQuoter: IntentQuoter + Send + Sync + 'static,
{
    let mut engine = Engine::<Event, Action>::default();

    // Set up collectors.
    let intents_collector = Box::new(SwapIntentCollector::new(swap_intents_source));
    engine.add_collector(intents_collector);

    let proof_collector = Box::new(ProofsCollector::new(proof_source));
    engine.add_collector(proof_collector);

    // Set up strategies.
    let intents_strategy = Box::new(IntentsStrategy::new(state_manager, intent_quoter));
    engine.add_strategy(intents_strategy);

    // Set up executors.
    let lock_intent_tokens_executor = LockIntentTokensExecutor::new(lock_intent_tokens_handler);
    engine.add_executor(Box::new(lock_intent_tokens_executor));

    let settle_intent_executor = Box::new(SettleIntentExecutor::new(settle_intent_handler));
    engine.add_executor(settle_intent_executor);

    engine
}
