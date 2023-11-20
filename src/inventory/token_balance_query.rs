use crate::inventory::token::Token;
use async_trait::async_trait;
use ethers::addressbook::Address;
use ethers::prelude::U256;

#[async_trait]
pub trait TokenBalanceQuery {
    async fn get_balance(&self, token: &Token, owner: Address) -> anyhow::Result<U256>;
}
