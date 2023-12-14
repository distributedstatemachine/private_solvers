use anyhow::{Context, Result};
use ethers::prelude::LocalWallet;
use solver_common::config::Config;
use solver_common::connectors::Connector;
use std::env;
use std::path::Path;

// https://sepolia.etherscan.io/address/0x18F814fA6CB21cC51ae0C5594418766F17DFb6A9
// https://testnet.snowtrace.io/address/0x18F814fA6CB21cC51ae0C5594418766F17DFb6A9
pub const E2E_PRIVATE_KEY_HEX: &str =
    "0x4f91dd71525e3acf4b83ffb493d16e5ed9bcdea36e8076eb3d74f361ae7dc0ff";

pub async fn create_connector() -> Result<Connector> {
    let wallet: LocalWallet = E2E_PRIVATE_KEY_HEX
        .parse::<LocalWallet>()
        .context("Failed to parse private key")?;

    let config: Config = create_e2e_config()?;
    let connector = Connector::new(config, wallet).await?;

    Ok(connector)
}

pub fn create_e2e_config() -> Result<Config> {
    let config_path =
        env::var("CONFIG_FILE").unwrap_or_else(|_| "../config/config.json".to_string());

    Config::read_config(&config_path)
        .context(format!("Failed to read config from file: {}", config_path))
}
