use crate::config::chain::ChainId;
use crate::inventory::amount::Amount;
use ethers::types::{Address, U256};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Mul;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Token {
    pub chain_id: ChainId,
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

impl Amount {
    pub fn from_user_units_token(user_units: U256, token: &Token) -> Amount {
        let multiplier = U256::exp10(token.decimals as usize);
        let base_units = user_units.mul(multiplier);
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token {{ chain_id: {}, address: {}, name: {}, symbol: {}, decimals: {} }}",
            self.chain_id, self.address, self.name, self.symbol, self.decimals
        )
    }
}
