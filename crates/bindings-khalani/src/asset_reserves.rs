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
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TOKEN_MINTERBURNER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TOKEN_MINTERBURNER_ROLE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addTokenMinterBurnerRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addTokenMinterBurnerRole",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoleMember"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMember"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("InvalidAdminRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidAdminRole"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTokenMinterBurnerRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidTokenMinterBurnerRole",
                            ),
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
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0)\xFD8\x03\x80b\0)\xFD\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x02\x16V[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x163\x17\x90\x91U`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x91\x90\x93\x16\x17\x90U\x81\x16`\x80Rb\0\0x`\0b\0\0r3\x90V[b\0\0\xACV[b\0\0\xA4\x7F\x91\xC0\xA0\xC8`\xF8B\xE6\x1E\xA5\xD1\xB9\xBB\xF1\xCB\xACBX\xB1\xEC\x93\xED\x9D:\"\xF1\xF5y\xF7\x91\x83M\x83b\0\0\xACV[PPb\0\x02NV[b\0\0\xB8\x82\x82b\0\0\xBCV[PPV[b\0\0\xC8\x82\x82b\0\0\xE7V[`\0\x82\x81R`\x01` R`@\x90 b\0\0\xE2\x90\x82b\0\x01\x87V[PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\0\xB8W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01C3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0b\0\x01\x9E\x83`\x01`\x01`\xA0\x1B\x03\x84\x16b\0\x01\xA7V[\x90P[\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Tb\0\x01\xF0WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ub\0\x01\xA1V[P`\0b\0\x01\xA1V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02\x11W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x02*W`\0\x80\xFD[b\0\x025\x83b\0\x01\xF9V[\x91Pb\0\x02E` \x84\x01b\0\x01\xF9V[\x90P\x92P\x92\x90PV[`\x80Qa'wb\0\x02\x86`\09`\0\x81\x81a\x03\xCD\x01R\x81\x81a\t>\x01R\x81\x81a\tw\x01R\x81\x81a\x0F\xAD\x01Ra\x10\r\x01Ra'w`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80c\x90\x10\xD0|\x11a\0\xC3W\x80c\xBF\x98\x8B\xF5\x11a\0|W\x80c\xBF\x98\x8B\xF5\x14a\x03\x10W\x80c\xCA\x15\xC8s\x14a\x03|W\x80c\xD5Gt\x1F\x14a\x03\x8FW\x80c\xDC\x02n\xF6\x14a\x03\xA2W\x80c\xE2D\xED\xB2\x14a\x03\xB5W\x80c\xE8\xA2\xB1j\x14a\x03\xC8W`\0\x80\xFD[\x80c\x90\x10\xD0|\x14a\x02\xA9W\x80c\x91\xD1HT\x14a\x02\xBCW\x80c\x92\x94R<\x14a\x02\xCFW\x80c\xA2\x17\xFD\xDF\x14a\x02\xE2W\x80c\xA3\xF5\xC1\xD2\x14a\x02\xEAW\x80c\xB1\x06H-\x14a\x02\xFDW`\0\x80\xFD[\x80c3\n'g\x11a\x01\x15W\x80c3\n'g\x14a\x02\x04W\x80c6V\x8A\xBE\x14a\x02\x17W\x80c;\xF3\xD1\xF8\x14a\x02*W\x80cx\xA0\xD5\x97\x14a\x02VW\x80c\x8D\xA5\xCB[\x14a\x02iW\x80c\x8E}S\xA1\x14a\x02\x94W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01]W\x80c\x05\xF0^Q\x14a\x01\x85W\x80c\r\xCB\xD4\x06\x14a\x01\x9AW\x80c!\xC2\x81\x91\x14a\x01\xADW\x80c$\x8A\x9C\xA3\x14a\x01\xC0W\x80c//\xF1]\x14a\x01\xF1W[`\0\x80\xFD[a\x01pa\x01k6`\x04a\"IV[a\x03\xEFV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x98a\x01\x936`\x04a\"\x8FV[a\x04\x1AV[\0[a\x01\x98a\x01\xA86`\x04a\"\xB9V[a\x04\xA6V[a\x01\x98a\x01\xBB6`\x04a\"\xB9V[a\x05\x1DV[a\x01\xE3a\x01\xCE6`\x04a\"\xD4V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01|V[a\x01\x98a\x01\xFF6`\x04a\"\xEDV[a\x08hV[a\x01\x98a\x02\x126`\x04a#\x19V[a\x08\x8DV[a\x01\x98a\x02%6`\x04a\"\xEDV[a\x0E\x1FV[a\x01pa\x0286`\x04a\"\xB9V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x90V[a\x01\x98a\x02d6`\x04a\"\xB9V[a\x0E\x9DV[`\x03Ta\x02|\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01|V[a\x01\xE3`\0\x80Q` a'\"\x839\x81Q\x91R\x81V[a\x02|a\x02\xB76`\x04a#\x9FV[a\x0E\xE0V[a\x01pa\x02\xCA6`\x04a\"\xEDV[a\x0E\xFFV[a\x01\x98a\x02\xDD6`\x04a#\x19V[a\x0F(V[a\x01\xE3`\0\x81V[`\x02Ta\x02|\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x98a\x03\x0B6`\x04a#\xC1V[a\x13JV[a\x03Ra\x03\x1E6`\x04a\"\xB9V[`\x04` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T`\xFF\x82\x16\x92a\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x90\x84V[`@\x80Q\x94\x15\x15\x85R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x85\x01R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01|V[a\x01\xE3a\x03\x8A6`\x04a\"\xD4V[a\x13\xDFV[a\x01\x98a\x03\x9D6`\x04a\"\xEDV[a\x13\xF6V[a\x01\x98a\x03\xB06`\x04a\"\xB9V[a\x14\x1BV[a\x01\x98a\x03\xC36`\x04a#\xFDV[a\x14\xA2V[a\x02|\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x04\x14WPa\x04\x14\x82a\x19\x15V[\x92\x91PPV[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04EW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R\x7F%\xA8\xDD\xEF\x15\x04\x10p\xAEUxq\xE4k\x9C\x88\xF8\xD77_\xF8\xE9`\xA0\xC6b\"\xFA\x90\xAC\x92\x1D\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x04` R`@\x90 `\x02\x01UV[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xD1W`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x04` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fp\x12\xF2\xDDH\x9A#\xD7\x0Cz\x9F\xF3U\xB2`N0U\xCD\x12\x98\x93A\t\xB8\xF5\x13\xCE\x7F\xA0\0U\x91\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x81\x81R`\x04` \x81\x81R`@\x80\x84 \x81Q`\x80\x81\x01\x83R\x81T`\xFF\x81\x16\x15\x15\x82Ra\x01\0\x90\x04\x90\x97\x16\x92\x87\x01\x92\x90\x92R`\x01\x82\x01T\x86\x82\x01R`\x02\x90\x91\x01T``\x86\x01RQcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R\x90\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xCE\x91\x90a$9V[\x90P\x81``\x01Q\x81\x11\x15a\x06\xA6W`\0\x82``\x01Q\x82a\x05\xEE\x91\x90a$hV[\x90Pa\x05\xFF\x84\x84` \x01Q\x83a\x19JV[` \x83\x01Q`@QcnU?e`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cnU?e\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06v\x91\x90a$9V[P`@Q\x7F\x1E\x91\xA0\x19R\xD1\xE7<\x03\xAD\x8F'\xA0oU\xA8\xD17\xF7G\x9EP7\xB7U\xC1\xE9\x93\xC2\x0Ey$\x90`\0\x90\xA1PPPPV[` \x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x15\x91\x90a$9V[\x90P`\0\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16cL\xDA\xD5\x06\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07K\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x8C\x91\x90a$9V[\x90P`\0\x83\x85``\x01Qa\x07\xA0\x91\x90a$hV[\x90P`\0\x81\x11\x80\x15a\x07\xB1WP\x80\x82\x11[\x15a\x08_W` \x85\x01Q`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01\x81\x90R`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x084\x91\x90a$9V[P`@Q\x7F\x1E\x91\xA0\x19R\xD1\xE7<\x03\xAD\x8F'\xA0oU\xA8\xD17\xF7G\x9EP7\xB7U\xC1\xE9\x93\xC2\x0Ey$\x90`\0\x90\xA1[PPP[PPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x08\x83\x81a\x1A\x92V[a\x08c\x83\x83a\x1A\x9CV[a\x08\xA5`\0\x80Q` a'\"\x839\x81Q\x91R3a\x0E\xFFV[a\x08\xC2W`@Qce\xB8UK`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7FoC\xAB't\xA9\xDD\x8B\xAE\xAA\x9A\x01\xFD\x83,v\x0B|)vFs\xFE\xE2\xC9L\xD2\xA90[\xDB\x89\x83\x83`@Qa\x08\xFD\x92\x91\x90a${V[`@Q\x80\x91\x03\x90\xA2`\0[\x81\x81\x10\x15a\x0E\x19W`\0\x83\x83\x83\x81\x81\x10a\t$Wa\t$a$\xD2V[a\t:\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\n)W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x19\x86\x86\x86\x86\x81\x81\x10a\t\xB7Wa\t\xB7a$\xD2V[\x90P`@\x02\x01` \x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xF2\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\x0CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n W=`\0\x80>=`\0\xFD[PPPPa\x0E\x10V[`\0`\x04`\0\x86\x86\x86\x81\x81\x10a\nAWa\nAa$\xD2V[a\nW\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91P`\xFF\x16\x15a\r\xBCW\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15a\rjW\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x03\x91\x90a$9V[\x82T`@Qc&mj\x83`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\0\x91a\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cL\xDA\xD5\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0By\x91\x90a$9V[\x90P\x86\x86\x86\x81\x81\x10a\x0B\x8DWa\x0B\x8Da$\xD2V[\x90P`@\x02\x01` \x015\x83`\x01\x01`\0\x82\x82Ta\x0B\xAA\x91\x90a$hV[\x90\x91UP\x81\x90P\x87\x87\x87\x81\x81\x10a\x0B\xC3Wa\x0B\xC3a$\xD2V[\x90P`@\x02\x01` \x015\x11\x15a\x0C\xB3W\x82T`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`$\x83\x01R0`D\x83\x01Ra\x01\0\x90\x92\x04\x90\x91\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CU\x91\x90a$9V[Pa\x0C\xAE\x87\x87\x87\x81\x81\x10a\x0CkWa\x0Cka$\xD2V[a\x0C\x81\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x89\x83\x8A\x8A\x8A\x81\x81\x10a\x0C\x95Wa\x0C\x95a$\xD2V[\x90P`@\x02\x01` \x015a\x0C\xA9\x91\x90a$hV[a\x1A\xBEV[a\rcV[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16c\xB4`\xAF\x94\x88\x88\x88\x81\x81\x10a\x0C\xDAWa\x0C\xDAa$\xD2V[\x90P`@\x02\x01` \x015\x8A0`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\x1E\x93\x92\x91\x90\x92\x83R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ra\x91\x90a$9V[P[PPa\x0E\x0EV[a\r\xB7\x85\x85\x85\x81\x81\x10a\r\x7FWa\r\x7Fa$\xD2V[a\r\x95\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x87\x87\x87\x87\x81\x81\x10a\r\xA8Wa\r\xA8a$\xD2V[\x90P`@\x02\x01` \x015a\x1A\xBEV[a\x0E\x0EV[\x84\x84\x84\x81\x81\x10a\r\xCEWa\r\xCEa$\xD2V[a\r\xE4\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[`@Qc\x06|x\x7F`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[P[P`\x01\x01a\t\x08V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0E\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x0E\x05V[a\x0E\x99\x82\x82a\x1A\xEEV[PPV[a\x0E\xA8`\x003a\x0E\xFFV[a\x0E\xC5W`@Qc-M{\xB3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xDD`\0\x80Q` a'\"\x839\x81Q\x91R\x82a\x08hV[PV[`\0\x82\x81R`\x01` R`@\x81 a\x0E\xF8\x90\x83a\x1B\x10V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x0F@`\0\x80Q` a'\"\x839\x81Q\x91R3a\x0E\xFFV[a\x0F]W`@Qce\xB8UK`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7Fv\n5\x8DH\n\xCDb\xE3\t\xA72K\xC2\x89n\xBA\xC8E\xFB\x1B.\xBE\xC73\x9Eg\x0B\xDC\x17A\xA5\x83\x83`@Qa\x0F\x98\x92\x91\x90a${V[`@Q\x80\x91\x03\x90\xA2`\0[\x81\x81\x10\x15a\x0E\x19W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83\x83\x83\x81\x81\x10a\x0F\xE7Wa\x0F\xE7a$\xD2V[a\x0F\xFD\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x10\xBFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cy\xCCg\x90\x85\x85\x85\x85\x81\x81\x10a\x10MWa\x10Ma$\xD2V[\x90P`@\x02\x01` \x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x88\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\xA2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xB6W=`\0\x80>=`\0\xFD[PPPPa\x13BV[`\0`\x04`\0\x85\x85\x85\x81\x81\x10a\x10\xD7Wa\x10\xD7a$\xD2V[a\x10\xED\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91P`\xFF\x16\x15a\x12\xF3Wa\x11a\x84\x84\x84\x81\x81\x10a\x11(Wa\x11(a$\xD2V[a\x11>\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x860\x87\x87\x87\x81\x81\x10a\x11RWa\x11Ra$\xD2V[\x90P`@\x02\x01` \x015a\x1B\x1CV[\x83\x83\x83\x81\x81\x10a\x11sWa\x11sa$\xD2V[\x90P`@\x02\x01` \x015\x81`\x01\x01`\0\x82\x82Ta\x11\x90\x91\x90a$\xE8V[\x90\x91UPP\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15a\x12\xE8Wa\x12\x06\x84\x84\x84\x81\x81\x10a\x11\xBFWa\x11\xBFa$\xD2V[a\x11\xD5\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x86\x81\x81\x10a\x11\xF7Wa\x11\xF7a$\xD2V[\x90P`@\x02\x01` \x015a\x19JV[\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16cnU?e\x85\x85\x85\x81\x81\x10a\x12-Wa\x12-a$\xD2V[\x90P`@\x02\x01` \x0150`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12g\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x12\xA2WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x12\x9F\x91\x81\x01\x90a$9V[`\x01[a\x12\xEDWa\x12\xE8\x84\x84\x84\x81\x81\x10a\x12\xBBWa\x12\xBBa$\xD2V[a\x12\xD1\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\0a\x19JV[a\x13@V[Pa\x13@V[\x83\x83\x83\x81\x81\x10a\x13\x05Wa\x13\x05a$\xD2V[a\x13\x1B\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[`@Qc\x16\xFCB)`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x0E\x05V[P[`\x01\x01a\x0F\xA3V[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13uW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x04` R`@\x80\x82 \x80T\x94\x87\x16a\x01\0\x02`\x01`\x01`\xA8\x1B\x03\x19\x90\x95\x16\x94\x90\x94\x17`\x01\x17\x84U`\x02\x84\x01\x85\x90UQ\x7Fp\x12\xF2\xDDH\x9A#\xD7\x0Cz\x9F\xF3U\xB2`N0U\xCD\x12\x98\x93A\t\xB8\xF5\x13\xCE\x7F\xA0\0U\x91\x90\xA2PPPPV[`\0\x81\x81R`\x01` R`@\x81 a\x04\x14\x90a\x1BTV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x14\x11\x81a\x1A\x92V[a\x08c\x83\x83a\x1A\xEEV[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14FW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x04` R`@\x80\x82 \x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x81U`\x01\x81\x01\x83\x90U`\x02\x01\x82\x90UQ\x7F\x9Dd\x11\n\xB8D\x13\xB3!#m\x9A\xE6{IMQi\x1F\xFA8\xBCY\xD9\xD8X\x14?'\r42\x91\x90\xA2PV[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xCDW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a'\x10\x82\x11\x15a\x14\xF0W`@Qc\x0F\xF03\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x90\x91\x04\x90\x95\x16\x92\x81\x01\x92\x90\x92R`\x01\x81\x01T\x92\x82\x01\x92\x90\x92R`\x02\x90\x91\x01T``\x82\x01R\x90a\x15\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FProvided asset is not whiteliste`D\x82\x01R`\x19`\xFA\x1B`d\x82\x01R`\x84\x01a\x0E\x05V[` \x81\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\r\x91\x90a$9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16{\x91\x90a$9V[\x90P`\0\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16cL\xDA\xD5\x06\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xB1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xF2\x91\x90a$9V[\x90P`\0a\x17\0\x83\x83a$\xE8V[\x90P\x80\x15a\x19\x0BW`\0\x85`@\x01Q\x82a\x17\x1A\x91\x90a$hV[\x90P\x83\x81\x11\x15a\x17\xB2W`\0a\x170\x85\x83a$hV[` \x88\x01Q`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01\x81\x90R`D\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xAF\x91\x90a$9V[PP[`\0a'\x10a\x17\xC1\x8A\x84a$\xFBV[a\x17\xCB\x91\x90a%\x12V[`\x03T\x90\x91Pa\x17\xEA\x90\x8B\x90`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xA9\x84\x86a$hV[`\x02Ta\x18\x02\x90\x8B\x90`\x01`\x01`\xA0\x1B\x03\x16\x83a\x19JV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\x19W\x90PP\x90P\x8A\x81`\0\x81Q\x81\x10a\x18VWa\x18Va$\xD2V[` \x02` \x01\x01Q`\0\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x81`\0\x81Q\x81\x10a\x18\x8EWa\x18\x8Ea$\xD2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01R`\x02T`@Qc$\xB9\xC7\x15`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x92\xE7\x1CT\x90a\x18\xD5\x90a'\x1C\x90\x85\x90`\x01\x90\x8F\x90`\x04\x01a%JV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xEFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\x03W=`\0\x80>=`\0\xFD[PPPPPPP[PPPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\x14WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x04\x14V[\x80\x15\x80a\x19\xC4WP`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xC2\x91\x90a$9V[\x15[a\x1A/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FSafeERC20: approve from non-zero`D\x82\x01Ru to non-zero allowance`P\x1B`d\x82\x01R`\x84\x01a\x0E\x05V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x08c\x90\x84\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1B^V[a\x0E\xDD\x813a\x1C3V[a\x1A\xA6\x82\x82a\x1C\x8CV[`\0\x82\x81R`\x01` R`@\x90 a\x08c\x90\x82a\x1D\x10V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x08c\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\x1A[V[a\x1A\xF8\x82\x82a\x1D%V[`\0\x82\x81R`\x01` R`@\x90 a\x08c\x90\x82a\x1D\x8AV[`\0a\x0E\xF8\x83\x83a\x1D\x9FV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x0E\x19\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01a\x1A[V[`\0a\x04\x14\x82T\x90V[`\0a\x1B\xB3\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x1D\xC9\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x1B\xD4WP\x80\x80` \x01\x90Q\x81\x01\x90a\x1B\xD4\x91\x90a%\xEAV[a\x08cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x0E\x05V[a\x1C=\x82\x82a\x0E\xFFV[a\x0E\x99Wa\x1CJ\x81a\x1D\xE0V[a\x1CU\x83` a\x1D\xF2V[`@Q` \x01a\x1Cf\x92\x91\x90a&0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x0E\x05\x91`\x04\x01a&\xA5V[a\x1C\x96\x82\x82a\x0E\xFFV[a\x0E\x99W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1C\xCC3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x0E\xF8\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1F\x8EV[a\x1D/\x82\x82a\x0E\xFFV[\x15a\x0E\x99W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x0E\xF8\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1F\xDDV[`\0\x82`\0\x01\x82\x81T\x81\x10a\x1D\xB6Wa\x1D\xB6a$\xD2V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[``a\x1D\xD8\x84\x84`\0\x85a \xD0V[\x94\x93PPPPV[``a\x04\x14`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x1E\x01\x83`\x02a$\xFBV[a\x1E\x0C\x90`\x02a$\xE8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E$Wa\x1E$a%4V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1ENW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x1EiWa\x1Eia$\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x1E\x98Wa\x1E\x98a$\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x1E\xBC\x84`\x02a$\xFBV[a\x1E\xC7\x90`\x01a$\xE8V[\x90P[`\x01\x81\x11\x15a\x1F?Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x1E\xFBWa\x1E\xFBa$\xD2V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1F\x11Wa\x1F\x11a$\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x1F8\x81a&\xD8V[\x90Pa\x1E\xCAV[P\x83\x15a\x0E\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x0E\x05V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x1F\xD5WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x04\x14V[P`\0a\x04\x14V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a \xC6W`\0a \x01`\x01\x83a$hV[\x85T\x90\x91P`\0\x90a \x15\x90`\x01\x90a$hV[\x90P\x81\x81\x14a zW`\0\x86`\0\x01\x82\x81T\x81\x10a 5Wa 5a$\xD2V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a XWa Xa$\xD2V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a \x8BWa \x8Ba&\xEFV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x04\x14V[`\0\x91PPa\x04\x14V[``\x82G\x10\x15a!1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x0E\x05V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa!M\x91\x90a'\x05V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a!\x8AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a!\x8FV[``\x91P[P\x91P\x91Pa!\xA0\x87\x83\x83\x87a!\xABV[\x97\x96PPPPPPPV[``\x83\x15a\"\x1AW\x82Q`\0\x03a\"\x13W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\"\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0E\x05V[P\x81a\x1D\xD8V[a\x1D\xD8\x83\x83\x81Q\x15a\"/W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\x05\x91\x90a&\xA5V[`\0` \x82\x84\x03\x12\x15a\"[W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0E\xF8W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"\x8AW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\"\xA2W`\0\x80\xFD[a\"\xAB\x83a\"sV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\"\xCBW`\0\x80\xFD[a\x0E\xF8\x82a\"sV[`\0` \x82\x84\x03\x12\x15a\"\xE6W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a#\0W`\0\x80\xFD[\x825\x91Pa#\x10` \x84\x01a\"sV[\x90P\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a#.W`\0\x80\xFD[a#7\x84a\"sV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#TW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a#hW`\0\x80\xFD[\x815\x81\x81\x11\x15a#wW`\0\x80\xFD[\x87` \x82`\x06\x1B\x85\x01\x01\x11\x15a#\x8CW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a#\xB2W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a#\xD6W`\0\x80\xFD[a#\xDF\x84a\"sV[\x92Pa#\xED` \x85\x01a\"sV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a$\x12W`\0\x80\xFD[a$\x1B\x84a\"sV[\x92P` \x84\x015\x91Pa$0`@\x85\x01a\"sV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a$KW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\x14Wa\x04\x14a$RV[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@\x80\x84\x01\x86\x84[\x87\x81\x10\x15a$\xC5W`\x01`\x01`\xA0\x1B\x03a$\xAA\x83a\"sV[\x16\x83R\x81\x85\x015\x85\x84\x01R\x91\x83\x01\x91\x90\x83\x01\x90`\x01\x01a$\x91V[P\x90\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04\x14Wa\x04\x14a$RV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x14Wa\x04\x14a$RV[`\0\x82a%/WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\xC0\x82\x01\x86\x83R` `\xC0\x81\x85\x01R\x81\x87Q\x80\x84R`\xE0\x86\x01\x91P\x82\x89\x01\x93P`\0[\x81\x81\x10\x15a%\x9FW\x84Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x84\x01Q\x84\x84\x01R\x93\x83\x01\x93`@\x90\x92\x01\x91`\x01\x01a%oV[PP\x84\x81\x03`@\x86\x01R`\0\x81R\x86\x15\x15``\x86\x01R` \x01\x91Pa%\xC1\x90PV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`\x80\x83\x01RP\x80\x82\x03`\xA0\x90\x91\x01R`\0\x81R` \x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a%\xFCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0E\xF8W`\0\x80\xFD[`\0[\x83\x81\x10\x15a&'W\x81\x81\x01Q\x83\x82\x01R` \x01a&\x0FV[PP`\0\x91\x01RV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa&h\x81`\x17\x85\x01` \x88\x01a&\x0CV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa&\x99\x81`(\x84\x01` \x88\x01a&\x0CV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra&\xC4\x81`@\x85\x01` \x87\x01a&\x0CV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x81a&\xE7Wa&\xE7a$RV[P`\0\x19\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82Qa'\x17\x81\x84` \x87\x01a&\x0CV[\x91\x90\x91\x01\x92\x91PPV\xFE\x91\xC0\xA0\xC8`\xF8B\xE6\x1E\xA5\xD1\xB9\xBB\xF1\xCB\xACBX\xB1\xEC\x93\xED\x9D:\"\xF1\xF5y\xF7\x91\x83M\xA2dipfsX\"\x12 :\x82\xC4\xAA\xE8>\xE7D2\x13\x83\x8F\x82\x87\x97\x88\x18\r\xCF\xC9^@\x110\xA5+\x1E\x15\xD7\xA1n\xE1dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static ASSETRESERVES_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01XW`\x005`\xE0\x1C\x80c\x90\x10\xD0|\x11a\0\xC3W\x80c\xBF\x98\x8B\xF5\x11a\0|W\x80c\xBF\x98\x8B\xF5\x14a\x03\x10W\x80c\xCA\x15\xC8s\x14a\x03|W\x80c\xD5Gt\x1F\x14a\x03\x8FW\x80c\xDC\x02n\xF6\x14a\x03\xA2W\x80c\xE2D\xED\xB2\x14a\x03\xB5W\x80c\xE8\xA2\xB1j\x14a\x03\xC8W`\0\x80\xFD[\x80c\x90\x10\xD0|\x14a\x02\xA9W\x80c\x91\xD1HT\x14a\x02\xBCW\x80c\x92\x94R<\x14a\x02\xCFW\x80c\xA2\x17\xFD\xDF\x14a\x02\xE2W\x80c\xA3\xF5\xC1\xD2\x14a\x02\xEAW\x80c\xB1\x06H-\x14a\x02\xFDW`\0\x80\xFD[\x80c3\n'g\x11a\x01\x15W\x80c3\n'g\x14a\x02\x04W\x80c6V\x8A\xBE\x14a\x02\x17W\x80c;\xF3\xD1\xF8\x14a\x02*W\x80cx\xA0\xD5\x97\x14a\x02VW\x80c\x8D\xA5\xCB[\x14a\x02iW\x80c\x8E}S\xA1\x14a\x02\x94W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01]W\x80c\x05\xF0^Q\x14a\x01\x85W\x80c\r\xCB\xD4\x06\x14a\x01\x9AW\x80c!\xC2\x81\x91\x14a\x01\xADW\x80c$\x8A\x9C\xA3\x14a\x01\xC0W\x80c//\xF1]\x14a\x01\xF1W[`\0\x80\xFD[a\x01pa\x01k6`\x04a\"IV[a\x03\xEFV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x98a\x01\x936`\x04a\"\x8FV[a\x04\x1AV[\0[a\x01\x98a\x01\xA86`\x04a\"\xB9V[a\x04\xA6V[a\x01\x98a\x01\xBB6`\x04a\"\xB9V[a\x05\x1DV[a\x01\xE3a\x01\xCE6`\x04a\"\xD4V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01|V[a\x01\x98a\x01\xFF6`\x04a\"\xEDV[a\x08hV[a\x01\x98a\x02\x126`\x04a#\x19V[a\x08\x8DV[a\x01\x98a\x02%6`\x04a\"\xEDV[a\x0E\x1FV[a\x01pa\x0286`\x04a\"\xB9V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x90V[a\x01\x98a\x02d6`\x04a\"\xB9V[a\x0E\x9DV[`\x03Ta\x02|\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01|V[a\x01\xE3`\0\x80Q` a'\"\x839\x81Q\x91R\x81V[a\x02|a\x02\xB76`\x04a#\x9FV[a\x0E\xE0V[a\x01pa\x02\xCA6`\x04a\"\xEDV[a\x0E\xFFV[a\x01\x98a\x02\xDD6`\x04a#\x19V[a\x0F(V[a\x01\xE3`\0\x81V[`\x02Ta\x02|\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x98a\x03\x0B6`\x04a#\xC1V[a\x13JV[a\x03Ra\x03\x1E6`\x04a\"\xB9V[`\x04` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T`\xFF\x82\x16\x92a\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x90\x84V[`@\x80Q\x94\x15\x15\x85R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x85\x01R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01|V[a\x01\xE3a\x03\x8A6`\x04a\"\xD4V[a\x13\xDFV[a\x01\x98a\x03\x9D6`\x04a\"\xEDV[a\x13\xF6V[a\x01\x98a\x03\xB06`\x04a\"\xB9V[a\x14\x1BV[a\x01\x98a\x03\xC36`\x04a#\xFDV[a\x14\xA2V[a\x02|\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x04\x14WPa\x04\x14\x82a\x19\x15V[\x92\x91PPV[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04EW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R\x7F%\xA8\xDD\xEF\x15\x04\x10p\xAEUxq\xE4k\x9C\x88\xF8\xD77_\xF8\xE9`\xA0\xC6b\"\xFA\x90\xAC\x92\x1D\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x04` R`@\x90 `\x02\x01UV[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xD1W`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x04` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fp\x12\xF2\xDDH\x9A#\xD7\x0Cz\x9F\xF3U\xB2`N0U\xCD\x12\x98\x93A\t\xB8\xF5\x13\xCE\x7F\xA0\0U\x91\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x81\x81R`\x04` \x81\x81R`@\x80\x84 \x81Q`\x80\x81\x01\x83R\x81T`\xFF\x81\x16\x15\x15\x82Ra\x01\0\x90\x04\x90\x97\x16\x92\x87\x01\x92\x90\x92R`\x01\x82\x01T\x86\x82\x01R`\x02\x90\x91\x01T``\x86\x01RQcp\xA0\x821`\xE0\x1B\x81R0\x91\x81\x01\x91\x90\x91R\x90\x91\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xAAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xCE\x91\x90a$9V[\x90P\x81``\x01Q\x81\x11\x15a\x06\xA6W`\0\x82``\x01Q\x82a\x05\xEE\x91\x90a$hV[\x90Pa\x05\xFF\x84\x84` \x01Q\x83a\x19JV[` \x83\x01Q`@QcnU?e`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cnU?e\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06v\x91\x90a$9V[P`@Q\x7F\x1E\x91\xA0\x19R\xD1\xE7<\x03\xAD\x8F'\xA0oU\xA8\xD17\xF7G\x9EP7\xB7U\xC1\xE9\x93\xC2\x0Ey$\x90`\0\x90\xA1PPPPV[` \x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x15\x91\x90a$9V[\x90P`\0\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16cL\xDA\xD5\x06\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07K\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x8C\x91\x90a$9V[\x90P`\0\x83\x85``\x01Qa\x07\xA0\x91\x90a$hV[\x90P`\0\x81\x11\x80\x15a\x07\xB1WP\x80\x82\x11[\x15a\x08_W` \x85\x01Q`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01\x81\x90R`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x084\x91\x90a$9V[P`@Q\x7F\x1E\x91\xA0\x19R\xD1\xE7<\x03\xAD\x8F'\xA0oU\xA8\xD17\xF7G\x9EP7\xB7U\xC1\xE9\x93\xC2\x0Ey$\x90`\0\x90\xA1[PPP[PPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x08\x83\x81a\x1A\x92V[a\x08c\x83\x83a\x1A\x9CV[a\x08\xA5`\0\x80Q` a'\"\x839\x81Q\x91R3a\x0E\xFFV[a\x08\xC2W`@Qce\xB8UK`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7FoC\xAB't\xA9\xDD\x8B\xAE\xAA\x9A\x01\xFD\x83,v\x0B|)vFs\xFE\xE2\xC9L\xD2\xA90[\xDB\x89\x83\x83`@Qa\x08\xFD\x92\x91\x90a${V[`@Q\x80\x91\x03\x90\xA2`\0[\x81\x81\x10\x15a\x0E\x19W`\0\x83\x83\x83\x81\x81\x10a\t$Wa\t$a$\xD2V[a\t:\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\n)W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x19\x86\x86\x86\x86\x81\x81\x10a\t\xB7Wa\t\xB7a$\xD2V[\x90P`@\x02\x01` \x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xF2\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\x0CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n W=`\0\x80>=`\0\xFD[PPPPa\x0E\x10V[`\0`\x04`\0\x86\x86\x86\x81\x81\x10a\nAWa\nAa$\xD2V[a\nW\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91P`\xFF\x16\x15a\r\xBCW\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15a\rjW\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x03\x91\x90a$9V[\x82T`@Qc&mj\x83`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\0\x91a\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cL\xDA\xD5\x06\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0By\x91\x90a$9V[\x90P\x86\x86\x86\x81\x81\x10a\x0B\x8DWa\x0B\x8Da$\xD2V[\x90P`@\x02\x01` \x015\x83`\x01\x01`\0\x82\x82Ta\x0B\xAA\x91\x90a$hV[\x90\x91UP\x81\x90P\x87\x87\x87\x81\x81\x10a\x0B\xC3Wa\x0B\xC3a$\xD2V[\x90P`@\x02\x01` \x015\x11\x15a\x0C\xB3W\x82T`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`$\x83\x01R0`D\x83\x01Ra\x01\0\x90\x92\x04\x90\x91\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0C1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CU\x91\x90a$9V[Pa\x0C\xAE\x87\x87\x87\x81\x81\x10a\x0CkWa\x0Cka$\xD2V[a\x0C\x81\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x89\x83\x8A\x8A\x8A\x81\x81\x10a\x0C\x95Wa\x0C\x95a$\xD2V[\x90P`@\x02\x01` \x015a\x0C\xA9\x91\x90a$hV[a\x1A\xBEV[a\rcV[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16c\xB4`\xAF\x94\x88\x88\x88\x81\x81\x10a\x0C\xDAWa\x0C\xDAa$\xD2V[\x90P`@\x02\x01` \x015\x8A0`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\x1E\x93\x92\x91\x90\x92\x83R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ra\x91\x90a$9V[P[PPa\x0E\x0EV[a\r\xB7\x85\x85\x85\x81\x81\x10a\r\x7FWa\r\x7Fa$\xD2V[a\r\x95\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x87\x87\x87\x87\x81\x81\x10a\r\xA8Wa\r\xA8a$\xD2V[\x90P`@\x02\x01` \x015a\x1A\xBEV[a\x0E\x0EV[\x84\x84\x84\x81\x81\x10a\r\xCEWa\r\xCEa$\xD2V[a\r\xE4\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[`@Qc\x06|x\x7F`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[P[P`\x01\x01a\t\x08V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0E\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x0E\x05V[a\x0E\x99\x82\x82a\x1A\xEEV[PPV[a\x0E\xA8`\x003a\x0E\xFFV[a\x0E\xC5W`@Qc-M{\xB3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\xDD`\0\x80Q` a'\"\x839\x81Q\x91R\x82a\x08hV[PV[`\0\x82\x81R`\x01` R`@\x81 a\x0E\xF8\x90\x83a\x1B\x10V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[a\x0F@`\0\x80Q` a'\"\x839\x81Q\x91R3a\x0E\xFFV[a\x0F]W`@Qce\xB8UK`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7Fv\n5\x8DH\n\xCDb\xE3\t\xA72K\xC2\x89n\xBA\xC8E\xFB\x1B.\xBE\xC73\x9Eg\x0B\xDC\x17A\xA5\x83\x83`@Qa\x0F\x98\x92\x91\x90a${V[`@Q\x80\x91\x03\x90\xA2`\0[\x81\x81\x10\x15a\x0E\x19W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83\x83\x83\x81\x81\x10a\x0F\xE7Wa\x0F\xE7a$\xD2V[a\x0F\xFD\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x10\xBFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cy\xCCg\x90\x85\x85\x85\x85\x81\x81\x10a\x10MWa\x10Ma$\xD2V[\x90P`@\x02\x01` \x015`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\x88\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x10\xA2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xB6W=`\0\x80>=`\0\xFD[PPPPa\x13BV[`\0`\x04`\0\x85\x85\x85\x81\x81\x10a\x10\xD7Wa\x10\xD7a$\xD2V[a\x10\xED\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T\x90\x91P`\xFF\x16\x15a\x12\xF3Wa\x11a\x84\x84\x84\x81\x81\x10a\x11(Wa\x11(a$\xD2V[a\x11>\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x860\x87\x87\x87\x81\x81\x10a\x11RWa\x11Ra$\xD2V[\x90P`@\x02\x01` \x015a\x1B\x1CV[\x83\x83\x83\x81\x81\x10a\x11sWa\x11sa$\xD2V[\x90P`@\x02\x01` \x015\x81`\x01\x01`\0\x82\x82Ta\x11\x90\x91\x90a$\xE8V[\x90\x91UPP\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15a\x12\xE8Wa\x12\x06\x84\x84\x84\x81\x81\x10a\x11\xBFWa\x11\xBFa$\xD2V[a\x11\xD5\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x86\x81\x81\x10a\x11\xF7Wa\x11\xF7a$\xD2V[\x90P`@\x02\x01` \x015a\x19JV[\x80Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16cnU?e\x85\x85\x85\x81\x81\x10a\x12-Wa\x12-a$\xD2V[\x90P`@\x02\x01` \x0150`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12g\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x12\xA2WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x12\x9F\x91\x81\x01\x90a$9V[`\x01[a\x12\xEDWa\x12\xE8\x84\x84\x84\x81\x81\x10a\x12\xBBWa\x12\xBBa$\xD2V[a\x12\xD1\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[\x82Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\0a\x19JV[a\x13@V[Pa\x13@V[\x83\x83\x83\x81\x81\x10a\x13\x05Wa\x13\x05a$\xD2V[a\x13\x1B\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\"\xB9V[`@Qc\x16\xFCB)`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x0E\x05V[P[`\x01\x01a\x0F\xA3V[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13uW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x04` R`@\x80\x82 \x80T\x94\x87\x16a\x01\0\x02`\x01`\x01`\xA8\x1B\x03\x19\x90\x95\x16\x94\x90\x94\x17`\x01\x17\x84U`\x02\x84\x01\x85\x90UQ\x7Fp\x12\xF2\xDDH\x9A#\xD7\x0Cz\x9F\xF3U\xB2`N0U\xCD\x12\x98\x93A\t\xB8\xF5\x13\xCE\x7F\xA0\0U\x91\x90\xA2PPPPV[`\0\x81\x81R`\x01` R`@\x81 a\x04\x14\x90a\x1BTV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x14\x11\x81a\x1A\x92V[a\x08c\x83\x83a\x1A\xEEV[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14FW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x04` R`@\x80\x82 \x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x81U`\x01\x81\x01\x83\x90U`\x02\x01\x82\x90UQ\x7F\x9Dd\x11\n\xB8D\x13\xB3!#m\x9A\xE6{IMQi\x1F\xFA8\xBCY\xD9\xD8X\x14?'\r42\x91\x90\xA2PV[`\x03T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xCDW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a'\x10\x82\x11\x15a\x14\xF0W`@Qc\x0F\xF03\x89`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x90\x91\x04\x90\x95\x16\x92\x81\x01\x92\x90\x92R`\x01\x81\x01T\x92\x82\x01\x92\x90\x92R`\x02\x90\x91\x01T``\x82\x01R\x90a\x15\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FProvided asset is not whiteliste`D\x82\x01R`\x19`\xFA\x1B`d\x82\x01R`\x84\x01a\x0E\x05V[` \x81\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\r\x91\x90a$9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16{\x91\x90a$9V[\x90P`\0\x83` \x01Q`\x01`\x01`\xA0\x1B\x03\x16cL\xDA\xD5\x06\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\xB1\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xF2\x91\x90a$9V[\x90P`\0a\x17\0\x83\x83a$\xE8V[\x90P\x80\x15a\x19\x0BW`\0\x85`@\x01Q\x82a\x17\x1A\x91\x90a$hV[\x90P\x83\x81\x11\x15a\x17\xB2W`\0a\x170\x85\x83a$hV[` \x88\x01Q`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01\x81\x90R`D\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xAF\x91\x90a$9V[PP[`\0a'\x10a\x17\xC1\x8A\x84a$\xFBV[a\x17\xCB\x91\x90a%\x12V[`\x03T\x90\x91Pa\x17\xEA\x90\x8B\x90`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xA9\x84\x86a$hV[`\x02Ta\x18\x02\x90\x8B\x90`\x01`\x01`\xA0\x1B\x03\x16\x83a\x19JV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\x19W\x90PP\x90P\x8A\x81`\0\x81Q\x81\x10a\x18VWa\x18Va$\xD2V[` \x02` \x01\x01Q`\0\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x81`\0\x81Q\x81\x10a\x18\x8EWa\x18\x8Ea$\xD2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01R`\x02T`@Qc$\xB9\xC7\x15`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x92\xE7\x1CT\x90a\x18\xD5\x90a'\x1C\x90\x85\x90`\x01\x90\x8F\x90`\x04\x01a%JV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x18\xEFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\x03W=`\0\x80>=`\0\xFD[PPPPPPP[PPPPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x04\x14WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x04\x14V[\x80\x15\x80a\x19\xC4WP`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xC2\x91\x90a$9V[\x15[a\x1A/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FSafeERC20: approve from non-zero`D\x82\x01Ru to non-zero allowance`P\x1B`d\x82\x01R`\x84\x01a\x0E\x05V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x08c\x90\x84\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x1B^V[a\x0E\xDD\x813a\x1C3V[a\x1A\xA6\x82\x82a\x1C\x8CV[`\0\x82\x81R`\x01` R`@\x90 a\x08c\x90\x82a\x1D\x10V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x08c\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\x1A[V[a\x1A\xF8\x82\x82a\x1D%V[`\0\x82\x81R`\x01` R`@\x90 a\x08c\x90\x82a\x1D\x8AV[`\0a\x0E\xF8\x83\x83a\x1D\x9FV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x0E\x19\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01a\x1A[V[`\0a\x04\x14\x82T\x90V[`\0a\x1B\xB3\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x1D\xC9\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x1B\xD4WP\x80\x80` \x01\x90Q\x81\x01\x90a\x1B\xD4\x91\x90a%\xEAV[a\x08cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x0E\x05V[a\x1C=\x82\x82a\x0E\xFFV[a\x0E\x99Wa\x1CJ\x81a\x1D\xE0V[a\x1CU\x83` a\x1D\xF2V[`@Q` \x01a\x1Cf\x92\x91\x90a&0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x0E\x05\x91`\x04\x01a&\xA5V[a\x1C\x96\x82\x82a\x0E\xFFV[a\x0E\x99W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1C\xCC3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x0E\xF8\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1F\x8EV[a\x1D/\x82\x82a\x0E\xFFV[\x15a\x0E\x99W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x0E\xF8\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x1F\xDDV[`\0\x82`\0\x01\x82\x81T\x81\x10a\x1D\xB6Wa\x1D\xB6a$\xD2V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[``a\x1D\xD8\x84\x84`\0\x85a \xD0V[\x94\x93PPPPV[``a\x04\x14`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x1E\x01\x83`\x02a$\xFBV[a\x1E\x0C\x90`\x02a$\xE8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E$Wa\x1E$a%4V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x1ENW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x1EiWa\x1Eia$\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x1E\x98Wa\x1E\x98a$\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x1E\xBC\x84`\x02a$\xFBV[a\x1E\xC7\x90`\x01a$\xE8V[\x90P[`\x01\x81\x11\x15a\x1F?Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x1E\xFBWa\x1E\xFBa$\xD2V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x1F\x11Wa\x1F\x11a$\xD2V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x1F8\x81a&\xD8V[\x90Pa\x1E\xCAV[P\x83\x15a\x0E\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x0E\x05V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x1F\xD5WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x04\x14V[P`\0a\x04\x14V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a \xC6W`\0a \x01`\x01\x83a$hV[\x85T\x90\x91P`\0\x90a \x15\x90`\x01\x90a$hV[\x90P\x81\x81\x14a zW`\0\x86`\0\x01\x82\x81T\x81\x10a 5Wa 5a$\xD2V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a XWa Xa$\xD2V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a \x8BWa \x8Ba&\xEFV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x04\x14V[`\0\x91PPa\x04\x14V[``\x82G\x10\x15a!1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x0E\x05V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa!M\x91\x90a'\x05V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a!\x8AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a!\x8FV[``\x91P[P\x91P\x91Pa!\xA0\x87\x83\x83\x87a!\xABV[\x97\x96PPPPPPPV[``\x83\x15a\"\x1AW\x82Q`\0\x03a\"\x13W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\"\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0E\x05V[P\x81a\x1D\xD8V[a\x1D\xD8\x83\x83\x81Q\x15a\"/W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\x05\x91\x90a&\xA5V[`\0` \x82\x84\x03\x12\x15a\"[W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0E\xF8W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\"\x8AW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\"\xA2W`\0\x80\xFD[a\"\xAB\x83a\"sV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\"\xCBW`\0\x80\xFD[a\x0E\xF8\x82a\"sV[`\0` \x82\x84\x03\x12\x15a\"\xE6W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a#\0W`\0\x80\xFD[\x825\x91Pa#\x10` \x84\x01a\"sV[\x90P\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a#.W`\0\x80\xFD[a#7\x84a\"sV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#TW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a#hW`\0\x80\xFD[\x815\x81\x81\x11\x15a#wW`\0\x80\xFD[\x87` \x82`\x06\x1B\x85\x01\x01\x11\x15a#\x8CW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a#\xB2W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0\x80`\0``\x84\x86\x03\x12\x15a#\xD6W`\0\x80\xFD[a#\xDF\x84a\"sV[\x92Pa#\xED` \x85\x01a\"sV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a$\x12W`\0\x80\xFD[a$\x1B\x84a\"sV[\x92P` \x84\x015\x91Pa$0`@\x85\x01a\"sV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a$KW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\x14Wa\x04\x14a$RV[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@\x80\x84\x01\x86\x84[\x87\x81\x10\x15a$\xC5W`\x01`\x01`\xA0\x1B\x03a$\xAA\x83a\"sV[\x16\x83R\x81\x85\x015\x85\x84\x01R\x91\x83\x01\x91\x90\x83\x01\x90`\x01\x01a$\x91V[P\x90\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04\x14Wa\x04\x14a$RV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\x14Wa\x04\x14a$RV[`\0\x82a%/WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0`\xC0\x82\x01\x86\x83R` `\xC0\x81\x85\x01R\x81\x87Q\x80\x84R`\xE0\x86\x01\x91P\x82\x89\x01\x93P`\0[\x81\x81\x10\x15a%\x9FW\x84Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x84\x01Q\x84\x84\x01R\x93\x83\x01\x93`@\x90\x92\x01\x91`\x01\x01a%oV[PP\x84\x81\x03`@\x86\x01R`\0\x81R\x86\x15\x15``\x86\x01R` \x01\x91Pa%\xC1\x90PV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16`\x80\x83\x01RP\x80\x82\x03`\xA0\x90\x91\x01R`\0\x81R` \x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a%\xFCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0E\xF8W`\0\x80\xFD[`\0[\x83\x81\x10\x15a&'W\x81\x81\x01Q\x83\x82\x01R` \x01a&\x0FV[PP`\0\x91\x01RV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa&h\x81`\x17\x85\x01` \x88\x01a&\x0CV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa&\x99\x81`(\x84\x01` \x88\x01a&\x0CV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra&\xC4\x81`@\x85\x01` \x87\x01a&\x0CV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x81a&\xE7Wa&\xE7a$RV[P`\0\x19\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82Qa'\x17\x81\x84` \x87\x01a&\x0CV[\x91\x90\x91\x01\x92\x91PPV\xFE\x91\xC0\xA0\xC8`\xF8B\xE6\x1E\xA5\xD1\xB9\xBB\xF1\xCB\xACBX\xB1\xEC\x93\xED\x9D:\"\xF1\xF5y\xF7\x91\x83M\xA2dipfsX\"\x12 :\x82\xC4\xAA\xE8>\xE7D2\x13\x83\x8F\x82\x87\x97\x88\x18\r\xCF\xC9^@\x110\xA5+\x1E\x15\xD7\xA1n\xE1dsolcC\0\x08\x13\x003";
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
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TOKEN_MINTERBURNER_ROLE` (0x8e7d53a1) function
        pub fn token_minterburner_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([142, 125, 83, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addTokenMinterBurnerRole` (0x78a0d597) function
        pub fn add_token_minter_burner_role(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 160, 213, 151], account)
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMember` (0x9010d07c) function
        pub fn get_role_member(
            &self,
            role: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([144, 16, 208, 124], (role, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMemberCount` (0xca15c873) function
        pub fn get_role_member_count(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 21, 200, 115], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
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
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
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
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
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
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
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
    ///Custom Error type `InvalidAdminRole` with signature `InvalidAdminRole()` and selector `0x2d4d7bb3`
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
    #[etherror(name = "InvalidAdminRole", abi = "InvalidAdminRole()")]
    pub struct InvalidAdminRole;
    ///Custom Error type `InvalidTokenMinterBurnerRole` with signature `InvalidTokenMinterBurnerRole()` and selector `0x65b8554b`
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
        name = "InvalidTokenMinterBurnerRole",
        abi = "InvalidTokenMinterBurnerRole()"
    )]
    pub struct InvalidTokenMinterBurnerRole;
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
        InvalidAdminRole(InvalidAdminRole),
        InvalidTokenMinterBurnerRole(InvalidTokenMinterBurnerRole),
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
            if let Ok(decoded) = <InvalidAdminRole as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidAdminRole(decoded));
            }
            if let Ok(decoded) = <InvalidTokenMinterBurnerRole as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidTokenMinterBurnerRole(decoded));
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
                Self::InvalidAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTokenMinterBurnerRole(element) => {
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
                    == <InvalidAdminRole as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTokenMinterBurnerRole as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
                Self::InvalidAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTokenMinterBurnerRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<InvalidAdminRole> for AssetReservesErrors {
        fn from(value: InvalidAdminRole) -> Self {
            Self::InvalidAdminRole(value)
        }
    }
    impl ::core::convert::From<InvalidTokenMinterBurnerRole> for AssetReservesErrors {
        fn from(value: InvalidTokenMinterBurnerRole) -> Self {
            Self::InvalidTokenMinterBurnerRole(value)
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
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
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
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(AssetReservesEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(AssetReservesEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(AssetReservesEvents::RoleRevokedFilter(decoded));
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
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<RoleAdminChangedFilter> for AssetReservesEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for AssetReservesEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for AssetReservesEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<TargetAvailableLiquidityChangedFilter>
    for AssetReservesEvents {
        fn from(value: TargetAvailableLiquidityChangedFilter) -> Self {
            Self::TargetAvailableLiquidityChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `TOKEN_MINTERBURNER_ROLE` function with signature `TOKEN_MINTERBURNER_ROLE()` and selector `0x8e7d53a1`
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
    #[ethcall(name = "TOKEN_MINTERBURNER_ROLE", abi = "TOKEN_MINTERBURNER_ROLE()")]
    pub struct TokenMinterburnerRoleCall;
    ///Container type for all input parameters for the `addTokenMinterBurnerRole` function with signature `addTokenMinterBurnerRole(address)` and selector `0x78a0d597`
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
        name = "addTokenMinterBurnerRole",
        abi = "addTokenMinterBurnerRole(address)"
    )]
    pub struct AddTokenMinterBurnerRoleCall {
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    #[ethcall(name = "getRoleMember", abi = "getRoleMember(bytes32,uint256)")]
    pub struct GetRoleMemberCall {
        pub role: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
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
        DefaultAdminRole(DefaultAdminRoleCall),
        TokenMinterburnerRole(TokenMinterburnerRoleCall),
        AddTokenMinterBurnerRole(AddTokenMinterBurnerRoleCall),
        AddWhiteListedAsset(AddWhiteListedAssetCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IsAssetWhiteListed(IsAssetWhiteListedCall),
        Kai(KaiCall),
        LockOrBurn(LockOrBurnCall),
        MintOrUnlock(MintOrUnlockCall),
        Nexus(NexusCall),
        Owner(OwnerCall),
        Rebalance(RebalanceCall),
        RemoveWhiteListedAsset(RemoveWhiteListedAssetCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SendExcessYield(SendExcessYieldCall),
        SetTargetAvailableLiquidity(SetTargetAvailableLiquidityCall),
        SupportsInterface(SupportsInterfaceCall),
        WhiteListedAssetInfo(WhiteListedAssetInfoCall),
        WhitelistAssetAddParams(WhitelistAssetAddParamsCall),
    }
    impl ::ethers::core::abi::AbiDecode for AssetReservesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <TokenMinterburnerRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenMinterburnerRole(decoded));
            }
            if let Ok(decoded) = <AddTokenMinterBurnerRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddTokenMinterBurnerRole(decoded));
            }
            if let Ok(decoded) = <AddWhiteListedAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddWhiteListedAsset(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMember(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMemberCount(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRole(decoded));
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
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
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
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
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
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenMinterburnerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddTokenMinterBurnerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddWhiteListedAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMember(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMemberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendExcessYield(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTargetAvailableLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
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
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenMinterburnerRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddTokenMinterBurnerRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddWhiteListedAsset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendExcessYield(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTargetAvailableLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhiteListedAssetInfo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WhitelistAssetAddParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for AssetReservesCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<TokenMinterburnerRoleCall> for AssetReservesCalls {
        fn from(value: TokenMinterburnerRoleCall) -> Self {
            Self::TokenMinterburnerRole(value)
        }
    }
    impl ::core::convert::From<AddTokenMinterBurnerRoleCall> for AssetReservesCalls {
        fn from(value: AddTokenMinterBurnerRoleCall) -> Self {
            Self::AddTokenMinterBurnerRole(value)
        }
    }
    impl ::core::convert::From<AddWhiteListedAssetCall> for AssetReservesCalls {
        fn from(value: AddWhiteListedAssetCall) -> Self {
            Self::AddWhiteListedAsset(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for AssetReservesCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall> for AssetReservesCalls {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for AssetReservesCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for AssetReservesCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for AssetReservesCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
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
    impl ::core::convert::From<RenounceRoleCall> for AssetReservesCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for AssetReservesCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
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
    impl ::core::convert::From<SupportsInterfaceCall> for AssetReservesCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
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
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `TOKEN_MINTERBURNER_ROLE` function with signature `TOKEN_MINTERBURNER_ROLE()` and selector `0x8e7d53a1`
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
    pub struct TokenMinterburnerRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    pub struct GetRoleMemberReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    pub struct GetRoleMemberCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
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
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
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
