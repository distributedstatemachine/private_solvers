use std::ops::Range;
use std::sync::Arc;

use anyhow::Result;
use ethers::types::U256;

use khalani_solver::config::chain::KHALANI_CHAIN_ID;
use khalani_solver::inventory::Inventory;
use khalani_solver::quote::intent_quoter::IntentQuoter;
use khalani_solver::quote::interchain_liquidity_hub::interchain_liquidity_hub_quoter::InterchainLiquidityHubQuoter;
use khalani_solver::types::swap_intent::SwapIntent;

use crate::common::create_connector;
use crate::common::create_e2e_config;

#[tokio::test]
async fn test_interchain_liquidity_hub_quoter() -> Result<()> {
    let config = create_e2e_config();
    let connector = create_connector().await.unwrap();
    let connector = Arc::new(connector);
    let inventory = Inventory::new(config.clone(), connector.clone()).await?;
    let inventory = Arc::new(inventory);
    let quoter = InterchainLiquidityHubQuoter::new(
        connector,
        inventory.clone(),
        config.addresses.clone(),
        config.balancer.clone(),
    );

    let usdc_sepolia = inventory
        .find_token_by_symbol("USDC.sepolia".into(), KHALANI_CHAIN_ID)
        .unwrap();
    let usdt_sepolia = inventory
        .find_token_by_symbol("USDT.sepolia".into(), KHALANI_CHAIN_ID)
        .unwrap();
    let usdc_fuji = inventory
        .find_token_by_symbol("USDC.fuji".into(), KHALANI_CHAIN_ID)
        .unwrap();
    let usdt_fuji = inventory
        .find_token_by_symbol("USDT.fuji".into(), KHALANI_CHAIN_ID)
        .unwrap();

    let source_amount = U256::from_str_radix("1000", 10).unwrap();
    let intent_get_usdc_sepolia = SwapIntent {
        intent_id: Default::default(),
        author: Default::default(),
        signature: Default::default(),
        source_chain_id: Default::default(),
        destination_chain_id: Default::default(),
        source_token: usdc_sepolia.address,
        destination_token: usdc_sepolia.address,
        source_amount,
        source_permit_2: Default::default(),
        deadline: Default::default(),
        nonce: Default::default(),
    };

    let intent_get_usdt_sepolia = SwapIntent {
        source_token: usdt_sepolia.address,
        destination_token: usdt_sepolia.address,
        ..intent_get_usdc_sepolia.clone()
    };

    let intent_get_usdc_fuji = SwapIntent {
        source_token: usdc_fuji.address,
        destination_token: usdc_fuji.address,
        ..intent_get_usdc_sepolia.clone()
    };

    let intent_get_usdt_fuji = SwapIntent {
        source_token: usdt_fuji.address,
        destination_token: usdt_fuji.address,
        ..intent_get_usdc_sepolia.clone()
    };

    let amounts: Vec<U256> = vec![
        quoter.quote_intent(intent_get_usdc_sepolia).await?,
        quoter.quote_intent(intent_get_usdt_sepolia).await?,
        quoter.quote_intent(intent_get_usdc_fuji).await?,
        quoter.quote_intent(intent_get_usdt_fuji).await?,
    ]
    .iter()
    .map(|quoted_intent| quoted_intent.kai_amount.base_units)
    .collect();

    // From 990 to 1010 per 1.
    let expected_range = U256::from(990000000000000u64)..U256::from(1010000000000000u64);

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
