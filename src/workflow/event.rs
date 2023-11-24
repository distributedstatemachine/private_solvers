use crate::config::chain::ChainId;
use crate::types::proof_id::ProofId;
use crate::types::swap_intent::SwapIntent;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewSwapIntent(SwapIntent),
    NewProofReceived(ProofId, ChainId),
}
