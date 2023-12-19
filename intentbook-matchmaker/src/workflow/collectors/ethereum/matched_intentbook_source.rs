use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_trait::async_trait;
use bindings_khalani::base_intent_book::BaseIntentBook;
use bindings_khalani::base_intent_book::IntentMatchFilter;
use ethers::contract::Event as ContractEvent;
use ethers::types::{Address, ValueOrArray};

use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::ethereum::event_indexer::{EventFetcher, EventSource};

use crate::workflow::collectors::matched_intent_collector::MatchedIntentsSource;
use solver_common::types::intent_id::IntentId;

#[derive(Debug, Clone)]
pub struct MatchedIntentbookIntentSource {
    rpc_client: Arc<RpcClient>,
    intentbook: BaseIntentBook<RpcClient>,
    intentbook_address: Address,
}

impl MatchedIntentbookIntentSource {
    pub fn new(connector: Arc<Connector>, intentbook_address: Address) -> Self {
        let rpc_client = connector.get_rpc_client(ChainId::Khalani).unwrap();
        let intentbook = BaseIntentBook::new(intentbook_address, rpc_client.clone());

        Self {
            rpc_client,
            intentbook,
            intentbook_address,
        }
    }
}

#[async_trait]
impl EventSource for MatchedIntentbookIntentSource {
    type EventFilter = IntentMatchFilter;
    type EventResult = IntentId;

    fn create_event_filter(&self) -> ContractEvent<Arc<RpcClient>, RpcClient, Self::EventFilter> {
        self.intentbook
            .intent_match_filter()
            .address(ValueOrArray::Value(self.intentbook_address))
    }

    fn parse_event(&self, event: Self::EventFilter) -> Result<Self::EventResult> {
        Ok(IntentId::from(event.intent_id))
    }
}

#[async_trait]
impl MatchedIntentsSource for MatchedIntentbookIntentSource {
    async fn get_matched_intents_source(&self) -> Result<CollectorStream<'_, IntentId>> {
        let event_fetcher = EventFetcher::new(
            format!("Intentbook (Matched) {}", self.intentbook_address),
            self.rpc_client.clone(),
            self.clone(),
        );
        event_fetcher.fetch_events().await
    }
}
