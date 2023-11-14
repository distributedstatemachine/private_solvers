pub use i_asset_reserves::*;
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
pub mod i_asset_reserves {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("kai"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("kai"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IERC20MintableBurnable",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lockOrBurn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lockOrBurn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Token[]"),
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
                    ::std::borrow::ToOwned::to_owned("mintOrUnlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintOrUnlock"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mirrorTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Token[]"),
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
                    ::std::borrow::ToOwned::to_owned("AssetAddedToWhitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AssetAddedToWhitelist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssetRebalanced"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssetRebalanced"),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssetRemovedFromWhitelist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AssetRemovedFromWhitelist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LockOrBurn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LockOrBurn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MintOrUnlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MintOrUnlock"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("mirrorTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MirrorTokenSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MirrorTokenSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("mirrorToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NexusChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NexusChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldNexus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newNexus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TargetAvailableLiquidityChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TargetAvailableLiquidityChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAmount"),
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
    pub static IASSETRESERVES_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct IAssetReserves<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IAssetReserves<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IAssetReserves<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IAssetReserves<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IAssetReserves<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IAssetReserves))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IAssetReserves<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IASSETRESERVES_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `kai` (0xe8a2b16a) function
        pub fn kai(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([232, 162, 177, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockOrBurn` (0x9294523c) function
        pub fn lock_or_burn(
            &self,
            sender: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<Token>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 148, 82, 60], (sender, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintOrUnlock` (0x330a2767) function
        pub fn mint_or_unlock(
            &self,
            target: ::ethers::core::types::Address,
            mirror_tokens: ::std::vec::Vec<Token>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 10, 39, 103], (target, mirror_tokens))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AssetAddedToWhitelist` event
        pub fn asset_added_to_whitelist_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssetAddedToWhitelistFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssetRebalanced` event
        pub fn asset_rebalanced_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssetRebalancedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AssetRemovedFromWhitelist` event
        pub fn asset_removed_from_whitelist_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssetRemovedFromWhitelistFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LockOrBurn` event
        pub fn lock_or_burn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LockOrBurnFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MintOrUnlock` event
        pub fn mint_or_unlock_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MintOrUnlockFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MirrorTokenSet` event
        pub fn mirror_token_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MirrorTokenSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NexusChanged` event
        pub fn nexus_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NexusChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TargetAvailableLiquidityChanged` event
        pub fn target_available_liquidity_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TargetAvailableLiquidityChangedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IAssetReservesEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IAssetReserves<M> {
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
    #[ethevent(name = "AssetAddedToWhitelist", abi = "AssetAddedToWhitelist(address)")]
    pub struct AssetAddedToWhitelistFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
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
    #[ethevent(name = "AssetRebalanced", abi = "AssetRebalanced()")]
    pub struct AssetRebalancedFilter;
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
    #[ethevent(
        name = "AssetRemovedFromWhitelist",
        abi = "AssetRemovedFromWhitelist(address)"
    )]
    pub struct AssetRemovedFromWhitelistFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
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
    #[ethevent(name = "LockOrBurn", abi = "LockOrBurn(address,(address,uint256)[])")]
    pub struct LockOrBurnFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<Token>,
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
    #[ethevent(name = "MintOrUnlock", abi = "MintOrUnlock(address,(address,uint256)[])")]
    pub struct MintOrUnlockFilter {
        #[ethevent(indexed)]
        pub target: ::ethers::core::types::Address,
        pub mirror_tokens: ::std::vec::Vec<Token>,
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
    #[ethevent(name = "MirrorTokenSet", abi = "MirrorTokenSet(address,address)")]
    pub struct MirrorTokenSetFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub mirror_token: ::ethers::core::types::Address,
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
    #[ethevent(name = "NexusChanged", abi = "NexusChanged(address,address)")]
    pub struct NexusChangedFilter {
        pub old_nexus: ::ethers::core::types::Address,
        pub new_nexus: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "TargetAvailableLiquidityChanged",
        abi = "TargetAvailableLiquidityChanged(address,uint256)"
    )]
    pub struct TargetAvailableLiquidityChangedFilter {
        pub token: ::ethers::core::types::Address,
        pub new_amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
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
    pub enum IAssetReservesEvents {
        AssetAddedToWhitelistFilter(AssetAddedToWhitelistFilter),
        AssetRebalancedFilter(AssetRebalancedFilter),
        AssetRemovedFromWhitelistFilter(AssetRemovedFromWhitelistFilter),
        LockOrBurnFilter(LockOrBurnFilter),
        MintOrUnlockFilter(MintOrUnlockFilter),
        MirrorTokenSetFilter(MirrorTokenSetFilter),
        NexusChangedFilter(NexusChangedFilter),
        TargetAvailableLiquidityChangedFilter(TargetAvailableLiquidityChangedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IAssetReservesEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssetAddedToWhitelistFilter::decode_log(log) {
                return Ok(IAssetReservesEvents::AssetAddedToWhitelistFilter(decoded));
            }
            if let Ok(decoded) = AssetRebalancedFilter::decode_log(log) {
                return Ok(IAssetReservesEvents::AssetRebalancedFilter(decoded));
            }
            if let Ok(decoded) = AssetRemovedFromWhitelistFilter::decode_log(log) {
                return Ok(
                    IAssetReservesEvents::AssetRemovedFromWhitelistFilter(decoded),
                );
            }
            if let Ok(decoded) = LockOrBurnFilter::decode_log(log) {
                return Ok(IAssetReservesEvents::LockOrBurnFilter(decoded));
            }
            if let Ok(decoded) = MintOrUnlockFilter::decode_log(log) {
                return Ok(IAssetReservesEvents::MintOrUnlockFilter(decoded));
            }
            if let Ok(decoded) = MirrorTokenSetFilter::decode_log(log) {
                return Ok(IAssetReservesEvents::MirrorTokenSetFilter(decoded));
            }
            if let Ok(decoded) = NexusChangedFilter::decode_log(log) {
                return Ok(IAssetReservesEvents::NexusChangedFilter(decoded));
            }
            if let Ok(decoded) = TargetAvailableLiquidityChangedFilter::decode_log(log) {
                return Ok(
                    IAssetReservesEvents::TargetAvailableLiquidityChangedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IAssetReservesEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetAddedToWhitelistFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssetRebalancedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssetRemovedFromWhitelistFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LockOrBurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintOrUnlockFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MirrorTokenSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NexusChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TargetAvailableLiquidityChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AssetAddedToWhitelistFilter> for IAssetReservesEvents {
        fn from(value: AssetAddedToWhitelistFilter) -> Self {
            Self::AssetAddedToWhitelistFilter(value)
        }
    }
    impl ::core::convert::From<AssetRebalancedFilter> for IAssetReservesEvents {
        fn from(value: AssetRebalancedFilter) -> Self {
            Self::AssetRebalancedFilter(value)
        }
    }
    impl ::core::convert::From<AssetRemovedFromWhitelistFilter>
    for IAssetReservesEvents {
        fn from(value: AssetRemovedFromWhitelistFilter) -> Self {
            Self::AssetRemovedFromWhitelistFilter(value)
        }
    }
    impl ::core::convert::From<LockOrBurnFilter> for IAssetReservesEvents {
        fn from(value: LockOrBurnFilter) -> Self {
            Self::LockOrBurnFilter(value)
        }
    }
    impl ::core::convert::From<MintOrUnlockFilter> for IAssetReservesEvents {
        fn from(value: MintOrUnlockFilter) -> Self {
            Self::MintOrUnlockFilter(value)
        }
    }
    impl ::core::convert::From<MirrorTokenSetFilter> for IAssetReservesEvents {
        fn from(value: MirrorTokenSetFilter) -> Self {
            Self::MirrorTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<NexusChangedFilter> for IAssetReservesEvents {
        fn from(value: NexusChangedFilter) -> Self {
            Self::NexusChangedFilter(value)
        }
    }
    impl ::core::convert::From<TargetAvailableLiquidityChangedFilter>
    for IAssetReservesEvents {
        fn from(value: TargetAvailableLiquidityChangedFilter) -> Self {
            Self::TargetAvailableLiquidityChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `kai` function with signature `kai()` and selector `0xe8a2b16a`
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
    #[ethcall(name = "kai", abi = "kai()")]
    pub struct KaiCall;
    ///Container type for all input parameters for the `lockOrBurn` function with signature `lockOrBurn(address,(address,uint256)[])` and selector `0x9294523c`
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
    #[ethcall(name = "lockOrBurn", abi = "lockOrBurn(address,(address,uint256)[])")]
    pub struct LockOrBurnCall {
        pub sender: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<Token>,
    }
    ///Container type for all input parameters for the `mintOrUnlock` function with signature `mintOrUnlock(address,(address,uint256)[])` and selector `0x330a2767`
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
    #[ethcall(name = "mintOrUnlock", abi = "mintOrUnlock(address,(address,uint256)[])")]
    pub struct MintOrUnlockCall {
        pub target: ::ethers::core::types::Address,
        pub mirror_tokens: ::std::vec::Vec<Token>,
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
    pub enum IAssetReservesCalls {
        Kai(KaiCall),
        LockOrBurn(LockOrBurnCall),
        MintOrUnlock(MintOrUnlockCall),
    }
    impl ::ethers::core::abi::AbiDecode for IAssetReservesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <KaiCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Kai(decoded));
            }
            if let Ok(decoded) = <LockOrBurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LockOrBurn(decoded));
            }
            if let Ok(decoded) = <MintOrUnlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintOrUnlock(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IAssetReservesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Kai(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockOrBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintOrUnlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IAssetReservesCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Kai(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockOrBurn(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintOrUnlock(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<KaiCall> for IAssetReservesCalls {
        fn from(value: KaiCall) -> Self {
            Self::Kai(value)
        }
    }
    impl ::core::convert::From<LockOrBurnCall> for IAssetReservesCalls {
        fn from(value: LockOrBurnCall) -> Self {
            Self::LockOrBurn(value)
        }
    }
    impl ::core::convert::From<MintOrUnlockCall> for IAssetReservesCalls {
        fn from(value: MintOrUnlockCall) -> Self {
            Self::MintOrUnlock(value)
        }
    }
    ///Container type for all return fields from the `kai` function with signature `kai()` and selector `0xe8a2b16a`
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
    pub struct KaiReturn(pub ::ethers::core::types::Address);
}
