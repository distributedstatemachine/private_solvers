use anyhow::Result;
use bindings_khalani::shared_types::IntentBid as ContractIntent;
use bindings_khalani::swap_intent_book::SwapIntentBid as ContractSwapIntentBid;
use ethers::abi::{encode_packed, AbiDecode, AbiEncode, Token as AbiToken};
use ethers::types::{Address, Bytes, H256, U256};
use ethers::utils::keccak256;

use crate::types::swap_intent::SwapIntent;
use solver_common::types::intent_id::{IntentBidId, IntentId, WithIntentIdAndBidId};
use solver_common::types::proof_id::ProofId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwapIntentBid {
    pub intent_id: IntentId,
    pub intent_bid_id: IntentBidId,
    pub filler: Address,
    pub fill_amount: U256,
}

impl TryFrom<WithIntentIdAndBidId<ContractIntent>> for SwapIntentBid {
    type Error = anyhow::Error;

    fn try_from(value: WithIntentIdAndBidId<ContractIntent>) -> Result<Self, Self::Error> {
        let (intent_id, intent_bid_id, intent_bid) = value;
        let value: ContractSwapIntentBid = ContractSwapIntentBid::decode(intent_bid.bid)?;
        Ok(SwapIntentBid {
            intent_id,
            intent_bid_id,
            filler: value.filler,
            fill_amount: value.fill_amount,
        })
    }
}

impl From<SwapIntentBid> for ContractSwapIntentBid {
    fn from(value: SwapIntentBid) -> Self {
        Self {
            filler: value.filler,
            fill_amount: value.fill_amount,
            fill_timestamp: U256::zero(), // TODO: will be removed.
        }
    }
}

impl From<SwapIntentBid> for bindings_khalani::base_intent_book::IntentBid {
    fn from(value: SwapIntentBid) -> Self {
        let bid: ContractSwapIntentBid = value.clone().into();
        Self {
            intent_id: value.intent_id.into(),
            bid: Bytes::from(bid.encode()),
        }
    }
}

impl SwapIntentBid {
    pub fn get_expected_proofs(&self, swap_intent: &SwapIntent) -> Result<Vec<ProofId>> {
        let swap_intent_id = swap_intent.calculate_swap_intent_id();
        Ok(vec![
            self.get_swap_intent_token_lock_proof_id(swap_intent_id),
            self.get_swap_intent_filled_proof_id(swap_intent_id),
        ])
    }

    fn get_swap_intent_token_lock_proof_id(&self, swap_intent_id: H256) -> ProofId {
        keccak256(
            // TODO[solidity]: ensure this encoding is exactly what Solidity returns (write a test).
            encode_packed(&[
                AbiToken::String(String::from("SwapIntentTokenLock")),
                AbiToken::FixedBytes(Vec::from(swap_intent_id.as_bytes())),
            ])
            .unwrap(),
        )
        .into()
    }

    fn get_swap_intent_filled_proof_id(&self, swap_intent_id: H256) -> ProofId {
        keccak256(
            // TODO[solidity]: ensure this encoding is exactly what Solidity returns (write a test).
            encode_packed(&[
                AbiToken::String(String::from("SwapIntentFilled")),
                AbiToken::FixedBytes(Vec::from(swap_intent_id.as_bytes())),
                AbiToken::Address(self.filler),
                AbiToken::FixedBytes(Vec::from(
                    H256::from_low_u64_be(self.fill_amount.as_u64()).as_bytes(),
                )),
            ])
            .unwrap(),
        )
        .into()
    }
}
