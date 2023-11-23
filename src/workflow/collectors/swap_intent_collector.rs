use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;

use crate::types::swap_intent::SwapIntent;
use crate::workflow::event::Event;
use futures::stream::StreamExt;

#[async_trait]
pub trait SwapIntentSource {
    async fn get_new_swap_intents_stream(&self) -> Result<CollectorStream<'_, SwapIntent>>;
}

pub struct SwapIntentCollector<S: SwapIntentSource>(S);

impl<S: SwapIntentSource> SwapIntentCollector<S> {
    pub fn new(source: S) -> Self {
        SwapIntentCollector(source)
    }
}

#[async_trait]
impl<S: SwapIntentSource + Sync + Send> Collector<Event> for SwapIntentCollector<S> {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        let intents_stream = self.0.get_new_swap_intents_stream().await?;
        let intents_stream = intents_stream.map(|swap_intent| Event::NewSwapIntent(swap_intent));
        Ok(Box::pin(intents_stream))
    }
}
