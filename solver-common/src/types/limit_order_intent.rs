use crate::config::chain::ChainId;
use anyhow::anyhow;
use bindings_khalani::limit_order_intent_book::{
    Intent as ContractIntent, LimitOrder as ContractLimitOrder,
};
use ethers::types::{Address, Bytes, U256};
use std::sync::Arc;

use crate::inventory::{amount::Amount, token::Token, Inventory};
use crate::types::intent_id::{IntentId, WithIntentId};
use crate::types::swap_intent::{abi_decode_tuple, abi_encode_tuple};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LimitOrderIntent {
    pub intent_id: IntentId,
    pub author: Address,
    pub signature: Bytes,
    pub volume: Amount,
    pub token: Token,
    pub out_token: Token,
    pub price: U256,
}

impl TryFrom<WithIntentId<(Arc<Inventory>, ContractIntent)>> for LimitOrderIntent {
    type Error = anyhow::Error;

    fn try_from(
        value: WithIntentId<(Arc<Inventory>, ContractIntent)>,
    ) -> Result<Self, Self::Error> {
        let (intent_id, inventory_and_intent) = value;
        let (inventory, intent) = inventory_and_intent;
        let limit_order: ContractLimitOrder = abi_decode_tuple(intent.intent)?;
        let token = inventory
            .find_token_by_address(limit_order.token, ChainId::Khalani)
            .ok_or(anyhow!("Unknown LimitOrder token {}", limit_order.token))?;
        let out_token = inventory
            .find_token_by_address(limit_order.out_token, ChainId::Khalani)
            .ok_or(anyhow!(
                "Unknown LimitOrder out token {}",
                limit_order.out_token
            ))?;
        Ok(Self {
            intent_id,
            author: limit_order.author,
            signature: intent.signature,
            price: limit_order.price,
            token: token.clone(),
            out_token: out_token.clone(),
            volume: Amount::from_token_base_units(limit_order.volume, token),
        })
    }
}

impl From<LimitOrderIntent> for ContractLimitOrder {
    fn from(value: LimitOrderIntent) -> Self {
        Self {
            author: value.author,
            token: value.token.address,
            price: value.price,
            volume: value.volume.base_units,
            out_token: value.out_token.address,
        }
    }
}

impl From<LimitOrderIntent> for ContractIntent {
    fn from(value: LimitOrderIntent) -> Self {
        let limit_order: ContractLimitOrder = value.clone().into();
        Self {
            intent: abi_encode_tuple(limit_order),
            signature: value.signature.clone(),
        }
    }
}
