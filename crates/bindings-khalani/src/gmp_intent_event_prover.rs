pub use gmp_intent_event_prover::*;
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
pub mod gmp_intent_event_prover {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_gmpEventVerifier"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_hyperlaneMailbox"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_hyperlaneIgp"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_destinationChainDomain",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("addEventRegisterer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addEventRegisterer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventRegisterer"),
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
                    ::std::borrow::ToOwned::to_owned("destinationChainDomain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "destinationChainDomain",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("gmpEventVerifierDestination"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "gmpEventVerifierDestination",
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
                    ::std::borrow::ToOwned::to_owned("hyperlaneIgp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hyperlaneIgp"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IInterchainGasPaymaster",
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
                    ::std::borrow::ToOwned::to_owned("hyperlaneMailbox"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hyperlaneMailbox"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IMailbox"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerEvent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("registerSpokeCalledEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerSpokeCalledEvent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_event"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentEventLibrary.SpokeCalled",
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
                    ::std::borrow::ToOwned::to_owned("registerSwapIntentFilledEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerSwapIntentFilledEvent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_event"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentEventLibrary.SwapIntentFilled",
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
                                    name: ::std::borrow::ToOwned::to_owned("_event"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_event"),
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
            ]),
            events: ::core::convert::From::from([
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GMPINTENTEVENTPROVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x14\x168\x03\x80b\0\x14\x16\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x02\x02V[\x83\x83\x83\x83b\0\0E`\x003b\0\0\xA7V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x03\x80T\x94\x86\x16\x94\x90\x91\x16\x93\x90\x93\x17\x90\x92U`\x04\x80Tc\xFF\xFF\xFF\xFF\x90\x93\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16\x91\x90\x93\x16\x17\x17\x90UPb\0\x02i\x92PPPV[b\0\0\xB3\x82\x82b\0\0\xD2V[`\0\x82\x81R`\x01` R`@\x90 b\0\0\xCD\x90\x82b\0\x01sV[PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\x01oW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01.3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\0b\0\x01\x8A\x83`\x01`\x01`\xA0\x1B\x03\x84\x16b\0\x01\x93V[\x90P[\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Tb\0\x01\xDCWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ub\0\x01\x8DV[P`\0b\0\x01\x8DV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xFDW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x02\x19W`\0\x80\xFD[b\0\x02$\x85b\0\x01\xE5V[\x93Pb\0\x024` \x86\x01b\0\x01\xE5V[\x92Pb\0\x02D`@\x86\x01b\0\x01\xE5V[\x91P``\x85\x01Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x02^W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[a\x11\x9D\x80b\0\x02y`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01!W`\x005`\xE0\x1C\x80c~\xB6 \xA7\x11a\0\xADW\x80c\xD5Gt\x1F\x11a\0qW\x80c\xD5Gt\x1F\x14a\x02rW\x80c\xD6k\"\xC8\x14a\x02\x85W\x80c\xDAH5\xC6\x14a\x02\x98W\x80c\xE6\x9B\xC5%\x14a\x02\xABW\x80c\xF2\x8F\x04+\x14a\x02\xD7W`\0\x80\xFD[\x80c~\xB6 \xA7\x14a\x02\x1EW\x80c\x90\x10\xD0|\x14a\x021W\x80c\x91\xD1HT\x14a\x02DW\x80c\xA2\x17\xFD\xDF\x14a\x02WW\x80c\xCA\x15\xC8s\x14a\x02_W`\0\x80\xFD[\x80c//\xF1]\x11a\0\xF4W\x80c//\xF1]\x14a\x01\xBFW\x80c6V\x8A\xBE\x14a\x01\xD2W\x80cI\xD3&L\x14a\x01\xE5W\x80c]\x06\x80\xDC\x14a\x01\xF8W\x80co\xF3\xFE\x95\x14a\x02\x0BW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01&W\x80c\x13\xD8\xB9T\x14a\x01NW\x80c\x1AnM\xB4\x14a\x01cW\x80c$\x8A\x9C\xA3\x14a\x01\x8EW[`\0\x80\xFD[a\x019a\x0146`\x04a\x0B\xF2V[a\x02\xEAV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01aa\x01\\6`\x04a\x0C4V[a\x03\x15V[\0[`\x03Ta\x01v\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01EV[a\x01\xB1a\x01\x9C6`\x04a\x0CPV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01EV[a\x01aa\x01\xCD6`\x04a\x0C\x85V[a\x03=V[a\x01aa\x01\xE06`\x04a\x0C\x85V[a\x03gV[`\x04Ta\x01v\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01aa\x02\x066`\x04a\x0C4V[a\x03\xE6V[a\x01aa\x02\x196`\x04a\x0C\xB1V[a\x03\xFFV[a\x01aa\x02,6`\x04a\x0C\xCCV[a\x047V[a\x01va\x02?6`\x04a\x0C\xDEV[a\x04PV[a\x019a\x02R6`\x04a\x0C\x85V[a\x04oV[a\x01\xB1`\0\x81V[a\x01\xB1a\x02m6`\x04a\x0CPV[a\x04\x98V[a\x01aa\x02\x806`\x04a\x0C\x85V[a\x04\xAFV[a\x01aa\x02\x936`\x04a\x0CPV[a\x04\xD4V[a\x01aa\x02\xA66`\x04a\r\0V[a\x05\xA8V[`\x04Ta\x02\xC2\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01EV[`\x02Ta\x01v\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x03\x0FWPa\x03\x0F\x82a\x05\xBBV[\x92\x91PPV[`\0a\x03.a\x03)6\x84\x90\x03\x84\x01\x84a\r\xEDV[a\x05\xF0V[\x90Pa\x039\x81a\x04\xD4V[PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x03X\x81a\x06=V[a\x03b\x83\x83a\x06GV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x039\x82\x82a\x06iV[`\0a\x03.a\x03\xFA6\x84\x90\x03\x84\x01\x84a\r\xEDV[a\x06\x8BV[a\x04\n`\x003a\x06\xBFV[a\x044\x7F\x13\t\x10=\x1D\xD9\xD9$$I\n\xBF\x1CA\xCE\xF2\x02\xD9E\x9E\x99\x9A7\xBA\xDD\xF4$5\x0BH+$\x82a\x06GV[PV[`\0a\x03.a\x04K6\x84\x90\x03\x84\x01\x84a\x0E\tV[a\x07\x18V[`\0\x82\x81R`\x01` R`@\x81 a\x04h\x90\x83a\x07\x84V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x81\x81R`\x01` R`@\x81 a\x03\x0F\x90a\x07\x90V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x04\xCA\x81a\x06=V[a\x03b\x83\x83a\x06iV[a\x04\xFE\x7F\x13\t\x10=\x1D\xD9\xD9$$I\n\xBF\x1CA\xCE\xF2\x02\xD9E\x9E\x99\x9A7\xBA\xDD\xF4$5\x0BH+$3a\x06\xBFV[`\x03T`\x04T`\x02T`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92c\xFA1\xDE\x01\x92`\x01`\xA0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x16`@\x80Q` \x81\x01\x88\x90R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05e\x93\x92\x91\x90a\x0E\xC4V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03b\x91\x90a\x0E\xF2V[`\0a\x03.a\x05\xB6\x83a\x0F\x0BV[a\x07\x9AV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x03\x0FWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x03\x0FV[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a\x044\x813a\x06\xBFV[a\x06Q\x82\x82a\x07\xC5V[`\0\x82\x81R`\x01` R`@\x90 a\x03b\x90\x82a\x08IV[a\x06s\x82\x82a\x08^V[`\0\x82\x81R`\x01` R`@\x90 a\x03b\x90\x82a\x08\xC3V[\x80Q`@Qr)\xBB\xB0\xB8$\xB7:2\xB7:*7\xB5\xB2\xB7!:\xB97`i\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x06 V[a\x06\xC9\x82\x82a\x04oV[a\x039Wa\x06\xD6\x81a\x08\xD8V[a\x06\xE1\x83` a\x08\xEAV[`@Q` \x01a\x06\xF2\x92\x91\x90a\x0F\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03\xD3\x91`\x04\x01a\x10SV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x06 \x94\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x94\x90\x94R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x84\x01R`D\x83\x01R`d\x82\x01R`\x84\x01\x90V[`\0a\x04h\x83\x83a\n\x86V[`\0a\x03\x0F\x82T\x90V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q\x92Q`\0\x96a\x06 \x96\x90\x95\x94\x91\x01a\x10fV[a\x07\xCF\x82\x82a\x04oV[a\x039W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x08\x053\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x04h\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\n\xB0V[a\x08h\x82\x82a\x04oV[\x15a\x039W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x04h\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\n\xFFV[``a\x03\x0F`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x08\xF9\x83`\x02a\x10\xE7V[a\t\x04\x90`\x02a\x10\xFEV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x1CWa\t\x1Ca\r;V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\tFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\taWa\taa\x11\x11V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\t\x90Wa\t\x90a\x11\x11V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\t\xB4\x84`\x02a\x10\xE7V[a\t\xBF\x90`\x01a\x10\xFEV[\x90P[`\x01\x81\x11\x15a\n7Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\t\xF3Wa\t\xF3a\x11\x11V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\n\tWa\n\ta\x11\x11V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\n0\x81a\x11'V[\x90Pa\t\xC2V[P\x83\x15a\x04hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03\xD3V[`\0\x82`\0\x01\x82\x81T\x81\x10a\n\x9DWa\n\x9Da\x11\x11V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\n\xF7WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x03\x0FV[P`\0a\x03\x0FV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x0B\xE8W`\0a\x0B#`\x01\x83a\x11>V[\x85T\x90\x91P`\0\x90a\x0B7\x90`\x01\x90a\x11>V[\x90P\x81\x81\x14a\x0B\x9CW`\0\x86`\0\x01\x82\x81T\x81\x10a\x0BWWa\x0BWa\x11\x11V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x0BzWa\x0Bza\x11\x11V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x0B\xADWa\x0B\xADa\x11QV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x03\x0FV[`\0\x91PPa\x03\x0FV[`\0` \x82\x84\x03\x12\x15a\x0C\x04W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x04hW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0C.W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0CFW`\0\x80\xFD[a\x04h\x83\x83a\x0C\x1CV[`\0` \x82\x84\x03\x12\x15a\x0CbW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\x80W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x98W`\0\x80\xFD[\x825\x91Pa\x0C\xA8` \x84\x01a\x0CiV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0C\xC3W`\0\x80\xFD[a\x04h\x82a\x0CiV[`\0`\x80\x82\x84\x03\x12\x15a\x0C.W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xF1W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\r\x12W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r)W`\0\x80\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a\x04hW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\rtWa\rta\r;V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\r\xA3Wa\r\xA3a\r;V[`@R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xBDW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\r\xE0Wa\r\xE0a\r;V[`@R\x915\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xFFW`\0\x80\xFD[a\x04h\x83\x83a\r\xABV[`\0`\x80\x82\x84\x03\x12\x15a\x0E\x1BW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0E>Wa\x0E>a\r;V[`@R\x825\x81Ra\x0EQ` \x84\x01a\x0CiV[` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0[\x83\x81\x10\x15a\x0E\x8FW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0EwV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0E\xB0\x81` \x86\x01` \x86\x01a\x0EtV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[c\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x0E\xE9``\x83\x01\x84a\x0E\x98V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\x04W`\0\x80\xFD[PQ\x91\x90PV[`\0`\xA0\x826\x03\x12\x15a\x0F\x1DW`\0\x80\xFD[a\x0F%a\rQV[\x825\x81R` a\x0F6\x81\x85\x01a\x0CiV[\x81\x83\x01R`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FUW`\0\x80\xFD[\x90\x85\x01\x906`\x1F\x83\x01\x12a\x0FhW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0FzWa\x0Fza\r;V[a\x0F\x8C`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\rzV[\x91P\x80\x82R6\x84\x82\x85\x01\x01\x11\x15a\x0F\xA2W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80`@\x85\x01RPPPa\x0F\xC8``\x84\x01a\x0CiV[``\x82\x01R`\x80\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x10\x16\x81`\x17\x85\x01` \x88\x01a\x0EtV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x10G\x81`(\x84\x01` \x88\x01a\x0EtV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0a\x04h` \x83\x01\x84a\x0E\x98V[j\x14\xDC\x1B\xDA\xD9P\xD8[\x1B\x19Y`\xAA\x1B\x81R\x85`\x0B\x82\x01R`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x87``\x1B\x16`+\x84\x01R\x85Qa\x10\xAB\x81`?\x86\x01` \x8A\x01a\x0EtV[``\x95\x90\x95\x1B\x16\x91\x90\x93\x01`?\x81\x01\x91\x90\x91R`S\x81\x01\x91\x90\x91R`s\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x0FWa\x03\x0Fa\x10\xD1V[\x80\x82\x01\x80\x82\x11\x15a\x03\x0FWa\x03\x0Fa\x10\xD1V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x116Wa\x116a\x10\xD1V[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x03\x0FWa\x03\x0Fa\x10\xD1V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xC3\x85:\"\xC7t`6\xCB>82\xDE\xF6\x8B\x87\xDB?\x16\xFE%01\x7F\x87\xE0K\x84\x94E,edsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static GMPINTENTEVENTPROVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01!W`\x005`\xE0\x1C\x80c~\xB6 \xA7\x11a\0\xADW\x80c\xD5Gt\x1F\x11a\0qW\x80c\xD5Gt\x1F\x14a\x02rW\x80c\xD6k\"\xC8\x14a\x02\x85W\x80c\xDAH5\xC6\x14a\x02\x98W\x80c\xE6\x9B\xC5%\x14a\x02\xABW\x80c\xF2\x8F\x04+\x14a\x02\xD7W`\0\x80\xFD[\x80c~\xB6 \xA7\x14a\x02\x1EW\x80c\x90\x10\xD0|\x14a\x021W\x80c\x91\xD1HT\x14a\x02DW\x80c\xA2\x17\xFD\xDF\x14a\x02WW\x80c\xCA\x15\xC8s\x14a\x02_W`\0\x80\xFD[\x80c//\xF1]\x11a\0\xF4W\x80c//\xF1]\x14a\x01\xBFW\x80c6V\x8A\xBE\x14a\x01\xD2W\x80cI\xD3&L\x14a\x01\xE5W\x80c]\x06\x80\xDC\x14a\x01\xF8W\x80co\xF3\xFE\x95\x14a\x02\x0BW`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01&W\x80c\x13\xD8\xB9T\x14a\x01NW\x80c\x1AnM\xB4\x14a\x01cW\x80c$\x8A\x9C\xA3\x14a\x01\x8EW[`\0\x80\xFD[a\x019a\x0146`\x04a\x0B\xF2V[a\x02\xEAV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01aa\x01\\6`\x04a\x0C4V[a\x03\x15V[\0[`\x03Ta\x01v\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01EV[a\x01\xB1a\x01\x9C6`\x04a\x0CPV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01EV[a\x01aa\x01\xCD6`\x04a\x0C\x85V[a\x03=V[a\x01aa\x01\xE06`\x04a\x0C\x85V[a\x03gV[`\x04Ta\x01v\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01aa\x02\x066`\x04a\x0C4V[a\x03\xE6V[a\x01aa\x02\x196`\x04a\x0C\xB1V[a\x03\xFFV[a\x01aa\x02,6`\x04a\x0C\xCCV[a\x047V[a\x01va\x02?6`\x04a\x0C\xDEV[a\x04PV[a\x019a\x02R6`\x04a\x0C\x85V[a\x04oV[a\x01\xB1`\0\x81V[a\x01\xB1a\x02m6`\x04a\x0CPV[a\x04\x98V[a\x01aa\x02\x806`\x04a\x0C\x85V[a\x04\xAFV[a\x01aa\x02\x936`\x04a\x0CPV[a\x04\xD4V[a\x01aa\x02\xA66`\x04a\r\0V[a\x05\xA8V[`\x04Ta\x02\xC2\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01EV[`\x02Ta\x01v\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x03\x0FWPa\x03\x0F\x82a\x05\xBBV[\x92\x91PPV[`\0a\x03.a\x03)6\x84\x90\x03\x84\x01\x84a\r\xEDV[a\x05\xF0V[\x90Pa\x039\x81a\x04\xD4V[PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x03X\x81a\x06=V[a\x03b\x83\x83a\x06GV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x039\x82\x82a\x06iV[`\0a\x03.a\x03\xFA6\x84\x90\x03\x84\x01\x84a\r\xEDV[a\x06\x8BV[a\x04\n`\x003a\x06\xBFV[a\x044\x7F\x13\t\x10=\x1D\xD9\xD9$$I\n\xBF\x1CA\xCE\xF2\x02\xD9E\x9E\x99\x9A7\xBA\xDD\xF4$5\x0BH+$\x82a\x06GV[PV[`\0a\x03.a\x04K6\x84\x90\x03\x84\x01\x84a\x0E\tV[a\x07\x18V[`\0\x82\x81R`\x01` R`@\x81 a\x04h\x90\x83a\x07\x84V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x81\x81R`\x01` R`@\x81 a\x03\x0F\x90a\x07\x90V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x04\xCA\x81a\x06=V[a\x03b\x83\x83a\x06iV[a\x04\xFE\x7F\x13\t\x10=\x1D\xD9\xD9$$I\n\xBF\x1CA\xCE\xF2\x02\xD9E\x9E\x99\x9A7\xBA\xDD\xF4$5\x0BH+$3a\x06\xBFV[`\x03T`\x04T`\x02T`\0\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92c\xFA1\xDE\x01\x92`\x01`\xA0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x16`@\x80Q` \x81\x01\x88\x90R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05e\x93\x92\x91\x90a\x0E\xC4V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03b\x91\x90a\x0E\xF2V[`\0a\x03.a\x05\xB6\x83a\x0F\x0BV[a\x07\x9AV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x03\x0FWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x03\x0FV[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a\x044\x813a\x06\xBFV[a\x06Q\x82\x82a\x07\xC5V[`\0\x82\x81R`\x01` R`@\x90 a\x03b\x90\x82a\x08IV[a\x06s\x82\x82a\x08^V[`\0\x82\x81R`\x01` R`@\x90 a\x03b\x90\x82a\x08\xC3V[\x80Q`@Qr)\xBB\xB0\xB8$\xB7:2\xB7:*7\xB5\xB2\xB7!:\xB97`i\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x06 V[a\x06\xC9\x82\x82a\x04oV[a\x039Wa\x06\xD6\x81a\x08\xD8V[a\x06\xE1\x83` a\x08\xEAV[`@Q` \x01a\x06\xF2\x92\x91\x90a\x0F\xDEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03\xD3\x91`\x04\x01a\x10SV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x06 \x94\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x94\x90\x94R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x84\x01R`D\x83\x01R`d\x82\x01R`\x84\x01\x90V[`\0a\x04h\x83\x83a\n\x86V[`\0a\x03\x0F\x82T\x90V[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q\x92Q`\0\x96a\x06 \x96\x90\x95\x94\x91\x01a\x10fV[a\x07\xCF\x82\x82a\x04oV[a\x039W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x08\x053\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x04h\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\n\xB0V[a\x08h\x82\x82a\x04oV[\x15a\x039W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x04h\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\n\xFFV[``a\x03\x0F`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x08\xF9\x83`\x02a\x10\xE7V[a\t\x04\x90`\x02a\x10\xFEV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x1CWa\t\x1Ca\r;V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\tFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\taWa\taa\x11\x11V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\t\x90Wa\t\x90a\x11\x11V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\t\xB4\x84`\x02a\x10\xE7V[a\t\xBF\x90`\x01a\x10\xFEV[\x90P[`\x01\x81\x11\x15a\n7Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\t\xF3Wa\t\xF3a\x11\x11V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\n\tWa\n\ta\x11\x11V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\n0\x81a\x11'V[\x90Pa\t\xC2V[P\x83\x15a\x04hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03\xD3V[`\0\x82`\0\x01\x82\x81T\x81\x10a\n\x9DWa\n\x9Da\x11\x11V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\n\xF7WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x03\x0FV[P`\0a\x03\x0FV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x0B\xE8W`\0a\x0B#`\x01\x83a\x11>V[\x85T\x90\x91P`\0\x90a\x0B7\x90`\x01\x90a\x11>V[\x90P\x81\x81\x14a\x0B\x9CW`\0\x86`\0\x01\x82\x81T\x81\x10a\x0BWWa\x0BWa\x11\x11V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x0BzWa\x0Bza\x11\x11V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x0B\xADWa\x0B\xADa\x11QV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x03\x0FV[`\0\x91PPa\x03\x0FV[`\0` \x82\x84\x03\x12\x15a\x0C\x04W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x04hW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0C.W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0CFW`\0\x80\xFD[a\x04h\x83\x83a\x0C\x1CV[`\0` \x82\x84\x03\x12\x15a\x0CbW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\x80W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\x98W`\0\x80\xFD[\x825\x91Pa\x0C\xA8` \x84\x01a\x0CiV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0C\xC3W`\0\x80\xFD[a\x04h\x82a\x0CiV[`\0`\x80\x82\x84\x03\x12\x15a\x0C.W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xF1W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a\r\x12W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r)W`\0\x80\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a\x04hW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\rtWa\rta\r;V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\r\xA3Wa\r\xA3a\r;V[`@R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xBDW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\r\xE0Wa\r\xE0a\r;V[`@R\x915\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\xFFW`\0\x80\xFD[a\x04h\x83\x83a\r\xABV[`\0`\x80\x82\x84\x03\x12\x15a\x0E\x1BW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0E>Wa\x0E>a\r;V[`@R\x825\x81Ra\x0EQ` \x84\x01a\x0CiV[` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0[\x83\x81\x10\x15a\x0E\x8FW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0EwV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0E\xB0\x81` \x86\x01` \x86\x01a\x0EtV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[c\xFF\xFF\xFF\xFF\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x0E\xE9``\x83\x01\x84a\x0E\x98V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\x04W`\0\x80\xFD[PQ\x91\x90PV[`\0`\xA0\x826\x03\x12\x15a\x0F\x1DW`\0\x80\xFD[a\x0F%a\rQV[\x825\x81R` a\x0F6\x81\x85\x01a\x0CiV[\x81\x83\x01R`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FUW`\0\x80\xFD[\x90\x85\x01\x906`\x1F\x83\x01\x12a\x0FhW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0FzWa\x0Fza\r;V[a\x0F\x8C`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\rzV[\x91P\x80\x82R6\x84\x82\x85\x01\x01\x11\x15a\x0F\xA2W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80`@\x85\x01RPPPa\x0F\xC8``\x84\x01a\x0CiV[``\x82\x01R`\x80\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x10\x16\x81`\x17\x85\x01` \x88\x01a\x0EtV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x10G\x81`(\x84\x01` \x88\x01a\x0EtV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0a\x04h` \x83\x01\x84a\x0E\x98V[j\x14\xDC\x1B\xDA\xD9P\xD8[\x1B\x19Y`\xAA\x1B\x81R\x85`\x0B\x82\x01R`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x87``\x1B\x16`+\x84\x01R\x85Qa\x10\xAB\x81`?\x86\x01` \x8A\x01a\x0EtV[``\x95\x90\x95\x1B\x16\x91\x90\x93\x01`?\x81\x01\x91\x90\x91R`S\x81\x01\x91\x90\x91R`s\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x0FWa\x03\x0Fa\x10\xD1V[\x80\x82\x01\x80\x82\x11\x15a\x03\x0FWa\x03\x0Fa\x10\xD1V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a\x116Wa\x116a\x10\xD1V[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x03\x0FWa\x03\x0Fa\x10\xD1V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xC3\x85:\"\xC7t`6\xCB>82\xDE\xF6\x8B\x87\xDB?\x16\xFE%01\x7F\x87\xE0K\x84\x94E,edsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static GMPINTENTEVENTPROVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GMPIntentEventProver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GMPIntentEventProver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GMPIntentEventProver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GMPIntentEventProver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GMPIntentEventProver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GMPIntentEventProver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GMPIntentEventProver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GMPINTENTEVENTPROVER_ABI.clone(),
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
                GMPINTENTEVENTPROVER_ABI.clone(),
                GMPINTENTEVENTPROVER_BYTECODE.clone().into(),
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
        ///Calls the contract's `addEventRegisterer` (0x6ff3fe95) function
        pub fn add_event_registerer(
            &self,
            event_registerer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 243, 254, 149], event_registerer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `destinationChainDomain` (0xe69bc525) function
        pub fn destination_chain_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([230, 155, 197, 37], ())
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
        ///Calls the contract's `gmpEventVerifierDestination` (0xf28f042b) function
        pub fn gmp_event_verifier_destination(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([242, 143, 4, 43], ())
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
        ///Calls the contract's `hyperlaneIgp` (0x49d3264c) function
        pub fn hyperlane_igp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([73, 211, 38, 76], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hyperlaneMailbox` (0x1a6e4db4) function
        pub fn hyperlane_mailbox(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([26, 110, 77, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerEvent` (0xd66b22c8) function
        pub fn register_event(
            &self,
            event_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 107, 34, 200], event_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerSpokeCalledEvent` (0xda4835c6) function
        pub fn register_spoke_called_event(
            &self,
            event: SpokeCalled,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 72, 53, 198], (event,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerSwapIntentFilledEvent` (0x7eb620a7) function
        pub fn register_swap_intent_filled_event(
            &self,
            event: SwapIntentFilled,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 182, 32, 167], (event,))
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
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GMPIntentEventProverEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GMPIntentEventProver<M> {
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
    pub enum GMPIntentEventProverEvents {
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for GMPIntentEventProverEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(GMPIntentEventProverEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(GMPIntentEventProverEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(GMPIntentEventProverEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GMPIntentEventProverEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for GMPIntentEventProverEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for GMPIntentEventProverEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for GMPIntentEventProverEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
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
    ///Container type for all input parameters for the `addEventRegisterer` function with signature `addEventRegisterer(address)` and selector `0x6ff3fe95`
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
    #[ethcall(name = "addEventRegisterer", abi = "addEventRegisterer(address)")]
    pub struct AddEventRegistererCall {
        pub event_registerer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `destinationChainDomain` function with signature `destinationChainDomain()` and selector `0xe69bc525`
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
    #[ethcall(name = "destinationChainDomain", abi = "destinationChainDomain()")]
    pub struct DestinationChainDomainCall;
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
    ///Container type for all input parameters for the `gmpEventVerifierDestination` function with signature `gmpEventVerifierDestination()` and selector `0xf28f042b`
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
        name = "gmpEventVerifierDestination",
        abi = "gmpEventVerifierDestination()"
    )]
    pub struct GmpEventVerifierDestinationCall;
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
    ///Container type for all input parameters for the `hyperlaneIgp` function with signature `hyperlaneIgp()` and selector `0x49d3264c`
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
    #[ethcall(name = "hyperlaneIgp", abi = "hyperlaneIgp()")]
    pub struct HyperlaneIgpCall;
    ///Container type for all input parameters for the `hyperlaneMailbox` function with signature `hyperlaneMailbox()` and selector `0x1a6e4db4`
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
    #[ethcall(name = "hyperlaneMailbox", abi = "hyperlaneMailbox()")]
    pub struct HyperlaneMailboxCall;
    ///Container type for all input parameters for the `registerEvent` function with signature `registerEvent(bytes32)` and selector `0xd66b22c8`
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
    #[ethcall(name = "registerEvent", abi = "registerEvent(bytes32)")]
    pub struct RegisterEventCall {
        pub event_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `registerSpokeCalledEvent` function with signature `registerSpokeCalledEvent((bytes32,address,bytes,address,uint256))` and selector `0xda4835c6`
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
        name = "registerSpokeCalledEvent",
        abi = "registerSpokeCalledEvent((bytes32,address,bytes,address,uint256))"
    )]
    pub struct RegisterSpokeCalledEventCall {
        pub event: SpokeCalled,
    }
    ///Container type for all input parameters for the `registerSwapIntentFilledEvent` function with signature `registerSwapIntentFilledEvent((bytes32,address,uint256,uint256))` and selector `0x7eb620a7`
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
        name = "registerSwapIntentFilledEvent",
        abi = "registerSwapIntentFilledEvent((bytes32,address,uint256,uint256))"
    )]
    pub struct RegisterSwapIntentFilledEventCall {
        pub event: SwapIntentFilled,
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
    pub enum GMPIntentEventProverCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        AddEventRegisterer(AddEventRegistererCall),
        DestinationChainDomain(DestinationChainDomainCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GmpEventVerifierDestination(GmpEventVerifierDestinationCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        HyperlaneIgp(HyperlaneIgpCall),
        HyperlaneMailbox(HyperlaneMailboxCall),
        RegisterEvent(RegisterEventCall),
        RegisterSpokeCalledEvent(RegisterSpokeCalledEventCall),
        RegisterSwapIntentFilledEvent(RegisterSwapIntentFilledEventCall),
        RegisterSwapIntentTokenBurnEvent(RegisterSwapIntentTokenBurnEventCall),
        RegisterSwapIntentTokenLockEvent(RegisterSwapIntentTokenLockEventCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for GMPIntentEventProverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <AddEventRegistererCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddEventRegisterer(decoded));
            }
            if let Ok(decoded) = <DestinationChainDomainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DestinationChainDomain(decoded));
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
            if let Ok(decoded) = <GmpEventVerifierDestinationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GmpEventVerifierDestination(decoded));
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
            if let Ok(decoded) = <HyperlaneIgpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HyperlaneIgp(decoded));
            }
            if let Ok(decoded) = <HyperlaneMailboxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HyperlaneMailbox(decoded));
            }
            if let Ok(decoded) = <RegisterEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterEvent(decoded));
            }
            if let Ok(decoded) = <RegisterSpokeCalledEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterSpokeCalledEvent(decoded));
            }
            if let Ok(decoded) = <RegisterSwapIntentFilledEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterSwapIntentFilledEvent(decoded));
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
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GMPIntentEventProverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddEventRegisterer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DestinationChainDomain(element) => {
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
                Self::GmpEventVerifierDestination(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HyperlaneIgp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HyperlaneMailbox(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterSpokeCalledEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterSwapIntentFilledEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterSwapIntentTokenBurnEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterSwapIntentTokenLockEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GMPIntentEventProverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddEventRegisterer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DestinationChainDomain(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GmpEventVerifierDestination(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HyperlaneIgp(element) => ::core::fmt::Display::fmt(element, f),
                Self::HyperlaneMailbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterEvent(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterSpokeCalledEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterSwapIntentFilledEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterSwapIntentTokenBurnEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterSwapIntentTokenLockEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for GMPIntentEventProverCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<AddEventRegistererCall> for GMPIntentEventProverCalls {
        fn from(value: AddEventRegistererCall) -> Self {
            Self::AddEventRegisterer(value)
        }
    }
    impl ::core::convert::From<DestinationChainDomainCall>
    for GMPIntentEventProverCalls {
        fn from(value: DestinationChainDomainCall) -> Self {
            Self::DestinationChainDomain(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for GMPIntentEventProverCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall> for GMPIntentEventProverCalls {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for GMPIntentEventProverCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GmpEventVerifierDestinationCall>
    for GMPIntentEventProverCalls {
        fn from(value: GmpEventVerifierDestinationCall) -> Self {
            Self::GmpEventVerifierDestination(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for GMPIntentEventProverCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for GMPIntentEventProverCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<HyperlaneIgpCall> for GMPIntentEventProverCalls {
        fn from(value: HyperlaneIgpCall) -> Self {
            Self::HyperlaneIgp(value)
        }
    }
    impl ::core::convert::From<HyperlaneMailboxCall> for GMPIntentEventProverCalls {
        fn from(value: HyperlaneMailboxCall) -> Self {
            Self::HyperlaneMailbox(value)
        }
    }
    impl ::core::convert::From<RegisterEventCall> for GMPIntentEventProverCalls {
        fn from(value: RegisterEventCall) -> Self {
            Self::RegisterEvent(value)
        }
    }
    impl ::core::convert::From<RegisterSpokeCalledEventCall>
    for GMPIntentEventProverCalls {
        fn from(value: RegisterSpokeCalledEventCall) -> Self {
            Self::RegisterSpokeCalledEvent(value)
        }
    }
    impl ::core::convert::From<RegisterSwapIntentFilledEventCall>
    for GMPIntentEventProverCalls {
        fn from(value: RegisterSwapIntentFilledEventCall) -> Self {
            Self::RegisterSwapIntentFilledEvent(value)
        }
    }
    impl ::core::convert::From<RegisterSwapIntentTokenBurnEventCall>
    for GMPIntentEventProverCalls {
        fn from(value: RegisterSwapIntentTokenBurnEventCall) -> Self {
            Self::RegisterSwapIntentTokenBurnEvent(value)
        }
    }
    impl ::core::convert::From<RegisterSwapIntentTokenLockEventCall>
    for GMPIntentEventProverCalls {
        fn from(value: RegisterSwapIntentTokenLockEventCall) -> Self {
            Self::RegisterSwapIntentTokenLockEvent(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for GMPIntentEventProverCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for GMPIntentEventProverCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for GMPIntentEventProverCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
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
    ///Container type for all return fields from the `destinationChainDomain` function with signature `destinationChainDomain()` and selector `0xe69bc525`
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
    pub struct DestinationChainDomainReturn(pub u32);
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
    ///Container type for all return fields from the `gmpEventVerifierDestination` function with signature `gmpEventVerifierDestination()` and selector `0xf28f042b`
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
    pub struct GmpEventVerifierDestinationReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `hyperlaneIgp` function with signature `hyperlaneIgp()` and selector `0x49d3264c`
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
    pub struct HyperlaneIgpReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `hyperlaneMailbox` function with signature `hyperlaneMailbox()` and selector `0x1a6e4db4`
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
    pub struct HyperlaneMailboxReturn(pub ::ethers::core::types::Address);
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
    ///`SpokeCalled(bytes32,address,bytes,address,uint256)`
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
    pub struct SpokeCalled {
        pub spoke_chain_call_intent_id: [u8; 32],
        pub contract_to_call: ::ethers::core::types::Address,
        pub call_data: ::ethers::core::types::Bytes,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
}
