use std::ops::Range;
use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Executor;
use bindings_khalani::utilities::SwapIntent;
use ethers::abi;
use ethers::types::H160;
use ethers::types::U256;

use solver_common::config::chain::ChainId;
use solver_common::inventory::token_allowance_query::TokenAllowanceQuery;
use solver_common::inventory::Inventory;
use swap_intent_settler::quote::intent_quoter::IntentQuoter;
use swap_intent_settler::quote::interchain_liquidity_hub::interchain_liquidity_hub_quoter::InterchainLiquidityHubQuoter;
// use swap_intent_settler::types::swap_intent::SwapIntent;
use swap_intent_settler::workflow::action::Action;
use swap_intent_settler::workflow::executors::approve_tokens_executor::ApproveTokensExecutor;
use swap_intent_settler::workflow::executors::ethereum::send_transaction_approve_tokens_handler::SendTransactionApproveTokensHandler;
use swap_intent_settler::workflow::executors::ethereum::send_transaction_swap_and_bridge_handler::SendTransactionSwapAndBridgeHandler;
use swap_intent_settler::workflow::executors::swap_and_bridge_executor::SwapAndBridgeExecutor;

use spoke_chain_caller::types::intent::SpokeChainCallIntent;

use bindings_khalani::escrow::Escrow;

use crate::common::create_connector;
use crate::common::create_e2e_config;

#[tokio::test]
async fn test_settler() -> Result<()> {
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

    let source_chain_id = ChainId::Sepolia;
    let rpc_client = connector.get_rpc_client(source_chain_id)?;

    let escrow_address = H160::default();
    let escrow = Escrow::new(escrow_address, rpc_client);

    let source_amount = U256::from_str_radix("1000000000", 10).unwrap();
    let intent_swap_usdc_to_usdt_sepolia = SwapIntent {
        source_token: usdc_sepolia.address,
        destination_token: usdt_sepolia.address,
        destination_chain_id: ChainId::Sepolia.into(),

        source_amount: Default::default(),
        author: Default::default(),
        deadline: Default::default(),
        nonce: Default::default(),
        source_chain_id: source_chain_id.into(),
        signature: Default::default(),
        source_permit_2: Default::default(),
    };
    let spoke_chain_call_intent = SpokeChainCallIntent {
        amount: source_amount,
        call_data: escrow
            .lock_tokens(intent_swap_usdc_to_usdt_sepolia)
            .calldata()
            .unwrap(),
        chain_id: ChainId::Sepolia.into(),
        intent_id: Default::default(),
        token: Default::default(),
    };

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
