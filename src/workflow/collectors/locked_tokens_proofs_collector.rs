use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use futures::StreamExt;

use crate::types::intent_id::IntentId;
use crate::workflow::event::Event;

#[async_trait]
pub trait LockedTokensProofSource {
    async fn get_intents_with_locked_tokens(&self) -> Result<CollectorStream<'_, IntentId>>;
}

pub struct LockedTokensProofCollector<S: LockedTokensProofSource>(S);

impl<S: LockedTokensProofSource> LockedTokensProofCollector<S> {
    pub fn new(source: S) -> Self {
        LockedTokensProofCollector(source)
    }
}

#[async_trait]
impl<S: LockedTokensProofSource + Sync + Send> Collector<Event> for LockedTokensProofCollector<S> {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        let intents_stream = self.0.get_intents_with_locked_tokens().await?;
        let intents_stream = intents_stream.map(|intent_id| Event::TokensLocked { intent_id });
        Ok(Box::pin(intents_stream))
    }
}
