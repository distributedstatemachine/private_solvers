mod common;
mod inventory;

use crate::common::create_provider_and_wallet;
use anyhow::Result;
use common::E2E_WALLET_ADDRESS;
use ethers::providers::Middleware;
use ethers::signers::Signer;
use ethers::types::Address;

#[tokio::test]
async fn test_chain_id() -> Result<()> {
    let (provider, wallet) = create_provider_and_wallet().await.unwrap();
    let chain_id = provider.get_chainid().await?.as_u64();
    assert_eq!(11155111, chain_id);
    assert_eq!(
        E2E_WALLET_ADDRESS.parse::<Address>().unwrap(),
        wallet.address()
    );
    Ok(())
}
