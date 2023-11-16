use std::sync::Arc;

use anyhow::Result;
use bindings_khalani::erc20_mintable_burnable::ERC20MintableBurnable;
use ethers::providers::Middleware;
use ethers::types::{Address, U256};

#[derive(Debug)]
pub struct Token<M> {
    token: Address,
    client: Arc<M>,
}

impl<M: Middleware> Token<M> {
    pub fn new(token: Address, client: Arc<M>) -> Self {
        Self { token, client }
    }
}

impl<M> Token<M>
where
    M: Middleware + 'static,
    M::Error: 'static,
{
    pub async fn get_balance(&self, owner: Address) -> Result<U256> {
        let erc20 = ERC20MintableBurnable::new(self.token, self.client.clone());
        let result = erc20.balance_of(owner).await;
        Ok(result?)
    }
}
