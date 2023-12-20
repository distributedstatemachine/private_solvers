use ethers::types::H256;

pub type ProofId = H256;

// TODO: calculate the expected proof IDs when the Settler agent is ready.
pub fn get_expected_proof_id_for_escrow_tokens_locked_event() -> ProofId {
    todo!()
}

pub fn get_expected_proof_id_for_swap_filled_event() -> ProofId {
    todo!()
}
