use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::config::balancer::BalancerConfig;
use crate::config::chain::KHALANI_CHAIN_ID;
use crate::connectors::{Connector, RpcClient};
use crate::ethereum::transaction::submit_transaction;
use crate::inventory::token::Token;
use crate::inventory::token_allowance_query::TokenAllowanceQuery;
use crate::inventory::token_balance_query::TokenBalanceQuery;
use crate::inventory::Inventory;
use crate::quote::interchain_liquidity_hub::interchain_liquidity_hub_quoter::InterchainLiquidityHubQuoter;
use crate::quote::quoted_intent::QuotedIntent;
use crate::workflow::executors::swap_and_bridge_executor::SwapAndBridgeHandler;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bindings_balancer::vault::{BatchSwapStep, FundManagement, Vault};
use bindings_khalani::interchain_liquidity_hub_wrapper::{
    BatchSwapStep as BatchSwapStepInterchainLiquidityHubWrapper, InterchainLiquidityHubWrapper,
};
use ethers::contract::ContractCall;
use ethers::types::{Address, Bytes, U256};
use tracing::info;

const SWAP_GIVEN_IN_SWAP_KIND: u8 = 0;

pub struct BalancerSwapTokensInvolved {
    pub destination_mirror_token: Token,
    pub kai_token: Token,
    pub kln_token: Token,
}

pub struct SendTransactionSwapAndBridgeHandler {
    connector: Arc<Connector>,
    interchain_liquidity_hub: InterchainLiquidityHubWrapper<RpcClient>,
    inventory: Arc<Inventory>,
    quoter: Arc<InterchainLiquidityHubQuoter>,
    vault_contract: Vault<RpcClient>,
}

impl SendTransactionSwapAndBridgeHandler {
    pub fn new(
        balancer_config: BalancerConfig,
        connector: Arc<Connector>,
        quoter: Arc<InterchainLiquidityHubQuoter>,
        inventory: Arc<Inventory>,
    ) -> Self {
        let client = connector.get_rpc_client(KHALANI_CHAIN_ID).unwrap();
        let vault_contract = Vault::new(balancer_config.vault_address, client.clone());
        let interchain_liquidity_hub = InterchainLiquidityHubWrapper::new(
            balancer_config.interchain_liquidity_hub_address,
            client,
        );

        Self {
            connector,
            interchain_liquidity_hub,
            inventory,
            quoter,
            vault_contract,
        }
    }
}

#[async_trait]
impl SwapAndBridgeHandler for SendTransactionSwapAndBridgeHandler {
    async fn swap_and_bridge(&self, quoted_intent: QuotedIntent) -> Result<()> {
        info!(?quoted_intent, "Executing swap and bridge");

        let transaction = self.build_swap_and_bridge_tx(quoted_intent.clone()).await?;
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(
            ?quoted_intent,
            %tx_hash,
            "Swap and bridge has been executed"
        );
        Ok(())
    }
}

impl SendTransactionSwapAndBridgeHandler {
    pub async fn build_swap_and_bridge_tx(
        &self,
        quoted_intent: QuotedIntent,
    ) -> Result<ContractCall<RpcClient, Vec<bindings_khalani::abstract_request_processor::Token>>>
    {
        let swap_deadline = Duration::from_secs(1000);
        let deadline: U256 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .add(swap_deadline)
            .as_secs()
            .into();

        let tokens = self.quoter.get_tokens(
            quoted_intent.swap_intent.destination_token,
            quoted_intent.swap_intent.destination_chain_id.into(),
        )?;

        let assets = vec![
            tokens.kai_token.address,
            tokens.kln_token.address,
            tokens.destination_mirror_token.address,
        ];

        let batch_swaps = self.get_batch_swap_steps_given_in(
            quoted_intent.kai_amount.base_units,
            tokens.destination_mirror_token.address,
        )?;

        let batch_swaps_liquidity_hub = batch_swaps
            .iter()
            .map(|i| BatchSwapStepInterchainLiquidityHubWrapper {
                pool_id: i.pool_id,
                asset_in_index: i.asset_in_index,
                asset_out_index: i.asset_out_index,
                amount: i.amount,
                user_data: i.user_data.clone(),
            })
            .collect();

        let sender = self.connector.get_address();

        let fund_management = FundManagement {
            sender,
            recipient: sender,
            from_internal_balance: false,
            to_internal_balance: false,
        };
        let query_batch_swap_function = self.vault_contract.query_batch_swap(
            SWAP_GIVEN_IN_SWAP_KIND,
            batch_swaps,
            assets.clone(),
            fund_management,
        );

        let estimations = query_batch_swap_function.call().await?;
        let limits = estimations.clone();

        let kai_balance = self
            .inventory
            .get_balance(&tokens.kai_token, sender)
            .await?;

        if kai_balance.lt(&quoted_intent.kai_amount) {
            return Err(anyhow!(
                "KAI balance is too small to execute swap and bridge {} < {}",
                kai_balance,
                quoted_intent.kai_amount
            ));
        }

        let kai_allowance = self
            .inventory
            .get_allowance(
                &tokens.kai_token,
                sender,
                self.interchain_liquidity_hub.address(),
            )
            .await?;

        if kai_allowance.lt(&quoted_intent.kai_amount) {
            return Err(anyhow!(
                "KAI allowance is too small to execute swap and bridge {} < {}",
                kai_allowance,
                quoted_intent.kai_amount
            ));
        }

        let mut withdraw_liquidity_function = self.interchain_liquidity_hub.withdraw_liquidity(
            quoted_intent.swap_intent.destination_chain_id.into(),
            sender,
            batch_swaps_liquidity_hub,
            assets,
            limits,
            deadline,
        );
        withdraw_liquidity_function
            .tx
            .set_chain_id(KHALANI_CHAIN_ID);
        withdraw_liquidity_function.tx.set_gas_price(8);
        withdraw_liquidity_function.tx.set_gas(7000000);

        Ok(withdraw_liquidity_function)
    }

    fn get_batch_swap_steps_given_in(
        &self,
        source_amount: U256,
        destination_mirror_token_address: Address,
    ) -> Result<Vec<BatchSwapStep>> {
        let mut swaps: Vec<BatchSwapStep> = vec![];
        let mut intermediate_pools_addresses = self
            .quoter
            .get_intermediate_pools_addresses(destination_mirror_token_address)?
            .clone();

        intermediate_pools_addresses.reverse();

        for (index, pool) in intermediate_pools_addresses.iter().enumerate() {
            if index == 0 {
                swaps.push(BatchSwapStep {
                    pool_id: pool.id.into(),
                    asset_in_index: 0.into(),
                    asset_out_index: 1.into(),
                    amount: source_amount,
                    user_data: "".parse::<Bytes>()?,
                })
            } else {
                swaps.push(BatchSwapStep {
                    pool_id: pool.id.into(),
                    asset_in_index: 1.into(),
                    asset_out_index: 2.into(),
                    amount: U256::from_str_radix("0", 10)?,
                    user_data: "".parse::<Bytes>()?,
                })
            }
        }

        Ok(swaps)
    }
}
