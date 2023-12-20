use std::ops::Range;
use std::sync::Arc;

use anyhow::{Context, Result};
use ethers::types::U256;

use solver_common::config::chain::ChainId;
use solver_common::inventory::Inventory;
use solver_common::tests::connector::{create_connector, create_e2e_config};
use swap_intent_settler::quote::intent_quoter::IntentQuoter;
use swap_intent_settler::quote::interchain_liquidity_hub::interchain_liquidity_hub_quoter::InterchainLiquidityHubQuoter;
use swap_intent_settler::types::swap_intent::SwapIntent;

// TODO: Remove ignore directive once intent quoter is fixed. Currently it would return the default values for quotes , 0
#[tokio::test]
async fn test_interchain_liquidity_hub_quoter() -> Result<()> {
    let config = create_e2e_config().unwrap();
    let connector = create_connector().await?;
    let connector = Arc::new(connector);
    let inventory = Inventory::new(config.clone(), connector.clone()).await?;
    let inventory = Arc::new(inventory);
    let quoter =
        InterchainLiquidityHubQuoter::new(connector, inventory.clone(), config.balancer.clone());

    let usdc_sepolia = inventory.find_token_by_symbol("USDC".into(), ChainId::Sepolia.into())?;
    let usdt_sepolia = inventory.find_token_by_symbol("USDT".into(), ChainId::Sepolia.into())?;
    let usdc_fuji = inventory.find_token_by_symbol("USDC".into(), ChainId::Fuji.into())?;
    let usdt_fuji = inventory.find_token_by_symbol("USDT".into(), ChainId::Fuji.into())?;

    let source_amount =
        U256::from_str_radix("1000000000", 10).context("Failed to parse source amount")?;
    let intent_swap_usdc_to_usdt_sepolia = SwapIntent {
        source_token: usdc_sepolia.address,
        destination_token: usdt_sepolia.address,
        destination_chain_id: ChainId::Sepolia.into(),
        source_amount,

        intent_id: Default::default(),
        author: Default::default(),
        signature: Default::default(),
        source_permit_2: Default::default(),
        deadline: Default::default(),
        source_chain_id: ChainId::Fuji,
        nonce: Default::default(),
    };

    let intent_swap_usdt_to_usdc_sepolia = SwapIntent {
        source_token: usdt_sepolia.address,
        destination_token: usdc_sepolia.address,
        ..intent_swap_usdc_to_usdt_sepolia.clone()
    };

    let intent_swap_usdc_to_usdt_fuji = SwapIntent {
        source_token: usdc_fuji.address,
        destination_token: usdt_fuji.address,
        destination_chain_id: ChainId::Fuji.into(),
        ..intent_swap_usdc_to_usdt_sepolia.clone()
    };

    let intent_swap_usdt_to_usdc_fuji = SwapIntent {
        source_token: usdt_fuji.address,
        destination_token: usdc_fuji.address,
        destination_chain_id: ChainId::Fuji.into(),
        ..intent_swap_usdc_to_usdt_sepolia.clone()
    };

    let amounts: Vec<U256> = vec![
        quoter
            .quote_intent(intent_swap_usdc_to_usdt_sepolia)
            .await?,
        quoter
            .quote_intent(intent_swap_usdt_to_usdc_sepolia)
            .await?,
        quoter.quote_intent(intent_swap_usdc_to_usdt_fuji).await?,
        quoter.quote_intent(intent_swap_usdt_to_usdc_fuji).await?,
    ]
    .iter()
    .map(|quoted_intent| quoted_intent.kai_amount.base_units)
    .collect();

    // From 990 to 1010 per 1.
    let expected_range =
        U256::from(990000000000000000000u128)..U256::from(1010000000000000000000u128);

    for amount in amounts.iter() {
        assert_in_range(*amount, expected_range.clone());
    }

    Ok(())
}

fn assert_in_range(value: U256, range: Range<U256>) {
    assert!(
        range.contains(&value),
        "Value {} is not within the expected range {:?}",
        value,
        range
    );
}
