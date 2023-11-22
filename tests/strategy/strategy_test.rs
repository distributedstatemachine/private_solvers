use std::sync::Arc;

use anyhow::Result;
use ethers::types::U256;

use khalani_solver::config::chain::KHALANI_CHAIN_ID;
use khalani_solver::inventory::amount::Amount;
use khalani_solver::inventory::inventory::Inventory;
use khalani_solver::{
    collectors::intents_collector::NewSwapIntent, strategies::intents_strategy::IntentsStrategy,
    types::swap_intent::SwapIntent,
};

use crate::common::create_connector;
use crate::common::create_e2e_config;

#[tokio::test]
async fn test_strategy() -> Result<()> {
    let config = create_e2e_config();
    let connector = create_connector().await.unwrap();
    let connector = Arc::new(connector);
    let inventory = Inventory::new(config.clone(), connector.clone()).await?;
    let inventory = Arc::new(inventory);
    let mut strategy = IntentsStrategy::new(
        connector,
        inventory.clone(),
        config.addresses.vault_address,
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
    let kai = inventory
        .find_token_by_symbol("KAI".into(), KHALANI_CHAIN_ID)
        .unwrap();

    let intent_get_usdc_sepolia: NewSwapIntent = NewSwapIntent(SwapIntent {
        intent_id: Default::default(),
        author: Default::default(),
        signature: Default::default(),
        source_chain_id: Default::default(),
        destination_chain_id: Default::default(),
        source_token: usdc_sepolia.address,
        destination_token: usdc_sepolia.address,
        source_amount: U256::from_str_radix("1000", 10).unwrap(),
        source_permit_2: Default::default(),
    });

    let intent_get_usdt_sepolia: NewSwapIntent = NewSwapIntent(SwapIntent {
        source_token: usdt_sepolia.address,
        destination_token: usdt_sepolia.address,
        ..intent_get_usdc_sepolia.0.clone()
    });

    let intent_get_usdc_fuji: NewSwapIntent = NewSwapIntent(SwapIntent {
        source_token: usdc_fuji.address,
        destination_token: usdc_fuji.address,
        ..intent_get_usdc_sepolia.0.clone()
    });

    let intent_get_usdt_fuji: NewSwapIntent = NewSwapIntent(SwapIntent {
        source_token: usdt_fuji.address,
        destination_token: usdt_fuji.address,
        ..intent_get_usdc_sepolia.0.clone()
    });

    let amounts = vec![
        strategy
            .get_kai_amount_to_fulfill_intent(intent_get_usdc_sepolia)
            .await?,
        strategy
            .get_kai_amount_to_fulfill_intent(intent_get_usdt_sepolia)
            .await?,
        strategy
            .get_kai_amount_to_fulfill_intent(intent_get_usdc_fuji)
            .await?,
        strategy
            .get_kai_amount_to_fulfill_intent(intent_get_usdt_fuji)
            .await?,
    ];
    let expected_ranges: Vec<Vec<i64>> = vec![
        vec![990005080912622, 997705080912622],
        vec![996965540372730, 999965540372730],
        vec![1000640051552452, 1009640051552452],
        vec![990593362546388, 999593362546388],
    ];

    println!("quoted amounts: {:?}", amounts);

    for (index, amount) in amounts.iter().enumerate() {
        assert!(
            *amount >= Amount::from_base_units(expected_ranges[index][0].into(), kai.decimals),
            "quoted amount is lower than minimum expected amount"
        );
        assert!(
            *amount <= Amount::from_base_units(expected_ranges[index][1].into(), kai.decimals),
            "quoted amount is higher than maximum expected amount"
        );
    }

    Ok(())
}
