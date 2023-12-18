use std::sync::Arc;

use crate::quote::intent_quoter::IntentQuoter;
use crate::quote::quoted_intent::QuotedIntent;
use crate::types::limit_order_intent::LimitOrderIntent;
use solver_common::config::addresses::AddressesConfig;
use solver_common::config::chain::ChainId;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::inventory::amount::Amount;
use solver_common::inventory::Inventory;

use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::intents_mempool::IntentsMempool;
use ethers::types::U256;
use tracing::info;

pub struct LimitOrderIntentQuoter {
    inventory: Arc<Inventory>,
    intents_book: AddressesConfig,
    intents_book_contract: IntentsMempool<RpcClient>,
}

impl LimitOrderIntentQuoter {
    pub fn new(
        connector: Arc<Connector>,
        inventory: Arc<Inventory>,
        intents_book: AddressesConfig,
    ) -> Self {
        let client = connector.get_rpc_client(ChainId::Khalani).unwrap();
        let intents_book_contract =
            IntentsMempool::new(intents_book.intents_mempool_address, client.clone());

        Self {
            intents_book,
            inventory,
            intents_book_contract,
        }
    }
}

#[allow(unreachable_code)]
#[async_trait]
impl IntentQuoter for LimitOrderIntentQuoter {
    async fn quote_intent(&self, limit_order_intent: LimitOrderIntent) -> Result<QuotedIntent> {
        info!(?limit_order_intent, "Quoting Limit Order Intent");

        // Naive Strategy , always charge by over 1 percent
        let one_percent =
            Amount::from_base_units(U256::from(101), limit_order_intent.amount.decimals);
        let maker_amount = limit_order_intent.amount * one_percent;
        let taker_amount = limit_order_intent.amount;
        let quoted_intent = QuotedIntent {
            limit_order_intent,
            maker_amount,
            taker_amount,
        };
        info!(?quoted_intent, "Intent quoted");
        Ok(quoted_intent)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::intent_id::IntentId;

    use super::*;
    use ethers::core::rand::rngs::OsRng;
    use ethers::signers::Wallet;
    use ethers::types::U256;
    use solver_common::config::addresses::AddressesConfig;
    use solver_common::config::chain::ChainId;
    use solver_common::config::Config;
    use solver_common::connectors::Connector;
    use solver_common::inventory::amount::Amount;
    use solver_common::inventory::token::Token;
    use solver_common::inventory::Inventory;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_quote_intent() {
        let config = Config::read_config("../../config/config.json").unwrap();
        let wallet = Wallet::new(&mut OsRng);
        let connector = Arc::new(Connector::new(config.clone(), wallet).await.unwrap());
        let inventory = Arc::new(Inventory::new(config, connector).await.unwrap());
        let intents_book = AddressesConfig {
            intents_mempool_address: "0x0000000000000000000000000000000000000000"
                .parse()
                .unwrap(),
            settlement_reactor_address: "0x0000000000000000000000000000000000000000"
                .parse()
                .unwrap(),
            escrows: HashMap::new(),
            verifiers: Vec::new(),
            swap_intent_fillers: HashMap::new(),
        };

        let quoter = LimitOrderIntentQuoter::new(connector, inventory, intents_book);
        let token = Token {
            chain_id: ChainId::Khalani,
            address: "0x00000000000000000000000000000000000000021"
                .parse()
                .unwrap(),
            name: "Test Token".to_string(),
            symbol: "TT".to_string(),
            decimals: 18,
        };

        let limit_order_intent = LimitOrderIntent {
            intent_id: IntentId::default(),
            author: "0x0000000000000000000000000000000000000000"
                .parse()
                .unwrap(),
            signature: vec![0u8; 65].into(),
            amount: Amount::from_base_units(U256::from(100), 18),
            token: token,
        };

        let quoted_intent = quoter.quote_intent(limit_order_intent).await.unwrap();

        assert_eq!(
            quoted_intent.maker_amount,
            Amount::from_base_units(U256::from(101), 18)
        );
        assert_eq!(
            quoted_intent.taker_amount,
            Amount::from_base_units(U256::from(100), 18)
        );
    }
}
