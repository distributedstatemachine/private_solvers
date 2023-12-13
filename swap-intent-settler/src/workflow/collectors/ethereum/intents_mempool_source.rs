use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_stream::__private::AsyncStream;
use async_trait::async_trait;
use bindings_khalani::intents_mempool::IntentsMempool;
use ethers::middleware::Middleware;
use ethers::types::{Address, ValueOrArray};
use futures::StreamExt;
use std::time::Duration;
use tracing::{debug, error, info};

use crate::types::swap_intent::SwapIntent;
use crate::workflow::collectors::swap_intent_collector::SwapIntentSource;
use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};

pub struct IntentsMempoolSource {
    rpc_client: Arc<RpcClient>,
    intents_mempool: IntentsMempool<RpcClient>,
    intents_mempool_address: Address,
}

impl IntentsMempoolSource {
    pub fn new(connector: Arc<Connector>, intents_mempool_address: Address) -> Self {
        let rpc_client = connector.get_rpc_client(ChainId::Khalani).unwrap();
        let intents_mempool = IntentsMempool::new(intents_mempool_address, rpc_client.clone());

        Self {
            rpc_client,
            intents_mempool,
            intents_mempool_address,
        }
    }
}

#[async_trait]
impl SwapIntentSource for IntentsMempoolSource {
    async fn get_new_swap_intents_stream(&self) -> Result<CollectorStream<'_, SwapIntent>> {
        let mut previous_block_number = match self.rpc_client.get_block_number().await {
            Ok(block_number) => block_number,
            Err(e) => {
                error!(?e, "Error fetching block");
                return Err(e.into());
            }
        };
        info!(?previous_block_number, "Starting block number");
        let mut logged_last_indexed_block_number = previous_block_number;

        let event_stream: AsyncStream<Result<SwapIntent>, _> = async_stream::try_stream! {
            // TODO: use sub graphs connection or at least extract common implementation for indexers.
            loop {
                let current_block_number = match self.rpc_client.get_block_number().await {
                    Ok(block_number) => block_number,
                    Err(e) => {
                        error!(?e, "Error fetching block");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                };

                if previous_block_number >= current_block_number {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue
                }

                if logged_last_indexed_block_number + 50 < current_block_number {
                    debug!(?previous_block_number, ?current_block_number, "Indexing the IntentsMempool");
                    logged_last_indexed_block_number = current_block_number;
                }

                let event = self
                    .intents_mempool
                    .intent_created_filter()
                    .address(ValueOrArray::Value(self.intents_mempool_address))
                    .from_block(previous_block_number)
                    .to_block(current_block_number);

                let events = match event.query().await {
                    Ok(events) => events,
                    Err(e) => {
                        error!(?e, "Error querying events");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                };

                for event in events {
                    let swap_intent: Result<SwapIntent> = event.intent.try_into();
                    match swap_intent {
                        Ok(swap_intent) => {
                            info!(?swap_intent, "New swap intent received");
                            yield SwapIntent {
                                intent_id: event.intent_id.into(),
                                ..swap_intent
                            };
                        }
                        Err(e) => {
                            error!(?e, "Error parsing intent");
                            continue;
                        }
                    }

                }

                previous_block_number = current_block_number + 1;
            }
        };
        let event_stream = Box::pin(event_stream);
        let event_stream = event_stream.filter_map(|result| async move {
            match result {
                Ok(swap_intent) => Some(swap_intent),
                Err(_) => None,
            }
        });
        Ok(Box::pin(event_stream))
    }
}
