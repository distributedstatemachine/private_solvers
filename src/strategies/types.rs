use crate::types::swap_intent::SwapIntent;
use ethers::types::H256;

/// Core Event enum.
#[derive(Debug, Clone)]
pub enum Event {
    NewSwapIntent(SwapIntent),
    TokensLocked { intent_id: H256 },
}

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    LockTokens(SwapIntent),
    SettleIntent(SwapIntent),
}
