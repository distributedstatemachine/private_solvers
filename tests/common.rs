use anyhow::Result;
use ethers::prelude::LocalWallet;
use ethers::types::Address;
use khalani_solver::config::addresses::AddressesConfig;
use khalani_solver::config::chain::{ChainConfig, SEPOLIA_CHAIN_ID};
use khalani_solver::config::config::Config;
use khalani_solver::config::token::TokenConfig;
use khalani_solver::connectors::connector::Connector;

pub const SEPOLIA_RPC_URL: &str = "https://ethereum-sepolia.publicnode.com";
pub const SEPOLIA_WS_URL: &str = "wss://ethereum-sepolia.publicnode.com";

pub const E2E_PRIVATE_KEY_HEX: &str =
    "0x4f91dd71525e3acf4b83ffb493d16e5ed9bcdea36e8076eb3d74f361ae7dc0ff";

// https://sepolia.etherscan.io/token/0x24bcc5d1f6538f4a84cd9009ebd4fa614904fa59
pub const SEPOLIA_USDC_ADDRESS: &str = "0x24bcc5d1f6538f4a84cd9009ebd4fa614904fa59";

pub async fn create_connector() -> Result<Connector> {
    let wallet: LocalWallet = E2E_PRIVATE_KEY_HEX.parse::<LocalWallet>().unwrap();

    let config = create_e2e_config();
    let connector = Connector::new(config, wallet).await?;

    return Ok(connector);
}

pub fn create_e2e_config() -> Config {
    let sepolia_chain_config = ChainConfig {
        name: "sepolia".to_string(),
        chain_id: SEPOLIA_CHAIN_ID,
        rpc_url: SEPOLIA_RPC_URL.to_string(),
        ws_url: SEPOLIA_WS_URL.to_string(),
    };
    Config {
        addresses: AddressesConfig {
            intents_mempool_address: Default::default(),
        },
        chains: vec![sepolia_chain_config],
        tokens: vec![TokenConfig {
            chain_id: SEPOLIA_CHAIN_ID,
            address: SEPOLIA_USDC_ADDRESS.parse::<Address>().unwrap(),
        }],
    }
}
