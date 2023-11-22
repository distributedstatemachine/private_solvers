use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings_khalani::vault::Vault;
use ethers::types::Address;
use tracing::info;

use crate::config::balancer::BalancerConfig;
use crate::config::chain::KHALANI_CHAIN_ID;
use crate::connectors::connector::{Connector, RpcClient};
use crate::inventory::inventory::Inventory;
use crate::strategies::types::{Action, Event};
use crate::types::swap_intent::SwapIntent;

#[allow(dead_code)]
pub struct IntentsStrategy {
    balancer_config: BalancerConfig,
    inventory: Arc<Inventory>,
    vault_contract: Vault<RpcClient>,
}

impl IntentsStrategy {
    pub fn new(
        connector: Arc<Connector>,
        inventory: Arc<Inventory>,
        vault_address: Address,
        balancer_config: BalancerConfig,
    ) -> Self {
        let client = connector.get_rpc_client(KHALANI_CHAIN_ID).unwrap();

        Self {
            balancer_config,
            inventory,
            vault_contract: Vault::new(vault_address, client),
        }
    }
}

#[async_trait]
impl Strategy<Event, Action> for IntentsStrategy {
    async fn sync_state(&mut self) -> Result<()> {
        info!("Syncing state");
        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        let option = match event {
            Event::NewSwapIntent(swap_intent) => self.process_new_swap_intent(swap_intent).await,
            _ => None,
        };
        option.into_iter().collect()
    }
}

impl IntentsStrategy {
    async fn process_new_swap_intent(&self, swap_intent: SwapIntent) -> Option<Action> {
        Some(Action::SettleIntent(swap_intent))
    }
}
