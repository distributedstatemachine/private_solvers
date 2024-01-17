use std::{error::Error, fmt};

use ethers::types::H256;
// use sqlx::types::Binary;
use sqlx::types::Type;
// use sqlx::types::PgBytea;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Copy, PartialOrd, Ord)]
pub struct IntentId(H256);
pub type IntentBidId = H256;

pub type WithIntentId<T> = (IntentId, T);
pub type WithIntentIdAndBidId<T> = (IntentId, IntentBidId, T);

impl IntentId {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Display for IntentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Hash for IntentId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<[u8; 32]> for IntentId {
    fn from(bytes: [u8; 32]) -> Self {
        IntentId(H256::from(bytes))
    }
}

impl From<IntentId> for [u8; 32] {
    fn from(intent_id: IntentId) -> Self {
        intent_id.0 .0
    }
}

impl Decode<'_, Postgres> for IntentId {
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> std::result::Result<IntentId, Box<(dyn Error + std::marker::Send + Sync + 'static)>> {
        let bytes = value.as_bytes()?;
        Ok(IntentId(H256::from_slice(bytes)))
    }
}

impl Type<Postgres> for IntentId {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <Vec<u8> as Type<Postgres>>::type_info()
    }
}

// Convert IntentId to H256
impl From<IntentId> for H256 {
    fn from(wrapper: IntentId) -> Self {
        wrapper.0
    }
}

impl From<H256> for IntentId {
    fn from(h256: H256) -> Self {
        IntentId(h256)
    }
}

impl Default for IntentId {
    fn default() -> Self {
        IntentId(H256::zero())
    }
}
