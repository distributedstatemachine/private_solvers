use std::sync::Arc;

use anyhow::Result;
use ethers::providers::Middleware;
use ethers::signers::Signer;
use ethers::types::Address;

use khalani_solver::inventory::token::Token;

use crate::common::{create_provider_and_wallet, SEPOLIA_USDC_ADDRESS};

#[tokio::test]
async fn test_token() -> Result<()> {
    let (provider, wallet) = create_provider_and_wallet().await.unwrap();
    let token_address = SEPOLIA_USDC_ADDRESS.parse::<Address>().unwrap();
    let owner = wallet.address();
    let token = Token::new(token_address, Arc::new(provider));
    let balance = token.get_balance(owner).await?;
    println!("{}", balance);
    Ok(())
}
