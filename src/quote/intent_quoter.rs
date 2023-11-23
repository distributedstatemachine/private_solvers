use crate::quote::quoted_intent::QuotedIntent;
use crate::types::swap_intent::SwapIntent;
use async_trait::async_trait;

#[async_trait]
pub trait IntentQuoter {
    async fn quote_intent(&self, swap_intent: SwapIntent) -> anyhow::Result<QuotedIntent>;
}
