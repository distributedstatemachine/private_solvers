use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;

use crate::quote::quoted_intent::QuotedIntent;
use crate::workflow::action::Action;

#[async_trait]
pub trait LockIntentTokensHandler {
    async fn lock_tokens(&self, quoted_intent: QuotedIntent) -> Result<()>;
}

pub struct LockIntentTokensExecutor<H: LockIntentTokensHandler>(H);

impl<H: LockIntentTokensHandler> LockIntentTokensExecutor<H> {
    pub fn new(handler: H) -> Self {
        LockIntentTokensExecutor(handler)
    }
}

#[async_trait]
impl<H: LockIntentTokensHandler + Sync + Send> Executor<Action> for LockIntentTokensExecutor<H> {
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::LockTokens(quoted_intent) => self.0.lock_tokens(quoted_intent).await,
            _ => Ok(()),
        }
    }
}
