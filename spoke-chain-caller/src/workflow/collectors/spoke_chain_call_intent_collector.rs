use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;

use crate::workflow::event::Event;
use futures::stream::StreamExt;
use intentbook_matchmaker::types::spoke_chain_call::SpokeChainCall;

#[async_trait]
pub trait SpokeChainCallIntentSource {
    async fn get_new_spoke_chain_call_intents_stream(
        &self,
    ) -> Result<CollectorStream<'_, SpokeChainCall>>;
}

pub struct SpokeChainCallIntentCollector<S: SpokeChainCallIntentSource>(S);

impl<S: SpokeChainCallIntentSource> SpokeChainCallIntentCollector<S> {
    pub fn new(source: S) -> Self {
        SpokeChainCallIntentCollector(source)
    }
}

#[async_trait]
impl<S: SpokeChainCallIntentSource + Sync + Send> Collector<Event> for SpokeChainCallIntentCollector<S> {
    // TODO: Add additional streams for other events
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        let intents_stream = self.0.get_new_spoke_chain_call_intents_stream().await?;
        let intents_stream = intents_stream.map(Event::NewSpokeChainCall);
        Ok(Box::pin(intents_stream))
    }
}
