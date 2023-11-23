use std::sync::Arc;

use crate::config::chain::SEPOLIA_CHAIN_ID;
use crate::connectors::connector::{Connector, WsClient};
use crate::types::swap_intent::SwapIntent;
use crate::workflow::event::Event;
use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use bindings_khalani::intents_mempool::{IntentCreatedFilter, IntentsMempool};
use ethers::contract::Event as ContractEvent;
use ethers::types::Address;
use futures::StreamExt;

/// A collector that listens for new intents in the mempool.
pub struct MempoolIntentsCollector {
    intent_created_filter: ContractEvent<Arc<WsClient>, WsClient, IntentCreatedFilter>,
}

impl MempoolIntentsCollector {
    pub fn new(connector: Arc<Connector>, intents_mempool_address: Address) -> Self {
        // TODO: replace with the Khalani Chain ID.
        let ws_client = connector.get_ws_client(SEPOLIA_CHAIN_ID).unwrap();
        let intents_mempool = IntentsMempool::new(intents_mempool_address, ws_client);
        Self {
            intent_created_filter: intents_mempool.intent_created_filter(),
        }
    }
}

#[async_trait]
impl Collector<Event> for MempoolIntentsCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        let intents_stream = self.intent_created_filter.subscribe().await?;
        let intents_stream = intents_stream.filter_map(|event| async {
            match event {
                Ok(event) => Some(Event::NewSwapIntent(SwapIntent {
                    intent_id: event.intent_id.into(),
                    ..SwapIntent::from(event.intent)
                })),
                Err(_) => None, // TODO: consider better error handling.
            }
        });
        Ok(Box::pin(intents_stream))
    }
}
