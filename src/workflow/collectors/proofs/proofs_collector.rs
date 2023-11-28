use crate::config::chain::ChainId;
use crate::connectors::Connector;
use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use futures::lock::Mutex;
use futures::StreamExt;
use std::sync::Arc;
use tracing::{info, warn};

use crate::types::proof_id::ProofId;
use crate::workflow::collectors::proofs::proofs_to_events_mapper::ProofsToEventsMapper;
use crate::workflow::event::Event;
use crate::workflow::state::state_manager::StateManager;

#[async_trait]
pub trait ProofSource {
    async fn get_proof_ids_stream(&self) -> Result<CollectorStream<'_, ProofId>>;

    fn get_chain_id(&self) -> ChainId;
}

pub struct ProofsCollector<PS: ProofSource, SM: StateManager> {
    proof_source: PS,
    state_manager: Arc<Mutex<SM>>,
    proofs_to_events_mapper: ProofsToEventsMapper,
}

impl<PS: ProofSource, SM: StateManager> ProofsCollector<PS, SM> {
    pub fn new(proof_source: PS, state_manager: Arc<Mutex<SM>>, connector: Arc<Connector>) -> Self {
        ProofsCollector {
            proof_source,
            state_manager,
            proofs_to_events_mapper: ProofsToEventsMapper::new(connector),
        }
    }
}

#[async_trait]
impl<PS: ProofSource + Sync + Send, SM: StateManager + Sync + Send> Collector<Event>
    for ProofsCollector<PS, SM>
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        let proof_ids_stream = self.proof_source.get_proof_ids_stream().await?;
        let chain_id = self.proof_source.get_chain_id();
        let proof_ids_stream =
            proof_ids_stream.filter_map(move |proof_id| self.handle(proof_id, chain_id));
        Ok(Box::pin(proof_ids_stream))
    }
}

impl<PS: ProofSource, SM: StateManager> ProofsCollector<PS, SM> {
    async fn handle(&self, proof_id: ProofId, chain_id: ChainId) -> Option<Event> {
        info!(?chain_id, ?proof_id, "Received new proof");
        let all_intents = self.state_manager.lock().await.get_all_intents();
        let event =
            self.proofs_to_events_mapper
                .map_new_proof_to_event(all_intents, chain_id, proof_id);
        if let Some(event) = &event {
            info!(
                ?chain_id,
                ?proof_id,
                ?event,
                "Proof has been mapped to event"
            );
        } else {
            warn!(?proof_id, ?chain_id, "No mapping found for the proof");
        }
        event
    }
}