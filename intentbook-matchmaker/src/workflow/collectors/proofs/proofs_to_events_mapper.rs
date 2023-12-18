use ethers::abi::{encode_packed, Token as AbiToken};
use ethers::utils::keccak256;
use tracing::debug;

use crate::types::intent::Intent;
use crate::types::proof_id::ProofId;
use crate::types::IntentId;
use crate::workflow::event::Event;
use crate::workflow::state::IntentState;
use spoke_chain_caller::types::spoke_chain_call::SpokeChainCall;

#[derive(Default)]
pub struct ProofsToEventsMapper {}

impl ProofsToEventsMapper {
    pub fn map_new_proof_to_event(
        &self,
        all_intents: Vec<IntentState>,
        proof_id: ProofId,
    ) -> Option<Event> {
        for intent_state in all_intents {
            if let Intent::SpokeChainCall(spoke_chain_caller) = &intent_state.intent {
                let event = self.try_map_spoke_chain_call(
                    proof_id,
                    intent_state.intent.id(),
                    spoke_chain_caller,
                );
                if event.is_some() {
                    return event;
                }
            }
        }
        None
    }

    fn try_map_spoke_chain_call(
        &self,
        proof_id: ProofId,
        intent_id: IntentId,
        spoke_chain_call: &SpokeChainCall,
    ) -> Option<Event> {
        let spoke_chain_call = encode_packed(&[
            AbiToken::String(String::from("SpokeCalled")),
            AbiToken::FixedBytes(Vec::from(intent_id.as_bytes())),
            AbiToken::Address(spoke_chain_call.contract_to_call),
            AbiToken::Bytes(spoke_chain_call.call_data.to_vec()),
            AbiToken::Address(spoke_chain_call.token),
            AbiToken::Uint(spoke_chain_call.amount),
        ])
        .ok()?;
        let expected_proof_id = keccak256(spoke_chain_call).into();
        debug!(
            ?intent_id,
            ?proof_id,
            ?expected_proof_id,
            "Trying to map SpokeCalled proof onto event"
        );
        if proof_id == expected_proof_id {
            Some(Event::ProvedSpokeChainCall(intent_id))
        } else {
            None
        }
    }
}