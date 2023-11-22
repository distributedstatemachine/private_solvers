use crate::config::chain::ChainId;
use crate::inventory::amount::Amount;
use anyhow::Context;
use ethers::types::{Address, U256};

#[derive(Clone, Debug)]
pub struct Token {
    pub chain_id: ChainId,
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

impl Amount {
    pub fn from_token(user_units: U256, token: &Token) -> Amount {
        let multiplier = U256::exp10(token.decimals as usize);
        let base_units = user_units
            .checked_mul(multiplier)
            .context(format!("Token {}", token.address))
            .unwrap();
        Amount {
            decimals: token.decimals,
            base_units,
        }
    }

    pub fn from_token_base_units(base_units: U256, token: &Token) -> Amount {
        Amount {
            decimals: token.decimals,
            base_units,
        }
    }
}
