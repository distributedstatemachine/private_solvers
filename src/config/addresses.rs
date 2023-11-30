use crate::config::chain::ChainId;
use ethers::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressesConfigRaw {
    pub intents_mempool_address: String,
    pub vault_address: String,
    pub escrow_address: String,
    pub khalani_chain_event_verifier_address: String,
    pub interchain_liquidity_hub_address: String,
    pub swap_intent_fillers: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AddressesConfig {
    pub intents_mempool_address: Address,
    pub vault_address: Address,
    pub escrow_address: Address,
    pub khalani_chain_event_verifier_address: Address,
    pub interchain_liquidity_hub_address: Address,
    pub swap_intent_fillers: HashMap<ChainId, Address>,
}
