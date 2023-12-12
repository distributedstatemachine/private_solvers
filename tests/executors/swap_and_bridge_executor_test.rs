use std::ops::Range;
use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Executor;
use ethers::types::U256;

use khalani_solver::config::chain::ChainId;
use khalani_solver::inventory::token_allowance_query::TokenAllowanceQuery;
use khalani_solver::inventory::Inventory;
use khalani_solver::quote::intent_quoter::IntentQuoter;
use khalani_solver::quote::interchain_liquidity_hub::interchain_liquidity_hub_quoter::InterchainLiquidityHubQuoter;
use khalani_solver::types::swap_intent::SwapIntent;
use khalani_solver::workflow::action::Action;
use khalani_solver::workflow::executors::approve_tokens_executor::ApproveTokensExecutor;
use khalani_solver::workflow::executors::ethereum::send_transaction_approve_tokens_handler::SendTransactionApproveTokensHandler;
use khalani_solver::workflow::executors::ethereum::send_transaction_swap_and_bridge_handler::SendTransactionSwapAndBridgeHandler;
use khalani_solver::workflow::executors::swap_and_bridge_executor::SwapAndBridgeExecutor;

use crate::common::create_connector;
use crate::common::create_e2e_config;

#[tokio::test]
async fn test_swap_and_bridge_preview() -> Result<()> {
    let config = create_e2e_config().unwrap();
    let connector = create_connector().await?;
    let connector = Arc::new(connector);
    let inventory = Inventory::new(config.clone(), connector.clone()).await?;
    let inventory = Arc::new(inventory);
    let quoter = InterchainLiquidityHubQuoter::new(
        connector.clone(),
        inventory.clone(),
        config.balancer.clone(),
    );
    let quoter = Arc::new(quoter);
    let handler = SendTransactionSwapAndBridgeHandler::new(
        config.balancer.clone(),
        connector.clone(),
        quoter.clone(),
        inventory.clone(),
    )?;

    let kai_token = inventory.find_token_by_symbol("KAI".into(), ChainId::Khalani)?;

    let usdc_sepolia = inventory.find_token_by_symbol("USDC".into(), ChainId::Sepolia)?;
    let usdt_sepolia = inventory.find_token_by_symbol("USDT".into(), ChainId::Sepolia)?;

    let usdt_sepolia_mirror_token =
        inventory.find_token_by_symbol("USDT.sepolia".into(), ChainId::Khalani)?;

    let sender = connector.get_address();
    let source_amount = U256::from_str_radix("1000000000", 10).unwrap();
    let intent_swap_usdc_to_usdt_sepolia = SwapIntent {
        source_token: usdc_sepolia.address,
        destination_token: usdt_sepolia.address,
        destination_chain_id: ChainId::Sepolia.into(),
        source_amount,

        intent_id: Default::default(),
        author: Default::default(),
        deadline: Default::default(),
        nonce: Default::default(),
        source_chain_id: ChainId::Fuji,
        signature: Default::default(),
        source_permit_2: Default::default(),
    };

    let quoted_intent = quoter
        .quote_intent(intent_swap_usdc_to_usdt_sepolia)
        .await?;

    let kai_allowance = inventory
        .get_allowance(
            kai_token,
            sender,
            config.balancer.interchain_liquidity_hub_address,
        )
        .await?;

    if kai_allowance.lt(&quoted_intent.kai_amount) {
        println!("Approving tokens");
        let approve_handler = SendTransactionApproveTokensHandler::new(
            config.balancer.clone(),
            connector.clone(),
            inventory.clone(),
        );
        let approve_executor = ApproveTokensExecutor::new(approve_handler);
        approve_executor
            .execute(Action::ApproveTokens(quoted_intent.clone()))
            .await?;
    }

    let preview = handler
        .build_swap_and_bridge_tx(quoted_intent)
        .await?
        .call()
        .await?;

    let expected_range = U256::from(900000000u64)..U256::from(1100000000u64);

    assert_eq!(preview.len(), 1);
    assert_eq!(preview[0].token_address, usdt_sepolia_mirror_token.address);
    assert_in_range(preview[0].amount, expected_range.clone());

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test_swap_and_bridge_executor() -> Result<()> {
    let config = create_e2e_config().unwrap();
    let connector = create_connector().await?;
    let connector = Arc::new(connector);
    let inventory = Inventory::new(config.clone(), connector.clone()).await?;
    let inventory = Arc::new(inventory);
    let quoter = InterchainLiquidityHubQuoter::new(
        connector.clone(),
        inventory.clone(),
        config.balancer.clone(),
    );
    let quoter = Arc::new(quoter);
    let handler = SendTransactionSwapAndBridgeHandler::new(
        config.balancer.clone(),
        connector,
        quoter.clone(),
        inventory.clone(),
    )
    .unwrap();
    let executor = SwapAndBridgeExecutor::new(handler);

    let usdc_sepolia = inventory.find_token_by_symbol("USDC".into(), ChainId::Sepolia.into())?;
    let usdt_sepolia = inventory.find_token_by_symbol("USDT".into(), ChainId::Sepolia.into())?;

    let source_amount = U256::from_str_radix("1000000000", 10).unwrap();
    let intent_swap_usdc_to_usdt_sepolia = SwapIntent {
        source_token: usdc_sepolia.address,
        destination_token: usdt_sepolia.address,
        destination_chain_id: ChainId::Sepolia.into(),
        source_amount,

        intent_id: Default::default(),
        author: Default::default(),
        deadline: Default::default(),
        nonce: Default::default(),
        source_chain_id: ChainId::Fuji,
        signature: Default::default(),
        source_permit_2: Default::default(),
    };

    let quoted_intent = quoter
        .quote_intent(intent_swap_usdc_to_usdt_sepolia)
        .await?;
    executor
        .execute(Action::SwapAndBridge(quoted_intent))
        .await?;

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
