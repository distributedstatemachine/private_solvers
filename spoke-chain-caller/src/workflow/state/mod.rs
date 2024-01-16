use intentbook_matchmaker::types::spoke_chain_call::SpokeChainCall;
use serde::{Deserialize, Serialize};
use solver_common::types::intent_id::IntentId;

pub mod database_state_manager;
pub mod state_manager;

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "intent_status", rename_all = "lowercase")]
pub enum IntentStatus {
    NotExists,
    New,
    InProgress,
    Settled,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct IntentState {
    pub intent_id: IntentId,
    pub status: IntentStatus,
    pub intent_bid_id: String,
    pub spoke_chain_call: SpokeChainCall,
    pub block_number: u64,
}
