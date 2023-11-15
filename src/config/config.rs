use ethers::types::Address;

// TODO: read config from the JSON files
#[derive(Debug, Clone)]
pub struct Config {
    pub intents_mempool_address: Address,
}
