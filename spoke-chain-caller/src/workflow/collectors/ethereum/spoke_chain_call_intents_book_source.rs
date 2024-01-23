use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_trait::async_trait;
use bindings_khalani::spoke_chain_call_intent_book::{
    IntentCreatedFilter, SpokeChainCallIntentBook,
};
use ethers::contract::Event as ContractEvent;
use ethers::types::ValueOrArray;
use solver_common::config::addresses::IntentbookAddresses;

use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::ethereum::event_indexer::{EventFetcher, EventSource};
use solver_common::inventory::Inventory;
use solver_common::types::intent_id::{IntentId, WithIntentId};

use crate::workflow::collectors::spoke_chain_call_intent_collector::SpokeChainCallIntentSource;
use solver_common::types::spoke_chain_call::SpokeChainCall;

#[derive(Debug, Clone)]
pub struct SpokeChainCallIntentbookSource {
    pub rpc_client: Arc<RpcClient>,
    pub inventory: Arc<Inventory>,
    pub spoke_chain_call_intentbook: SpokeChainCallIntentBook<RpcClient>,
}

impl SpokeChainCallIntentbookSource {
    pub fn new(
        connector: Arc<Connector>,
        inventory: Arc<Inventory>,
        intentbook_addresses: IntentbookAddresses,
    ) -> Self {
        let rpc_client = connector.get_rpc_client(ChainId::Khalani).unwrap();
        let spoke_chain_call_intentbook = SpokeChainCallIntentBook::new(
            intentbook_addresses.spoke_chain_call_intentbook,
            rpc_client.clone(),
        );

        Self {
            rpc_client,
            inventory,
            spoke_chain_call_intentbook,
        }
    }
}

#[async_trait]
impl EventSource for SpokeChainCallIntentbookSource {
    type EventFilter = IntentCreatedFilter;
    type EventResult = SpokeChainCall;

    fn create_event_filter(&self) -> ContractEvent<Arc<RpcClient>, RpcClient, Self::EventFilter> {
        self.spoke_chain_call_intentbook
            .intent_created_filter()
            .address(ValueOrArray::Value(
                self.spoke_chain_call_intentbook.address(),
            ))
    }

    fn parse_event(&self, event: Self::EventFilter) -> Option<Result<Self::EventResult>> {
        let intent_id: IntentId = event.intent_id.into();
        let with_intent_id: WithIntentId<bindings_khalani::base_intent_book::Intent> =
            (intent_id, event.intent);

        if let Ok(spoke_chain_call) = with_intent_id.try_into() {
            Some(Ok(spoke_chain_call))
        } else {
            None
        }
    }
}

#[async_trait]
impl SpokeChainCallIntentSource for SpokeChainCallIntentbookSource {
    async fn get_new_spoke_chain_call_intents_stream(
        &self,
    ) -> Result<CollectorStream<'_, SpokeChainCall>> {
        let event_fetcher = EventFetcher::new(
            String::from("SpokeChainCallIntentbook"),
            self.rpc_client.clone(),
            self.clone(),
        );
        event_fetcher.fetch_events().await
    }
}
