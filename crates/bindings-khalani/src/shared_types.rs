///`BatchSwapStep(bytes32,uint256,uint256,uint256,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct BatchSwapStep {
    pub pool_id: [u8; 32],
    pub asset_in_index: ::ethers::core::types::U256,
    pub asset_out_index: ::ethers::core::types::U256,
    pub amount: ::ethers::core::types::U256,
    pub user_data: ::ethers::core::types::Bytes,
}
///`FundManagement(address,bool,address,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FundManagement {
    pub sender: ::ethers::core::types::Address,
    pub from_internal_balance: bool,
    pub recipient: ::ethers::core::types::Address,
    pub to_internal_balance: bool,
}
///`FacetCut(address,uint8,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FacetCut {
    pub facet_address: ::ethers::core::types::Address,
    pub action: u8,
    pub function_selectors: ::std::vec::Vec<[u8; 4]>,
}
///`Facet(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Facet {
    pub facet_address: ::ethers::core::types::Address,
    pub function_selectors: ::std::vec::Vec<[u8; 4]>,
}
///`FuzzSelector(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}
///`Token(address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Token {
    pub token_address: ::ethers::core::types::Address,
    pub amount: ::ethers::core::types::U256,
}
