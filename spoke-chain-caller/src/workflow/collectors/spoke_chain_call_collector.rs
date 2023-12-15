use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;

use crate::types::spoke_chain_call::SpokeChainCall;
use crate::workflow::event::Event;
use futures::stream::StreamExt;

#[async_trait]
pub trait SpokeChainCallSource {
    async fn get_new_spoke_chain_call_stream(&self) -> Result<CollectorStream<'_, SpokeChainCall>>;
}

pub struct SpokeChainCallCollector<S: SpokeChainCallSource>(S);

impl<S: SpokeChainCallSource> SpokeChainCallCollector<S> {
    pub fn new(source: S) -> Self {
        SpokeChainCallCollector(source)
    }
}

#[async_trait]
impl<S: SpokeChainCallSource + Sync + Send> Collector<Event> for SpokeChainCallCollector<S> {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        let intents_stream = self.0.get_new_spoke_chain_call_stream().await?;
        let intents_stream = intents_stream.map(Event::IntentMatch);
        Ok(Box::pin(intents_stream))
    }
}
