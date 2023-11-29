pub mod addresses;
pub mod balancer;
pub mod chain;
pub mod token;

use anyhow::{Context, Result};
use ethers::types::{Address, H256};

// TODO: read config from the JSON files

use crate::config::addresses::{AddressesConfig, AddressesConfigRaw};
use crate::config::balancer::BalancerPool;
use crate::config::chain::{ChainConfig, ChainConfigRaw};
use crate::config::token::{TokenConfig, TokenConfigRaw};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs};

use crate::config::balancer::{BalancerConfig, BalancerConfigRaw};
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigRaw {
    pub addresses: AddressesConfigRaw,
    pub balancer: BalancerConfigRaw,
    pub chains: Vec<ChainConfigRaw>,
    pub tokens: HashMap<String, Vec<TokenConfigRaw>>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub addresses: AddressesConfig,
    pub balancer: BalancerConfig,
    pub chains: Vec<ChainConfig>,
    pub tokens: Vec<TokenConfig>,
}

impl Config {
    pub fn read_config(file_path: &str) -> Result<Config> {
        let file_content = fs::read_to_string(file_path)?;
        let config: ConfigRaw = serde_json::from_str(&file_content)?;
        let addresses = AddressesConfig {
            vault_address: config.addresses.vault_address.parse::<Address>().unwrap(),
            intents_mempool_address: config
                .addresses
                .intents_mempool_address
                .parse::<Address>()
                .unwrap(),
            escrow_address: config.addresses.escrow_address.parse::<Address>().unwrap(),
            khalani_chain_event_verifier_address: config
                .addresses
                .khalani_chain_event_verifier_address
                .parse::<Address>()
                .unwrap(),
            interchain_liquidity_hub_address: config
                .addresses
                .interchain_liquidity_hub_address
                .parse::<Address>()
                .unwrap(),
        };
        let chains: Vec<ChainConfig> = config
            .chains
            .iter()
            .map(Self::create_chain_config)
            .collect();
        let tokens: Vec<TokenConfig> = config
            .tokens
            .iter()
            .flat_map(|(chain_name, tokens)| {
                let chain_config = chains
                    .iter()
                    .find(|chain| &chain.name == chain_name)
                    .unwrap();
                tokens.iter().map(|token| TokenConfig {
                    chain_id: chain_config.chain_id,
                    address: token.address.parse::<Address>().unwrap(),
                })
            })
            .collect();

        let batch_swap_steps_from_kai: HashMap<TokenConfig, Vec<BalancerPool>> = config
            .balancer
            .batch_swap_steps_from_kai
            .iter()
            .map(|(token_address, pools_addresses)| {
                (
                    (tokens
                        .iter()
                        .find(|token_config| {
                            token_config.address == token_address.parse::<Address>().unwrap()
                        })
                        .unwrap())
                    .clone(),
                    pools_addresses
                        .iter()
                        .map(|i| BalancerPool {
                            id: i.parse::<H256>().unwrap(),
                        })
                        .collect(),
                )
            })
            .collect();

        Ok(Config {
            addresses,
            balancer: BalancerConfig {
                batch_swap_steps_from_kai,
            },
            chains,
            tokens,
        })
    }

    fn create_chain_config(chain_raw: &ChainConfigRaw) -> ChainConfig {
        let rpc_url = Self::parse_url(chain_raw.rpc_url.as_str());
        let ws_url = Self::parse_url(chain_raw.ws_url.as_str());
        ChainConfig {
            name: chain_raw.name.clone(),
            chain_id: chain_raw.chain_id,
            rpc_url,
            ws_url,
        }
    }

    fn parse_url(url: &str) -> String {
        if url.starts_with('$') {
            env::var(url.trim_start_matches('$'))
                .context(format!("Unable to find ENV variable {}", url))
                .unwrap()
        } else {
            String::from(url)
        }
    }
}
