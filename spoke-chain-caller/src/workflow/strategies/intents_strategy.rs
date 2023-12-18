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
                vec![Action::MatchIntent()]
            }
            Event::IntentMatched(spoke_chain_call) => {
                info!(?spoke_chain_call, "New Spoke Chain Call intent");
                vec![Action::CallSpoke()]
            }
        };
    }
}
