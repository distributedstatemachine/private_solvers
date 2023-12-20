use std::ops::Range;
use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::Executor;
use ethers::types::U256;

use solver_common::config::chain::ChainId;
use solver_common::inventory::token_allowance_query::TokenAllowanceQuery;
use solver_common::inventory::Inventory;
use solver_common::tests::connector::{create_connector, create_e2e_config};

use spoke_chain_caller::workflow::executors::call_spoke_executor::CallSpokeExecutor;
use spoke_chain_caller::workflow::executors::ethereum::send_transaction_call_spoke_handler::SendTransactionCallSpokeHandler;
use spoke_chain_caller::workflow::action::Action;
use spoke_chain_caller::types::spoke_chain_call::SpokeChainCall;

abigen!(
    MockContractToCall,
    r#"[
        function mockFunction(address _token, uint256 _amount) external
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

#[ignore]
#[tokio::test]
async fn test_swap_and_bridge_executor() -> Result<()> {
    let config = create_e2e_config().unwrap();
    let connector = create_connector().await?;
    let connector = Arc::new(connector);
    let inventory = Inventory::new(config.clone(), connector.clone()).await?;
    let inventory = Arc::new(inventory);
    let handler = SendTransactionCallSpokeHandler::new(
        config.addresses.clone(),
        connector,
    )
    .unwrap();
    let executor = CallSpokeExecutor::new(handler);

    let usdc_sepolia = inventory.find_token_by_symbol("USDC".into(), ChainId::Sepolia.into())?;

    let contract_to_call_address = "0xbfa2dd37EC75C27CDbaDdd0c19e0f69fD277c16b"; // Replace with your actual address
    let call_data = MockContractToCall::mock_function(usdc_sepolia.address, amount)?;

    //TO-DO: mock and execute contract to call
    let amount = U256::from_str_radix("1000000000", 10).unwrap();
    let spoke_chain_call_payload = SpokeChainCall {
        token: usdc_sepolia.address,
        amount,
        intent_id: Default::default(),
        author: Default::default(),
        chain_id: ChainId::Sepolia,
        contract_to_call: contract_to_call_address.to_string(),
        call_data: call_data.into(),
    };

    executor
        .execute(Action::CallSpoke(spoke_chain_call_payload))
        .await?;

    Ok(())
}