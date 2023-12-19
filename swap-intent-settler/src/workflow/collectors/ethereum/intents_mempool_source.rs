use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_trait::async_trait;
use bindings_khalani::base_intent_book::{BaseIntentBook, IntentCreatedFilter};
use ethers::contract::Event as ContractEvent;
use ethers::types::ValueOrArray;
use solver_common::config::addresses::IntentbookAddresses;

use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::ethereum::event_indexer::{EventFetcher, EventSource};

use crate::types::swap_intent::SwapIntent;
use crate::workflow::collectors::swap_intent_collector::SwapIntentSource;

#[derive(Debug, Clone)]
pub struct IntentsMempoolSource {
    rpc_client: Arc<RpcClient>,
    intentbook: BaseIntentBook<RpcClient>,
}

impl IntentsMempoolSource {
    pub fn new(connector: Arc<Connector>, intentbook_addresses: IntentbookAddresses) -> Self {
        let rpc_client = connector.get_rpc_client(ChainId::Khalani).unwrap();
        let intentbook = BaseIntentBook::new(
            intentbook_addresses.limit_order_intentbook,
            rpc_client.clone(),
        );

        Self {
            rpc_client,
            intentbook,
        }
    }
}

#[async_trait]
impl EventSource for IntentsMempoolSource {
    type EventFilter = IntentCreatedFilter;
    type EventResult = SwapIntent;

    fn create_event_filter(&self) -> ContractEvent<Arc<RpcClient>, RpcClient, Self::EventFilter> {
        self.intentbook
            .intent_created_filter()
            .address(ValueOrArray::Value(self.intentbook.address()))
    }

    fn parse_event(&self, event: Self::EventFilter) -> Option<Result<Self::EventResult>> {
        Some((event.intent_id.into(), event.intent).try_into())
    }
}

#[async_trait]
impl SwapIntentSource for IntentsMempoolSource {
    async fn get_new_swap_intents_stream(&self) -> Result<CollectorStream<'_, SwapIntent>> {
        let event_fetcher = EventFetcher::new(
            String::from("IntentsMempool"),
            self.rpc_client.clone(),
            self.clone(),
        );
        event_fetcher.fetch_events().await
    }
}
