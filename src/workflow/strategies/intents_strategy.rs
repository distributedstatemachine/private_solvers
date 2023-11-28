use std::sync::Arc;
use std::vec;

use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use futures::lock::Mutex;
use tracing::info;

use crate::quote::intent_quoter::IntentQuoter;
use crate::workflow::action::Action;
use crate::workflow::event::Event;
use crate::workflow::state::state_manager::StateManager;

pub struct IntentsStrategy<S: StateManager, Q: IntentQuoter> {
    state_manager: Arc<Mutex<S>>,
    intent_quoter: Q,
}

impl<S, Q> IntentsStrategy<S, Q>
where
    S: StateManager + Sync + Send,
    Q: IntentQuoter + Sync + Send,
{
    pub fn new(state_manager: Arc<Mutex<S>>, intent_quoter: Q) -> Self {
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
                info!(?swap_intent, "Received new swap intent");
                let intent_id = swap_intent.intent_id;
                self.state_manager
                    .lock()
                    .await
                    .create_intent_state(intent_id, swap_intent.clone());
                if let Ok(quoted_intent) = self.intent_quoter.quote_intent(swap_intent).await {
                    return self.process_event(Event::IntentQuoted(quoted_intent)).await;
                } // TODO: handle erroneous quoting.
            }
            Event::IntentQuoted(quoted_intent) => {
                info!(?quoted_intent, "Intent is quoted and ready to be filled");
                self.state_manager
                    .lock()
                    .await
                    .update_intent_state(quoted_intent.clone().swap_intent.intent_id, |intent| {
                        intent.quoted_intent = Some(quoted_intent.clone())
                    });
                return vec![Action::LockTokensOnSourceChain(quoted_intent.swap_intent)];
            }
            Event::TokensLockedOnSourceChain(locked_tokens_intent) => {
                info!(
                    ?locked_tokens_intent,
                    "Tokens have been locked on the source chain, awaiting the proof on the Khalani Chain"
                );

                let swap_intent = locked_tokens_intent.swap_intent;
                let intent_id = swap_intent.intent_id;
                let intent_state = self
                    .state_manager
                    .lock()
                    .await
                    .update_intent_state(intent_id, |intent| {
                        intent.is_tokens_locked_on_source_chain = true
                    });
                if let Some(intent_state) = intent_state {
                    // TODO: try to avoid this assertion by enforcing intent state subtypes.
                    let quoted_intent = intent_state.quoted_intent.unwrap();
                    return vec![Action::FillIntentOnDestinationChain(quoted_intent.clone())];
                }
            }
            Event::ProvedTokensLockedOnSourceChain(intent_id) => {
                info!(
                    ?intent_id,
                    "Received the proof on Khalani Chain that source tokens were locked on the source chain"
                );

                let intent_state = self
                    .state_manager
                    .lock()
                    .await
                    .update_intent_state(intent_id, |intent| {
                        intent.is_proved_that_tokens_locked_on_source_chain = true
                    });

                if let Some(intent_state) = intent_state {
                    if intent_state.is_ready_to_settle() {
                        info!(?intent_id, "Intent  ready to be settled");
                        return vec![Action::SettleIntent(intent_state.swap_intent)];
                    }
                }
            }
            Event::IntentFilledOnDestination(filled_intent_result) => {
                info!(?filled_intent_result, "Intent filled on destination");
                self.state_manager.lock().await.update_intent_state(
                    filled_intent_result.quoted_intent.swap_intent.intent_id,
                    |intent| {
                        intent.is_filled_on_destination = true;
                        intent.fill_timestamp = Some(filled_intent_result.fill_timestamp);
                    },
                );
            }
            Event::ProvedSwapIntentFilledOnDestinationChain(intent_id) => {
                info!(?intent_id, "Received the proof on Khalani Chain that the intent was filled on the destination chain");

                let intent_state =
                    self.state_manager
                        .lock()
                        .await
                        .update_intent_state(intent_id, |intent| {
                            intent.is_proved_that_filled_on_destination_chain = true;
                        });

                if let Some(intent_state) = intent_state {
                    if intent_state.is_ready_to_settle() {
                        info!(?intent_id, "Intent is ready to be settled");
                        return vec![Action::SettleIntent(intent_state.swap_intent)];
                    }
                }
            }
        }
        return Vec::default();
    }
}
