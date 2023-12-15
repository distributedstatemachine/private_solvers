use crate::config::chain::ChainId;
use crate::inventory::amount::Amount;
use std::ops::Mul;

use anyhow::Result;
use ethers::types::{Address, U256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub chain_id: ChainId,
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

impl Amount {
    pub fn from_user_units_token(user_units: U256, token: &Token) -> Result<Amount> {
        let multiplier = U256::exp10(token.decimals as usize);
        let base_units = user_units.mul(multiplier);
        Ok(Amount {
            decimals: token.decimals,
            base_units,
        })
    }

    pub fn from_token_base_units(base_units: U256, token: &Token) -> Amount {
        Amount {
            decimals: token.decimals,
            base_units,
        }
    }
}
