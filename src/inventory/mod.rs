pub mod amount;
pub mod token;
pub mod token_balance_query;

use crate::config::chain::ChainId;
use crate::config::token::TokenConfig;
use crate::config::Config;
use crate::connectors::Connector;
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

    pub fn find_token_by_address(&self, address: Address, chain_id: ChainId) -> Option<&Token> {
        self.tokens
            .iter()
            .find(|&i| i.address == address && i.chain_id == chain_id)
    }

    pub fn find_token_by_symbol(&self, symbol: String, chain_id: ChainId) -> Option<&Token> {
        self.tokens
            .iter()
            .find(|&i| i.symbol == symbol && i.chain_id == chain_id)
    }

    pub fn find_token_by_symbol_partial_match(
        &self,
        symbol: String,
        chain_id: ChainId,
    ) -> Option<&Token> {
        self.tokens.iter().find(|&i| {
            i.symbol.matches(&symbol).collect::<Vec<&str>>().len() == 1 && i.chain_id == chain_id
        })
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
