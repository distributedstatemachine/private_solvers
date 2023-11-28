use anyhow::Result;
use ethers::prelude::LocalWallet;
use khalani_solver::config::Config;
use khalani_solver::connectors::Connector;
use std::path::Path;

// https://sepolia.etherscan.io/address/0x18F814fA6CB21cC51ae0C5594418766F17DFb6A9
// https://testnet.snowtrace.io/address/0x18F814fA6CB21cC51ae0C5594418766F17DFb6A9
pub const E2E_PRIVATE_KEY_HEX: &str =
    "0x4f91dd71525e3acf4b83ffb493d16e5ed9bcdea36e8076eb3d74f361ae7dc0ff";

pub async fn create_connector() -> Result<Connector> {
    let wallet: LocalWallet = E2E_PRIVATE_KEY_HEX.parse::<LocalWallet>().unwrap();

    let config = create_e2e_config();
    let connector = Connector::new(config, wallet).await?;

    Ok(connector)
}

pub fn create_e2e_config() -> Config {
    let config_path = if Path::new(".local.config.json").exists() {
        ".local.config.json"
    } else {
        "./config/config.json"
    };
    Config::read_config(config_path).unwrap()
}
