use crate::types::intent::Intent;

/// Core Action enum.
#[derive(Debug, Clone)]
pub enum Action {
    MatchIntent(Intent),
    CallSpoke(Intent),
}
