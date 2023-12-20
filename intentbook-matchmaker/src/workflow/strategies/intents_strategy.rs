use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use futures::lock::Mutex;
use std::sync::Arc;
use std::vec;
use tracing::{info, warn};

use crate::workflow::action::Action;
use crate::workflow::event::Event;
use crate::workflow::executors::settle_intent_executor::IntentSettlementData;
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
            Event::NewIntent(intent) => {
                info!(?intent, "New intent");
                self.state_manager
                    .lock()
                    .await
                    .create_intent_state(intent.clone());
                vec![]
            }
            Event::NewMatchedIntent(intent_bid) => {
                let intent_id = intent_bid.intent_id();
                info!(?intent_id, "Intent matched");
                let intent = self
                    .state_manager
                    .lock()
                    .await
                    .update_intent_state(intent_id, |intent| {
                        intent.handle_match(intent_bid.clone())
                    });
                if intent.is_none() {
                    warn!(?intent_id, "Unknown intent ID")
                }
                vec![]
            }
            Event::ProvedSpokeChainCall(
                intent_id,
                proof_id,
                _spoke_chain_call,
                _spoke_chain_call_bid,
            ) => {
                info!(?intent_id, "SpokeChainCall intent is proved");
                let mut state_mutex = self.state_manager.lock().await;
                let intent = state_mutex
                    .update_intent_state(intent_id, |intent| intent.is_spoke_chain_called = true);

                let mut actions = Vec::new();

                if let Some(intent_state) = intent {
                    if intent_state.is_ready_to_settle() {
                        info!(?intent_id, "Intent ready to be settled");
                        actions.push(Action::Settle(IntentSettlementData { intent_id }));
                    }
                } else {
                    warn!(?intent_id, "Unknown intent ID")
                }

                let all_intents = state_mutex.get_all_intents();
                for intent in all_intents {
                    let intent_id = intent.intent.id();
                    if intent.expected_proofs.contains(&proof_id) {
                        let intent = state_mutex.update_intent_state(intent_id, |intent| {
                            intent.received_proofs.insert(proof_id);
                        });
                        if let Some(intent) = intent {
                            if intent.is_ready_to_settle() {
                                info!(?intent_id, "Intent ready to be settled");
                                actions.push(Action::Settle(IntentSettlementData { intent_id }));
                            }
                        }
                    }
                }

                actions
            }
        };
    }
}
