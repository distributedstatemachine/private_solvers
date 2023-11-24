use crate::config::chain::ChainId;
use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use futures::StreamExt;

use crate::types::proof_id::ProofId;
use crate::workflow::event::Event;

#[async_trait]
pub trait ProofSource {
    async fn get_proof_ids_stream(&self) -> Result<CollectorStream<'_, ProofId>>;

    fn get_chain_id(&self) -> ChainId;
}

pub struct ProofsCollector<S: ProofSource>(S);

impl<S: ProofSource> ProofsCollector<S> {
    pub fn new(source: S) -> Self {
        ProofsCollector(source)
    }
}

#[async_trait]
impl<S: ProofSource + Sync + Send> Collector<Event> for ProofsCollector<S> {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        let proof_ids_stream = self.0.get_proof_ids_stream().await?;
        let chain_id = self.0.get_chain_id();
        let proof_ids_stream =
            proof_ids_stream.map(move |proof_id| Event::NewProofReceived(proof_id, chain_id));
        Ok(Box::pin(proof_ids_stream))
    }
}
