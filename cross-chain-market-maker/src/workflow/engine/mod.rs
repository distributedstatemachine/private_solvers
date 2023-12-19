use std::sync::Arc;

use artemis_core::engine::Engine;
use futures::lock::Mutex;

use solver_common::config::Config;
use solver_common::connectors::Connector;
use solver_common::inventory::Inventory;

use crate::workflow::action::Action;
use crate::workflow::collectors::ethereum::intents_book_source::LimitOrderIntentbookSource;
use crate::workflow::collectors::limit_order_intent_collector::LimitOrderIntentCollector;
use crate::workflow::event::Event;
use crate::workflow::state::in_memory_state_manager::InMemoryStateManager;
use crate::workflow::strategies::intents_strategy::IntentsStrategy;

pub fn configure_engine(
    config: &Config,
    state_manager: InMemoryStateManager,
    connector: Arc<Connector>,
    inventory: Arc<Inventory>,
) -> Engine<Event, Action> {
    let intents_mempool_source = LimitOrderIntentbookSource::new(
        connector.clone(),
        inventory.clone(),
        config.addresses.intentbook_addresses.clone(),
    );

    let state_manager = Arc::new(Mutex::new(state_manager));

    let mut engine = Engine::<Event, Action>::default();

    // Set up collectors.
    let intents_collector = Box::new(LimitOrderIntentCollector::new(intents_mempool_source));
    engine.add_collector(intents_collector);

    // Set up strategies.
    let intents_strategy = Box::new(IntentsStrategy::new(state_manager.clone()));
    engine.add_strategy(intents_strategy);

    engine
}
