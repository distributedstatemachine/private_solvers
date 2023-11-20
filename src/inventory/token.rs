use ethers::types::Address;

use crate::config::chain::ChainId;

#[derive(Debug)]
pub struct Token {
    pub chain_id: ChainId,
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}
