use std::sync::Arc;

use artemis_core::engine::Engine;
use futures::lock::Mutex;

use solver_common::config::Config;
use solver_common::connectors::Connector;

use crate::workflow::action::Action;
use crate::workflow::collectors::ethereum::matched_intentbook_source::MatchedIntentbookIntentSource;
use crate::workflow::collectors::ethereum::new_intentbook_source::NewIntentbookIntentSource;
use crate::workflow::collectors::matched_intent_collector::MatchedIntentCollector;
use crate::workflow::collectors::new_intent_collector::NewIntentCollector;
use crate::workflow::event::Event;
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;
use crate::workflow::strategies::intents_strategy::IntentsStrategy;

pub fn configure_engine(
    config: &Config,
    connector: Arc<Connector>,
    state_manager: InMemoryStateManager,
) -> Engine<Event, Action> {
    let state_manager = Arc::new(Mutex::new(state_manager));

    let intentbook_addresses = vec![
        config
            .addresses
            .intentbook_addresses
            .spoke_chain_call_intentbook,
        config.addresses.intentbook_addresses.swap_intent_intentbook,
        config.addresses.intentbook_addresses.limit_order_intentbook,
    ];

    let mut engine = Engine::<Event, Action>::default();

    for intentbook_address in &intentbook_addresses {
        let new_intentbook_source =
            NewIntentbookIntentSource::new(connector.clone(), *intentbook_address);

        let new_intent_collector = NewIntentCollector::new(new_intentbook_source);
        engine.add_collector(Box::new(new_intent_collector));
    }

    for intentbook_address in &intentbook_addresses {
        let matched_intent_intentbook_source =
            MatchedIntentbookIntentSource::new(connector.clone(), *intentbook_address);

        let matched_intent_collector =
            MatchedIntentCollector::new(matched_intent_intentbook_source);
        engine.add_collector(Box::new(matched_intent_collector));
    }

    // Set up strategies.
    let intents_strategy = Box::new(IntentsStrategy::new(state_manager));
    engine.add_strategy(intents_strategy);

    // Set up executors.

    engine
}
