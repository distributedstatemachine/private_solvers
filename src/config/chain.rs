use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub name: String,
    pub chain_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChainConfigRaw {
    pub name: String,
    pub chain_id: u64,
}
