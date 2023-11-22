use std::sync::Arc;

use crate::config::addresses::AddressesConfig;
use crate::config::chain::SEPOLIA_CHAIN_ID;
use crate::connectors::connector::{Connector, WsClient};
use crate::strategies::types::Event;
use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use bindings_khalani::escrow::{Escrow, TokensLockedFilter};
use ethers::contract::Event as ContractEvent;
use futures::StreamExt;

pub struct LockedTokensCollector {
    tokens_locked_filter: ContractEvent<Arc<WsClient>, WsClient, TokensLockedFilter>,
}

impl LockedTokensCollector {
    pub fn new(connector: Arc<Connector>, addresses_config: AddressesConfig) -> Self {
        // TODO: replace with the Khalani Chain ID.
        // TODO: connect to the Event Verifier on the Khalani Chain.
        let ws_client = connector.get_ws_client(SEPOLIA_CHAIN_ID).unwrap();
        let escrow = Escrow::new(addresses_config.escrow_address, ws_client);
        Self {
            tokens_locked_filter: escrow.tokens_locked_filter(),
        }
    }
}

#[async_trait]
impl Collector<Event> for LockedTokensCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        let events_stream = self.tokens_locked_filter.subscribe().await?;
        let intents_stream = events_stream.filter_map(|event| async {
            match event {
                Ok(event) => Some(Event::TokensLocked {
                    intent_id: event.intent_id.into(),
                }),
                Err(_) => None, // TODO: consider better error handling.
            }
        });
        Ok(Box::pin(intents_stream))
    }
}
