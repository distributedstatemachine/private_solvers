use crate::config::chain::{ChainId, KHALANI_CHAIN_ID};
use crate::connectors::Connector;
use crate::types::proof_id::ProofId;
use crate::workflow::event::Event;
use crate::workflow::state::IntentState;
use ethers::abi::{encode_packed, Token as AbiToken};
use ethers::types::H256;
use ethers::utils::keccak256;
use std::sync::Arc;

pub struct ProofsToEventsMapper {
    connector: Arc<Connector>,
}

impl ProofsToEventsMapper {
    pub fn new(connector: Arc<Connector>) -> Self {
        Self { connector }
    }

    pub fn map_new_proof_to_event(
        &self,
        all_intents: Vec<IntentState>,
        chain_id: ChainId,
        proof_id: ProofId,
    ) -> Option<Event> {
        for intent_state in all_intents {
            let event = self.try_map_source_chain_tokens_lock_proof_event(
                proof_id,
                chain_id,
                &intent_state,
            );
            if event.is_some() {
                return event;
            }

            let event =
                self.try_map_swap_intent_filled_proof_event(proof_id, chain_id, &intent_state);
            if event.is_some() {
                return event;
            }
        }
        None
    }

    fn try_map_source_chain_tokens_lock_proof_event(
        &self,
        proof_id: ProofId,
        chain_id: ChainId,
        intent_state: &IntentState,
    ) -> Option<Event> {
        if chain_id != KHALANI_CHAIN_ID {
            // Proof of tokens locked is expected only on the Khalani Chain's verifier.
            return None;
        }
        let intent_id = intent_state.intent_id;
        let swap_intent_token_lock_event = encode_packed(&[
            AbiToken::String(String::from("SwapIntentTokenLock")),
            AbiToken::FixedBytes(Vec::from(intent_id.as_bytes())),
        ])
        .unwrap();
        let expected_proof_id = keccak256(swap_intent_token_lock_event).into();
        if proof_id == expected_proof_id {
            return Some(Event::ProvedTokensLockedOnSourceChain(intent_id));
        }
        None
    }

    fn try_map_swap_intent_filled_proof_event(
        &self,
        proof_id: ProofId,
        chain_id: ChainId,
        intent_state: &IntentState,
    ) -> Option<Event> {
        if chain_id != KHALANI_CHAIN_ID {
            // Proof of Swap Intent Fill is expected only on the Khalani Chain's verifier.
            return None;
        }

        // TODO: try to avoid these assertions by enforcing intent state subtypes.
        let fill_timestamp = intent_state.clone().fill_timestamp.unwrap();
        let quoted_intent = intent_state.clone().quoted_intent.unwrap();

        let intent_id = intent_state.intent_id;

        let filler_address = self.connector.get_address();
        let fill_amount = quoted_intent.destination_amount.base_units;
        let swap_intent_filled_event = encode_packed(&[
            AbiToken::String(String::from("SwapIntentFilled")),
            AbiToken::FixedBytes(Vec::from(intent_id.as_bytes())),
            AbiToken::Address(filler_address),
            AbiToken::FixedBytes(Vec::from(
                H256::from_low_u64_be(fill_timestamp.as_u64()).as_bytes(),
            )),
            AbiToken::FixedBytes(Vec::from(
                H256::from_low_u64_be(fill_amount.as_u64()).as_bytes(),
            )),
        ])
        .unwrap();
        let expected_proof_id = keccak256(swap_intent_filled_event).into();
        if proof_id == expected_proof_id {
            return Some(Event::ProvedSwapIntentFilledOnDestinationChain(intent_id));
        }
        None
    }
}
