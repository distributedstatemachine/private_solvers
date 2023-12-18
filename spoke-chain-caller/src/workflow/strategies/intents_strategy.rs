use std::sync::Arc;
use std::vec;

use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use futures::lock::Mutex;
use tracing::info;

use crate::workflow::action::Action;
use crate::workflow::event::Event;
use crate::workflow::state::state_manager::StateManager;

pub struct IntentsStrategy<S: StateManager> {
    state_manager: Arc<Mutex<S>>,
}

impl<S> IntentsStrategy<S>
where
    S: StateManager + Sync + Send,
{
    pub fn new(state_manager: Arc<Mutex<S>>) -> Self {
        Self { state_manager }
    }
}

#[async_trait]
impl<S> Strategy<Event, Action> for IntentsStrategy<S>
where
    S: StateManager + Sync + Send,
{
    async fn sync_state(&mut self) -> Result<()> {
        info!("Syncing state");
        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        return match event {
            Event::NewSpokeChainCall(spoke_chain_call) => {
                info!(?spoke_chain_call, "New Spoke Chain Call intent");
                self.state_manager
                    .lock()
                    .await
                    .create_intent_state(spoke_chain_call.clone());
                vec![Action::MatchIntent(spoke_chain_call)]
            }
            Event::IntentMatched(spoke_chain_call) => {
                info!(?spoke_chain_call, "Spoke Chain Call matched");
                vec![Action::CallSpoke(spoke_chain_call.spoke_chain_call)]
            }
            Event::CallSpokeConfirmed(call_spoke_handler_result) => {
                info!(?call_spoke_handler_result, "Spoke Chain Call confirmed");
                vec![]
            }
        };
    }
}
