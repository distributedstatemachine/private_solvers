use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_trait::async_trait;
use bindings_khalani::limit_order_intent_book::{IntentCreatedFilter, LimitOrderIntentBook};
use ethers::contract::Event as ContractEvent;
use ethers::types::ValueOrArray;
use solver_common::config::addresses::IntentbookAddresses;

use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::ethereum::event_indexer::{EventFetcher, EventSource};
use solver_common::inventory::Inventory;

use crate::types::limit_order_intent::LimitOrderIntent;
use crate::workflow::collectors::limit_order_intent_collector::LimitOrderIntentSource;

#[derive(Debug, Clone)]
pub struct LimitOrderIntentbookSource {
    rpc_client: Arc<RpcClient>,
    inventory: Arc<Inventory>,
    limit_order_intentbook: LimitOrderIntentBook<RpcClient>,
}

impl LimitOrderIntentbookSource {
    pub fn new(
        connector: Arc<Connector>,
        inventory: Arc<Inventory>,
        intentbook_addresses: IntentbookAddresses,
    ) -> Self {
        let rpc_client = connector.get_rpc_client(ChainId::Khalani).unwrap();
        let limit_order_intentbook = LimitOrderIntentBook::new(
            intentbook_addresses.limit_order_intentbook,
            rpc_client.clone(),
        );

        Self {
            rpc_client,
            inventory,
            limit_order_intentbook,
        }
    }
}

#[async_trait]
impl EventSource for LimitOrderIntentbookSource {
    type EventFilter = IntentCreatedFilter;
    type EventResult = LimitOrderIntent;

    fn create_event_filter(&self) -> ContractEvent<Arc<RpcClient>, RpcClient, Self::EventFilter> {
        self.limit_order_intentbook
            .intent_created_filter()
            .address(ValueOrArray::Value(self.limit_order_intentbook.address()))
    }

    fn parse_event(&self, event: Self::EventFilter) -> Option<Result<Self::EventResult>> {
        Some(
            (
                event.intent_id.into(),
                (self.inventory.clone(), event.intent),
            )
                .try_into(),
        )
    }
}

#[async_trait]
impl LimitOrderIntentSource for LimitOrderIntentbookSource {
    async fn get_new_limit_order_intents_stream(
        &self,
    ) -> Result<CollectorStream<'_, LimitOrderIntent>> {
        let event_fetcher = EventFetcher::new(
            String::from("LimitOrderIntentbook"),
            self.rpc_client.clone(),
            self.clone(),
        );
        event_fetcher.fetch_events().await
    }
}
