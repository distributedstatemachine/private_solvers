use ethers::abi::{encode_packed, Token};
use ethers::utils::keccak256;

use solver_common::types::intent_id::IntentId;

use crate::types::limit_order_intent::LimitOrderIntent;
use crate::types::spoke_chain_call::SpokeChainCall;
use crate::types::swap_intent::SwapIntent;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Intent {
    SpokeChainCall(SpokeChainCall),
    LimitOrder(LimitOrderIntent),
    SwapIntent(SwapIntent),
}

impl Intent {
    pub fn id(&self) -> IntentId {
        match self {
            Intent::SpokeChainCall(spoke_chain_caller) => spoke_chain_caller.intent_id,
            Intent::LimitOrder(limit_order_intent) => limit_order_intent.intent_id,
            Intent::SwapIntent(swap_intent) => swap_intent.intent_id,
        }
    }
}

impl From<Intent> for bindings_khalani::base_intent_book::Intent {
    fn from(value: Intent) -> Self {
        match value {
            Intent::SpokeChainCall(spoke_chain_call) => spoke_chain_call.into(),
            Intent::LimitOrder(limit_order_intent) => limit_order_intent.into(),
            Intent::SwapIntent(swap_intent) => swap_intent.into(),
        }
    }
}

pub fn calculate_intent_id(intent: bindings_khalani::base_intent_book::Intent) -> IntentId {
    keccak256(
        encode_packed(&[
            Token::Bytes(intent.intent.to_vec()),
            Token::Bytes(intent.signature.to_vec()),
        ])
        .unwrap(),
    )
    .into()
}

#[cfg(test)]
mod tests {
    use ethers::abi::AbiDecode;
    use ethers::types::{Address, Bytes, H256};
    use ethers::utils::hex::FromHex;

    use solver_common::config::chain::ChainId;

    use crate::types::intent::SwapIntent;

    use super::*;

    #[test]
    fn test_calculate_intent_id() {
        let swap_intent = SwapIntent {
            intent_id: Default::default(),
            signature: Bytes::from_hex("0xe368bbf77611d60510b61ea01042b987b5484390e3c7402333b73a8b1fbb406f5c21ace1fdade823ed7dc0b4a4a1c1ffaab63eba7e3f6cff923b6e1e29f6566a1c").unwrap(),
            source_chain_id: ChainId::Sepolia,

            destination_chain_id: ChainId::Fuji,
            author: "0x7f6371EC539b3b47A75FAa609748fC10C8bB6791"
                .parse::<Address>()
                .unwrap(),
            source_token: "0x2B5AD5c4795c026514f8317c7a215E218DcCD6cF"
                .parse::<Address>()
                .unwrap(),
            destination_token: "0x6813Eb9362372EEF6200f3b1dbC3f819671cBA69"
                .parse::<Address>()
                .unwrap(),
            source_amount: Default::default(),
            source_permit_2: Default::default(),
            deadline: Default::default(),
            nonce: Default::default(),
        };
        let contract_intent: bindings_khalani::base_intent_book::Intent = swap_intent.into();
        let intent_id = calculate_intent_id(contract_intent);
        let expected_intent_id = IntentId::from(
            H256::decode_hex("0x76347f79cb00041c374090d5368c061abd8ee93081c67a2318b35f7bb65192a3")
                .unwrap(),
        );
        assert_eq!(expected_intent_id, intent_id);
    }
}
