use intentbook_matchmaker::types::spoke_chain_call::SpokeChainCall;
use serde::{Deserialize, Serialize};
use solver_common::types::intent_id::IntentId;

pub mod database_state_manager;
pub mod state_manager;
use std::fmt;

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
    pub spoke_chain_call: SpokeChainCall,
    pub block_number: Option<i64>,
}

impl fmt::Display for IntentState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IntentState {{ intent_id: {}, status: {}, intent_bid_id: {}, spoke_chain_call: {}, block_number: {} }}",
            self.intent_id,
            self.status,
            self.intent_bid_id.as_ref().unwrap_or(&"None".to_string()),
            self.spoke_chain_call,
            self.block_number.as_ref().unwrap_or(&0)
        )
    }
}
