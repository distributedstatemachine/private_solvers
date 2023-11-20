use crate::config::config::Config;
use crate::config::token::TokenConfig;
use crate::connectors::connector::Connector;
use crate::inventory::amount::Amount;
use crate::inventory::token::Token;
use crate::inventory::token_balance_query::TokenBalanceQuery;
use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::erc20_mintable_burnable::ERC20MintableBurnable;
use ethers::types::Address;
use std::default::Default;
use std::sync::Arc;

pub struct Inventory {
    connector: Arc<Connector>,
    config: Config,
    pub tokens: Vec<Token>,
}

impl Inventory {
    pub async fn new(config: Config, connector: Arc<Connector>) -> Result<Self> {
        let mut inventory = Inventory {
            connector,
            config,
            tokens: Vec::default(),
        };

        inventory.request_tokens().await?;
        Ok(inventory)
    }

    async fn request_tokens(&mut self) -> Result<()> {
        for token_config in &self.config.tokens {
            let token = self.request_token(token_config).await?;
            self.tokens.push(token);
        }
        Ok(())
    }

    async fn request_token(&self, token_config: &TokenConfig) -> Result<Token> {
        let rpc_client = self
            .connector
            .get_rpc_client(token_config.chain_id)
            .unwrap();
        let erc20 = ERC20MintableBurnable::new(token_config.address, rpc_client.clone());
        let name = erc20.name().await?;
        let symbol = erc20.symbol().await?;
        let decimals = erc20.decimals().await?;
        let token = Token {
            address: token_config.address,
            name,
            symbol,
            chain_id: token_config.chain_id,
            decimals,
        };
        Ok(token)
    }
}

#[async_trait]
impl TokenBalanceQuery for Inventory {
    async fn get_balance(&self, token: &Token, owner: Address) -> Result<Amount> {
        let rpc_client = self.connector.get_rpc_client(token.chain_id).unwrap();
        let erc20 = ERC20MintableBurnable::new(token.address, rpc_client);
        let balance = erc20.balance_of(owner).await?;
        let amount = Amount::from_token(balance, token);
        Ok(amount)
    }
}
