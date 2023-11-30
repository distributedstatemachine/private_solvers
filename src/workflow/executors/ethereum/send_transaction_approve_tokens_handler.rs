use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::erc20::ERC20;

use crate::config::balancer::BalancerConfig;
use crate::config::chain::KHALANI_CHAIN_ID;
use crate::connectors::Connector;
use crate::ethereum::transaction::submit_transaction;
use crate::inventory::Inventory;
use crate::quote::quoted_intent::QuotedIntent;
use crate::workflow::executors::approve_tokens_executor::ApproveTokensHandler;
use tracing::info;

pub struct SendTransactionApproveTokensHandler {
    balancer_config: BalancerConfig,
    connector: Arc<Connector>,
    inventory: Arc<Inventory>,
}

impl SendTransactionApproveTokensHandler {
    pub fn new(
        balancer_config: BalancerConfig,
        connector: Arc<Connector>,
        inventory: Arc<Inventory>,
    ) -> Self {
        Self {
            balancer_config,
            connector,
            inventory,
        }
    }
}

#[async_trait]
impl ApproveTokensHandler for SendTransactionApproveTokensHandler {
    async fn approve_tokens(&self, quoted_intent: QuotedIntent) -> Result<()> {
        info!(?quoted_intent, "Approving tokens before the Vault trade");
        let kai_token = self
            .inventory
            .find_token_by_symbol("KAI".into(), KHALANI_CHAIN_ID)
            .unwrap();

        let spender = self.balancer_config.interchain_liquidity_hub_address;
        let rpc_client = self.connector.get_rpc_client(KHALANI_CHAIN_ID).unwrap();
        let erc20 = ERC20::new(kai_token.address, rpc_client);
        let mut function = erc20.approve(spender, quoted_intent.kai_amount.base_units);
        function.tx.set_chain_id(KHALANI_CHAIN_ID);
        function.tx.set_gas_price(8);
        function.tx.set_gas(7000000);
        let receipt = submit_transaction(function).await?;
        let tx_hash = receipt.transaction_hash;

        info!(
            ?quoted_intent,
            %tx_hash,
            "Tokens have been approved"
        );
        Ok(())
    }
}
