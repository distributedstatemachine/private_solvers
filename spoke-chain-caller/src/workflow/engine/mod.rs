use std::sync::Arc;

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
use solver_common::config::Config;
use solver_common::connectors::Connector;
use solver_common::inventory::Inventory;

pub fn configure_agent_engine(
    config: &Config,
    state_manager: YourStateManager,
    connector: Arc<Connector>,
    inventory: Arc<Inventory>,
) -> Engine<IntentMatchEvent, Action> {
    // Set up SpokeChainCall specific intent source.
    let spoke_chain_call_intent_source =
        SpokeChainCallIntentSource::new(connector.clone(), config.addresses.spoke_chain_call_source);

    let spoke_chain_call_executor =
        SpokeChainCallExecutor::new(config.addresses.clone(), connector.clone());

    let state_manager = Arc::new(Mutex::new(state_manager));

    let mut engine = Engine::<IntentMatchEvent, Action>::default();

    // Set up collectors.
    let intent_match_collector = Box::new(IntentMatchCollector::new(spoke_chain_call_intent_source));
    engine.add_collector(intent_match_collector);

    // Set up strategies.
    let spoke_chain_call_strategy = Box::new(SpokeChainCallStrategy::new(
        state_manager.clone(),
        spoke_chain_call_executor,
    ));
    engine.add_strategy(spoke_chain_call_strategy);

    engine
}
