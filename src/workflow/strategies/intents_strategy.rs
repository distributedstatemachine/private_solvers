use crate::quote::intent_quoter::IntentQuoter;
use crate::workflow::action::Action;
use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use std::vec;
use tracing::info;

use crate::workflow::event::Event;
use crate::workflow::state::state::IntentState::{IntentQuoted, NewIntent, TokensLocked};
use crate::workflow::state::state_manager::StateManager;

pub struct IntentsStrategy<S: StateManager, Q: IntentQuoter> {
    state_manager: S,
    intent_quoter: Q,
}

impl<S, Q> IntentsStrategy<S, Q>
where
    S: StateManager + Sync + Send,
    Q: IntentQuoter + Sync + Send,
{
    pub fn new(state_manager: S, intent_quoter: Q) -> Self {
        Self {
            state_manager,
            intent_quoter,
        }
    }
}

#[async_trait]
impl<S, Q> Strategy<Event, Action> for IntentsStrategy<S, Q>
where
    S: StateManager + Sync + Send,
    Q: IntentQuoter + Sync + Send,
{
    async fn sync_state(&mut self) -> Result<()> {
        info!("Syncing state");
        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::NewSwapIntent(swap_intent) => {
                self.state_manager
                    .update_state(swap_intent.intent_id, NewIntent(swap_intent.clone()));
                let quoted_intent = self.intent_quoter.quote_intent(swap_intent).await;
                if let Ok(quoted_intent) = quoted_intent {
                    self.state_manager.update_state(
                        quoted_intent.swap_intent.intent_id,
                        IntentQuoted(quoted_intent.clone()),
                    );
                    return vec![Action::LockTokens(quoted_intent)];
                } // TODO: handle erroneous quoting.
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
                } // TODO: handle wrong previous state.
            }
        }
        return Vec::default();
    }
}
