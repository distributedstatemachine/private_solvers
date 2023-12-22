use bindings_khalani::shared_types::Intent as ContractIntent;
use bindings_khalani::shared_types::SwapIntent as ContractSwapIntent;
use ethers::abi::{encode_packed, AbiDecode, AbiEncode, Token as AbiToken};
use ethers::types::{Address, BigEndianHash, Bytes, H256, U256};
use ethers::utils::hex::FromHex;
use ethers::utils::keccak256;

use solver_common::config::chain::ChainId;
use solver_common::types::intent_id::{IntentId, WithIntentId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwapIntent {
    pub intent_id: IntentId,
    pub author: Address,
    pub signature: Bytes,
    pub source_chain_id: ChainId,
    pub destination_chain_id: ChainId,
    pub source_token: Address,
    pub destination_token: Address,
    pub source_amount: U256,
    pub source_permit_2: Bytes,
    pub deadline: U256,
    pub nonce: U256,
}

impl SwapIntent {
    pub fn calculate_swap_intent_id(&self) -> H256 {
        keccak256(
            encode_packed(&[
                AbiToken::Address(self.author),
                AbiToken::FixedBytes(
                    Into::<u32>::into(self.source_chain_id)
                        .to_be_bytes()
                        .to_vec(),
                ),
                AbiToken::FixedBytes(
                    Into::<u32>::into(self.destination_chain_id)
                        .to_be_bytes()
                        .to_vec(),
                ),
                AbiToken::Address(self.source_token),
                AbiToken::Address(self.destination_token),
                AbiToken::FixedBytes(
                    H256::from_uint(&self.source_amount)
                        .to_fixed_bytes()
                        .to_vec(),
                ),
                AbiToken::Bytes(self.source_permit_2.to_vec()),
                AbiToken::FixedBytes(H256::from_uint(&self.nonce).to_fixed_bytes().to_vec()),
                AbiToken::FixedBytes(H256::from_uint(&self.deadline).to_fixed_bytes().to_vec()),
            ])
            .unwrap(),
        )
        .into()
    }
}

impl TryFrom<WithIntentId<ContractIntent>> for SwapIntent {
    type Error = anyhow::Error;

    fn try_from(value: WithIntentId<ContractIntent>) -> Result<Self, Self::Error> {
        let (intent_id, intent) = value;
        let value: ContractSwapIntent = ContractSwapIntent::decode(intent.intent)?;
        Ok(SwapIntent {
            intent_id,
            author: value.author,
            signature: intent.signature,
            source_chain_id: value.source_chain_id.try_into()?,
            destination_chain_id: value.destination_chain_id.try_into()?,
            source_token: value.source_token,
            destination_token: value.destination_token,
            source_amount: value.source_amount,
            source_permit_2: value.source_permit_2,
            deadline: value.deadline,
            nonce: value.nonce,
        })
    }
}

impl From<SwapIntent> for ContractSwapIntent {
    fn from(value: SwapIntent) -> Self {
        Self {
            author: value.author,
            source_chain_id: value.source_chain_id.into(),
            destination_chain_id: value.destination_chain_id.into(),
            source_token: value.source_token,
            destination_token: value.destination_token,
            source_amount: value.source_amount,
            source_permit_2: value.source_permit_2,
            deadline: value.deadline,
            nonce: value.nonce,
        }
    }
}

impl From<SwapIntent> for bindings_khalani::base_intent_book::Intent {
    fn from(value: SwapIntent) -> Self {
        let contract_swap_intent: ContractSwapIntent = value.clone().into();
        bindings_khalani::base_intent_book::Intent {
            intent: abi_encode_with_prefix(contract_swap_intent),
            signature: value.signature,
        }
    }
}

// TODO: I don't know why Solidity's abi.encode(...) adds a prefix of 0x00..20 but ethers-rs AbiEncode::encode does not.
pub fn abi_encode_with_prefix<T>(t: T) -> Bytes
where
    T: AbiEncode,
{
    let mut encoded =
        Bytes::from_hex("0x0000000000000000000000000000000000000000000000000000000000000020")
            .unwrap()
            .to_vec();
    encoded.extend(AbiEncode::encode(t));
    Bytes::from(encoded)
}

#[cfg(test)]
mod tests {
    use ethers::abi::AbiDecode;
    use ethers::types::{Address, H256};

    use super::*;

    #[test]
    fn test_calculate_swap_intent_id() {
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
            source_permit_2: Bytes::from_hex("0xabcd").unwrap(),
            deadline: Default::default(),
            nonce: Default::default(),
        };
        let swap_intent_id = swap_intent.calculate_swap_intent_id();
        let expected_swap_intent_id =
            H256::decode_hex("0x897a3b81b3017617c14e99aba8c6373315c68ee8054aebb944c274710ad8b406")
                .unwrap();
        assert_eq!(expected_swap_intent_id, swap_intent_id);
    }
}
