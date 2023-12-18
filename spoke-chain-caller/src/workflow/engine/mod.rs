use std::sync::Arc;

use crate::workflow::action::Action;
use crate::workflow::collectors::spoke_chain_call_collector::SpokeChainCallCollector;
use crate::workflow::event::Event;
use crate::workflow::executors::ethereum::send_transaction_match_intent_handler::SendTransactionMatchIntentHandler;
use crate::workflow::strategies::intents_strategy::IntentsStrategy;

use crate::workflow::collectors::ethereum::spoke_chain_call_intentbook_source::SpokeChainCallIntentbookSource;
use crate::workflow::executors::call_spoke_executor::CallSpokeExecutor;
use crate::workflow::executors::ethereum::send_transaction_call_spoke_handler::SendTransactionCallSpokeHandler;
use crate::workflow::executors::match_intent_executor::MatchIntentExecutor;
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;
use artemis_core::engine::Engine;
use futures::lock::Mutex;
use solver_common::config::Config;
use solver_common::connectors::Connector;

pub fn configure_engine(
    config: &Config,
    connector: Arc<Connector>,
    state_manager: InMemoryStateManager,
) -> Engine<Event, Action> {
    // Set up SpokeChainCall specific intent source.
    let state_manager = Arc::new(Mutex::new(state_manager));

    let spoke_chain_call_intent_source = SpokeChainCallIntentbookSource::new(
        connector.clone(),
        config
            .addresses
            .intentbook_addresses
            .spoke_chain_call_intentbook,
    );

    let send_transaction_match_intent_handler =
        SendTransactionMatchIntentHandler::new(config.addresses.clone(), connector.clone());

    let call_spoke_handler =
        SendTransactionCallSpokeHandler::new(config.addresses.clone(), connector.clone());

    let mut engine = Engine::<Event, Action>::default();

    // Set up collectors.
    let intents_collector = Box::new(SpokeChainCallCollector::new(spoke_chain_call_intent_source));
    engine.add_collector(intents_collector);

    // Set up strategies.
    let intents_strategy = Box::new(IntentsStrategy::new(state_manager));
    engine.add_strategy(intents_strategy);

    // Set up executors.
    let (call_spoke_executor, call_spoke_executor_confirmation_collector) =
        CallSpokeExecutor::new(call_spoke_handler);
    engine.add_executor(Box::new(call_spoke_executor));
    engine.add_collector(call_spoke_executor_confirmation_collector);

    let (match_intent_executor, match_intent_executor_collector) =
        MatchIntentExecutor::new(send_transaction_match_intent_handler);
    engine.add_executor(Box::new(match_intent_executor));
    engine.add_collector(match_intent_executor_collector);

    engine
}
