use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use bindings_balancer::vault::{BatchSwapStep, FundManagement, Vault};
use ethers::types::{Bytes, U256};

use crate::config::addresses::AddressesConfig;
use crate::config::balancer::BalancerConfig;
use crate::config::chain::KHALANI_CHAIN_ID;
use crate::connectors::connector::{Connector, RpcClient};
use crate::inventory::amount::Amount;
use crate::inventory::inventory::Inventory;
use crate::quote::intent_quoter::IntentQuoter;
use crate::quote::quoted_intent::QuotedIntent;
use crate::types::swap_intent::SwapIntent;

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
        addresses_config: AddressesConfig,
        balancer_config: BalancerConfig,
    ) -> Self {
        let client = connector.get_rpc_client(KHALANI_CHAIN_ID).unwrap();
        let vault_contract = Vault::new(addresses_config.vault_address, client);

        Self {
            balancer_config,
            inventory,
            vault_contract,
        }
    }
}

#[async_trait]
impl IntentQuoter for InterchainLiquidityHubQuoter {
    async fn quote_intent(&self, swap_intent: SwapIntent) -> anyhow::Result<QuotedIntent> {
        let destination_token = self
            .inventory
            .find_token_by_address(swap_intent.destination_token, KHALANI_CHAIN_ID)
            .ok_or(anyhow!(
                "Unsupported destination token {}",
                swap_intent.destination_token
            ))?;

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

        let assets = vec![
            kai_token.address,
            kln_token.address,
            swap_intent.destination_token,
        ];

        let mut swaps: Vec<BatchSwapStep> = vec![];
        let intermediate_pools_addresses = self
            .balancer_config
            .batch_swap_steps_from_kai
            .iter()
            .find(|(token_config, _)| token_config.address == destination_token.address)
            .unwrap()
            .1;

        for (index, pool) in intermediate_pools_addresses.iter().enumerate() {
            if index == 0 {
                swaps.push(BatchSwapStep {
                    pool_id: pool.id.into(),
                    asset_in_index: 1.into(),
                    asset_out_index: 2.into(),
                    amount: swap_intent.source_amount,
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
        let kai_amount = Amount::from_token_base_units(U256::try_from(estimations[0])?, kai_token);

        // TODO: for now we set the destination (expected) amount to be equal to the source amount.
        //  Handle the decimals difference between source / destination tokens.
        let destination_amount =
            Amount::from_token_base_units(swap_intent.source_amount, &destination_token);
        Ok(QuotedIntent {
            swap_intent,
            kai_amount,
            destination_amount,
        })
    }
}
