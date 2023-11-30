pub use deploy_constants::*;
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
pub mod deploy_constants {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("KHALANI_CHAIN_ID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("KHALANI_CHAIN_ID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("KHALANI_CHAIN_URL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("KHALANI_CHAIN_URL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("KHALANI_MAILBOX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("KHALANI_MAILBOX"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SPOKE_CHAIN_ID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("SPOKE_CHAIN_ID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SPOKE_CHAIN_MAILBOX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SPOKE_CHAIN_MAILBOX",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SPOKE_CHAIN_URL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("SPOKE_CHAIN_URL"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
    pub static DEPLOYCONSTANTS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct DeployConstants<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DeployConstants<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DeployConstants<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DeployConstants<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DeployConstants<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DeployConstants))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DeployConstants<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEPLOYCONSTANTS_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `KHALANI_CHAIN_ID` (0x8a7d0b46) function
        pub fn khalani_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([138, 125, 11, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `KHALANI_CHAIN_URL` (0x4aaddb5f) function
        pub fn khalani_chain_url(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([74, 173, 219, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `KHALANI_MAILBOX` (0x509752f9) function
        pub fn khalani_mailbox(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([80, 151, 82, 249], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SPOKE_CHAIN_ID` (0xab049489) function
        pub fn spoke_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([171, 4, 148, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SPOKE_CHAIN_MAILBOX` (0x80059ef4) function
        pub fn spoke_chain_mailbox(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([128, 5, 158, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SPOKE_CHAIN_URL` (0x8fbb9369) function
        pub fn spoke_chain_url(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([143, 187, 147, 105], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DeployConstants<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `KHALANI_CHAIN_ID` function with signature `KHALANI_CHAIN_ID()` and selector `0x8a7d0b46`
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
    #[ethcall(name = "KHALANI_CHAIN_ID", abi = "KHALANI_CHAIN_ID()")]
    pub struct KhalaniChainIdCall;
    ///Container type for all input parameters for the `KHALANI_CHAIN_URL` function with signature `KHALANI_CHAIN_URL()` and selector `0x4aaddb5f`
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
    #[ethcall(name = "KHALANI_CHAIN_URL", abi = "KHALANI_CHAIN_URL()")]
    pub struct KhalaniChainUrlCall;
    ///Container type for all input parameters for the `KHALANI_MAILBOX` function with signature `KHALANI_MAILBOX()` and selector `0x509752f9`
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
    #[ethcall(name = "KHALANI_MAILBOX", abi = "KHALANI_MAILBOX()")]
    pub struct KhalaniMailboxCall;
    ///Container type for all input parameters for the `SPOKE_CHAIN_ID` function with signature `SPOKE_CHAIN_ID()` and selector `0xab049489`
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
    #[ethcall(name = "SPOKE_CHAIN_ID", abi = "SPOKE_CHAIN_ID()")]
    pub struct SpokeChainIdCall;
    ///Container type for all input parameters for the `SPOKE_CHAIN_MAILBOX` function with signature `SPOKE_CHAIN_MAILBOX()` and selector `0x80059ef4`
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
    #[ethcall(name = "SPOKE_CHAIN_MAILBOX", abi = "SPOKE_CHAIN_MAILBOX()")]
    pub struct SpokeChainMailboxCall;
    ///Container type for all input parameters for the `SPOKE_CHAIN_URL` function with signature `SPOKE_CHAIN_URL()` and selector `0x8fbb9369`
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
    #[ethcall(name = "SPOKE_CHAIN_URL", abi = "SPOKE_CHAIN_URL()")]
    pub struct SpokeChainUrlCall;
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
    pub enum DeployConstantsCalls {
        KhalaniChainId(KhalaniChainIdCall),
        KhalaniChainUrl(KhalaniChainUrlCall),
        KhalaniMailbox(KhalaniMailboxCall),
        SpokeChainId(SpokeChainIdCall),
        SpokeChainMailbox(SpokeChainMailboxCall),
        SpokeChainUrl(SpokeChainUrlCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeployConstantsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <KhalaniChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KhalaniChainId(decoded));
            }
            if let Ok(decoded) = <KhalaniChainUrlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KhalaniChainUrl(decoded));
            }
            if let Ok(decoded) = <KhalaniMailboxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KhalaniMailbox(decoded));
            }
            if let Ok(decoded) = <SpokeChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SpokeChainId(decoded));
            }
            if let Ok(decoded) = <SpokeChainMailboxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SpokeChainMailbox(decoded));
            }
            if let Ok(decoded) = <SpokeChainUrlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SpokeChainUrl(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployConstantsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::KhalaniChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KhalaniChainUrl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KhalaniMailbox(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpokeChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpokeChainMailbox(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpokeChainUrl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DeployConstantsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::KhalaniChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::KhalaniChainUrl(element) => ::core::fmt::Display::fmt(element, f),
                Self::KhalaniMailbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpokeChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpokeChainMailbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpokeChainUrl(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<KhalaniChainIdCall> for DeployConstantsCalls {
        fn from(value: KhalaniChainIdCall) -> Self {
            Self::KhalaniChainId(value)
        }
    }
    impl ::core::convert::From<KhalaniChainUrlCall> for DeployConstantsCalls {
        fn from(value: KhalaniChainUrlCall) -> Self {
            Self::KhalaniChainUrl(value)
        }
    }
    impl ::core::convert::From<KhalaniMailboxCall> for DeployConstantsCalls {
        fn from(value: KhalaniMailboxCall) -> Self {
            Self::KhalaniMailbox(value)
        }
    }
    impl ::core::convert::From<SpokeChainIdCall> for DeployConstantsCalls {
        fn from(value: SpokeChainIdCall) -> Self {
            Self::SpokeChainId(value)
        }
    }
    impl ::core::convert::From<SpokeChainMailboxCall> for DeployConstantsCalls {
        fn from(value: SpokeChainMailboxCall) -> Self {
            Self::SpokeChainMailbox(value)
        }
    }
    impl ::core::convert::From<SpokeChainUrlCall> for DeployConstantsCalls {
        fn from(value: SpokeChainUrlCall) -> Self {
            Self::SpokeChainUrl(value)
        }
    }
    ///Container type for all return fields from the `KHALANI_CHAIN_ID` function with signature `KHALANI_CHAIN_ID()` and selector `0x8a7d0b46`
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
    pub struct KhalaniChainIdReturn(pub u32);
    ///Container type for all return fields from the `KHALANI_CHAIN_URL` function with signature `KHALANI_CHAIN_URL()` and selector `0x4aaddb5f`
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
    pub struct KhalaniChainUrlReturn(pub ::std::string::String);
    ///Container type for all return fields from the `KHALANI_MAILBOX` function with signature `KHALANI_MAILBOX()` and selector `0x509752f9`
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
    pub struct KhalaniMailboxReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `SPOKE_CHAIN_ID` function with signature `SPOKE_CHAIN_ID()` and selector `0xab049489`
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
    pub struct SpokeChainIdReturn(pub u32);
    ///Container type for all return fields from the `SPOKE_CHAIN_MAILBOX` function with signature `SPOKE_CHAIN_MAILBOX()` and selector `0x80059ef4`
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
    pub struct SpokeChainMailboxReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `SPOKE_CHAIN_URL` function with signature `SPOKE_CHAIN_URL()` and selector `0x8fbb9369`
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
    pub struct SpokeChainUrlReturn(pub ::std::string::String);
}
