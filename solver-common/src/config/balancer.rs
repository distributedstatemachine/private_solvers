use std::collections::HashMap;

use ethers::types::{Address, H256};
use serde::{Deserialize, Serialize};

use super::token::TokenConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalancerPool {
    pub id: H256,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalancerConfigRaw {
    pub vault_address: String,
    pub interchain_liquidity_hub_address: String,
    pub batch_swap_steps_from_kai: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct BalancerConfig {
    pub vault_address: Address,
    pub interchain_liquidity_hub_address: Address,
    pub batch_swap_steps_from_kai: HashMap<TokenConfig, Vec<BalancerPool>>,
}
