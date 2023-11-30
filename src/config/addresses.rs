use crate::config::chain::ChainId;
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressesConfigRaw {
    pub intents_mempool_address: String,
    pub escrows: HashMap<String, String>,
    pub verifiers: HashMap<String, HashMap<String, String>>,
    pub swap_intent_fillers: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct VerifierAddress {
    pub verifier_chain_id: ChainId,
    pub prover_chain_id: ChainId,
    pub verifier_address: Address,
}

#[derive(Debug, Clone)]
pub struct AddressesConfig {
    pub intents_mempool_address: Address,
    pub escrows: HashMap<ChainId, Address>,
    pub verifiers: Vec<VerifierAddress>,
    pub swap_intent_fillers: HashMap<ChainId, Address>,
}
