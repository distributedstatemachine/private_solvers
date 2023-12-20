use crate::types::intent::Intent;
use crate::types::intent_bid::IntentBid;
use crate::types::proof_id::ProofId;
use std::collections::HashSet;
use swap_intent_settler::types::proof_id::{
    get_expected_proof_id_for_escrow_tokens_locked_event,
    get_expected_proof_id_for_swap_filled_event,
};

pub mod in_memory_state_manager;
pub mod state_manager;

#[derive(Debug, Clone)]
pub struct IntentState {
    pub intent: Intent,
    pub matched_bid: Option<IntentBid>,

    pub expected_proofs: HashSet<ProofId>,
    pub received_proofs: HashSet<ProofId>,

    // TODO: this flag only applies to SpokeChainCall intent. Refactor structs and move it there.
    pub is_spoke_chain_called: bool,
}

impl IntentState {
    pub fn new(intent: Intent) -> Self {
        IntentState {
            intent,
            matched_bid: None,
            is_spoke_chain_called: false,
            expected_proofs: HashSet::default(),
            received_proofs: HashSet::default(),
        }
    }

    pub fn is_ready_to_settle(&self) -> bool {
        match &self.intent {
            Intent::SpokeChainCall(..) => self.is_spoke_chain_called,
            Intent::SwapIntent(..) => self.expected_proofs == self.received_proofs,
            _ => false,
        }
    }

    pub fn handle_match(&mut self, intent_bid: IntentBid) {
        if let Intent::SwapIntent(_swap_intent) = &self.intent {
            if let IntentBid::SwapIntentBid(_swap_intent_bid) = &intent_bid {
                self.expected_proofs
                    .insert(get_expected_proof_id_for_escrow_tokens_locked_event());
                self.expected_proofs
                    .insert(get_expected_proof_id_for_swap_filled_event());
            }
        }
    }
}
