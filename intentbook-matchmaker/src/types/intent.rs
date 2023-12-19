use anyhow::anyhow;
use bindings_khalani::base_intent_book::Intent as ContractIntent;
use solver_common::types::intent_id::IntentId;
use spoke_chain_caller::types::spoke_chain_call::SpokeChainCall;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Intent {
    SpokeChainCall(SpokeChainCall),
    LimitOrder(IntentId),
    SwapIntent(IntentId),
}

impl Intent {
    pub fn id(&self) -> IntentId {
        match self {
            Intent::SpokeChainCall(spoke_chain_caller) => spoke_chain_caller.intent_id,
            Intent::LimitOrder(intent_id) => *intent_id,
            Intent::SwapIntent(intent_id) => *intent_id,
        }
    }
}

impl TryFrom<ContractIntent> for Intent {
    type Error = anyhow::Error;

    fn try_from(value: ContractIntent) -> Result<Self, Self::Error> {
        if let Ok(spoke_chain_caller) = value.try_into() {
            return Ok(Intent::SpokeChainCall(spoke_chain_caller));
        }
        // TODO: parse other types of intents.
        Err(anyhow!("Unknown intent type"))
    }
}
