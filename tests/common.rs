use anyhow::Result;
use ethers::middleware::Middleware;
use ethers::prelude::LocalWallet;
use ethers::providers::{Http, Provider};
use ethers::signers::Signer;

pub const SEPOLIA_RPC_URL: &str = "https://ethereum-sepolia.publicnode.com";

// https://sepolia.etherscan.io/address/0x18F814fA6CB21cC51ae0C5594418766F17DFb6A9
pub const E2E_WALLET_ADDRESS: &'static str = "0x18F814fA6CB21cC51ae0C5594418766F17DFb6A9";

pub const E2E_PRIVATE_KEY_HEX: &str =
    "0x4f91dd71525e3acf4b83ffb493d16e5ed9bcdea36e8076eb3d74f361ae7dc0ff";

// https://sepolia.etherscan.io/token/0x24bcc5d1f6538f4a84cd9009ebd4fa614904fa59
pub const SEPOLIA_USDC_ADDRESS: &str = "0x24bcc5d1f6538f4a84cd9009ebd4fa614904fa59";

pub async fn create_provider_and_wallet() -> Result<(Provider<Http>, LocalWallet)> {
    let provider = Provider::<Http>::try_from(SEPOLIA_RPC_URL).unwrap();

    let chain_id = provider.get_chainid().await?.as_u64();

    let wallet: LocalWallet = E2E_PRIVATE_KEY_HEX
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);

    return Ok((provider, wallet));
}
