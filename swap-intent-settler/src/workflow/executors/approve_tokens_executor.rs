use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;

use crate::quote::quoted_intent::QuotedIntent;
use crate::workflow::action::Action;

#[async_trait]
pub trait ApproveTokensHandler {
    async fn approve_tokens(&self, quoted_intent: QuotedIntent) -> Result<()>;
}

pub struct ApproveTokensExecutor<H: ApproveTokensHandler>(H);

impl<H: ApproveTokensHandler> ApproveTokensExecutor<H> {
    pub fn new(handler: H) -> Self {
        ApproveTokensExecutor(handler)
    }
}

#[async_trait]
impl<H: ApproveTokensHandler + Sync + Send> Executor<Action> for ApproveTokensExecutor<H> {
    async fn execute(&self, action: Action) -> Result<()> {
        match action {
            Action::ApproveTokens(quoted_intent) => self.0.approve_tokens(quoted_intent).await,
            _ => Ok(()),
        }
    }
}
