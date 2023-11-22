use ethers::types::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressesConfigRaw {
    pub intents_mempool_address: String,
    pub vault_address: String,
}

#[derive(Debug, Clone)]
pub struct AddressesConfig {
    pub intents_mempool_address: Address,
    pub vault_address: Address,
}
