use crate::workflow::action::Action;
use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use tracing::info;

use crate::workflow::event::Event;

#[allow(dead_code)]
pub struct IntentsStrategy;

impl IntentsStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Strategy<Event, Action> for IntentsStrategy {
    async fn sync_state(&mut self) -> Result<()> {
        info!("Syncing state");
        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        let option = match event {
            Event::NewSwapIntent(swap_intent) => Some(Action::QuoteIntent(swap_intent)),
            Event::IntentQuoted(quoted_intent) => Some(Action::LockTokens(quoted_intent)),
            // Event::TokensLocked { intent_id} => Some(Action::SettleIntent())
            _ => None,
        };
        option.into_iter().collect()
    }
}
