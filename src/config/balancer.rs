use std::collections::HashMap;

use ethers::types::H256;
use serde::{Deserialize, Serialize};

use super::token::TokenConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalancerPool {
    pub id: H256,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BalancerConfigRaw {
    pub batch_swap_steps_from_kai: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct BalancerConfig {
    pub batch_swap_steps_from_kai: HashMap<TokenConfig, Vec<BalancerPool>>,
}
