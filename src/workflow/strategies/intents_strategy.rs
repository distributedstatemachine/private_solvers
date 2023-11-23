use crate::workflow::action::Action;
use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use std::vec;
use tracing::info;

use crate::workflow::event::Event;
use crate::workflow::state::state::IntentState::{IntentQuoted, NewIntent, TokensLocked};
use crate::workflow::state::state_manager::StateManager;

pub struct IntentsStrategy<S: StateManager + Sync + Send> {
    state_manager: S,
}

impl<S: StateManager + Sync + Send> IntentsStrategy<S> {
    pub fn new(state_manager: S) -> Self {
        Self { state_manager }
    }
}

#[async_trait]
impl<S: StateManager + Sync + Send> Strategy<Event, Action> for IntentsStrategy<S> {
    async fn sync_state(&mut self) -> Result<()> {
        info!("Syncing state");
        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        return match event {
            Event::NewSwapIntent(swap_intent) => {
                self.state_manager
                    .update_state(swap_intent.intent_id, NewIntent(swap_intent.clone()));
                vec![Action::QuoteIntent(swap_intent)]
            }
            Event::IntentQuoted(quoted_intent) => {
                self.state_manager.update_state(
                    quoted_intent.swap_intent.intent_id,
                    IntentQuoted(quoted_intent.clone()),
                );
                vec![Action::LockTokens(quoted_intent)]
            }
            Event::TokensLocked { intent_id } => {
                let previous_state = if let Some(IntentQuoted(quoted_intent)) =
                    self.state_manager.get_state(intent_id)
                {
                    Some(quoted_intent.clone())
                } else {
                    None
                };

                if let Some(quoted_intent) = previous_state {
                    self.state_manager
                        .update_state(intent_id, TokensLocked(quoted_intent.clone()));
                    return vec![Action::SettleIntent(quoted_intent)];
                }

                Vec::default()
            }
        };
    }
}
