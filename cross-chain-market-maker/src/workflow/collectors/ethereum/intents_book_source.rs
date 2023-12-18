use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_trait::async_trait;
use bindings_khalani::intents_mempool::{IntentCreatedFilter, IntentsMempool};
use ethers::contract::Event as ContractEvent;
use ethers::types::{Address, ValueOrArray};

use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::ethereum::event_indexer::{EventFetcher, EventSource};

use crate::types::limit_order_intent::LimitOrderIntent;
use crate::workflow::collectors::limit_order_intent_collector::LimitOrderIntentSource;

#[derive(Debug, Clone)]
pub struct IntentsMempoolSource {
    rpc_client: Arc<RpcClient>,
    intents_mempool_address: Address,
    intents_book: IntentsMempool<RpcClient>,
}

impl IntentsMempoolSource {
    pub fn new(connector: Arc<Connector>, intents_mempool_address: Address) -> Self {
        let rpc_client = connector.get_rpc_client(ChainId::Khalani).unwrap();
        let intents_book = IntentsMempool::new(intents_mempool_address, rpc_client.clone());

        Self {
            rpc_client,
            intents_book,
            intents_mempool_address,
        }
    }
}

#[async_trait]
impl EventSource for IntentsMempoolSource {
    type EventFilter = IntentCreatedFilter;
    type EventResult = LimitOrderIntent;

    fn create_event_filter(&self) -> ContractEvent<Arc<RpcClient>, RpcClient, Self::EventFilter> {
        self.intents_book
            .intent_created_filter()
            .address(ValueOrArray::Value(self.intents_mempool_address))
    }

    fn parse_event(&self, event: Self::EventFilter) -> Result<Self::EventResult> {
        let limit_order_intent: LimitOrderIntent = event.intent.try_into()?;
        Ok(LimitOrderIntent {
            intent_id: event.intent_id.into(),
            ..limit_order_intent
        })
    }
}

#[async_trait]
impl LimitOrderIntentSource for IntentsMempoolSource {
    async fn get_new_limit_order_intents_stream(
        &self,
    ) -> Result<CollectorStream<'_, LimitOrderIntent>> {
        let event_fetcher = EventFetcher::new(
            String::from("IntentsBook"),
            self.rpc_client.clone(),
            self.clone(),
        );
        event_fetcher.fetch_events().await
    }
}
