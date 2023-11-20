use std::sync::Arc;

use anyhow::Result;

use khalani_solver::inventory::inventory::Inventory;
use khalani_solver::inventory::token_balance_query::TokenBalanceQuery;

use crate::common::{create_connector, create_e2e_config};

#[tokio::test]
async fn test_inventory() -> Result<()> {
    let config = create_e2e_config();
    let connector = create_connector().await.unwrap();
    let connector = Arc::new(connector);
    let inventory = Inventory::new(config, connector.clone()).await?;
    let usdc_token = inventory.tokens.first().unwrap();
    println!("{:?}", usdc_token);
    let address = connector.get_address();
    let balance = inventory.get_balance(usdc_token, address).await?;
    println!("Balance of {} is {}", address, balance);
    Ok(())
}
