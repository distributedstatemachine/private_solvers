use std::sync::Arc;

use crate::config::balancer::{BalancerConfig, BalancerPool};
use crate::config::chain::{ChainId, KHALANI_CHAIN_ID};
use crate::connectors::{Connector, RpcClient};
use crate::inventory::amount::Amount;
use crate::inventory::Inventory;
use crate::quote::intent_quoter::IntentQuoter;
use crate::quote::quoted_intent::QuotedIntent;
use crate::types::swap_intent::SwapIntent;
use crate::workflow::executors::ethereum::send_transaction_swap_and_bridge_handler::BalancerSwapTokensInvolved;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use bindings_balancer::vault::{BatchSwapStep, FundManagement, Vault};
use ethers::types::{Address, Bytes, U256};
use tracing::info;

const SWAP_EXACT_OUT_SWAP_KIND: u8 = 1;

pub struct InterchainLiquidityHubQuoter {
    inventory: Arc<Inventory>,
    balancer_config: BalancerConfig,
    vault_contract: Vault<RpcClient>,
}

impl InterchainLiquidityHubQuoter {
    pub fn new(
        connector: Arc<Connector>,
        inventory: Arc<Inventory>,
        balancer_config: BalancerConfig,
    ) -> Self {
        let client = connector.get_rpc_client(KHALANI_CHAIN_ID).unwrap();
        let vault_contract = Vault::new(balancer_config.vault_address, client.clone());

        Self {
            balancer_config,
            inventory,
            vault_contract,
        }
    }

    pub fn get_intermediate_pools_addresses(
        &self,
        destination_mirror_token_address: Address,
    ) -> Result<&Vec<BalancerPool>> {
        let intermediate_pools_addresses = self
            .balancer_config
            .batch_swap_steps_from_kai
            .iter()
            .find(|(token_config, _)| token_config.address == destination_mirror_token_address)
            .ok_or(anyhow!("Can not find intermediate pool address config"))?
            .1;

        Ok(intermediate_pools_addresses)
    }

    pub fn get_batch_swap_steps_exact_out(
        &self,
        source_amount: U256,
        destination_mirror_token_address: Address,
    ) -> Result<Vec<BatchSwapStep>> {
        let mut swaps: Vec<BatchSwapStep> = vec![];
        let intermediate_pools_addresses =
            self.get_intermediate_pools_addresses(destination_mirror_token_address)?;

        for (index, pool) in intermediate_pools_addresses.iter().enumerate() {
            if index == 0 {
                swaps.push(BatchSwapStep {
                    pool_id: pool.id.into(),
                    asset_in_index: 1.into(),
                    asset_out_index: 2.into(),
                    amount: source_amount,
                    user_data: "".parse::<Bytes>()?,
                })
            } else {
                swaps.push(BatchSwapStep {
                    pool_id: pool.id.into(),
                    asset_in_index: 0.into(),
                    asset_out_index: 1.into(),
                    amount: U256::from_str_radix("0", 10)?,
                    user_data: "".parse::<Bytes>()?,
                })
            }
        }

        Ok(swaps)
    }

    pub fn get_tokens(
        &self,
        destination_token_address: Address,
        destination_chain_id: ChainId,
    ) -> Result<BalancerSwapTokensInvolved> {
        let destination_token = self
            .inventory
            .find_token_by_address(destination_token_address, destination_chain_id)
            .ok_or(anyhow!(
                "Unsupported destination token {}",
                destination_token_address
            ))?;

        let destination_mirror_token = self
            .inventory
            .find_mirror_token(destination_token_address, destination_chain_id)?;

        let mut kln_token_symbol: String = "kln".to_owned();
        let generalized_token_symbol: &str = &destination_token.symbol[..4];
        kln_token_symbol.push_str(generalized_token_symbol);

        let kai_token = self
            .inventory
            .find_token_by_symbol("KAI".into(), KHALANI_CHAIN_ID)
            .unwrap();
        let kln_token = self
            .inventory
            .find_token_by_symbol_partial_match(kln_token_symbol, KHALANI_CHAIN_ID)
            .unwrap();

        Ok(BalancerSwapTokensInvolved {
            destination_mirror_token: destination_mirror_token.clone(),
            kai_token: kai_token.clone(),
            kln_token: kln_token.clone(),
        })
    }
}

#[allow(unreachable_code)]
#[async_trait]
impl IntentQuoter for InterchainLiquidityHubQuoter {
    async fn quote_intent(&self, swap_intent: SwapIntent) -> Result<QuotedIntent> {
        info!(?swap_intent, "Quoting intent");

        let tokens = self.get_tokens(
            swap_intent.destination_token,
            swap_intent.destination_chain_id.into(),
        )?;

        let assets = vec![
            tokens.kai_token.address,
            tokens.kln_token.address,
            tokens.destination_mirror_token.address,
        ];

        let swaps = self.get_batch_swap_steps_exact_out(
            swap_intent.source_amount,
            tokens.destination_mirror_token.address,
        )?;

        let fund_management = FundManagement {
            sender: Default::default(),
            recipient: Default::default(),
            from_internal_balance: false,
            to_internal_balance: false,
        };
        let call = self.vault_contract.query_batch_swap(
            SWAP_EXACT_OUT_SWAP_KIND,
            swaps,
            assets,
            fund_management,
        );
        let estimations = call.call().await?;
        let kai_amount =
            Amount::from_token_base_units(U256::try_from(estimations[0])?, &tokens.kai_token);

        // TODO: for now we set the destination (expected) amount to be equal to the source amount.
        //  Handle the decimals difference between source / destination tokens.
        let destination_amount = Amount::from_token_base_units(
            swap_intent.source_amount,
            &tokens.destination_mirror_token,
        );

        let quoted_intent = QuotedIntent {
            swap_intent,
            kai_amount,
            destination_amount,
        };
        info!(?quoted_intent, "Intent quoted");
        Ok(quoted_intent)
    }
}
