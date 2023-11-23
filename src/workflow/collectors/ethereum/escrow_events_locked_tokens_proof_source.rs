use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_trait::async_trait;
use bindings_khalani::escrow::{Escrow, TokensLockedFilter};
use ethers::contract::Event as ContractEvent;
use futures::StreamExt;
use tracing::info;

use crate::config::addresses::AddressesConfig;
use crate::config::chain::SEPOLIA_CHAIN_ID;
use crate::connectors::connector::{Connector, WsClient};
use crate::types::intent_id::IntentId;
use crate::workflow::collectors::locked_tokens_proofs_collector::LockedTokensProofSource;

pub struct EscrowEventsLockedTokensProofSource {
    tokens_locked_filter: ContractEvent<Arc<WsClient>, WsClient, TokensLockedFilter>,
}

impl EscrowEventsLockedTokensProofSource {
    pub fn new(connector: Arc<Connector>, addresses_config: AddressesConfig) -> Self {
        // TODO: replace with the Khalani Chain ID.
        // TODO: connect to the Event Verifier on the Khalani Chain.
        let ws_client = connector.get_ws_client(SEPOLIA_CHAIN_ID).unwrap();
        let escrow = Escrow::new(addresses_config.escrow_address, ws_client);
        Self {
            tokens_locked_filter: escrow.tokens_locked_filter(),
        }
    }
}

#[async_trait]
impl LockedTokensProofSource for EscrowEventsLockedTokensProofSource {
    async fn get_intents_with_locked_tokens(&self) -> Result<CollectorStream<'_, IntentId>> {
        let events_stream = self.tokens_locked_filter.subscribe().await?;
        let intents_stream = events_stream.filter_map(|event| async {
            match event {
                Ok(event) => {
                    let intent_id = event.intent_id.into();
                    info!(%intent_id, "Event: swap intent's tokens locked");
                    Some(intent_id)
                }
                Err(_) => None, // TODO: consider better error handling.
            }
        });
        Ok(Box::pin(intents_stream))
    }
}
