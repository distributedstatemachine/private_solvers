use std::sync::Arc;

use artemis_core::engine::Engine;
use futures::lock::Mutex;

use crate::quote::one_to_one_intent_quoter::OneToOneIntentQuoter;
use intentbook_matchmaker::workflow::collectors::ethereum::new_intentbook_source::NewIntentbookIntentSource;
use intentbook_matchmaker::workflow::collectors::new_intent_collector::NewIntentCollector;
use solver_common::config::Config;
use solver_common::connectors::Connector;
use solver_common::inventory::Inventory;
use solver_common::workflow::collector_filter_map::CollectorFilterMap;

use crate::workflow::action::Action;
use crate::workflow::event::Event;
use crate::workflow::executors::ethereum::fill_spoke_chain_call_intent_creator_handler::FillSpokeChainCallIntentCreatorHandlerImpl;
use crate::workflow::executors::ethereum::lock_tokens_spoke_chain_call_intent_creator_handler::LockTokensSpokeChainCallIntentCreatorHandlerImpl;
use crate::workflow::executors::fill_spoke_chain_call_intent_creator_executor::FillSpokeChainCallIntentCreatorExecutor;
use crate::workflow::executors::lock_tokens_spoke_chain_call_intent_creator_executor::LockTokensSpokeChainCallIntentCreatorExecutor;
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;
use crate::workflow::strategies::intents_strategy::IntentsStrategy;

pub fn configure_engine(
    config: &Config,
    state_manager: InMemoryStateManager,
    connector: Arc<Connector>,
    inventory: Arc<Inventory>,
) -> Engine<Event, Action> {
    // Set up Ethereum specific clients.
    let intent_quoter = OneToOneIntentQuoter::new(inventory.clone());

    let state_manager = Arc::new(Mutex::new(state_manager));

    let mut engine = Engine::<Event, Action>::default();

    // Set up collectors.
    let new_intentbook_source = NewIntentbookIntentSource::new(
        connector.clone(),
        config.addresses.intentbook_addresses.swap_intent_intentbook,
    );
    let new_intent_collector = Box::new(NewIntentCollector::new(new_intentbook_source));
    let new_intent_collector = Box::new(CollectorFilterMap::new(new_intent_collector, |event| {
        if let intentbook_matchmaker::workflow::event::Event::NewIntent(intent) = event {
            Some(Event::NewIntent(intent))
        } else {
            None
        }
    }));
    engine.add_collector(new_intent_collector);

    // Set up strategies.
    let intents_strategy = Box::new(IntentsStrategy::new(state_manager.clone(), intent_quoter));
    engine.add_strategy(intents_strategy);

    // Set up executors.
    let (
        lock_tokens_spoke_chain_call_intent_creator_executor,
        lock_tokens_spoke_chain_call_intent_creator_executor_result_collector,
    ) = LockTokensSpokeChainCallIntentCreatorExecutor::new(
        LockTokensSpokeChainCallIntentCreatorHandlerImpl::new(
            config.addresses.clone(),
            connector.clone(),
        ),
    );
    engine.add_executor(Box::new(
        lock_tokens_spoke_chain_call_intent_creator_executor,
    ));
    engine.add_collector(lock_tokens_spoke_chain_call_intent_creator_executor_result_collector);

    let (
        fill_spoke_chain_call_intent_creator_executor,
        fill_spoke_chain_call_intent_creator_executor_result,
    ) = FillSpokeChainCallIntentCreatorExecutor::new(
        FillSpokeChainCallIntentCreatorHandlerImpl::new(
            config.addresses.clone(),
            connector.clone(),
            inventory.clone(),
        ),
    );
    engine.add_executor(Box::new(fill_spoke_chain_call_intent_creator_executor));
    engine.add_collector(fill_spoke_chain_call_intent_creator_executor_result);

    engine
}
