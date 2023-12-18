use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_stream::__private::AsyncStream;
use async_trait::async_trait;
//TO-DO: generate and import properly bindings
use bindings_khalani::intentbook::Intentbook;
use ethers::types::{Address, ValueOrArray};
use tracing::{debug, error, info};
use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};
use std::time::Duration;
use ethers::middleware::Middleware;
use futures::StreamExt;

use crate::types::spoke_chain_call::SpokeChainCall;
use crate::workflow::collectors::spoke_chain_call_collector::SpokeChainCallSource;


pub struct IntentbookSource {
    rpc_client: Arc<RpcClient>,
    intentbook: Intentbook<RpcClient>,
    intentbook_address: Address,
}

impl IntentbookSource {
    pub fn new(connector: Arc<Connector>, intentbook_address: Address) -> Self {
        let rpc_client = connector.get_rpc_client(ChainId::Khalani).unwrap();
        let intentbook = Intentbook::new(intentbook_address, rpc_client.clone());

        Self {
            rpc_client,
            intentbook,
            intentbook_address,
        }
    }
}

#[async_trait]
impl SpokeChainCallSource for IntentbookSource {
    async fn get_new_spoke_chain_call_stream(&self) -> Result<CollectorStream<'_, SpokeChainCall>> {
        let mut previous_block_number = match self.rpc_client.get_block_number().await {
            Ok(block_number) => block_number,
            Err(e) => {
                error!(?e, "Error fetching block");
                return Err(e.into());
            }
        };
        info!(?previous_block_number, "Starting block number");
        let mut logged_last_indexed_block_number = previous_block_number;

        let event_stream: AsyncStream<Result<SpokeChainCall>, _> = async_stream::try_stream! {
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
                    debug!(?previous_block_number, ?current_block_number, "Indexing the Intentbook");
                    logged_last_indexed_block_number = current_block_number;
                }

                let event = self
                    .intentbook
                    .spoke_chain_called_filter()
                    .address(ValueOrArray::Value(self.intentbook_address))
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
                    let spoke_chain_call: Result<SpokeChainCall> = event.intent.try_into();
                    match spoke_chain_call {
                        Ok(spoke_chain_call) => {
                            info!(?spoke_chain_call, "New spoke chain called event received");
                            yield SpokeChainCall {
                                intent_id: event.intent_id.into(),
                                ..spoke_chain_call
                            };
                        }
                        Err(e) => {
                            error!(?e, "Error parsing spoke chain called event");
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
                Ok(spoke_chain_call) => Some(spoke_chain_call),
                Err(_) => None,
            }
        });
        Ok(Box::pin(event_stream))
    }
}
