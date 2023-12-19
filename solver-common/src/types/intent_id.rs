use ethers::types::H256;

pub type IntentId = H256;

pub type WithIntentId<T> = (IntentId, T);
