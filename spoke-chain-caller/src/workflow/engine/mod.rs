use std::sync::Arc;

use crate::workflow::collectors::intentbook_source::IntentbookSource;
use crate::workflow::event::Event;
use solver_common::config::Config;
use solver_common::connectors::Connector;
use solver_common::inventory::Inventory;
use artemis_core::engine::Engine;

pub fn configure_agent_engine(
    config: &Config,
    connector: Arc<Connector>,
    inventory: Arc<Inventory>,
) -> Engine<Event, Action> {
    // Set up SpokeChainCall specific intent source.
    let spoke_chain_call_intent_source =
        IntentbookSource::new(connector.clone(), config.addresses.intentbook);

    
}
