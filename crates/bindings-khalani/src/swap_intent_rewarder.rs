pub use swap_intent_rewarder::*;
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
pub mod swap_intent_rewarder {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                    ::std::borrow::ToOwned::to_owned("addSettlementReactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addSettlementReactor",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("settlementReactor"),
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
                    ::std::borrow::ToOwned::to_owned("reward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("rewardSource"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardSource"),
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
                    ::std::borrow::ToOwned::to_owned("setRewardSource"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRewardSource"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewardSource"),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NotImplemented"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotImplemented"),
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
    pub static SWAPINTENTREWARDER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0\x1C`\x003a\0!V[a\x01TV[a\0+\x82\x82a\0HV[`\0\x82\x81R`\x01` R`@\x90 a\0C\x90\x82a\0\xE7V[PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16a\0\xE3W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\0\xA23\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4[PPV[`\0a\0\xFC\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x01\x05V[\x90P[\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x01LWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\0\xFFV[P`\0a\0\xFFV[a\x0E\xE6\x80a\x01c`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80cE0\xCC\x9F\x11a\0\x8CW\x80c\x9C\xFB\xC0\x02\x11a\0fW\x80c\x9C\xFB\xC0\x02\x14a\x01\xFAW\x80c\xA2\x17\xFD\xDF\x14a\x02\rW\x80c\xCA\x15\xC8s\x14a\x02\x15W\x80c\xD5Gt\x1F\x14a\x02(W`\0\x80\xFD[\x80cE0\xCC\x9F\x14a\x01\xA9W\x80c\x90\x10\xD0|\x14a\x01\xBCW\x80c\x91\xD1HT\x14a\x01\xE7W`\0\x80\xFD[\x80c$\x8A\x9C\xA3\x11a\0\xC8W\x80c$\x8A\x9C\xA3\x14a\x01?W\x80c//\xF1]\x14a\x01pW\x80c6V\x8A\xBE\x14a\x01\x83W\x80cD\xAA\xC9\xA7\x14a\x01\x96W`\0\x80\xFD[\x80c\x01c\xE5$\x14a\0\xEFW\x80c\x01\xFF\xC9\xA7\x14a\x01\x04W\x80c\x18\x06\xC3C\x14a\x01,W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a\nFV[a\x02;V[\0[a\x01\x17a\x01\x126`\x04a\ncV[a\x02rV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\x01:6`\x04a\n\x8DV[a\x02\x9DV[a\x01ba\x01M6`\x04a\n\xE9V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01#V[a\x01\x02a\x01~6`\x04a\x0B\x02V[a\x03\x9CV[a\x01\x02a\x01\x916`\x04a\x0B\x02V[a\x03\xC6V[a\x01\x02a\x01\xA46`\x04a\nFV[a\x04IV[a\x01\x02a\x01\xB76`\x04a\x0B2V[a\x04uV[a\x01\xCFa\x01\xCA6`\x04a\x0BtV[a\x04\x8EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01#V[a\x01\x17a\x01\xF56`\x04a\x0B\x02V[a\x04\xADV[`\x02Ta\x01\xCF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01b`\0\x81V[a\x01ba\x02#6`\x04a\n\xE9V[a\x04\xD6V[a\x01\x02a\x0266`\x04a\x0B\x02V[a\x04\xEDV[a\x02E`\0a\x05\x12V[a\x02o\x7F\xF7z9W\xA5\x04_3\xF8\xEF\xAF#\xD6\x1F\x10\xE1\xD9\xC0\xFFj\x0Fq\xDA\xEA\n\xB3Q\x04i\xA8\x95\xAC\x82a\x05\x1CV[PV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x02\x97WPa\x02\x97\x82a\x05>V[\x92\x91PPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02\xB4W\x90PP\x90P`@Q\x80`@\x01`@R\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x81RP\x81`\0\x81Q\x81\x10a\x03\x0EWa\x03\x0Ea\x0B\xACV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x02T`@Qc\xC2\x15PG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC2\x15PG\x90a\x03M\x90\x88\x90\x86\x90\x86\x90`\x04\x01a\x0B\xC2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03lW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\x94\x91\x90\x81\x01\x90a\x0C\x8EV[PPPPPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x03\xB7\x81a\x05\x12V[a\x03\xC1\x83\x83a\x05\x1CV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x04;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x04E\x82\x82a\x05sV[PPV[a\x04S`\0a\x05\x12V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@Qc\xD6#G%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x01` R`@\x81 a\x04\xA6\x90\x83a\x05\x95V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x81\x81R`\x01` R`@\x81 a\x02\x97\x90a\x05\xA1V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x05\x08\x81a\x05\x12V[a\x03\xC1\x83\x83a\x05sV[a\x02o\x813a\x05\xABV[a\x05&\x82\x82a\x06\x04V[`\0\x82\x81R`\x01` R`@\x90 a\x03\xC1\x90\x82a\x06\x88V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x02\x97WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x02\x97V[a\x05}\x82\x82a\x06\x9DV[`\0\x82\x81R`\x01` R`@\x90 a\x03\xC1\x90\x82a\x07\x02V[`\0a\x04\xA6\x83\x83a\x07\x17V[`\0a\x02\x97\x82T\x90V[a\x05\xB5\x82\x82a\x04\xADV[a\x04EWa\x05\xC2\x81a\x07AV[a\x05\xCD\x83` a\x07SV[`@Q` \x01a\x05\xDE\x92\x91\x90a\r\x88V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x042\x91`\x04\x01a\r\xFDV[a\x06\x0E\x82\x82a\x04\xADV[a\x04EW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x06D3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x04\xA6\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x08\xEFV[a\x06\xA7\x82\x82a\x04\xADV[\x15a\x04EW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x04\xA6\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\t>V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x07.Wa\x07.a\x0B\xACV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[``a\x02\x97`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x07b\x83`\x02a\x0EFV[a\x07m\x90`\x02a\x0E]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x85Wa\x07\x85a\x0B\x96V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x07\xAFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x07\xCAWa\x07\xCAa\x0B\xACV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x07\xF9Wa\x07\xF9a\x0B\xACV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x08\x1D\x84`\x02a\x0EFV[a\x08(\x90`\x01a\x0E]V[\x90P[`\x01\x81\x11\x15a\x08\xA0Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x08\\Wa\x08\\a\x0B\xACV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x08rWa\x08ra\x0B\xACV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x08\x99\x81a\x0EpV[\x90Pa\x08+V[P\x83\x15a\x04\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x042V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\t6WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02\x97V[P`\0a\x02\x97V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\n'W`\0a\tb`\x01\x83a\x0E\x87V[\x85T\x90\x91P`\0\x90a\tv\x90`\x01\x90a\x0E\x87V[\x90P\x81\x81\x14a\t\xDBW`\0\x86`\0\x01\x82\x81T\x81\x10a\t\x96Wa\t\x96a\x0B\xACV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\t\xB9Wa\t\xB9a\x0B\xACV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\t\xECWa\t\xECa\x0E\x9AV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x02\x97V[`\0\x91PPa\x02\x97V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02oW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\nXW`\0\x80\xFD[\x815a\x04\xA6\x81a\n1V[`\0` \x82\x84\x03\x12\x15a\nuW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x04\xA6W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\n\xA3W`\0\x80\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\n\xB7W`\0\x80\xFD[\x93P` \x85\x015a\n\xC7\x81a\n1V[\x92P`@\x85\x015\x91P``\x85\x015a\n\xDE\x81a\n1V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a\n\xFBW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\x15W`\0\x80\xFD[\x825\x91P` \x83\x015a\x0B'\x81a\n1V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0BGW`\0\x80\xFD[\x835a\x0BR\x81a\n1V[\x92P` \x84\x015\x91P`@\x84\x015a\x0Bi\x81a\n1V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\x87W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0``\x82\x01c\xFF\xFF\xFF\xFF\x86\x16\x83R` `\x01\x80`\xA0\x1B\x03\x80\x87\x16\x82\x86\x01R`@``\x81\x87\x01R\x83\x87Q\x80\x86R`\x80\x88\x01\x91P\x84\x89\x01\x95P`\0[\x81\x81\x10\x15a\x0C$W\x86Q\x80Q\x86\x16\x84R\x86\x01Q\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01a\x0B\xFDV[P\x90\x9A\x99PPPPPPPPPPV[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0CWWa\x0CWa\x0B\x96V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C\x86Wa\x0C\x86a\x0B\x96V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x0C\xA1W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\xB9W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0C\xCDW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0C\xDFWa\x0C\xDFa\x0B\x96V[a\x0C\xED\x84\x82`\x05\x1B\x01a\x0C]V[\x81\x81R\x84\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\r\rW`\0\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\rYW`@\x84\x89\x03\x12\x15a\r+W`\0\x80\x81\xFD[a\r3a\x0C4V[\x84Qa\r>\x81a\n1V[\x81R\x84\x86\x01Q\x86\x82\x01R\x83R`@\x90\x93\x01\x92\x91\x84\x01\x91a\r\x12V[\x97\x96PPPPPPPV[`\0[\x83\x81\x10\x15a\r\x7FW\x81\x81\x01Q\x83\x82\x01R` \x01a\rgV[PP`\0\x91\x01RV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\r\xC0\x81`\x17\x85\x01` \x88\x01a\rdV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\r\xF1\x81`(\x84\x01` \x88\x01a\rdV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0E\x1C\x81`@\x85\x01` \x87\x01a\rdV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x97Wa\x02\x97a\x0E0V[\x80\x82\x01\x80\x82\x11\x15a\x02\x97Wa\x02\x97a\x0E0V[`\0\x81a\x0E\x7FWa\x0E\x7Fa\x0E0V[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x02\x97Wa\x02\x97a\x0E0V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xFBY\x11\x99\xF1\xC4\x10\xA9on\x13\x8D\x82e\x1D\xCD\xB3\x01hag\xC7Uy\xC3\xC4\xE1\x8B\xC9\xF4(JdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SWAPINTENTREWARDER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80cE0\xCC\x9F\x11a\0\x8CW\x80c\x9C\xFB\xC0\x02\x11a\0fW\x80c\x9C\xFB\xC0\x02\x14a\x01\xFAW\x80c\xA2\x17\xFD\xDF\x14a\x02\rW\x80c\xCA\x15\xC8s\x14a\x02\x15W\x80c\xD5Gt\x1F\x14a\x02(W`\0\x80\xFD[\x80cE0\xCC\x9F\x14a\x01\xA9W\x80c\x90\x10\xD0|\x14a\x01\xBCW\x80c\x91\xD1HT\x14a\x01\xE7W`\0\x80\xFD[\x80c$\x8A\x9C\xA3\x11a\0\xC8W\x80c$\x8A\x9C\xA3\x14a\x01?W\x80c//\xF1]\x14a\x01pW\x80c6V\x8A\xBE\x14a\x01\x83W\x80cD\xAA\xC9\xA7\x14a\x01\x96W`\0\x80\xFD[\x80c\x01c\xE5$\x14a\0\xEFW\x80c\x01\xFF\xC9\xA7\x14a\x01\x04W\x80c\x18\x06\xC3C\x14a\x01,W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a\nFV[a\x02;V[\0[a\x01\x17a\x01\x126`\x04a\ncV[a\x02rV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\x01:6`\x04a\n\x8DV[a\x02\x9DV[a\x01ba\x01M6`\x04a\n\xE9V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01#V[a\x01\x02a\x01~6`\x04a\x0B\x02V[a\x03\x9CV[a\x01\x02a\x01\x916`\x04a\x0B\x02V[a\x03\xC6V[a\x01\x02a\x01\xA46`\x04a\nFV[a\x04IV[a\x01\x02a\x01\xB76`\x04a\x0B2V[a\x04uV[a\x01\xCFa\x01\xCA6`\x04a\x0BtV[a\x04\x8EV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01#V[a\x01\x17a\x01\xF56`\x04a\x0B\x02V[a\x04\xADV[`\x02Ta\x01\xCF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01b`\0\x81V[a\x01ba\x02#6`\x04a\n\xE9V[a\x04\xD6V[a\x01\x02a\x0266`\x04a\x0B\x02V[a\x04\xEDV[a\x02E`\0a\x05\x12V[a\x02o\x7F\xF7z9W\xA5\x04_3\xF8\xEF\xAF#\xD6\x1F\x10\xE1\xD9\xC0\xFFj\x0Fq\xDA\xEA\n\xB3Q\x04i\xA8\x95\xAC\x82a\x05\x1CV[PV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x02\x97WPa\x02\x97\x82a\x05>V[\x92\x91PPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x02\xB4W\x90PP\x90P`@Q\x80`@\x01`@R\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x81RP\x81`\0\x81Q\x81\x10a\x03\x0EWa\x03\x0Ea\x0B\xACV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x02T`@Qc\xC2\x15PG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC2\x15PG\x90a\x03M\x90\x88\x90\x86\x90\x86\x90`\x04\x01a\x0B\xC2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03lW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\x94\x91\x90\x81\x01\x90a\x0C\x8EV[PPPPPPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x03\xB7\x81a\x05\x12V[a\x03\xC1\x83\x83a\x05\x1CV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x04;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x04E\x82\x82a\x05sV[PPV[a\x04S`\0a\x05\x12V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@Qc\xD6#G%`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x81R`\x01` R`@\x81 a\x04\xA6\x90\x83a\x05\x95V[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x81\x81R`\x01` R`@\x81 a\x02\x97\x90a\x05\xA1V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x05\x08\x81a\x05\x12V[a\x03\xC1\x83\x83a\x05sV[a\x02o\x813a\x05\xABV[a\x05&\x82\x82a\x06\x04V[`\0\x82\x81R`\x01` R`@\x90 a\x03\xC1\x90\x82a\x06\x88V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x02\x97WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x02\x97V[a\x05}\x82\x82a\x06\x9DV[`\0\x82\x81R`\x01` R`@\x90 a\x03\xC1\x90\x82a\x07\x02V[`\0a\x04\xA6\x83\x83a\x07\x17V[`\0a\x02\x97\x82T\x90V[a\x05\xB5\x82\x82a\x04\xADV[a\x04EWa\x05\xC2\x81a\x07AV[a\x05\xCD\x83` a\x07SV[`@Q` \x01a\x05\xDE\x92\x91\x90a\r\x88V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x042\x91`\x04\x01a\r\xFDV[a\x06\x0E\x82\x82a\x04\xADV[a\x04EW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x06D3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x04\xA6\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\x08\xEFV[a\x06\xA7\x82\x82a\x04\xADV[\x15a\x04EW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x04\xA6\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\t>V[`\0\x82`\0\x01\x82\x81T\x81\x10a\x07.Wa\x07.a\x0B\xACV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[``a\x02\x97`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x07b\x83`\x02a\x0EFV[a\x07m\x90`\x02a\x0E]V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x85Wa\x07\x85a\x0B\x96V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x07\xAFW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x07\xCAWa\x07\xCAa\x0B\xACV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x07\xF9Wa\x07\xF9a\x0B\xACV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x08\x1D\x84`\x02a\x0EFV[a\x08(\x90`\x01a\x0E]V[\x90P[`\x01\x81\x11\x15a\x08\xA0Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x08\\Wa\x08\\a\x0B\xACV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x08rWa\x08ra\x0B\xACV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x08\x99\x81a\x0EpV[\x90Pa\x08+V[P\x83\x15a\x04\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x042V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\t6WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02\x97V[P`\0a\x02\x97V[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\n'W`\0a\tb`\x01\x83a\x0E\x87V[\x85T\x90\x91P`\0\x90a\tv\x90`\x01\x90a\x0E\x87V[\x90P\x81\x81\x14a\t\xDBW`\0\x86`\0\x01\x82\x81T\x81\x10a\t\x96Wa\t\x96a\x0B\xACV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\t\xB9Wa\t\xB9a\x0B\xACV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\t\xECWa\t\xECa\x0E\x9AV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x02\x97V[`\0\x91PPa\x02\x97V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02oW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\nXW`\0\x80\xFD[\x815a\x04\xA6\x81a\n1V[`\0` \x82\x84\x03\x12\x15a\nuW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x04\xA6W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\n\xA3W`\0\x80\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\n\xB7W`\0\x80\xFD[\x93P` \x85\x015a\n\xC7\x81a\n1V[\x92P`@\x85\x015\x91P``\x85\x015a\n\xDE\x81a\n1V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a\n\xFBW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\x15W`\0\x80\xFD[\x825\x91P` \x83\x015a\x0B'\x81a\n1V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0BGW`\0\x80\xFD[\x835a\x0BR\x81a\n1V[\x92P` \x84\x015\x91P`@\x84\x015a\x0Bi\x81a\n1V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\x87W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0``\x82\x01c\xFF\xFF\xFF\xFF\x86\x16\x83R` `\x01\x80`\xA0\x1B\x03\x80\x87\x16\x82\x86\x01R`@``\x81\x87\x01R\x83\x87Q\x80\x86R`\x80\x88\x01\x91P\x84\x89\x01\x95P`\0[\x81\x81\x10\x15a\x0C$W\x86Q\x80Q\x86\x16\x84R\x86\x01Q\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01a\x0B\xFDV[P\x90\x9A\x99PPPPPPPPPPV[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0CWWa\x0CWa\x0B\x96V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C\x86Wa\x0C\x86a\x0B\x96V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x0C\xA1W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\xB9W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0C\xCDW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0C\xDFWa\x0C\xDFa\x0B\x96V[a\x0C\xED\x84\x82`\x05\x1B\x01a\x0C]V[\x81\x81R\x84\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\r\rW`\0\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\rYW`@\x84\x89\x03\x12\x15a\r+W`\0\x80\x81\xFD[a\r3a\x0C4V[\x84Qa\r>\x81a\n1V[\x81R\x84\x86\x01Q\x86\x82\x01R\x83R`@\x90\x93\x01\x92\x91\x84\x01\x91a\r\x12V[\x97\x96PPPPPPPV[`\0[\x83\x81\x10\x15a\r\x7FW\x81\x81\x01Q\x83\x82\x01R` \x01a\rgV[PP`\0\x91\x01RV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\r\xC0\x81`\x17\x85\x01` \x88\x01a\rdV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\r\xF1\x81`(\x84\x01` \x88\x01a\rdV[\x01`(\x01\x94\x93PPPPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0E\x1C\x81`@\x85\x01` \x87\x01a\rdV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\x97Wa\x02\x97a\x0E0V[\x80\x82\x01\x80\x82\x11\x15a\x02\x97Wa\x02\x97a\x0E0V[`\0\x81a\x0E\x7FWa\x0E\x7Fa\x0E0V[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x02\x97Wa\x02\x97a\x0E0V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xFBY\x11\x99\xF1\xC4\x10\xA9on\x13\x8D\x82e\x1D\xCD\xB3\x01hag\xC7Uy\xC3\xC4\xE1\x8B\xC9\xF4(JdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static SWAPINTENTREWARDER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SwapIntentRewarder<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SwapIntentRewarder<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SwapIntentRewarder<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SwapIntentRewarder<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SwapIntentRewarder<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SwapIntentRewarder))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SwapIntentRewarder<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SWAPINTENTREWARDER_ABI.clone(),
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
                SWAPINTENTREWARDER_ABI.clone(),
                SWAPINTENTREWARDER_BYTECODE.clone().into(),
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
        ///Calls the contract's `addSettlementReactor` (0x0163e524) function
        pub fn add_settlement_reactor(
            &self,
            settlement_reactor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 99, 229, 36], settlement_reactor)
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
        ///Calls the contract's `reward` (0x1806c343) function
        pub fn reward_with_chain_id_and_token_and_amount(
            &self,
            chain_id: u32,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 6, 195, 67], (chain_id, token, amount, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reward` (0x4530cc9f) function
        pub fn reward(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
            p2: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 48, 204, 159], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardSource` (0x9cfbc002) function
        pub fn reward_source(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([156, 251, 192, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRewardSource` (0x44aac9a7) function
        pub fn set_reward_source(
            &self,
            reward_source: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 170, 201, 167], reward_source)
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
            SwapIntentRewarderEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SwapIntentRewarder<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotImplemented` with signature `NotImplemented()` and selector `0xd6234725`
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
    #[etherror(name = "NotImplemented", abi = "NotImplemented()")]
    pub struct NotImplemented;
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
    pub enum SwapIntentRewarderEvents {
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for SwapIntentRewarderEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(SwapIntentRewarderEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(SwapIntentRewarderEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(SwapIntentRewarderEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SwapIntentRewarderEvents {
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
    impl ::core::convert::From<RoleAdminChangedFilter> for SwapIntentRewarderEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for SwapIntentRewarderEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for SwapIntentRewarderEvents {
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
    ///Container type for all input parameters for the `addSettlementReactor` function with signature `addSettlementReactor(address)` and selector `0x0163e524`
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
    #[ethcall(name = "addSettlementReactor", abi = "addSettlementReactor(address)")]
    pub struct AddSettlementReactorCall {
        pub settlement_reactor: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `reward` function with signature `reward(uint32,address,uint256,address)` and selector `0x1806c343`
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
    #[ethcall(name = "reward", abi = "reward(uint32,address,uint256,address)")]
    pub struct RewardWithChainIdAndTokenAndAmountCall {
        pub chain_id: u32,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `reward` function with signature `reward(address,uint256,address)` and selector `0x4530cc9f`
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
    #[ethcall(name = "reward", abi = "reward(address,uint256,address)")]
    pub struct RewardCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `rewardSource` function with signature `rewardSource()` and selector `0x9cfbc002`
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
    #[ethcall(name = "rewardSource", abi = "rewardSource()")]
    pub struct RewardSourceCall;
    ///Container type for all input parameters for the `setRewardSource` function with signature `setRewardSource(address)` and selector `0x44aac9a7`
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
    #[ethcall(name = "setRewardSource", abi = "setRewardSource(address)")]
    pub struct SetRewardSourceCall {
        pub reward_source: ::ethers::core::types::Address,
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
    pub enum SwapIntentRewarderCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        AddSettlementReactor(AddSettlementReactorCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RewardWithChainIdAndTokenAndAmount(RewardWithChainIdAndTokenAndAmountCall),
        Reward(RewardCall),
        RewardSource(RewardSourceCall),
        SetRewardSource(SetRewardSourceCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for SwapIntentRewarderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <AddSettlementReactorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddSettlementReactor(decoded));
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
            if let Ok(decoded) = <RewardWithChainIdAndTokenAndAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RewardWithChainIdAndTokenAndAmount(decoded));
            }
            if let Ok(decoded) = <RewardCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Reward(decoded));
            }
            if let Ok(decoded) = <RewardSourceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RewardSource(decoded));
            }
            if let Ok(decoded) = <SetRewardSourceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetRewardSource(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SwapIntentRewarderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddSettlementReactor(element) => {
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
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardWithChainIdAndTokenAndAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Reward(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardSource(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRewardSource(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SwapIntentRewarderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddSettlementReactor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardWithChainIdAndTokenAndAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Reward(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardSource(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRewardSource(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for SwapIntentRewarderCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<AddSettlementReactorCall> for SwapIntentRewarderCalls {
        fn from(value: AddSettlementReactorCall) -> Self {
            Self::AddSettlementReactor(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for SwapIntentRewarderCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall> for SwapIntentRewarderCalls {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for SwapIntentRewarderCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for SwapIntentRewarderCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for SwapIntentRewarderCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for SwapIntentRewarderCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for SwapIntentRewarderCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<RewardWithChainIdAndTokenAndAmountCall>
    for SwapIntentRewarderCalls {
        fn from(value: RewardWithChainIdAndTokenAndAmountCall) -> Self {
            Self::RewardWithChainIdAndTokenAndAmount(value)
        }
    }
    impl ::core::convert::From<RewardCall> for SwapIntentRewarderCalls {
        fn from(value: RewardCall) -> Self {
            Self::Reward(value)
        }
    }
    impl ::core::convert::From<RewardSourceCall> for SwapIntentRewarderCalls {
        fn from(value: RewardSourceCall) -> Self {
            Self::RewardSource(value)
        }
    }
    impl ::core::convert::From<SetRewardSourceCall> for SwapIntentRewarderCalls {
        fn from(value: SetRewardSourceCall) -> Self {
            Self::SetRewardSource(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for SwapIntentRewarderCalls {
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
    ///Container type for all return fields from the `rewardSource` function with signature `rewardSource()` and selector `0x9cfbc002`
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
    pub struct RewardSourceReturn(pub ::ethers::core::types::Address);
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
