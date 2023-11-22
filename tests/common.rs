use anyhow::Result;
use ethers::prelude::LocalWallet;
use khalani_solver::config::config::Config;
use khalani_solver::connectors::connector::Connector;

pub const E2E_PRIVATE_KEY_HEX: &str =
    "0x4f91dd71525e3acf4b83ffb493d16e5ed9bcdea36e8076eb3d74f361ae7dc0ff";

pub async fn create_connector() -> Result<Connector> {
    let wallet: LocalWallet = E2E_PRIVATE_KEY_HEX.parse::<LocalWallet>().unwrap();

    let config = create_e2e_config();
    let connector = Connector::new(config, wallet).await?;

    return Ok(connector);
}

pub fn create_e2e_config() -> Config {
    Config::read_config("./config/config.json").unwrap()
}
