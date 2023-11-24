pub use intent_event_registerer::*;
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
pub mod intent_event_registerer {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("registerSwapIntentFulfilledEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerSwapIntentFulfilledEvent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("event_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentEventLibrary.SwapIntentFulfilled",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("registerSwapIntentTokenBurnEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerSwapIntentTokenBurnEvent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("event_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentEventLibrary.SwapIntentTokenBurn",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("registerSwapIntentTokenLockEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerSwapIntentTokenLockEvent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("event_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentEventLibrary.SwapIntentTokenLock",
                                        ),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static INTENTEVENTREGISTERER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IntentEventRegisterer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IntentEventRegisterer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IntentEventRegisterer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IntentEventRegisterer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IntentEventRegisterer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IntentEventRegisterer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IntentEventRegisterer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INTENTEVENTREGISTERER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `registerSwapIntentFulfilledEvent` (0xa4295156) function
        pub fn register_swap_intent_fulfilled_event(
            &self,
            event: SwapIntentFulfilled,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 41, 81, 86], (event,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerSwapIntentTokenBurnEvent` (0x5d0680dc) function
        pub fn register_swap_intent_token_burn_event(
            &self,
            event: SwapIntentTokenBurn,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 6, 128, 220], (event,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerSwapIntentTokenLockEvent` (0x13d8b954) function
        pub fn register_swap_intent_token_lock_event(
            &self,
            event: SwapIntentTokenLock,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 216, 185, 84], (event,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IntentEventRegisterer<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `registerSwapIntentFulfilledEvent` function with signature `registerSwapIntentFulfilledEvent((bytes32,address,uint256))` and selector `0xa4295156`
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
        name = "registerSwapIntentFulfilledEvent",
        abi = "registerSwapIntentFulfilledEvent((bytes32,address,uint256))"
    )]
    pub struct RegisterSwapIntentFulfilledEventCall {
        pub event: SwapIntentFulfilled,
    }
    ///Container type for all input parameters for the `registerSwapIntentTokenBurnEvent` function with signature `registerSwapIntentTokenBurnEvent((bytes32))` and selector `0x5d0680dc`
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
        name = "registerSwapIntentTokenBurnEvent",
        abi = "registerSwapIntentTokenBurnEvent((bytes32))"
    )]
    pub struct RegisterSwapIntentTokenBurnEventCall {
        pub event: SwapIntentTokenBurn,
    }
    ///Container type for all input parameters for the `registerSwapIntentTokenLockEvent` function with signature `registerSwapIntentTokenLockEvent((bytes32))` and selector `0x13d8b954`
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
        name = "registerSwapIntentTokenLockEvent",
        abi = "registerSwapIntentTokenLockEvent((bytes32))"
    )]
    pub struct RegisterSwapIntentTokenLockEventCall {
        pub event: SwapIntentTokenLock,
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
    pub enum IntentEventRegistererCalls {
        RegisterSwapIntentFulfilledEvent(RegisterSwapIntentFulfilledEventCall),
        RegisterSwapIntentTokenBurnEvent(RegisterSwapIntentTokenBurnEventCall),
        RegisterSwapIntentTokenLockEvent(RegisterSwapIntentTokenLockEventCall),
    }
    impl ::ethers::core::abi::AbiDecode for IntentEventRegistererCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <RegisterSwapIntentFulfilledEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterSwapIntentFulfilledEvent(decoded));
            }
            if let Ok(decoded) = <RegisterSwapIntentTokenBurnEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterSwapIntentTokenBurnEvent(decoded));
            }
            if let Ok(decoded) = <RegisterSwapIntentTokenLockEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterSwapIntentTokenLockEvent(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IntentEventRegistererCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::RegisterSwapIntentFulfilledEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterSwapIntentTokenBurnEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterSwapIntentTokenLockEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IntentEventRegistererCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RegisterSwapIntentFulfilledEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterSwapIntentTokenBurnEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterSwapIntentTokenLockEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<RegisterSwapIntentFulfilledEventCall>
    for IntentEventRegistererCalls {
        fn from(value: RegisterSwapIntentFulfilledEventCall) -> Self {
            Self::RegisterSwapIntentFulfilledEvent(value)
        }
    }
    impl ::core::convert::From<RegisterSwapIntentTokenBurnEventCall>
    for IntentEventRegistererCalls {
        fn from(value: RegisterSwapIntentTokenBurnEventCall) -> Self {
            Self::RegisterSwapIntentTokenBurnEvent(value)
        }
    }
    impl ::core::convert::From<RegisterSwapIntentTokenLockEventCall>
    for IntentEventRegistererCalls {
        fn from(value: RegisterSwapIntentTokenLockEventCall) -> Self {
            Self::RegisterSwapIntentTokenLockEvent(value)
        }
    }
}
