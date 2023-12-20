use crate::workflow::executors::call_spoke_executor::CallSpokeHandlerResult;
use crate::workflow::executors::match_intent_executor::MatchIntentHandlerResult;
use intentbook_matchmaker::types::spoke_chain_call::SpokeChainCall;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewSpokeChainCall(SpokeChainCall),
    IntentMatched(MatchIntentHandlerResult),
    CallSpokeConfirmed(CallSpokeHandlerResult),
}
