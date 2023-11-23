use crate::quote::quoted_intent::QuotedIntent;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    LockTokens(QuotedIntent),
    SettleIntent(QuotedIntent),
}
