use crate::config::chain::ChainConfig;
use ethers::types::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenConfigRaw {
    pub address: String,
}

#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub chain: ChainConfig,
    pub address: Address,
}
