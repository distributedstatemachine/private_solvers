use crate::quote::quoted_intent::QuotedIntent;
use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use futures::lock::Mutex;
use tokio::sync::mpsc::Receiver;

use crate::strategies::types::Event;

pub struct QuotedIntentsCollector {
    quoted_intents_receiver: Mutex<Receiver<QuotedIntent>>,
}

impl QuotedIntentsCollector {
    pub fn new(quoted_intents_receiver: Receiver<QuotedIntent>) -> Self {
        Self {
            quoted_intents_receiver: Mutex::new(quoted_intents_receiver),
        }
    }
}

#[async_trait]
impl Collector<Event> for QuotedIntentsCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, Event>> {
        let stream = async_stream::stream! {
            let mut receiver = self.quoted_intents_receiver.lock().await;
            while let Some(quoted_intent) = receiver.recv().await {
                yield Event::IntentQuoted(quoted_intent);
            }
        };

        Ok(Box::pin(stream))
    }
}
