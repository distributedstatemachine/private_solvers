use std::sync::Arc;

use artemis_core::types::CollectorStream;
use async_trait::async_trait;
use bindings_khalani::intents_mempool::{IntentCreatedFilter, IntentsMempool};
use ethers::contract::Event as ContractEvent;
use ethers::types::Address;
use futures::StreamExt;

use crate::config::chain::SEPOLIA_CHAIN_ID;
use crate::connectors::connector::{Connector, WsClient};
use crate::types::swap_intent::SwapIntent;
use crate::workflow::collectors::swap_intent_collector::SwapIntentSource;

pub struct IntentsMempoolSource {
    intent_created_filter: ContractEvent<Arc<WsClient>, WsClient, IntentCreatedFilter>,
}

impl IntentsMempoolSource {
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
impl SwapIntentSource for IntentsMempoolSource {
    async fn get_new_swap_intents_stream(&self) -> anyhow::Result<CollectorStream<'_, SwapIntent>> {
        let intents_stream = self.intent_created_filter.subscribe().await?;
        let intents_stream = intents_stream.filter_map(|event| async {
            match event {
                Ok(event) => Some(SwapIntent {
                    intent_id: event.intent_id.into(),
                    ..SwapIntent::from(event.intent)
                }),
                Err(_) => None, // TODO: consider better error handling.
            }
        });
        Ok(Box::pin(intents_stream))
    }
}
