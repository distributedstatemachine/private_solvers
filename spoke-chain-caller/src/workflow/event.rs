use crate::workflow::executors::call_spoke_executor::CallSpokeHandlerResult;
use intentbook_matchmaker::workflow::executors::match_intent_executor::MatchIntentHandlerResult;
use solver_common::types::spoke_chain_call::SpokeChainCall;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewSpokeChainCall(SpokeChainCall),
    IntentMatched(MatchIntentHandlerResult),
    CallSpokeConfirmed(CallSpokeHandlerResult),
}
