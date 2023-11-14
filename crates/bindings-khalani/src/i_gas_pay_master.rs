pub use i_gas_pay_master::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_gas_pay_master {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("destinationGasAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "destinationGasAmount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_destinationDomain",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_numTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payForGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("payForGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_destinationDomain",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_numTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_refundAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quoteSend"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quoteSend"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_destinationDomain",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_numTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDestinationGasOverhead"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setDestinationGasOverhead",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_destinationDomain",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasOverhead"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setUnitTokenGasOverhead"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setUnitTokenGasOverhead",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_unitTokenGasOverhead",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("GasPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GasPayment"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gasAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IGASPAYMASTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IGasPayMaster<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IGasPayMaster<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IGasPayMaster<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IGasPayMaster<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IGasPayMaster<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IGasPayMaster))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IGasPayMaster<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IGASPAYMASTER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `destinationGasAmount` (0x2fd88292) function
        pub fn destination_gas_amount(
            &self,
            destination_domain: u32,
            num_tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([47, 216, 130, 146], (destination_domain, num_tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payForGas` (0x11bf2c18) function
        pub fn pay_for_gas(
            &self,
            message_id: [u8; 32],
            destination_domain: u32,
            num_tokens: ::ethers::core::types::U256,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [17, 191, 44, 24],
                    (message_id, destination_domain, num_tokens, refund_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteSend` (0xfd802cac) function
        pub fn quote_send(
            &self,
            destination_domain: u32,
            num_tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([253, 128, 44, 172], (destination_domain, num_tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDestinationGasOverhead` (0x320a0bc5) function
        pub fn set_destination_gas_overhead(
            &self,
            destination_domain: u32,
            gas_overhead: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 10, 11, 197], (destination_domain, gas_overhead))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUnitTokenGasOverhead` (0x83a6e659) function
        pub fn set_unit_token_gas_overhead(
            &self,
            unit_token_gas_overhead: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 166, 230, 89], unit_token_gas_overhead)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `GasPayment` event
        pub fn gas_payment_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GasPaymentFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GasPaymentFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IGasPayMaster<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "GasPayment", abi = "GasPayment(bytes32,uint256,uint256)")]
    pub struct GasPaymentFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
        pub gas_amount: ::ethers::core::types::U256,
        pub payment: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `destinationGasAmount` function with signature `destinationGasAmount(uint32,uint256)` and selector `0x2fd88292`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "destinationGasAmount",
        abi = "destinationGasAmount(uint32,uint256)"
    )]
    pub struct DestinationGasAmountCall {
        pub destination_domain: u32,
        pub num_tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `payForGas` function with signature `payForGas(bytes32,uint32,uint256,address)` and selector `0x11bf2c18`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "payForGas", abi = "payForGas(bytes32,uint32,uint256,address)")]
    pub struct PayForGasCall {
        pub message_id: [u8; 32],
        pub destination_domain: u32,
        pub num_tokens: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `quoteSend` function with signature `quoteSend(uint32,uint256)` and selector `0xfd802cac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "quoteSend", abi = "quoteSend(uint32,uint256)")]
    pub struct QuoteSendCall {
        pub destination_domain: u32,
        pub num_tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setDestinationGasOverhead` function with signature `setDestinationGasOverhead(uint32,uint256)` and selector `0x320a0bc5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setDestinationGasOverhead",
        abi = "setDestinationGasOverhead(uint32,uint256)"
    )]
    pub struct SetDestinationGasOverheadCall {
        pub destination_domain: u32,
        pub gas_overhead: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setUnitTokenGasOverhead` function with signature `setUnitTokenGasOverhead(uint256)` and selector `0x83a6e659`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setUnitTokenGasOverhead",
        abi = "setUnitTokenGasOverhead(uint256)"
    )]
    pub struct SetUnitTokenGasOverheadCall {
        pub unit_token_gas_overhead: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum IGasPayMasterCalls {
        DestinationGasAmount(DestinationGasAmountCall),
        PayForGas(PayForGasCall),
        QuoteSend(QuoteSendCall),
        SetDestinationGasOverhead(SetDestinationGasOverheadCall),
        SetUnitTokenGasOverhead(SetUnitTokenGasOverheadCall),
    }
    impl ::ethers::core::abi::AbiDecode for IGasPayMasterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DestinationGasAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DestinationGasAmount(decoded));
            }
            if let Ok(decoded) = <PayForGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PayForGas(decoded));
            }
            if let Ok(decoded) = <QuoteSendCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteSend(decoded));
            }
            if let Ok(decoded) = <SetDestinationGasOverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDestinationGasOverhead(decoded));
            }
            if let Ok(decoded) = <SetUnitTokenGasOverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUnitTokenGasOverhead(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IGasPayMasterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DestinationGasAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayForGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteSend(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDestinationGasOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUnitTokenGasOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IGasPayMasterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DestinationGasAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PayForGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteSend(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDestinationGasOverhead(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetUnitTokenGasOverhead(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DestinationGasAmountCall> for IGasPayMasterCalls {
        fn from(value: DestinationGasAmountCall) -> Self {
            Self::DestinationGasAmount(value)
        }
    }
    impl ::core::convert::From<PayForGasCall> for IGasPayMasterCalls {
        fn from(value: PayForGasCall) -> Self {
            Self::PayForGas(value)
        }
    }
    impl ::core::convert::From<QuoteSendCall> for IGasPayMasterCalls {
        fn from(value: QuoteSendCall) -> Self {
            Self::QuoteSend(value)
        }
    }
    impl ::core::convert::From<SetDestinationGasOverheadCall> for IGasPayMasterCalls {
        fn from(value: SetDestinationGasOverheadCall) -> Self {
            Self::SetDestinationGasOverhead(value)
        }
    }
    impl ::core::convert::From<SetUnitTokenGasOverheadCall> for IGasPayMasterCalls {
        fn from(value: SetUnitTokenGasOverheadCall) -> Self {
            Self::SetUnitTokenGasOverhead(value)
        }
    }
    ///Container type for all return fields from the `destinationGasAmount` function with signature `destinationGasAmount(uint32,uint256)` and selector `0x2fd88292`
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
    pub struct DestinationGasAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `quoteSend` function with signature `quoteSend(uint32,uint256)` and selector `0xfd802cac`
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
    pub struct QuoteSendReturn(pub ::ethers::core::types::U256);
}
