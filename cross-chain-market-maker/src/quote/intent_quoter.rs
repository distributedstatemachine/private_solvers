use crate::quote::quoted_intent::QuotedIntent;
use crate::types::limit_order_intent::LimitOrderIntent;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait IntentQuoter {
    async fn quote_intent(&self, limit_order_intent: LimitOrderIntent) -> Result<QuotedIntent>;
}
