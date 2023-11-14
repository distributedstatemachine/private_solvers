use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use tracing::info;

use crate::collectors::intents_collector::NewSwapIntent;
use crate::strategies::types::{Action, Event};

#[derive(Debug)]
#[allow(dead_code)]
pub struct IntentsStrategy {}

impl IntentsStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Strategy<Event, Action> for IntentsStrategy {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        let option = match event {
            Event::NewSwapIntent(new_swap_intent) => {
                self.process_new_swap_intent(new_swap_intent).await
            }
        };
        option.into_iter().collect()
    }
}

impl IntentsStrategy {
    // Process new orders as they come in.
    async fn process_new_swap_intent(&mut self, event: NewSwapIntent) -> Option<Action> {
        Some(Action::LogSwapIntent(event.0))
    }
}
