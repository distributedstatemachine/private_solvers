pub use asset_reserves::*;
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
pub mod asset_reserves {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_nexus"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_kai"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addWhiteListedAsset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addWhiteListedAsset",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("isAssetWhiteListed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isAssetWhiteListed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
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
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_target"),
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
                    ::std::borrow::ToOwned::to_owned("nexus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nexus"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("rebalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rebalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("removeWhiteListedAsset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeWhiteListedAsset",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("sendExcessYield"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendExcessYield"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "khalaniPercentageBps",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "khalaniRewardReceiver",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("setTargetAvailableLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setTargetAvailableLiquidity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("whiteListedAssetInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "whiteListedAssetInfo",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("whitelisted"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategyVault"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "totalBridgedAmount",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetAvailableLiquidity",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("whitelistAssetAddParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "whitelistAssetAddParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_strategyVault"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_targetAvailableLiquidity",
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AssetNotFound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AssetNotFound"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AssetNotWhiteListed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AssetNotWhiteListed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "AssetReserves_Percentage_Increase_10000_BP",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AssetReserves_Percentage_Increase_10000_BP",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidNexus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidNexus"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotValidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotValidOwner"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ASSETRESERVES_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1E\xD68\x03\x80b\0\x1E\xD6\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\x85V[`\x01\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\0\x80T\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90U\x16`\x80Rb\0\0\xBDV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\x80W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\x99W`\0\x80\xFD[b\0\0\xA4\x83b\0\0hV[\x91Pb\0\0\xB4` \x84\x01b\0\0hV[\x90P\x92P\x92\x90PV[`\x80Qa\x1D\xE1b\0\0\xF5`\09`\0\x81\x81a\x02_\x01R\x81\x81a\x07r\x01R\x81\x81a\x07\xAB\x01R\x81\x81a\x0C\xCE\x01Ra\r.\x01Ra\x1D\xE1`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x92\x94R<\x11a\0\x8CW\x80c\xBF\x98\x8B\xF5\x11a\0fW\x80c\xBF\x98\x8B\xF5\x14a\x01\xC7W\x80c\xDC\x02n\xF6\x14a\x024W\x80c\xE2D\xED\xB2\x14a\x02GW\x80c\xE8\xA2\xB1j\x14a\x02ZW`\0\x80\xFD[\x80c\x92\x94R<\x14a\x01\x8EW\x80c\xA3\xF5\xC1\xD2\x14a\x01\xA1W\x80c\xB1\x06H-\x14a\x01\xB4W`\0\x80\xFD[\x80c\x05\xF0^Q\x14a\0\xD4W\x80c\r\xCB\xD4\x06\x14a\0\xE9W\x80c!\xC2\x81\x91\x14a\0\xFCW\x80c3\n'g\x14a\x01\x0FW\x80c;\xF3\xD1\xF8\x14a\x01\"W\x80c\x8D\xA5\xCB[\x14a\x01cW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x1A+V[a\x02\x81V[\0[a\0\xE7a\0\xF76`\x04a\x1AUV[a\x03\x0EV[a\0\xE7a\x01\n6`\x04a\x1AUV[a\x03\x85V[a\0\xE7a\x01\x1D6`\x04a\x1AwV[a\x06\xCBV[a\x01Na\x0106`\x04a\x1AUV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\x01v\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01ZV[a\0\xE7a\x01\x9C6`\x04a\x1AwV[a\x0CSV[`\0Ta\x01v\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xE7a\x01\xC26`\x04a\x1A\xFDV[a\x10kV[a\x02\na\x01\xD56`\x04a\x1AUV[`\x02` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x91\x90\x92\x01T`\xFF\x83\x16\x92a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x90\x84V[`@\x80Q\x94\x15\x15\x85R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x85\x01R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01ZV[a\0\xE7a\x02B6`\x04a\x1AUV[a\x11\x01V[a\0\xE7a\x02U6`\x04a\x1B9V[a\x11\x8AV[a\x01v\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xACW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R\x7F%\xA8\xDD\xEF\x15\x04\x10p\xAEUxq\xE4k\x9C\x88\xF8\xD77_\xF8\xE9`\xA0\xC6b\"\xFA\x90\xAC\x92\x1D\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x02` \x81\x90R`@\x90\x91 \x01UV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x039W`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fp\x12\xF2\xDDH\x9A#\xD7\x0Cz\x9F\xF3U\xB2`N0U\xCD\x12\x98\x93A\t\xB8\xF5\x13\xCE\x7F\xA0\0U\x91\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x81Q`\x80\x81\x01\x83R\x81T`\xFF\x81\x16\x15\x15\x82Ra\x01\0\x90\x04\x90\x97\x16\x92\x87\x01\x92\x90\x92R`\x01\x82\x01T\x86\x82\x01R\x91\x01T``\x85\x01RQcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x041\x91\x90a\x1BuV[\x90P\x81``\x01Q\x81\x11\x15a\x05\tW`\0\x82``\x01Q\x82a\x04Q\x91\x90a\x1B\xA4V[\x90Pa\x04b\x84\x84` \x01Q\x83a\x15\xFAV[` \x83\x01Q`@QcnU?e`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cnU?e\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xD9\x91\x90a\x1BuV[P`@Q\x7F\x1E\x91\xA0\x19R\xD1\xE7<\x03\xAD\x8F'\xA0oU\xA8\xD17\xF7G\x9EP7\xB7U\xC1\xE9\x93\xC2\x0Ey$\x90`\0\x90\xA1PPPPV[` \x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05x\x91\x90a\x1BuV[\x90P`\0\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16cL\xDA\xD5\x06\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xAE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xEF\x91\x90a\x1BuV[\x90P`\0\x83\x85``\x01Qa\x06\x03\x91\x90a\x1B\xA4V[\x90P`\0\x81\x11\x80\x15a\x06\x14WP\x80\x82\x11[\x15a\x06\xC2W` \x85\x01Q`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01\x81\x90R`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x97\x91\x90a\x1BuV[P`@Q\x7F\x1E\x91\xA0\x19R\xD1\xE7<\x03\xAD\x8F'\xA0oU\xA8\xD17\xF7G\x9EP7\xB7U\xC1\xE9\x93\xC2\x0Ey$\x90`\0\x90\xA1[PPP[PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xF6W`@Qc\x05\xED\x9F\x1D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7FoC\xAB't\xA9\xDD\x8B\xAE\xAA\x9A\x01\xFD\x83,v\x0B|)vFs\xFE\xE2\xC9L\xD2\xA90[\xDB\x89\x83\x83`@Qa\x071\x92\x91\x90a\x1B\xBDV[`@Q\x80\x91\x03\x90\xA2`\0[\x81\x81\x10\x15a\x0CMW`\0\x83\x83\x83\x81\x81\x10a\x07XWa\x07Xa\x1C\x14V[a\x07n\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\x08]W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x19\x86\x86\x86\x86\x81\x81\x10a\x07\xEBWa\x07\xEBa\x1C\x14V[\x90P`@\x02\x01` \x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08&\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08@W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08TW=`\0\x80>=`\0\xFD[PPPPa\x0CDV[`\0`\x02`\0\x86\x86\x86\x81\x81\x10a\x08uWa\x08ua\x1C\x14V[a\x08\x8B\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91P`\xFF\x16\x15a\x0B\xF0W\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B\x9EW\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t7\x91\x90a\x1BuV[\x82T`@Qc&mj\x83`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\0\x91a\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cL\xDA\xD5\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAD\x91\x90a\x1BuV[\x90P\x86\x86\x86\x81\x81\x10a\t\xC1Wa\t\xC1a\x1C\x14V[\x90P`@\x02\x01` \x015\x83`\x01\x01`\0\x82\x82Ta\t\xDE\x91\x90a\x1B\xA4V[\x90\x91UP\x81\x90P\x87\x87\x87\x81\x81\x10a\t\xF7Wa\t\xF7a\x1C\x14V[\x90P`@\x02\x01` \x015\x11\x15a\n\xE7W\x82T`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`$\x83\x01R0`D\x83\x01Ra\x01\0\x90\x92\x04\x90\x91\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\neW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x89\x91\x90a\x1BuV[Pa\n\xE2\x87\x87\x87\x81\x81\x10a\n\x9FWa\n\x9Fa\x1C\x14V[a\n\xB5\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x89\x83\x8A\x8A\x8A\x81\x81\x10a\n\xC9Wa\n\xC9a\x1C\x14V[\x90P`@\x02\x01` \x015a\n\xDD\x91\x90a\x1B\xA4V[a\x17BV[a\x0B\x97V[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16c\xB4`\xAF\x94\x88\x88\x88\x81\x81\x10a\x0B\x0EWa\x0B\x0Ea\x1C\x14V[\x90P`@\x02\x01` \x015\x8A0`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0BR\x93\x92\x91\x90\x92\x83R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0BqW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x95\x91\x90a\x1BuV[P[PPa\x0CBV[a\x0B\xEB\x85\x85\x85\x81\x81\x10a\x0B\xB3Wa\x0B\xB3a\x1C\x14V[a\x0B\xC9\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x87\x87\x87\x87\x81\x81\x10a\x0B\xDCWa\x0B\xDCa\x1C\x14V[\x90P`@\x02\x01` \x015a\x17BV[a\x0CBV[\x84\x84\x84\x81\x81\x10a\x0C\x02Wa\x0C\x02a\x1C\x14V[a\x0C\x18\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[`@Qc\x06|x\x7F`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[P[P`\x01\x01a\x07<V[PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C~W`@Qc\x05\xED\x9F\x1D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7Fv\n5\x8DH\n\xCDb\xE3\t\xA72K\xC2\x89n\xBA\xC8E\xFB\x1B.\xBE\xC73\x9Eg\x0B\xDC\x17A\xA5\x83\x83`@Qa\x0C\xB9\x92\x91\x90a\x1B\xBDV[`@Q\x80\x91\x03\x90\xA2`\0[\x81\x81\x10\x15a\x0CMW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83\x83\x83\x81\x81\x10a\r\x08Wa\r\x08a\x1C\x14V[a\r\x1E\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\r\xE0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cy\xCCg\x90\x85\x85\x85\x85\x81\x81\x10a\rnWa\rna\x1C\x14V[\x90P`@\x02\x01` \x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xA9\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xC3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\xD7W=`\0\x80>=`\0\xFD[PPPPa\x10cV[`\0`\x02`\0\x85\x85\x85\x81\x81\x10a\r\xF8Wa\r\xF8a\x1C\x14V[a\x0E\x0E\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91P`\xFF\x16\x15a\x10\x14Wa\x0E\x82\x84\x84\x84\x81\x81\x10a\x0EIWa\x0EIa\x1C\x14V[a\x0E_\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x860\x87\x87\x87\x81\x81\x10a\x0EsWa\x0Esa\x1C\x14V[\x90P`@\x02\x01` \x015a\x17rV[\x83\x83\x83\x81\x81\x10a\x0E\x94Wa\x0E\x94a\x1C\x14V[\x90P`@\x02\x01` \x015\x81`\x01\x01`\0\x82\x82Ta\x0E\xB1\x91\x90a\x1C*V[\x90\x91UPP\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15a\x10\tWa\x0F'\x84\x84\x84\x81\x81\x10a\x0E\xE0Wa\x0E\xE0a\x1C\x14V[a\x0E\xF6\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x86\x81\x81\x10a\x0F\x18Wa\x0F\x18a\x1C\x14V[\x90P`@\x02\x01` \x015a\x15\xFAV[\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16cnU?e\x85\x85\x85\x81\x81\x10a\x0FNWa\x0FNa\x1C\x14V[\x90P`@\x02\x01` \x0150`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x88\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x0F\xC3WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x0F\xC0\x91\x81\x01\x90a\x1BuV[`\x01[a\x10\x0EWa\x10\t\x84\x84\x84\x81\x81\x10a\x0F\xDCWa\x0F\xDCa\x1C\x14V[a\x0F\xF2\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\0a\x15\xFAV[a\x10aV[Pa\x10aV[\x83\x83\x83\x81\x81\x10a\x10&Wa\x10&a\x1C\x14V[a\x10<\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[`@Qc\x16\xFCB)`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x0C9V[P[`\x01\x01a\x0C\xC4V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\x96W`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x02` \x81\x90R`@\x80\x83 \x80T\x95\x88\x16a\x01\0\x02`\x01`\x01`\xA8\x1B\x03\x19\x90\x96\x16\x95\x90\x95\x17`\x01\x17\x85U\x90\x84\x01\x85\x90UQ\x7Fp\x12\xF2\xDDH\x9A#\xD7\x0Cz\x9F\xF3U\xB2`N0U\xCD\x12\x98\x93A\t\xB8\xF5\x13\xCE\x7F\xA0\0U\x91\x90\xA2PPPPV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11,W`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02` \x81\x90R`@\x80\x83 \x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x81U`\x01\x81\x01\x84\x90U\x90\x91\x01\x82\x90UQ\x7F\x9Dd\x11\n\xB8D\x13\xB3!#m\x9A\xE6{IMQi\x1F\xFA8\xBCY\xD9\xD8X\x14?'\r42\x91\x90\xA2PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\xB5W`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a'\x10\x82\x11\x15a\x11\xD8W`@Qc\x0F\xF03\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x90\x91\x04\x90\x96\x16\x92\x81\x01\x92\x90\x92R`\x01\x81\x01T\x93\x82\x01\x93\x90\x93R\x91\x01T``\x82\x01R\x90a\x12\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FProvided asset is not whiteliste`D\x82\x01R`\x19`\xFA\x1B`d\x82\x01R`\x84\x01a\x0C9V[` \x81\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF2\x91\x90a\x1BuV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13`\x91\x90a\x1BuV[\x90P`\0\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16cL\xDA\xD5\x06\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x96\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD7\x91\x90a\x1BuV[\x90P`\0a\x13\xE5\x83\x83a\x1C*V[\x90P\x80\x15a\x15\xF0W`\0\x85`@\x01Q\x82a\x13\xFF\x91\x90a\x1B\xA4V[\x90P\x83\x81\x11\x15a\x14\x97W`\0a\x14\x15\x85\x83a\x1B\xA4V[` \x88\x01Q`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01\x81\x90R`D\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x94\x91\x90a\x1BuV[PP[`\0a'\x10a\x14\xA6\x8A\x84a\x1C=V[a\x14\xB0\x91\x90a\x1CTV[`\x01T\x90\x91Pa\x14\xCF\x90\x8B\x90`\x01`\x01`\xA0\x1B\x03\x16a\n\xDD\x84\x86a\x1B\xA4V[`\0Ta\x14\xE7\x90\x8B\x90`\x01`\x01`\xA0\x1B\x03\x16\x83a\x15\xFAV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x14\xFEW\x90PP\x90P\x8A\x81`\0\x81Q\x81\x10a\x15;Wa\x15;a\x1C\x14V[` \x02` \x01\x01Q`\0\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x81`\0\x81Q\x81\x10a\x15sWa\x15sa\x1C\x14V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01R`\0T`@Qc$\xB9\xC7\x15`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x92\xE7\x1CT\x90a\x15\xBA\x90a'\x1C\x90\x85\x90`\x01\x90\x8F\x90`\x04\x01a\x1CvV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\xD4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xE8W=`\0\x80>=`\0\xFD[PPPPPPP[PPPPPPPPV[\x80\x15\x80a\x16tWP`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16r\x91\x90a\x1BuV[\x15[a\x16\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FSafeERC20: approve from non-zero`D\x82\x01Ru to non-zero allowance`P\x1B`d\x82\x01R`\x84\x01a\x0C9V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x06\xC6\x90\x84\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x17\xAAV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x06\xC6\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\x17\x0BV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x0CM\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01a\x17\x0BV[`\0a\x17\xFF\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x18\x7F\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x18 WP\x80\x80` \x01\x90Q\x81\x01\x90a\x18 \x91\x90a\x1D\x16V[a\x06\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x0C9V[``a\x18\x8E\x84\x84`\0\x85a\x18\x96V[\x94\x93PPPPV[``\x82G\x10\x15a\x18\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x0C9V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x19\x13\x91\x90a\x1D\\V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x19PW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19UV[``\x91P[P\x91P\x91Pa\x19f\x87\x83\x83\x87a\x19qV[\x97\x96PPPPPPPV[``\x83\x15a\x19\xE0W\x82Q`\0\x03a\x19\xD9W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x19\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0C9V[P\x81a\x18\x8EV[a\x18\x8E\x83\x83\x81Q\x15a\x19\xF5W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C9\x91\x90a\x1DxV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1A&W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A>W`\0\x80\xFD[a\x1AG\x83a\x1A\x0FV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x1AgW`\0\x80\xFD[a\x1Ap\x82a\x1A\x0FV[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1A\x8CW`\0\x80\xFD[a\x1A\x95\x84a\x1A\x0FV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1A\xB2W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1A\xC6W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1A\xD5W`\0\x80\xFD[\x87` \x82`\x06\x1B\x85\x01\x01\x11\x15a\x1A\xEAW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1B\x12W`\0\x80\xFD[a\x1B\x1B\x84a\x1A\x0FV[\x92Pa\x1B)` \x85\x01a\x1A\x0FV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1BNW`\0\x80\xFD[a\x1BW\x84a\x1A\x0FV[\x92P` \x84\x015\x91Pa\x1Bl`@\x85\x01a\x1A\x0FV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1B\x87W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x1B\xB7Wa\x1B\xB7a\x1B\x8EV[\x92\x91PPV[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@\x80\x84\x01\x86\x84[\x87\x81\x10\x15a\x1C\x07W`\x01`\x01`\xA0\x1B\x03a\x1B\xEC\x83a\x1A\x0FV[\x16\x83R\x81\x85\x015\x85\x84\x01R\x91\x83\x01\x91\x90\x83\x01\x90`\x01\x01a\x1B\xD3V[P\x90\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1B\xB7Wa\x1B\xB7a\x1B\x8EV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1B\xB7Wa\x1B\xB7a\x1B\x8EV[`\0\x82a\x1CqWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\xC0\x82\x01\x86\x83R` `\xC0\x81\x85\x01R\x81\x87Q\x80\x84R`\xE0\x86\x01\x91P\x82\x89\x01\x93P`\0[\x81\x81\x10\x15a\x1C\xCBW\x84Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x84\x01Q\x84\x84\x01R\x93\x83\x01\x93`@\x90\x92\x01\x91`\x01\x01a\x1C\x9BV[PP\x84\x81\x03`@\x86\x01R`\0\x81R\x86\x15\x15``\x86\x01R` \x01\x91Pa\x1C\xED\x90PV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`\x80\x83\x01RP\x80\x82\x03`\xA0\x90\x91\x01R`\0\x81R` \x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1D(W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1ApW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x1DSW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1D;V[PP`\0\x91\x01RV[`\0\x82Qa\x1Dn\x81\x84` \x87\x01a\x1D8V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x1D\x97\x81`@\x85\x01` \x87\x01a\x1D8V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xECs7\xAD\x83\xEA&I\xDFX\xA0\x9A&\xCF\xDD\x0F\x90\x10\xFF\xD3\xC1`\xC6\xC6\x18U\xB4\x99\x81\x8Br\x98dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static ASSETRESERVES_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x92\x94R<\x11a\0\x8CW\x80c\xBF\x98\x8B\xF5\x11a\0fW\x80c\xBF\x98\x8B\xF5\x14a\x01\xC7W\x80c\xDC\x02n\xF6\x14a\x024W\x80c\xE2D\xED\xB2\x14a\x02GW\x80c\xE8\xA2\xB1j\x14a\x02ZW`\0\x80\xFD[\x80c\x92\x94R<\x14a\x01\x8EW\x80c\xA3\xF5\xC1\xD2\x14a\x01\xA1W\x80c\xB1\x06H-\x14a\x01\xB4W`\0\x80\xFD[\x80c\x05\xF0^Q\x14a\0\xD4W\x80c\r\xCB\xD4\x06\x14a\0\xE9W\x80c!\xC2\x81\x91\x14a\0\xFCW\x80c3\n'g\x14a\x01\x0FW\x80c;\xF3\xD1\xF8\x14a\x01\"W\x80c\x8D\xA5\xCB[\x14a\x01cW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x1A+V[a\x02\x81V[\0[a\0\xE7a\0\xF76`\x04a\x1AUV[a\x03\x0EV[a\0\xE7a\x01\n6`\x04a\x1AUV[a\x03\x85V[a\0\xE7a\x01\x1D6`\x04a\x1AwV[a\x06\xCBV[a\x01Na\x0106`\x04a\x1AUV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\x01v\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01ZV[a\0\xE7a\x01\x9C6`\x04a\x1AwV[a\x0CSV[`\0Ta\x01v\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xE7a\x01\xC26`\x04a\x1A\xFDV[a\x10kV[a\x02\na\x01\xD56`\x04a\x1AUV[`\x02` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x91\x90\x92\x01T`\xFF\x83\x16\x92a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x90\x84V[`@\x80Q\x94\x15\x15\x85R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x85\x01R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01ZV[a\0\xE7a\x02B6`\x04a\x1AUV[a\x11\x01V[a\0\xE7a\x02U6`\x04a\x1B9V[a\x11\x8AV[a\x01v\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xACW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R\x7F%\xA8\xDD\xEF\x15\x04\x10p\xAEUxq\xE4k\x9C\x88\xF8\xD77_\xF8\xE9`\xA0\xC6b\"\xFA\x90\xAC\x92\x1D\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x02` \x81\x90R`@\x90\x91 \x01UV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x039W`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fp\x12\xF2\xDDH\x9A#\xD7\x0Cz\x9F\xF3U\xB2`N0U\xCD\x12\x98\x93A\t\xB8\xF5\x13\xCE\x7F\xA0\0U\x91\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x81\x81R`\x02` \x81\x81R`@\x80\x84 \x81Q`\x80\x81\x01\x83R\x81T`\xFF\x81\x16\x15\x15\x82Ra\x01\0\x90\x04\x90\x97\x16\x92\x87\x01\x92\x90\x92R`\x01\x82\x01T\x86\x82\x01R\x91\x01T``\x85\x01RQcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x041\x91\x90a\x1BuV[\x90P\x81``\x01Q\x81\x11\x15a\x05\tW`\0\x82``\x01Q\x82a\x04Q\x91\x90a\x1B\xA4V[\x90Pa\x04b\x84\x84` \x01Q\x83a\x15\xFAV[` \x83\x01Q`@QcnU?e`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cnU?e\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xD9\x91\x90a\x1BuV[P`@Q\x7F\x1E\x91\xA0\x19R\xD1\xE7<\x03\xAD\x8F'\xA0oU\xA8\xD17\xF7G\x9EP7\xB7U\xC1\xE9\x93\xC2\x0Ey$\x90`\0\x90\xA1PPPPV[` \x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05x\x91\x90a\x1BuV[\x90P`\0\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16cL\xDA\xD5\x06\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xAE\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xEF\x91\x90a\x1BuV[\x90P`\0\x83\x85``\x01Qa\x06\x03\x91\x90a\x1B\xA4V[\x90P`\0\x81\x11\x80\x15a\x06\x14WP\x80\x82\x11[\x15a\x06\xC2W` \x85\x01Q`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01\x81\x90R`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x97\x91\x90a\x1BuV[P`@Q\x7F\x1E\x91\xA0\x19R\xD1\xE7<\x03\xAD\x8F'\xA0oU\xA8\xD17\xF7G\x9EP7\xB7U\xC1\xE9\x93\xC2\x0Ey$\x90`\0\x90\xA1[PPP[PPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xF6W`@Qc\x05\xED\x9F\x1D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7FoC\xAB't\xA9\xDD\x8B\xAE\xAA\x9A\x01\xFD\x83,v\x0B|)vFs\xFE\xE2\xC9L\xD2\xA90[\xDB\x89\x83\x83`@Qa\x071\x92\x91\x90a\x1B\xBDV[`@Q\x80\x91\x03\x90\xA2`\0[\x81\x81\x10\x15a\x0CMW`\0\x83\x83\x83\x81\x81\x10a\x07XWa\x07Xa\x1C\x14V[a\x07n\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\x08]W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x19\x86\x86\x86\x86\x81\x81\x10a\x07\xEBWa\x07\xEBa\x1C\x14V[\x90P`@\x02\x01` \x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08&\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08@W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08TW=`\0\x80>=`\0\xFD[PPPPa\x0CDV[`\0`\x02`\0\x86\x86\x86\x81\x81\x10a\x08uWa\x08ua\x1C\x14V[a\x08\x8B\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91P`\xFF\x16\x15a\x0B\xF0W\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0B\x9EW\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t7\x91\x90a\x1BuV[\x82T`@Qc&mj\x83`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\0\x91a\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cL\xDA\xD5\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAD\x91\x90a\x1BuV[\x90P\x86\x86\x86\x81\x81\x10a\t\xC1Wa\t\xC1a\x1C\x14V[\x90P`@\x02\x01` \x015\x83`\x01\x01`\0\x82\x82Ta\t\xDE\x91\x90a\x1B\xA4V[\x90\x91UP\x81\x90P\x87\x87\x87\x81\x81\x10a\t\xF7Wa\t\xF7a\x1C\x14V[\x90P`@\x02\x01` \x015\x11\x15a\n\xE7W\x82T`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`$\x83\x01R0`D\x83\x01Ra\x01\0\x90\x92\x04\x90\x91\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\neW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x89\x91\x90a\x1BuV[Pa\n\xE2\x87\x87\x87\x81\x81\x10a\n\x9FWa\n\x9Fa\x1C\x14V[a\n\xB5\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x89\x83\x8A\x8A\x8A\x81\x81\x10a\n\xC9Wa\n\xC9a\x1C\x14V[\x90P`@\x02\x01` \x015a\n\xDD\x91\x90a\x1B\xA4V[a\x17BV[a\x0B\x97V[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16c\xB4`\xAF\x94\x88\x88\x88\x81\x81\x10a\x0B\x0EWa\x0B\x0Ea\x1C\x14V[\x90P`@\x02\x01` \x015\x8A0`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0BR\x93\x92\x91\x90\x92\x83R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0BqW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x95\x91\x90a\x1BuV[P[PPa\x0CBV[a\x0B\xEB\x85\x85\x85\x81\x81\x10a\x0B\xB3Wa\x0B\xB3a\x1C\x14V[a\x0B\xC9\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x87\x87\x87\x87\x81\x81\x10a\x0B\xDCWa\x0B\xDCa\x1C\x14V[\x90P`@\x02\x01` \x015a\x17BV[a\x0CBV[\x84\x84\x84\x81\x81\x10a\x0C\x02Wa\x0C\x02a\x1C\x14V[a\x0C\x18\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[`@Qc\x06|x\x7F`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[P[P`\x01\x01a\x07<V[PPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C~W`@Qc\x05\xED\x9F\x1D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7Fv\n5\x8DH\n\xCDb\xE3\t\xA72K\xC2\x89n\xBA\xC8E\xFB\x1B.\xBE\xC73\x9Eg\x0B\xDC\x17A\xA5\x83\x83`@Qa\x0C\xB9\x92\x91\x90a\x1B\xBDV[`@Q\x80\x91\x03\x90\xA2`\0[\x81\x81\x10\x15a\x0CMW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83\x83\x83\x81\x81\x10a\r\x08Wa\r\x08a\x1C\x14V[a\r\x1E\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\r\xE0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cy\xCCg\x90\x85\x85\x85\x85\x81\x81\x10a\rnWa\rna\x1C\x14V[\x90P`@\x02\x01` \x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\xA9\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xC3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\xD7W=`\0\x80>=`\0\xFD[PPPPa\x10cV[`\0`\x02`\0\x85\x85\x85\x81\x81\x10a\r\xF8Wa\r\xF8a\x1C\x14V[a\x0E\x0E\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91P`\xFF\x16\x15a\x10\x14Wa\x0E\x82\x84\x84\x84\x81\x81\x10a\x0EIWa\x0EIa\x1C\x14V[a\x0E_\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x860\x87\x87\x87\x81\x81\x10a\x0EsWa\x0Esa\x1C\x14V[\x90P`@\x02\x01` \x015a\x17rV[\x83\x83\x83\x81\x81\x10a\x0E\x94Wa\x0E\x94a\x1C\x14V[\x90P`@\x02\x01` \x015\x81`\x01\x01`\0\x82\x82Ta\x0E\xB1\x91\x90a\x1C*V[\x90\x91UPP\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15a\x10\tWa\x0F'\x84\x84\x84\x81\x81\x10a\x0E\xE0Wa\x0E\xE0a\x1C\x14V[a\x0E\xF6\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x86\x81\x81\x10a\x0F\x18Wa\x0F\x18a\x1C\x14V[\x90P`@\x02\x01` \x015a\x15\xFAV[\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16cnU?e\x85\x85\x85\x81\x81\x10a\x0FNWa\x0FNa\x1C\x14V[\x90P`@\x02\x01` \x0150`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x88\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x0F\xC3WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x0F\xC0\x91\x81\x01\x90a\x1BuV[`\x01[a\x10\x0EWa\x10\t\x84\x84\x84\x81\x81\x10a\x0F\xDCWa\x0F\xDCa\x1C\x14V[a\x0F\xF2\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\0a\x15\xFAV[a\x10aV[Pa\x10aV[\x83\x83\x83\x81\x81\x10a\x10&Wa\x10&a\x1C\x14V[a\x10<\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\x1AUV[`@Qc\x16\xFCB)`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x0C9V[P[`\x01\x01a\x0C\xC4V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\x96W`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x02` \x81\x90R`@\x80\x83 \x80T\x95\x88\x16a\x01\0\x02`\x01`\x01`\xA8\x1B\x03\x19\x90\x96\x16\x95\x90\x95\x17`\x01\x17\x85U\x90\x84\x01\x85\x90UQ\x7Fp\x12\xF2\xDDH\x9A#\xD7\x0Cz\x9F\xF3U\xB2`N0U\xCD\x12\x98\x93A\t\xB8\xF5\x13\xCE\x7F\xA0\0U\x91\x90\xA2PPPPV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11,W`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02` \x81\x90R`@\x80\x83 \x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x81U`\x01\x81\x01\x84\x90U\x90\x91\x01\x82\x90UQ\x7F\x9Dd\x11\n\xB8D\x13\xB3!#m\x9A\xE6{IMQi\x1F\xFA8\xBCY\xD9\xD8X\x14?'\r42\x91\x90\xA2PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\xB5W`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a'\x10\x82\x11\x15a\x11\xD8W`@Qc\x0F\xF03\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x90\x91\x04\x90\x96\x16\x92\x81\x01\x92\x90\x92R`\x01\x81\x01T\x93\x82\x01\x93\x90\x93R\x91\x01T``\x82\x01R\x90a\x12\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FProvided asset is not whiteliste`D\x82\x01R`\x19`\xFA\x1B`d\x82\x01R`\x84\x01a\x0C9V[` \x81\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF2\x91\x90a\x1BuV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13`\x91\x90a\x1BuV[\x90P`\0\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16cL\xDA\xD5\x06\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x96\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD7\x91\x90a\x1BuV[\x90P`\0a\x13\xE5\x83\x83a\x1C*V[\x90P\x80\x15a\x15\xF0W`\0\x85`@\x01Q\x82a\x13\xFF\x91\x90a\x1B\xA4V[\x90P\x83\x81\x11\x15a\x14\x97W`\0a\x14\x15\x85\x83a\x1B\xA4V[` \x88\x01Q`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01\x81\x90R`D\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x94\x91\x90a\x1BuV[PP[`\0a'\x10a\x14\xA6\x8A\x84a\x1C=V[a\x14\xB0\x91\x90a\x1CTV[`\x01T\x90\x91Pa\x14\xCF\x90\x8B\x90`\x01`\x01`\xA0\x1B\x03\x16a\n\xDD\x84\x86a\x1B\xA4V[`\0Ta\x14\xE7\x90\x8B\x90`\x01`\x01`\xA0\x1B\x03\x16\x83a\x15\xFAV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x14\xFEW\x90PP\x90P\x8A\x81`\0\x81Q\x81\x10a\x15;Wa\x15;a\x1C\x14V[` \x02` \x01\x01Q`\0\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x81`\0\x81Q\x81\x10a\x15sWa\x15sa\x1C\x14V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01R`\0T`@Qc$\xB9\xC7\x15`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x92\xE7\x1CT\x90a\x15\xBA\x90a'\x1C\x90\x85\x90`\x01\x90\x8F\x90`\x04\x01a\x1CvV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x15\xD4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xE8W=`\0\x80>=`\0\xFD[PPPPPPP[PPPPPPPPV[\x80\x15\x80a\x16tWP`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16r\x91\x90a\x1BuV[\x15[a\x16\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FSafeERC20: approve from non-zero`D\x82\x01Ru to non-zero allowance`P\x1B`d\x82\x01R`\x84\x01a\x0C9V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x06\xC6\x90\x84\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x17\xAAV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x06\xC6\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\x17\x0BV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x0CM\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01a\x17\x0BV[`\0a\x17\xFF\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x18\x7F\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x18 WP\x80\x80` \x01\x90Q\x81\x01\x90a\x18 \x91\x90a\x1D\x16V[a\x06\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x0C9V[``a\x18\x8E\x84\x84`\0\x85a\x18\x96V[\x94\x93PPPPV[``\x82G\x10\x15a\x18\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x0C9V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x19\x13\x91\x90a\x1D\\V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x19PW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x19UV[``\x91P[P\x91P\x91Pa\x19f\x87\x83\x83\x87a\x19qV[\x97\x96PPPPPPPV[``\x83\x15a\x19\xE0W\x82Q`\0\x03a\x19\xD9W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x19\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0C9V[P\x81a\x18\x8EV[a\x18\x8E\x83\x83\x81Q\x15a\x19\xF5W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C9\x91\x90a\x1DxV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1A&W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A>W`\0\x80\xFD[a\x1AG\x83a\x1A\x0FV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x1AgW`\0\x80\xFD[a\x1Ap\x82a\x1A\x0FV[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1A\x8CW`\0\x80\xFD[a\x1A\x95\x84a\x1A\x0FV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1A\xB2W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1A\xC6W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1A\xD5W`\0\x80\xFD[\x87` \x82`\x06\x1B\x85\x01\x01\x11\x15a\x1A\xEAW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1B\x12W`\0\x80\xFD[a\x1B\x1B\x84a\x1A\x0FV[\x92Pa\x1B)` \x85\x01a\x1A\x0FV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1BNW`\0\x80\xFD[a\x1BW\x84a\x1A\x0FV[\x92P` \x84\x015\x91Pa\x1Bl`@\x85\x01a\x1A\x0FV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x1B\x87W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x1B\xB7Wa\x1B\xB7a\x1B\x8EV[\x92\x91PPV[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@\x80\x84\x01\x86\x84[\x87\x81\x10\x15a\x1C\x07W`\x01`\x01`\xA0\x1B\x03a\x1B\xEC\x83a\x1A\x0FV[\x16\x83R\x81\x85\x015\x85\x84\x01R\x91\x83\x01\x91\x90\x83\x01\x90`\x01\x01a\x1B\xD3V[P\x90\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1B\xB7Wa\x1B\xB7a\x1B\x8EV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1B\xB7Wa\x1B\xB7a\x1B\x8EV[`\0\x82a\x1CqWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\xC0\x82\x01\x86\x83R` `\xC0\x81\x85\x01R\x81\x87Q\x80\x84R`\xE0\x86\x01\x91P\x82\x89\x01\x93P`\0[\x81\x81\x10\x15a\x1C\xCBW\x84Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x84\x01Q\x84\x84\x01R\x93\x83\x01\x93`@\x90\x92\x01\x91`\x01\x01a\x1C\x9BV[PP\x84\x81\x03`@\x86\x01R`\0\x81R\x86\x15\x15``\x86\x01R` \x01\x91Pa\x1C\xED\x90PV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`\x80\x83\x01RP\x80\x82\x03`\xA0\x90\x91\x01R`\0\x81R` \x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1D(W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1ApW`\0\x80\xFD[`\0[\x83\x81\x10\x15a\x1DSW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1D;V[PP`\0\x91\x01RV[`\0\x82Qa\x1Dn\x81\x84` \x87\x01a\x1D8V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x1D\x97\x81`@\x85\x01` \x87\x01a\x1D8V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xECs7\xAD\x83\xEA&I\xDFX\xA0\x9A&\xCF\xDD\x0F\x90\x10\xFF\xD3\xC1`\xC6\xC6\x18U\xB4\x99\x81\x8Br\x98dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static ASSETRESERVES_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AssetReserves<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AssetReserves<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AssetReserves<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AssetReserves<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AssetReserves<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AssetReserves))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AssetReserves<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ASSETRESERVES_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ASSETRESERVES_ABI.clone(),
                ASSETRESERVES_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addWhiteListedAsset` (0x0dcbd406) function
        pub fn add_white_listed_asset(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 203, 212, 6], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAssetWhiteListed` (0x3bf3d1f8) function
        pub fn is_asset_white_listed(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([59, 243, 209, 248], asset)
                .expect("method not found (this should never happen)")
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
            tokens: ::std::vec::Vec<Token>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 10, 39, 103], (target, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nexus` (0xa3f5c1d2) function
        pub fn nexus(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([163, 245, 193, 210], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalance` (0x21c28191) function
        pub fn rebalance(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 194, 129, 145], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeWhiteListedAsset` (0xdc026ef6) function
        pub fn remove_white_listed_asset(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([220, 2, 110, 246], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendExcessYield` (0xe244edb2) function
        pub fn send_excess_yield(
            &self,
            asset: ::ethers::core::types::Address,
            khalani_percentage_bps: ::ethers::core::types::U256,
            khalani_reward_receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [226, 68, 237, 178],
                    (asset, khalani_percentage_bps, khalani_reward_receiver),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTargetAvailableLiquidity` (0x05f05e51) function
        pub fn set_target_available_liquidity(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 240, 94, 81], (asset, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whiteListedAssetInfo` (0xbf988bf5) function
        pub fn white_listed_asset_info(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([191, 152, 139, 245], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistAssetAddParams` (0xb106482d) function
        pub fn whitelist_asset_add_params(
            &self,
            asset: ::ethers::core::types::Address,
            strategy_vault: ::ethers::core::types::Address,
            target_available_liquidity: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [177, 6, 72, 45],
                    (asset, strategy_vault, target_available_liquidity),
                )
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
            AssetReservesEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AssetReserves<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AssetNotFound` with signature `AssetNotFound(address)` and selector `0x67c787f0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AssetNotFound", abi = "AssetNotFound(address)")]
    pub struct AssetNotFound {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `AssetNotWhiteListed` with signature `AssetNotWhiteListed(address)` and selector `0x5bf108a4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AssetNotWhiteListed", abi = "AssetNotWhiteListed(address)")]
    pub struct AssetNotWhiteListed {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `AssetReserves_Percentage_Increase_10000_BP` with signature `AssetReserves_Percentage_Increase_10000_BP()` and selector `0x3fc0ce24`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "AssetReserves_Percentage_Increase_10000_BP",
        abi = "AssetReserves_Percentage_Increase_10000_BP()"
    )]
    pub struct AssetReserves_Percentage_Increase_10000_BP;
    ///Custom Error type `InvalidNexus` with signature `InvalidNexus()` and selector `0x2f6cf8e8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidNexus", abi = "InvalidNexus()")]
    pub struct InvalidNexus;
    ///Custom Error type `NotValidOwner` with signature `NotValidOwner()` and selector `0x912ecce7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotValidOwner", abi = "NotValidOwner()")]
    pub struct NotValidOwner;
    ///Container type for all of the contract's custom errors
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
    pub enum AssetReservesErrors {
        AssetNotFound(AssetNotFound),
        AssetNotWhiteListed(AssetNotWhiteListed),
        AssetReserves_Percentage_Increase_10000_BP(
            AssetReserves_Percentage_Increase_10000_BP,
        ),
        InvalidNexus(InvalidNexus),
        NotValidOwner(NotValidOwner),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AssetReservesErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AssetNotFound as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssetNotFound(decoded));
            }
            if let Ok(decoded) = <AssetNotWhiteListed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssetNotWhiteListed(decoded));
            }
            if let Ok(decoded) = <AssetReserves_Percentage_Increase_10000_BP as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssetReserves_Percentage_Increase_10000_BP(decoded));
            }
            if let Ok(decoded) = <InvalidNexus as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidNexus(decoded));
            }
            if let Ok(decoded) = <NotValidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotValidOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AssetReservesErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AssetNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssetNotWhiteListed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssetReserves_Percentage_Increase_10000_BP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidNexus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotValidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AssetReservesErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AssetNotFound as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AssetNotWhiteListed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AssetReserves_Percentage_Increase_10000_BP as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidNexus as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotValidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AssetReservesErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetNotWhiteListed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssetReserves_Percentage_Increase_10000_BP(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidNexus(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotValidOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AssetReservesErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AssetNotFound> for AssetReservesErrors {
        fn from(value: AssetNotFound) -> Self {
            Self::AssetNotFound(value)
        }
    }
    impl ::core::convert::From<AssetNotWhiteListed> for AssetReservesErrors {
        fn from(value: AssetNotWhiteListed) -> Self {
            Self::AssetNotWhiteListed(value)
        }
    }
    impl ::core::convert::From<AssetReserves_Percentage_Increase_10000_BP>
    for AssetReservesErrors {
        fn from(value: AssetReserves_Percentage_Increase_10000_BP) -> Self {
            Self::AssetReserves_Percentage_Increase_10000_BP(value)
        }
    }
    impl ::core::convert::From<InvalidNexus> for AssetReservesErrors {
        fn from(value: InvalidNexus) -> Self {
            Self::InvalidNexus(value)
        }
    }
    impl ::core::convert::From<NotValidOwner> for AssetReservesErrors {
        fn from(value: NotValidOwner) -> Self {
            Self::NotValidOwner(value)
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
    pub enum AssetReservesEvents {
        AssetAddedToWhitelistFilter(AssetAddedToWhitelistFilter),
        AssetRebalancedFilter(AssetRebalancedFilter),
        AssetRemovedFromWhitelistFilter(AssetRemovedFromWhitelistFilter),
        LockOrBurnFilter(LockOrBurnFilter),
        MintOrUnlockFilter(MintOrUnlockFilter),
        MirrorTokenSetFilter(MirrorTokenSetFilter),
        NexusChangedFilter(NexusChangedFilter),
        TargetAvailableLiquidityChangedFilter(TargetAvailableLiquidityChangedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AssetReservesEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssetAddedToWhitelistFilter::decode_log(log) {
                return Ok(AssetReservesEvents::AssetAddedToWhitelistFilter(decoded));
            }
            if let Ok(decoded) = AssetRebalancedFilter::decode_log(log) {
                return Ok(AssetReservesEvents::AssetRebalancedFilter(decoded));
            }
            if let Ok(decoded) = AssetRemovedFromWhitelistFilter::decode_log(log) {
                return Ok(AssetReservesEvents::AssetRemovedFromWhitelistFilter(decoded));
            }
            if let Ok(decoded) = LockOrBurnFilter::decode_log(log) {
                return Ok(AssetReservesEvents::LockOrBurnFilter(decoded));
            }
            if let Ok(decoded) = MintOrUnlockFilter::decode_log(log) {
                return Ok(AssetReservesEvents::MintOrUnlockFilter(decoded));
            }
            if let Ok(decoded) = MirrorTokenSetFilter::decode_log(log) {
                return Ok(AssetReservesEvents::MirrorTokenSetFilter(decoded));
            }
            if let Ok(decoded) = NexusChangedFilter::decode_log(log) {
                return Ok(AssetReservesEvents::NexusChangedFilter(decoded));
            }
            if let Ok(decoded) = TargetAvailableLiquidityChangedFilter::decode_log(log) {
                return Ok(
                    AssetReservesEvents::TargetAvailableLiquidityChangedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AssetReservesEvents {
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
    impl ::core::convert::From<AssetAddedToWhitelistFilter> for AssetReservesEvents {
        fn from(value: AssetAddedToWhitelistFilter) -> Self {
            Self::AssetAddedToWhitelistFilter(value)
        }
    }
    impl ::core::convert::From<AssetRebalancedFilter> for AssetReservesEvents {
        fn from(value: AssetRebalancedFilter) -> Self {
            Self::AssetRebalancedFilter(value)
        }
    }
    impl ::core::convert::From<AssetRemovedFromWhitelistFilter> for AssetReservesEvents {
        fn from(value: AssetRemovedFromWhitelistFilter) -> Self {
            Self::AssetRemovedFromWhitelistFilter(value)
        }
    }
    impl ::core::convert::From<LockOrBurnFilter> for AssetReservesEvents {
        fn from(value: LockOrBurnFilter) -> Self {
            Self::LockOrBurnFilter(value)
        }
    }
    impl ::core::convert::From<MintOrUnlockFilter> for AssetReservesEvents {
        fn from(value: MintOrUnlockFilter) -> Self {
            Self::MintOrUnlockFilter(value)
        }
    }
    impl ::core::convert::From<MirrorTokenSetFilter> for AssetReservesEvents {
        fn from(value: MirrorTokenSetFilter) -> Self {
            Self::MirrorTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<NexusChangedFilter> for AssetReservesEvents {
        fn from(value: NexusChangedFilter) -> Self {
            Self::NexusChangedFilter(value)
        }
    }
    impl ::core::convert::From<TargetAvailableLiquidityChangedFilter>
    for AssetReservesEvents {
        fn from(value: TargetAvailableLiquidityChangedFilter) -> Self {
            Self::TargetAvailableLiquidityChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addWhiteListedAsset` function with signature `addWhiteListedAsset(address)` and selector `0x0dcbd406`
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
    #[ethcall(name = "addWhiteListedAsset", abi = "addWhiteListedAsset(address)")]
    pub struct AddWhiteListedAssetCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isAssetWhiteListed` function with signature `isAssetWhiteListed(address)` and selector `0x3bf3d1f8`
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
    #[ethcall(name = "isAssetWhiteListed", abi = "isAssetWhiteListed(address)")]
    pub struct IsAssetWhiteListedCall {
        pub asset: ::ethers::core::types::Address,
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
        pub tokens: ::std::vec::Vec<Token>,
    }
    ///Container type for all input parameters for the `nexus` function with signature `nexus()` and selector `0xa3f5c1d2`
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
    #[ethcall(name = "nexus", abi = "nexus()")]
    pub struct NexusCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `rebalance` function with signature `rebalance(address)` and selector `0x21c28191`
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
    #[ethcall(name = "rebalance", abi = "rebalance(address)")]
    pub struct RebalanceCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removeWhiteListedAsset` function with signature `removeWhiteListedAsset(address)` and selector `0xdc026ef6`
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
    #[ethcall(name = "removeWhiteListedAsset", abi = "removeWhiteListedAsset(address)")]
    pub struct RemoveWhiteListedAssetCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sendExcessYield` function with signature `sendExcessYield(address,uint256,address)` and selector `0xe244edb2`
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
        name = "sendExcessYield",
        abi = "sendExcessYield(address,uint256,address)"
    )]
    pub struct SendExcessYieldCall {
        pub asset: ::ethers::core::types::Address,
        pub khalani_percentage_bps: ::ethers::core::types::U256,
        pub khalani_reward_receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setTargetAvailableLiquidity` function with signature `setTargetAvailableLiquidity(address,uint256)` and selector `0x05f05e51`
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
        name = "setTargetAvailableLiquidity",
        abi = "setTargetAvailableLiquidity(address,uint256)"
    )]
    pub struct SetTargetAvailableLiquidityCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `whiteListedAssetInfo` function with signature `whiteListedAssetInfo(address)` and selector `0xbf988bf5`
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
    #[ethcall(name = "whiteListedAssetInfo", abi = "whiteListedAssetInfo(address)")]
    pub struct WhiteListedAssetInfoCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `whitelistAssetAddParams` function with signature `whitelistAssetAddParams(address,address,uint256)` and selector `0xb106482d`
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
        name = "whitelistAssetAddParams",
        abi = "whitelistAssetAddParams(address,address,uint256)"
    )]
    pub struct WhitelistAssetAddParamsCall {
        pub asset: ::ethers::core::types::Address,
        pub strategy_vault: ::ethers::core::types::Address,
        pub target_available_liquidity: ::ethers::core::types::U256,
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
    pub enum AssetReservesCalls {
        AddWhiteListedAsset(AddWhiteListedAssetCall),
        IsAssetWhiteListed(IsAssetWhiteListedCall),
        Kai(KaiCall),
        LockOrBurn(LockOrBurnCall),
        MintOrUnlock(MintOrUnlockCall),
        Nexus(NexusCall),
        Owner(OwnerCall),
        Rebalance(RebalanceCall),
        RemoveWhiteListedAsset(RemoveWhiteListedAssetCall),
        SendExcessYield(SendExcessYieldCall),
        SetTargetAvailableLiquidity(SetTargetAvailableLiquidityCall),
        WhiteListedAssetInfo(WhiteListedAssetInfoCall),
        WhitelistAssetAddParams(WhitelistAssetAddParamsCall),
    }
    impl ::ethers::core::abi::AbiDecode for AssetReservesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddWhiteListedAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddWhiteListedAsset(decoded));
            }
            if let Ok(decoded) = <IsAssetWhiteListedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsAssetWhiteListed(decoded));
            }
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
            if let Ok(decoded) = <NexusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nexus(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RebalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Rebalance(decoded));
            }
            if let Ok(decoded) = <RemoveWhiteListedAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveWhiteListedAsset(decoded));
            }
            if let Ok(decoded) = <SendExcessYieldCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendExcessYield(decoded));
            }
            if let Ok(decoded) = <SetTargetAvailableLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTargetAvailableLiquidity(decoded));
            }
            if let Ok(decoded) = <WhiteListedAssetInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WhiteListedAssetInfo(decoded));
            }
            if let Ok(decoded) = <WhitelistAssetAddParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WhitelistAssetAddParams(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AssetReservesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddWhiteListedAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAssetWhiteListed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Kai(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockOrBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintOrUnlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nexus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Rebalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveWhiteListedAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendExcessYield(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTargetAvailableLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WhiteListedAssetInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WhitelistAssetAddParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AssetReservesCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddWhiteListedAsset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsAssetWhiteListed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Kai(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockOrBurn(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintOrUnlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nexus(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rebalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveWhiteListedAsset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendExcessYield(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTargetAvailableLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WhiteListedAssetInfo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WhitelistAssetAddParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddWhiteListedAssetCall> for AssetReservesCalls {
        fn from(value: AddWhiteListedAssetCall) -> Self {
            Self::AddWhiteListedAsset(value)
        }
    }
    impl ::core::convert::From<IsAssetWhiteListedCall> for AssetReservesCalls {
        fn from(value: IsAssetWhiteListedCall) -> Self {
            Self::IsAssetWhiteListed(value)
        }
    }
    impl ::core::convert::From<KaiCall> for AssetReservesCalls {
        fn from(value: KaiCall) -> Self {
            Self::Kai(value)
        }
    }
    impl ::core::convert::From<LockOrBurnCall> for AssetReservesCalls {
        fn from(value: LockOrBurnCall) -> Self {
            Self::LockOrBurn(value)
        }
    }
    impl ::core::convert::From<MintOrUnlockCall> for AssetReservesCalls {
        fn from(value: MintOrUnlockCall) -> Self {
            Self::MintOrUnlock(value)
        }
    }
    impl ::core::convert::From<NexusCall> for AssetReservesCalls {
        fn from(value: NexusCall) -> Self {
            Self::Nexus(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AssetReservesCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RebalanceCall> for AssetReservesCalls {
        fn from(value: RebalanceCall) -> Self {
            Self::Rebalance(value)
        }
    }
    impl ::core::convert::From<RemoveWhiteListedAssetCall> for AssetReservesCalls {
        fn from(value: RemoveWhiteListedAssetCall) -> Self {
            Self::RemoveWhiteListedAsset(value)
        }
    }
    impl ::core::convert::From<SendExcessYieldCall> for AssetReservesCalls {
        fn from(value: SendExcessYieldCall) -> Self {
            Self::SendExcessYield(value)
        }
    }
    impl ::core::convert::From<SetTargetAvailableLiquidityCall> for AssetReservesCalls {
        fn from(value: SetTargetAvailableLiquidityCall) -> Self {
            Self::SetTargetAvailableLiquidity(value)
        }
    }
    impl ::core::convert::From<WhiteListedAssetInfoCall> for AssetReservesCalls {
        fn from(value: WhiteListedAssetInfoCall) -> Self {
            Self::WhiteListedAssetInfo(value)
        }
    }
    impl ::core::convert::From<WhitelistAssetAddParamsCall> for AssetReservesCalls {
        fn from(value: WhitelistAssetAddParamsCall) -> Self {
            Self::WhitelistAssetAddParams(value)
        }
    }
    ///Container type for all return fields from the `isAssetWhiteListed` function with signature `isAssetWhiteListed(address)` and selector `0x3bf3d1f8`
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
    pub struct IsAssetWhiteListedReturn(pub bool);
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
    ///Container type for all return fields from the `nexus` function with signature `nexus()` and selector `0xa3f5c1d2`
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
    pub struct NexusReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `whiteListedAssetInfo` function with signature `whiteListedAssetInfo(address)` and selector `0xbf988bf5`
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
    pub struct WhiteListedAssetInfoReturn {
        pub whitelisted: bool,
        pub strategy_vault: ::ethers::core::types::Address,
        pub total_bridged_amount: ::ethers::core::types::U256,
        pub target_available_liquidity: ::ethers::core::types::U256,
    }
}
