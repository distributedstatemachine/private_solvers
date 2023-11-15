use crate::collectors::intents_collector::NewSwapIntent;
use crate::types::swap_intent::SwapIntent;

/// Core Event enum.
#[derive(Debug, Clone)]
pub enum Event {
    NewSwapIntent(NewSwapIntent),
}

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    SettleIntent(SwapIntent),
}
