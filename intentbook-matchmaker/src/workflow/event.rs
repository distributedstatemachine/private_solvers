use crate::types::intent::Intent;
use crate::types::IntentId;

/// Core Event enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    NewIntent(Intent),
    NewMatchedIntent(IntentId),
}
