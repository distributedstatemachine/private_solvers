use anyhow::Result;
use ethers::types::Address;

// TODO: read config from the JSON files

use crate::config::addresses::{AddressesConfig, AddressesConfigRaw};
use crate::config::chain::{ChainConfig, ChainConfigRaw};
use crate::config::token::{TokenConfig, TokenConfigRaw};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigRaw {
    pub addresses: AddressesConfigRaw,
    pub chains: Vec<ChainConfigRaw>,
    pub tokens: HashMap<String, Vec<TokenConfigRaw>>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub addresses: AddressesConfig,
    pub chains: Vec<ChainConfig>,
    pub tokens: Vec<TokenConfig>,
}

pub fn read_config(file_path: &str) -> Result<Config> {
    let file_content = fs::read_to_string(file_path)?;
    let config: ConfigRaw = serde_json::from_str(&file_content)?;
    let addresses = AddressesConfig {
        intents_mempool_address: config
            .addresses
            .intents_mempool_address
            .parse::<Address>()
            .unwrap(),
    };
    let chains: Vec<ChainConfig> = config
        .chains
        .iter()
        .map(|chain_raw| ChainConfig {
            name: chain_raw.name.clone(),
            chain_id: chain_raw.chain_id,
        })
        .collect();
    let tokens: Vec<TokenConfig> = config
        .tokens
        .iter()
        .flat_map(|(chain_name, tokens)| {
            let chain_config = chains
                .iter()
                .find(|chain| chain.name.eq(chain_name))
                .unwrap();
            tokens.iter().map(|token| TokenConfig {
                chain: chain_config.clone(),
                address: token.address.parse::<Address>().unwrap(),
            })
        })
        .collect();
    Ok(Config {
        addresses,
        chains,
        tokens,
    })
}
