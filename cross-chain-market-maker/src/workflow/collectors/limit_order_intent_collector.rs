use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;

use crate::workflow::event::Event;
use futures::stream::StreamExt;
use intentbook_matchmaker::types::limit_order_intent::LimitOrderIntent;

#[async_trait]
pub trait LimitOrderIntentSource {
    async fn get_new_limit_order_intents_stream(
        &self,
    ) -> Result<CollectorStream<'_, LimitOrderIntent>>;
}

pub struct LimitOrderIntentCollector<S: LimitOrderIntentSource>(S);

impl<S: LimitOrderIntentSource> LimitOrderIntentCollector<S> {
    pub fn new(source: S) -> Self {
        LimitOrderIntentCollector(source)
    }
}

#[async_trait]
impl<S: LimitOrderIntentSource + Sync + Send> Collector<Event> for LimitOrderIntentCollector<S> {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        let intents_stream = self.0.get_new_limit_order_intents_stream().await?;
        let intents_stream = intents_stream.map(Event::NewLimitOrderIntent);
        Ok(Box::pin(intents_stream))
    }
}
