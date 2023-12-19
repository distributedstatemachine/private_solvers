use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_trait::async_trait;
//TO-DO: generate and import properly bindings
use bindings_khalani::spoke_chain_call_intent_book::IntentCreatedFilter;
use bindings_khalani::spoke_chain_call_intent_book::SpokeChainCallIntentBook;
use ethers::contract::Event as ContractEvent;
use ethers::types::{Address, ValueOrArray};

use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::ethereum::event_indexer::{EventFetcher, EventSource};

use crate::types::spoke_chain_call::SpokeChainCall;
use crate::workflow::collectors::spoke_chain_call_collector::SpokeChainCallSource;

#[derive(Debug, Clone)]
pub struct SpokeChainCallIntentbookSource {
    rpc_client: Arc<RpcClient>,
    intentbook: SpokeChainCallIntentBook<RpcClient>,
    intentbook_address: Address,
}

impl SpokeChainCallIntentbookSource {
    pub fn new(connector: Arc<Connector>, intentbook_address: Address) -> Self {
        let rpc_client = connector.get_rpc_client(ChainId::Khalani).unwrap();
        let intentbook = SpokeChainCallIntentBook::new(intentbook_address, rpc_client.clone());

        Self {
            rpc_client,
            intentbook,
            intentbook_address,
        }
    }
}

#[async_trait]
impl EventSource for SpokeChainCallIntentbookSource {
    type EventFilter = IntentCreatedFilter;
    type EventResult = SpokeChainCall;

    fn create_event_filter(&self) -> ContractEvent<Arc<RpcClient>, RpcClient, Self::EventFilter> {
        self.intentbook
            .intent_created_filter()
            .address(ValueOrArray::Value(self.intentbook_address))
    }

    fn parse_event(&self, event: Self::EventFilter) -> Option<Result<Self::EventResult>> {
        Some((event.intent_id.into(), event.intent).try_into())
    }
}

#[async_trait]
impl SpokeChainCallSource for SpokeChainCallIntentbookSource {
    async fn get_new_spoke_chain_call_stream(&self) -> Result<CollectorStream<'_, SpokeChainCall>> {
        let event_fetcher = EventFetcher::new(
            String::from("SpokeChainCallIntentbook"),
            self.rpc_client.clone(),
            self.clone(),
        );
        event_fetcher.fetch_events().await
    }
}
