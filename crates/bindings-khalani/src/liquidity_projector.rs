pub use liquidity_projector::*;
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
pub mod liquidity_projector {
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
                    ::std::borrow::ToOwned::to_owned("lockOrBurn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lockOrBurn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("setMirrorToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setMirrorToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mirrorToken"),
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
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
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
                    ::std::borrow::ToOwned::to_owned("MirrorTokenSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MirrorTokenSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
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
    pub static LIQUIDITYPROJECTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x19O8\x03\x80b\0\x19O\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\xFDV[3`\xC0R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x80R\x81\x16`\xA0Rb\0\0_`\0b\0\0Y3\x90V[b\0\0\x93V[b\0\0\x8B\x7F\x91\xC0\xA0\xC8`\xF8B\xE6\x1E\xA5\xD1\xB9\xBB\xF1\xCB\xACBX\xB1\xEC\x93\xED\x9D:\"\xF1\xF5y\xF7\x91\x83M\x83b\0\0\x93V[PPb\0\x025V[b\0\0\x9F\x82\x82b\0\0\xA3V[PPV[b\0\0\xAF\x82\x82b\0\0\xCEV[`\0\x82\x81R`\x01` R`@\x90 b\0\0\xC9\x90\x82b\0\x01nV[PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\0\x9FW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01*3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0b\0\x01\x85\x83`\x01`\x01`\xA0\x1B\x03\x84\x16b\0\x01\x8EV[\x90P[\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Tb\0\x01\xD7WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ub\0\x01\x88V[P`\0b\0\x01\x88V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xF8W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x02\x11W`\0\x80\xFD[b\0\x02\x1C\x83b\0\x01\xE0V[\x91Pb\0\x02,` \x84\x01b\0\x01\xE0V[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Qa\x16\xD5b\0\x02z`\09`\0\x81\x81a\x01\xBC\x01Ra\x04\x06\x01R`\0\x81\x81a\x02\xBE\x01R\x81\x81a\x05\x99\x01Ra\x08\xD2\x01R`\0a\x02>\x01Ra\x16\xD5`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x0BW`\x005`\xE0\x1C\x80c\x90\x10\xD0|\x11a\0\xA2W\x80c\xB0bQ\xCA\x11a\0qW\x80c\xB0bQ\xCA\x14a\x02`W\x80c\xC2\x15PG\x14a\x02\x80W\x80c\xCA\x15\xC8s\x14a\x02\x93W\x80c\xD5Gt\x1F\x14a\x02\xA6W\x80c\xE8\xA2\xB1j\x14a\x02\xB9W`\0\x80\xFD[\x80c\x90\x10\xD0|\x14a\x02\x0BW\x80c\x91\xD1HT\x14a\x02\x1EW\x80c\xA2\x17\xFD\xDF\x14a\x021W\x80c\xA3\xF5\xC1\xD2\x14a\x029W`\0\x80\xFD[\x80cx\xA0\xD5\x97\x11a\0\xDEW\x80cx\xA0\xD5\x97\x14a\x01\x91W\x80c\x8CJ\xF1)\x14a\x01\xA4W\x80c\x8D\xA5\xCB[\x14a\x01\xB7W\x80c\x8E}S\xA1\x14a\x01\xF6W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x10W\x80c$\x8A\x9C\xA3\x14a\x018W\x80c//\xF1]\x14a\x01iW\x80c6V\x8A\xBE\x14a\x01~W[`\0\x80\xFD[a\x01#a\x01\x1E6`\x04a\x12$V[a\x02\xE0V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01[a\x01F6`\x04a\x12NV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01/V[a\x01|a\x01w6`\x04a\x12\x83V[a\x03\x0BV[\0[a\x01|a\x01\x8C6`\x04a\x12\x83V[a\x035V[a\x01|a\x01\x9F6`\x04a\x12\xAFV[a\x03\xB8V[a\x01|a\x01\xB26`\x04a\x12\xCAV[a\x03\xFBV[a\x01\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01/V[a\x01[`\0\x80Q` a\x16\x80\x839\x81Q\x91R\x81V[a\x01\xDEa\x02\x196`\x04a\x13\x06V[a\x04\xCBV[a\x01#a\x02,6`\x04a\x12\x83V[a\x04\xEAV[a\x01[`\0\x81V[a\x01\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02sa\x02n6`\x04a\x13\x98V[a\x05\x13V[`@Qa\x01/\x91\x90a\x14\x87V[a\x02sa\x02\x8E6`\x04a\x13\x98V[a\x07\x89V[a\x01[a\x02\xA16`\x04a\x12NV[a\t\xC3V[a\x01|a\x02\xB46`\x04a\x12\x83V[a\t\xDAV[a\x01\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x03\x05WPa\x03\x05\x82a\t\xFFV[\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x03&\x81a\n4V[a\x030\x83\x83a\n>V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\xB4\x82\x82a\n`V[PPV[a\x03\xC3`\x003a\x04\xEAV[a\x03\xE0W`@Qc-M{\xB3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xF8`\0\x80Q` a\x16\x80\x839\x81Q\x91R\x82a\x03\x0BV[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04DW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x80\x86R\x91\x84R\x82\x85 \x80T\x91\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x81\x17\x90\x91U\x88\x86R`\x03\x85R\x83\x86 \x81\x87R\x90\x94R\x82\x85 \x80T\x90\x91\x16\x82\x17\x90U\x90Q\x91\x92\x90\x91\x86\x91\x7F\xDD\x1Eo\xEFi\xB2\xF4\0\x03J\xF4\xD8\x19>it\xC5\x91f\x8D9G\xD3\xE5\xE1H+\xE6\xF4\xCE\xBA\xAA\x91\xA4PPPV[`\0\x82\x81R`\x01` R`@\x81 a\x04\xE3\x90\x83a\n\x82V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[``a\x05-`\0\x80Q` a\x16\x80\x839\x81Q\x91R3a\x04\xEAV[a\x05JW`@Qce\xB8UK`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7Fv\n5\x8DH\n\xCDb\xE3\t\xA72K\xC2\x89n\xBA\xC8E\xFB\x1B.\xBE\xC73\x9Eg\x0B\xDC\x17A\xA5\x83`@Qa\x05\x83\x91\x90a\x14\x87V[`@Q\x80\x91\x03\x90\xA2`\0[\x82Q\x81\x10\x15a\x07\x80W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x05\xD3Wa\x05\xD3a\x14\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x068Wa\x063\x83\x82\x81Q\x81\x10a\x06\x02Wa\x06\x02a\x14\xDFV[` \x02` \x01\x01Q`\0\x01Q\x850\x86\x85\x81Q\x81\x10a\x06\"Wa\x06\"a\x14\xDFV[` \x02` \x01\x01Q` \x01Qa\n\x8EV[a\x06\xE7V[\x82\x81\x81Q\x81\x10a\x06JWa\x06Ja\x14\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16cy\xCCg\x90\x85\x85\x84\x81Q\x81\x10a\x06wWa\x06wa\x14\xDFV[` \x02` \x01\x01Q` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xB4\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xCEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xE2W=`\0\x80>=`\0\xFD[PPPP[`\x03`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x84\x83\x81Q\x81\x10a\x07\x0EWa\x07\x0Ea\x14\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x07`Wa\x07`a\x14\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90R`\x01\x01a\x05\x8EV[P\x90\x93\x92PPPV[``a\x07\xA3`\0\x80Q` a\x16\x80\x839\x81Q\x91R3a\x04\xEAV[a\x07\xC0W`@Qce\xB8UK`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7FN8A\xFB\x84\x88\xF53\xA2wg\xAB\x10\x84\xC2\xBF\x87\x05\xFF\xF4\xBE\x10Z)o\xF1%&\x93\r\xE2\x19\x84`@Qa\x07\xFA\x91\x90a\x14\x87V[`@Q\x80\x91\x03\x90\xA3`\0[\x82Q\x81\x10\x15a\x07\x80W`\0\x85\x81R`\x02` R`@\x81 \x84Q\x82\x90\x86\x90\x85\x90\x81\x10a\x082Wa\x082a\x14\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x90\x82\x01\x92\x90\x92R`@\x01`\0 T\x16\x90P\x80a\x08\xA5W\x83\x82\x81Q\x81\x10a\x08tWa\x08ta\x14\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qc\x06|x\x7F`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x03\xA1V[\x80\x84\x83\x81Q\x81\x10a\x08\xB8Wa\x08\xB8a\x14\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x03a\t(Wa\t#\x81\x86\x86\x85\x81Q\x81\x10a\t\x12Wa\t\x12a\x14\xDFV[` \x02` \x01\x01Q` \x01Qa\n\xFFV[a\t\xBAV[\x80`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x19\x86\x86\x85\x81Q\x81\x10a\tJWa\tJa\x14\xDFV[` \x02` \x01\x01Q` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x87\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xA1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xB5W=`\0\x80>=`\0\xFD[PPPP[P`\x01\x01a\x08\x05V[`\0\x81\x81R`\x01` R`@\x81 a\x03\x05\x90a\x0B/V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\t\xF5\x81a\n4V[a\x030\x83\x83a\n`V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x03\x05WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x03\x05V[a\x03\xF8\x813a\x0B9V[a\nH\x82\x82a\x0B\x92V[`\0\x82\x81R`\x01` R`@\x90 a\x030\x90\x82a\x0C\x16V[a\nj\x82\x82a\x0C+V[`\0\x82\x81R`\x01` R`@\x90 a\x030\x90\x82a\x0C\x90V[`\0a\x04\xE3\x83\x83a\x0C\xA5V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\n\xF9\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x0C\xCFV[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x030\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\n\xC2V[`\0a\x03\x05\x82T\x90V[a\x0BC\x82\x82a\x04\xEAV[a\x03\xB4Wa\x0BP\x81a\r\xA4V[a\x0B[\x83` a\r\xB6V[`@Q` \x01a\x0Bl\x92\x91\x90a\x15\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03\xA1\x91`\x04\x01a\x15\x8EV[a\x0B\x9C\x82\x82a\x04\xEAV[a\x03\xB4W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0B\xD23\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x04\xE3\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x0FRV[a\x0C5\x82\x82a\x04\xEAV[\x15a\x03\xB4W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x04\xE3\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x0F\xA1V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x0C\xBCWa\x0C\xBCa\x14\xDFV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0a\r$\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x10\x94\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\rEWP\x80\x80` \x01\x90Q\x81\x01\x90a\rE\x91\x90a\x15\xC1V[a\x030W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x03\xA1V[``a\x03\x05`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\r\xC5\x83`\x02a\x15\xF9V[a\r\xD0\x90`\x02a\x16\x10V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xE8Wa\r\xE8a\x13(V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\x12W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x0E-Wa\x0E-a\x14\xDFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0E\\Wa\x0E\\a\x14\xDFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0E\x80\x84`\x02a\x15\xF9V[a\x0E\x8B\x90`\x01a\x16\x10V[\x90P[`\x01\x81\x11\x15a\x0F\x03Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0E\xBFWa\x0E\xBFa\x14\xDFV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0E\xD5Wa\x0E\xD5a\x14\xDFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0E\xFC\x81a\x16#V[\x90Pa\x0E\x8EV[P\x83\x15a\x04\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03\xA1V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x0F\x99WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x03\x05V[P`\0a\x03\x05V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x10\x8AW`\0a\x0F\xC5`\x01\x83a\x16:V[\x85T\x90\x91P`\0\x90a\x0F\xD9\x90`\x01\x90a\x16:V[\x90P\x81\x81\x14a\x10>W`\0\x86`\0\x01\x82\x81T\x81\x10a\x0F\xF9Wa\x0F\xF9a\x14\xDFV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x10\x1CWa\x10\x1Ca\x14\xDFV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x10OWa\x10Oa\x16MV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x03\x05V[`\0\x91PPa\x03\x05V[``a\x10\xA3\x84\x84`\0\x85a\x10\xABV[\x94\x93PPPPV[``\x82G\x10\x15a\x11\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x03\xA1V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x11(\x91\x90a\x16cV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x11eW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11jV[``\x91P[P\x91P\x91Pa\x11{\x87\x83\x83\x87a\x11\x86V[\x97\x96PPPPPPPV[``\x83\x15a\x11\xF5W\x82Q`\0\x03a\x11\xEEW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x11\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x03\xA1V[P\x81a\x10\xA3V[a\x10\xA3\x83\x83\x81Q\x15a\x12\nW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xA1\x91\x90a\x15\x8EV[`\0` \x82\x84\x03\x12\x15a\x126W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x04\xE3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12`W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12~W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x12\x96W`\0\x80\xFD[\x825\x91Pa\x12\xA6` \x84\x01a\x12gV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x12\xC1W`\0\x80\xFD[a\x04\xE3\x82a\x12gV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xDFW`\0\x80\xFD[\x835\x92Pa\x12\xEF` \x85\x01a\x12gV[\x91Pa\x12\xFD`@\x85\x01a\x12gV[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x13\x19W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13aWa\x13aa\x13(V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13\x90Wa\x13\x90a\x13(V[`@R\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\xADW`\0\x80\xFD[\x835\x92P` a\x13\xBE\x81\x86\x01a\x12gV[\x92P`@\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13\xDCW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x13\xF0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x14\x02Wa\x14\x02a\x13(V[a\x14\x10\x85\x82`\x05\x1B\x01a\x13gV[\x81\x81R\x85\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x85\x01\x90\x8A\x82\x11\x15a\x140W`\0\x80\xFD[\x92\x85\x01\x92[\x81\x84\x10\x15a\x14wW\x84\x84\x8C\x03\x12\x15a\x14MW`\0\x80\x81\xFD[a\x14Ua\x13>V[a\x14^\x85a\x12gV[\x81R\x84\x87\x015\x87\x82\x01R\x83R\x92\x84\x01\x92\x91\x85\x01\x91a\x145V[\x80\x96PPPPPPP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x14\xD2W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x14\xA4V[P\x91\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x15\x10W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14\xF8V[PP`\0\x91\x01RV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x15Q\x81`\x17\x85\x01` \x88\x01a\x14\xF5V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x15\x82\x81`(\x84\x01` \x88\x01a\x14\xF5V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x15\xAD\x81`@\x85\x01` \x87\x01a\x14\xF5V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\xD3W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x04\xE3W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x05Wa\x03\x05a\x15\xE3V[\x80\x82\x01\x80\x82\x11\x15a\x03\x05Wa\x03\x05a\x15\xE3V[`\0\x81a\x162Wa\x162a\x15\xE3V[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x03\x05Wa\x03\x05a\x15\xE3V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82Qa\x16u\x81\x84` \x87\x01a\x14\xF5V[\x91\x90\x91\x01\x92\x91PPV\xFE\x91\xC0\xA0\xC8`\xF8B\xE6\x1E\xA5\xD1\xB9\xBB\xF1\xCB\xACBX\xB1\xEC\x93\xED\x9D:\"\xF1\xF5y\xF7\x91\x83M\xA2dipfsX\"\x12 \xB4\x80\x9B\xE6k\x91\xF7\x93\x0F\x9C\x19g\xEC\xFDPgI\xDE1\x9B\xE1m+\xE1\t\xDA\xA7\x86\xC03\x1B\xC0dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static LIQUIDITYPROJECTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x0BW`\x005`\xE0\x1C\x80c\x90\x10\xD0|\x11a\0\xA2W\x80c\xB0bQ\xCA\x11a\0qW\x80c\xB0bQ\xCA\x14a\x02`W\x80c\xC2\x15PG\x14a\x02\x80W\x80c\xCA\x15\xC8s\x14a\x02\x93W\x80c\xD5Gt\x1F\x14a\x02\xA6W\x80c\xE8\xA2\xB1j\x14a\x02\xB9W`\0\x80\xFD[\x80c\x90\x10\xD0|\x14a\x02\x0BW\x80c\x91\xD1HT\x14a\x02\x1EW\x80c\xA2\x17\xFD\xDF\x14a\x021W\x80c\xA3\xF5\xC1\xD2\x14a\x029W`\0\x80\xFD[\x80cx\xA0\xD5\x97\x11a\0\xDEW\x80cx\xA0\xD5\x97\x14a\x01\x91W\x80c\x8CJ\xF1)\x14a\x01\xA4W\x80c\x8D\xA5\xCB[\x14a\x01\xB7W\x80c\x8E}S\xA1\x14a\x01\xF6W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x10W\x80c$\x8A\x9C\xA3\x14a\x018W\x80c//\xF1]\x14a\x01iW\x80c6V\x8A\xBE\x14a\x01~W[`\0\x80\xFD[a\x01#a\x01\x1E6`\x04a\x12$V[a\x02\xE0V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01[a\x01F6`\x04a\x12NV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01/V[a\x01|a\x01w6`\x04a\x12\x83V[a\x03\x0BV[\0[a\x01|a\x01\x8C6`\x04a\x12\x83V[a\x035V[a\x01|a\x01\x9F6`\x04a\x12\xAFV[a\x03\xB8V[a\x01|a\x01\xB26`\x04a\x12\xCAV[a\x03\xFBV[a\x01\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01/V[a\x01[`\0\x80Q` a\x16\x80\x839\x81Q\x91R\x81V[a\x01\xDEa\x02\x196`\x04a\x13\x06V[a\x04\xCBV[a\x01#a\x02,6`\x04a\x12\x83V[a\x04\xEAV[a\x01[`\0\x81V[a\x01\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02sa\x02n6`\x04a\x13\x98V[a\x05\x13V[`@Qa\x01/\x91\x90a\x14\x87V[a\x02sa\x02\x8E6`\x04a\x13\x98V[a\x07\x89V[a\x01[a\x02\xA16`\x04a\x12NV[a\t\xC3V[a\x01|a\x02\xB46`\x04a\x12\x83V[a\t\xDAV[a\x01\xDE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x03\x05WPa\x03\x05\x82a\t\xFFV[\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x03&\x81a\n4V[a\x030\x83\x83a\n>V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\xB4\x82\x82a\n`V[PPV[a\x03\xC3`\x003a\x04\xEAV[a\x03\xE0W`@Qc-M{\xB3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xF8`\0\x80Q` a\x16\x80\x839\x81Q\x91R\x82a\x03\x0BV[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04DW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x80\x86R\x91\x84R\x82\x85 \x80T\x91\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x81\x17\x90\x91U\x88\x86R`\x03\x85R\x83\x86 \x81\x87R\x90\x94R\x82\x85 \x80T\x90\x91\x16\x82\x17\x90U\x90Q\x91\x92\x90\x91\x86\x91\x7F\xDD\x1Eo\xEFi\xB2\xF4\0\x03J\xF4\xD8\x19>it\xC5\x91f\x8D9G\xD3\xE5\xE1H+\xE6\xF4\xCE\xBA\xAA\x91\xA4PPPV[`\0\x82\x81R`\x01` R`@\x81 a\x04\xE3\x90\x83a\n\x82V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[``a\x05-`\0\x80Q` a\x16\x80\x839\x81Q\x91R3a\x04\xEAV[a\x05JW`@Qce\xB8UK`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7Fv\n5\x8DH\n\xCDb\xE3\t\xA72K\xC2\x89n\xBA\xC8E\xFB\x1B.\xBE\xC73\x9Eg\x0B\xDC\x17A\xA5\x83`@Qa\x05\x83\x91\x90a\x14\x87V[`@Q\x80\x91\x03\x90\xA2`\0[\x82Q\x81\x10\x15a\x07\x80W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x05\xD3Wa\x05\xD3a\x14\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x068Wa\x063\x83\x82\x81Q\x81\x10a\x06\x02Wa\x06\x02a\x14\xDFV[` \x02` \x01\x01Q`\0\x01Q\x850\x86\x85\x81Q\x81\x10a\x06\"Wa\x06\"a\x14\xDFV[` \x02` \x01\x01Q` \x01Qa\n\x8EV[a\x06\xE7V[\x82\x81\x81Q\x81\x10a\x06JWa\x06Ja\x14\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16cy\xCCg\x90\x85\x85\x84\x81Q\x81\x10a\x06wWa\x06wa\x14\xDFV[` \x02` \x01\x01Q` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xB4\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xCEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xE2W=`\0\x80>=`\0\xFD[PPPP[`\x03`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x84\x83\x81Q\x81\x10a\x07\x0EWa\x07\x0Ea\x14\xDFV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x07`Wa\x07`a\x14\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90R`\x01\x01a\x05\x8EV[P\x90\x93\x92PPPV[``a\x07\xA3`\0\x80Q` a\x16\x80\x839\x81Q\x91R3a\x04\xEAV[a\x07\xC0W`@Qce\xB8UK`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7FN8A\xFB\x84\x88\xF53\xA2wg\xAB\x10\x84\xC2\xBF\x87\x05\xFF\xF4\xBE\x10Z)o\xF1%&\x93\r\xE2\x19\x84`@Qa\x07\xFA\x91\x90a\x14\x87V[`@Q\x80\x91\x03\x90\xA3`\0[\x82Q\x81\x10\x15a\x07\x80W`\0\x85\x81R`\x02` R`@\x81 \x84Q\x82\x90\x86\x90\x85\x90\x81\x10a\x082Wa\x082a\x14\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x90\x82\x01\x92\x90\x92R`@\x01`\0 T\x16\x90P\x80a\x08\xA5W\x83\x82\x81Q\x81\x10a\x08tWa\x08ta\x14\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qc\x06|x\x7F`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x03\xA1V[\x80\x84\x83\x81Q\x81\x10a\x08\xB8Wa\x08\xB8a\x14\xDFV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x03a\t(Wa\t#\x81\x86\x86\x85\x81Q\x81\x10a\t\x12Wa\t\x12a\x14\xDFV[` \x02` \x01\x01Q` \x01Qa\n\xFFV[a\t\xBAV[\x80`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x19\x86\x86\x85\x81Q\x81\x10a\tJWa\tJa\x14\xDFV[` \x02` \x01\x01Q` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x87\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\xA1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\xB5W=`\0\x80>=`\0\xFD[PPPP[P`\x01\x01a\x08\x05V[`\0\x81\x81R`\x01` R`@\x81 a\x03\x05\x90a\x0B/V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\t\xF5\x81a\n4V[a\x030\x83\x83a\n`V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x03\x05WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x03\x05V[a\x03\xF8\x813a\x0B9V[a\nH\x82\x82a\x0B\x92V[`\0\x82\x81R`\x01` R`@\x90 a\x030\x90\x82a\x0C\x16V[a\nj\x82\x82a\x0C+V[`\0\x82\x81R`\x01` R`@\x90 a\x030\x90\x82a\x0C\x90V[`\0a\x04\xE3\x83\x83a\x0C\xA5V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\n\xF9\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x0C\xCFV[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x030\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\n\xC2V[`\0a\x03\x05\x82T\x90V[a\x0BC\x82\x82a\x04\xEAV[a\x03\xB4Wa\x0BP\x81a\r\xA4V[a\x0B[\x83` a\r\xB6V[`@Q` \x01a\x0Bl\x92\x91\x90a\x15\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03\xA1\x91`\x04\x01a\x15\x8EV[a\x0B\x9C\x82\x82a\x04\xEAV[a\x03\xB4W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0B\xD23\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x04\xE3\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x0FRV[a\x0C5\x82\x82a\x04\xEAV[\x15a\x03\xB4W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x04\xE3\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x0F\xA1V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x0C\xBCWa\x0C\xBCa\x14\xDFV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0a\r$\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x10\x94\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\rEWP\x80\x80` \x01\x90Q\x81\x01\x90a\rE\x91\x90a\x15\xC1V[a\x030W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x03\xA1V[``a\x03\x05`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\r\xC5\x83`\x02a\x15\xF9V[a\r\xD0\x90`\x02a\x16\x10V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xE8Wa\r\xE8a\x13(V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\x12W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x0E-Wa\x0E-a\x14\xDFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0E\\Wa\x0E\\a\x14\xDFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0E\x80\x84`\x02a\x15\xF9V[a\x0E\x8B\x90`\x01a\x16\x10V[\x90P[`\x01\x81\x11\x15a\x0F\x03Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0E\xBFWa\x0E\xBFa\x14\xDFV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0E\xD5Wa\x0E\xD5a\x14\xDFV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0E\xFC\x81a\x16#V[\x90Pa\x0E\x8EV[P\x83\x15a\x04\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03\xA1V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x0F\x99WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x03\x05V[P`\0a\x03\x05V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x10\x8AW`\0a\x0F\xC5`\x01\x83a\x16:V[\x85T\x90\x91P`\0\x90a\x0F\xD9\x90`\x01\x90a\x16:V[\x90P\x81\x81\x14a\x10>W`\0\x86`\0\x01\x82\x81T\x81\x10a\x0F\xF9Wa\x0F\xF9a\x14\xDFV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\x10\x1CWa\x10\x1Ca\x14\xDFV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x10OWa\x10Oa\x16MV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x03\x05V[`\0\x91PPa\x03\x05V[``a\x10\xA3\x84\x84`\0\x85a\x10\xABV[\x94\x93PPPPV[``\x82G\x10\x15a\x11\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x03\xA1V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x11(\x91\x90a\x16cV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x11eW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x11jV[``\x91P[P\x91P\x91Pa\x11{\x87\x83\x83\x87a\x11\x86V[\x97\x96PPPPPPPV[``\x83\x15a\x11\xF5W\x82Q`\0\x03a\x11\xEEW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x11\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x03\xA1V[P\x81a\x10\xA3V[a\x10\xA3\x83\x83\x81Q\x15a\x12\nW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xA1\x91\x90a\x15\x8EV[`\0` \x82\x84\x03\x12\x15a\x126W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x04\xE3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12`W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12~W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x12\x96W`\0\x80\xFD[\x825\x91Pa\x12\xA6` \x84\x01a\x12gV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x12\xC1W`\0\x80\xFD[a\x04\xE3\x82a\x12gV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\xDFW`\0\x80\xFD[\x835\x92Pa\x12\xEF` \x85\x01a\x12gV[\x91Pa\x12\xFD`@\x85\x01a\x12gV[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x13\x19W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13aWa\x13aa\x13(V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13\x90Wa\x13\x90a\x13(V[`@R\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\xADW`\0\x80\xFD[\x835\x92P` a\x13\xBE\x81\x86\x01a\x12gV[\x92P`@\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13\xDCW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x13\xF0W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x14\x02Wa\x14\x02a\x13(V[a\x14\x10\x85\x82`\x05\x1B\x01a\x13gV[\x81\x81R\x85\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x85\x01\x90\x8A\x82\x11\x15a\x140W`\0\x80\xFD[\x92\x85\x01\x92[\x81\x84\x10\x15a\x14wW\x84\x84\x8C\x03\x12\x15a\x14MW`\0\x80\x81\xFD[a\x14Ua\x13>V[a\x14^\x85a\x12gV[\x81R\x84\x87\x015\x87\x82\x01R\x83R\x92\x84\x01\x92\x91\x85\x01\x91a\x145V[\x80\x96PPPPPPP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x14\xD2W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x14\xA4V[P\x91\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x15\x10W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14\xF8V[PP`\0\x91\x01RV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x15Q\x81`\x17\x85\x01` \x88\x01a\x14\xF5V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x15\x82\x81`(\x84\x01` \x88\x01a\x14\xF5V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x15\xAD\x81`@\x85\x01` \x87\x01a\x14\xF5V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x15\xD3W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x04\xE3W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x05Wa\x03\x05a\x15\xE3V[\x80\x82\x01\x80\x82\x11\x15a\x03\x05Wa\x03\x05a\x15\xE3V[`\0\x81a\x162Wa\x162a\x15\xE3V[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x03\x05Wa\x03\x05a\x15\xE3V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82Qa\x16u\x81\x84` \x87\x01a\x14\xF5V[\x91\x90\x91\x01\x92\x91PPV\xFE\x91\xC0\xA0\xC8`\xF8B\xE6\x1E\xA5\xD1\xB9\xBB\xF1\xCB\xACBX\xB1\xEC\x93\xED\x9D:\"\xF1\xF5y\xF7\x91\x83M\xA2dipfsX\"\x12 \xB4\x80\x9B\xE6k\x91\xF7\x93\x0F\x9C\x19g\xEC\xFDPgI\xDE1\x9B\xE1m+\xE1\t\xDA\xA7\x86\xC03\x1B\xC0dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDITYPROJECTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LiquidityProjector<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LiquidityProjector<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LiquidityProjector<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LiquidityProjector<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LiquidityProjector<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LiquidityProjector))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LiquidityProjector<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDITYPROJECTOR_ABI.clone(),
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
                LIQUIDITYPROJECTOR_ABI.clone(),
                LIQUIDITYPROJECTOR_BYTECODE.clone().into(),
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
        ///Calls the contract's `lockOrBurn` (0xb06251ca) function
        pub fn lock_or_burn(
            &self,
            chain_id: ::ethers::core::types::U256,
            sender: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<Token>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Token>> {
            self.0
                .method_hash([176, 98, 81, 202], (chain_id, sender, tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintOrUnlock` (0xc2155047) function
        pub fn mint_or_unlock(
            &self,
            chain_id: ::ethers::core::types::U256,
            target: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<Token>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Token>> {
            self.0
                .method_hash([194, 21, 80, 71], (chain_id, target, tokens))
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
        ///Calls the contract's `setMirrorToken` (0x8c4af129) function
        pub fn set_mirror_token(
            &self,
            chain_id: ::ethers::core::types::U256,
            token: ::ethers::core::types::Address,
            mirror_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 74, 241, 41], (chain_id, token, mirror_token))
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
            LiquidityProjectorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LiquidityProjector<M> {
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
    pub enum LiquidityProjectorErrors {
        AssetNotFound(AssetNotFound),
        InvalidAdminRole(InvalidAdminRole),
        InvalidTokenMinterBurnerRole(InvalidTokenMinterBurnerRole),
        NotValidOwner(NotValidOwner),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidityProjectorErrors {
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
    impl ::ethers::core::abi::AbiEncode for LiquidityProjectorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AssetNotFound(element) => {
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
    impl ::ethers::contract::ContractRevert for LiquidityProjectorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AssetNotFound as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for LiquidityProjectorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTokenMinterBurnerRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotValidOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LiquidityProjectorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AssetNotFound> for LiquidityProjectorErrors {
        fn from(value: AssetNotFound) -> Self {
            Self::AssetNotFound(value)
        }
    }
    impl ::core::convert::From<InvalidAdminRole> for LiquidityProjectorErrors {
        fn from(value: InvalidAdminRole) -> Self {
            Self::InvalidAdminRole(value)
        }
    }
    impl ::core::convert::From<InvalidTokenMinterBurnerRole>
    for LiquidityProjectorErrors {
        fn from(value: InvalidTokenMinterBurnerRole) -> Self {
            Self::InvalidTokenMinterBurnerRole(value)
        }
    }
    impl ::core::convert::From<NotValidOwner> for LiquidityProjectorErrors {
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
    #[ethevent(
        name = "MintOrUnlock",
        abi = "MintOrUnlock(uint256,address,(address,uint256)[])"
    )]
    pub struct MintOrUnlockFilter {
        #[ethevent(indexed)]
        pub chain_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub target: ::ethers::core::types::Address,
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
    #[ethevent(name = "MirrorTokenSet", abi = "MirrorTokenSet(uint256,address,address)")]
    pub struct MirrorTokenSetFilter {
        #[ethevent(indexed)]
        pub chain_id: ::ethers::core::types::U256,
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
    pub enum LiquidityProjectorEvents {
        LockOrBurnFilter(LockOrBurnFilter),
        MintOrUnlockFilter(MintOrUnlockFilter),
        MirrorTokenSetFilter(MirrorTokenSetFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for LiquidityProjectorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LockOrBurnFilter::decode_log(log) {
                return Ok(LiquidityProjectorEvents::LockOrBurnFilter(decoded));
            }
            if let Ok(decoded) = MintOrUnlockFilter::decode_log(log) {
                return Ok(LiquidityProjectorEvents::MintOrUnlockFilter(decoded));
            }
            if let Ok(decoded) = MirrorTokenSetFilter::decode_log(log) {
                return Ok(LiquidityProjectorEvents::MirrorTokenSetFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(LiquidityProjectorEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(LiquidityProjectorEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(LiquidityProjectorEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LiquidityProjectorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LockOrBurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintOrUnlockFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MirrorTokenSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LockOrBurnFilter> for LiquidityProjectorEvents {
        fn from(value: LockOrBurnFilter) -> Self {
            Self::LockOrBurnFilter(value)
        }
    }
    impl ::core::convert::From<MintOrUnlockFilter> for LiquidityProjectorEvents {
        fn from(value: MintOrUnlockFilter) -> Self {
            Self::MintOrUnlockFilter(value)
        }
    }
    impl ::core::convert::From<MirrorTokenSetFilter> for LiquidityProjectorEvents {
        fn from(value: MirrorTokenSetFilter) -> Self {
            Self::MirrorTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for LiquidityProjectorEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for LiquidityProjectorEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for LiquidityProjectorEvents {
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
    ///Container type for all input parameters for the `lockOrBurn` function with signature `lockOrBurn(uint256,address,(address,uint256)[])` and selector `0xb06251ca`
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
        name = "lockOrBurn",
        abi = "lockOrBurn(uint256,address,(address,uint256)[])"
    )]
    pub struct LockOrBurnCall {
        pub chain_id: ::ethers::core::types::U256,
        pub sender: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<Token>,
    }
    ///Container type for all input parameters for the `mintOrUnlock` function with signature `mintOrUnlock(uint256,address,(address,uint256)[])` and selector `0xc2155047`
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
        name = "mintOrUnlock",
        abi = "mintOrUnlock(uint256,address,(address,uint256)[])"
    )]
    pub struct MintOrUnlockCall {
        pub chain_id: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `setMirrorToken` function with signature `setMirrorToken(uint256,address,address)` and selector `0x8c4af129`
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
    #[ethcall(name = "setMirrorToken", abi = "setMirrorToken(uint256,address,address)")]
    pub struct SetMirrorTokenCall {
        pub chain_id: ::ethers::core::types::U256,
        pub token: ::ethers::core::types::Address,
        pub mirror_token: ::ethers::core::types::Address,
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
    pub enum LiquidityProjectorCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        TokenMinterburnerRole(TokenMinterburnerRoleCall),
        AddTokenMinterBurnerRole(AddTokenMinterBurnerRoleCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Kai(KaiCall),
        LockOrBurn(LockOrBurnCall),
        MintOrUnlock(MintOrUnlockCall),
        Nexus(NexusCall),
        Owner(OwnerCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        SetMirrorToken(SetMirrorTokenCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidityProjectorCalls {
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
            if let Ok(decoded) = <SetMirrorTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetMirrorToken(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidityProjectorCalls {
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
                Self::Kai(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockOrBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintOrUnlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nexus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMirrorToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidityProjectorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenMinterburnerRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddTokenMinterBurnerRole(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Kai(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockOrBurn(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintOrUnlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nexus(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMirrorToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for LiquidityProjectorCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<TokenMinterburnerRoleCall> for LiquidityProjectorCalls {
        fn from(value: TokenMinterburnerRoleCall) -> Self {
            Self::TokenMinterburnerRole(value)
        }
    }
    impl ::core::convert::From<AddTokenMinterBurnerRoleCall>
    for LiquidityProjectorCalls {
        fn from(value: AddTokenMinterBurnerRoleCall) -> Self {
            Self::AddTokenMinterBurnerRole(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for LiquidityProjectorCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall> for LiquidityProjectorCalls {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for LiquidityProjectorCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for LiquidityProjectorCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for LiquidityProjectorCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<KaiCall> for LiquidityProjectorCalls {
        fn from(value: KaiCall) -> Self {
            Self::Kai(value)
        }
    }
    impl ::core::convert::From<LockOrBurnCall> for LiquidityProjectorCalls {
        fn from(value: LockOrBurnCall) -> Self {
            Self::LockOrBurn(value)
        }
    }
    impl ::core::convert::From<MintOrUnlockCall> for LiquidityProjectorCalls {
        fn from(value: MintOrUnlockCall) -> Self {
            Self::MintOrUnlock(value)
        }
    }
    impl ::core::convert::From<NexusCall> for LiquidityProjectorCalls {
        fn from(value: NexusCall) -> Self {
            Self::Nexus(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidityProjectorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for LiquidityProjectorCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for LiquidityProjectorCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetMirrorTokenCall> for LiquidityProjectorCalls {
        fn from(value: SetMirrorTokenCall) -> Self {
            Self::SetMirrorToken(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for LiquidityProjectorCalls {
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
    ///Container type for all return fields from the `lockOrBurn` function with signature `lockOrBurn(uint256,address,(address,uint256)[])` and selector `0xb06251ca`
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
    pub struct LockOrBurnReturn(pub ::std::vec::Vec<Token>);
    ///Container type for all return fields from the `mintOrUnlock` function with signature `mintOrUnlock(uint256,address,(address,uint256)[])` and selector `0xc2155047`
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
    pub struct MintOrUnlockReturn(pub ::std::vec::Vec<Token>);
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
}
