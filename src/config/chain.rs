use serde::{Deserialize, Serialize};

pub type ChainId = u64;

// TODO: create an enum, use 'strum' crate?
pub const SEPOLIA_CHAIN_ID: ChainId = 11155111;
pub const FUJI_CHAIN_ID: ChainId = 43113;
pub const KHALANI_CHAIN_ID: ChainId = 10012;

#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub name: String,
    pub chain_id: ChainId,
    pub rpc_url: String,
    pub ws_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChainConfigRaw {
    pub name: String,
    pub chain_id: u64,
    // TODO: parse from ENV or a secret file.
    pub rpc_url: String,
    pub ws_url: String,
}
