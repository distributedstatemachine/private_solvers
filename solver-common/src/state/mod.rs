use crate::types::intent::Intent;
use crate::types::intent_id::IntentId;
use serde::{Deserialize, Serialize};

pub mod database_client;
pub mod database_state_manager;
pub mod state_manager;
use std::fmt;

// TODO: Move this to solver_common/types
#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "intent_status", rename_all = "lowercase")]
pub enum IntentStatus {
    NotExists,
    New,
    InProgress,
    Settled,
    Cancelled,
}

impl fmt::Display for IntentStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self {
            IntentStatus::NotExists => "NotExists",
            IntentStatus::New => "New",
            IntentStatus::InProgress => "InProgress",
            IntentStatus::Settled => "Settled",
            IntentStatus::Cancelled => "Cancelled",
        };
        write!(f, "{}", status_str)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize, sqlx::FromRow)]
pub struct IntentState {
    pub intent_id: IntentId,
    pub status: IntentStatus,
    pub intent_bid_id: Option<String>,
    pub intent: Intent,
    pub block_number: Option<i64>,
}
impl Default for IntentState {
    fn default() -> Self {
        Self {
            intent_id: IntentId::default(),
            status: IntentStatus::New,
            intent_bid_id: None,
            intent: Intent::default(), // assuming Intent also implements Default
            block_number: None,
        }
    }
}

impl fmt::Display for IntentState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IntentState {{ intent_id: {}, status: {}, intent_bid_id: {}, spoke_chain_call: {}, block_number: {} }}",
            self.intent_id,
            self.status,
            self.intent_bid_id.as_ref().unwrap_or(&"None".to_string()),
            self.intent,
            self.block_number.as_ref().unwrap_or(&0)
        )
    }
}
