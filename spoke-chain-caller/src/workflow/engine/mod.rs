use std::sync::Arc;

use crate::workflow::collectors::intentbook_source::IntentbookSource;
use crate::workflow::event::Event;
use crate::workflow::executors::ethereum::send_transaction_bid_intent_handler::SendTransactionBidIntentHandler;
use crate::workflow::executors::ethereum::send_transaction_match_intent_handler::SendTransactionMatchIntentHandler;
use crate::workflow::action::Action;
use crate::workflow::collectors::spoke_chain_call_collector::SpokeChainCallCollector;
use crate::workflow::strategies::intents_strategy::IntentsStrategy;
use crate::workflow::executors::bid_intent_executor::{BidIntentExecutor};

use solver_common::config::Config;
use solver_common::connectors::Connector;
use solver_common::inventory::Inventory;
use artemis_core::engine::Engine;

pub fn configure_engine(
    config: &Config,
    connector: Arc<Connector>,
    inventory: Arc<Inventory>,
) -> Engine<Event, Action> {
    // Set up SpokeChainCall specific intent source.
    let spoke_chain_call_intent_source =
        IntentbookSource::new(connector.clone(), config.addresses.intentbook_address);

    let send_transaction_bid_intent_handler =
    SendTransactionBidIntentHandler::new(config.addresses.clone(), connector.clone());
    let send_transaction_match_intent_handler =
    SendTransactionMatchIntentHandler::new(config.addresses.clone(), connector.clone());
    

    let mut engine = Engine::<Event, Action>::default();

    // Set up collectors.
    let intents_collector = Box::new(SpokeChainCallCollector::new(spoke_chain_call_intent_source));
    engine.add_collector(intents_collector);

    // Set up strategies.
    let intents_strategy = Box::new(IntentsStrategy::new(
        {}
    ));
    engine.add_strategy(intents_strategy);

    // Set up executors.
    let (bid_intent_executor, bid_intent_confirmation_collector) =
        BidIntentExecutor::new(send_transaction_bid_intent_handler);
    engine.add_executor(Box::new(bid_intent_executor));
    engine.add_collector(bid_intent_confirmation_collector);

    engine
}
