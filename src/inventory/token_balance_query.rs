use crate::inventory::amount::Amount;
use crate::inventory::token::Token;
use async_trait::async_trait;
use ethers::addressbook::Address;

#[async_trait]
pub trait TokenBalanceQuery {
    async fn get_balance(&self, token: &Token, owner: Address) -> anyhow::Result<Amount>;
}
