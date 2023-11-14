pub use gas_paymaster::*;
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
pub mod gas_paymaster {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_innerIgp"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_GAS_OVERHEAD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DEFAULT_GAS_OVERHEAD",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("destinationGasOverhead"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "destinationGasOverhead",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("innerIgp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("innerIgp"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("unitTokenGasOverhead"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "unitTokenGasOverhead",
                            ),
                            inputs: ::std::vec![],
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
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
    pub static GASPAYMASTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07\xE18\x03\x80a\x07\xE1\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\x99V[a\083a\0IV[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0\xC9V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0` \x82\x84\x03\x12\x15a\0\xABW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xC2W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x06\xEFa\0\xF2`\09`\0\x81\x81a\x01h\x01R\x81\x81a\x02O\x01Ra\x04$\x01Ra\x06\xEF`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xA7W`\x005`\xE0\x1C\x80c}\x8B\xC5\x9C\x11a\0dW\x80c}\x8B\xC5\x9C\x14a\x01\xA2W\x80c\x83\xA6\xE6Y\x14a\x01\xB8W\x80c\x86<x\x0E\x14a\x01\xD8W\x80c\x8D\xA5\xCB[\x14a\x01\xEFW\x80c\xF2\xFD\xE3\x8B\x14a\x02\rW\x80c\xFD\x80,\xAC\x14a\x02-W`\0\x80\xFD[\x80c\x02l3H\x14a\0\xACW\x80c\x11\xBF,\x18\x14a\0\xECW\x80c/\xD8\x82\x92\x14a\x01\x01W\x80c2\n\x0B\xC5\x14a\x01!W\x80cqP\x18\xA6\x14a\x01AW\x80cx\xEC\xC5\xAC\x14a\x01VW[`\0\x80\xFD[4\x80\x15a\0\xB8W`\0\x80\xFD[Pa\0\xD9a\0\xC76`\x04a\x05\x8AV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xFFa\0\xFA6`\x04a\x05\xBCV[a\x02MV[\0[4\x80\x15a\x01\rW`\0\x80\xFD[Pa\0\xD9a\x01\x1C6`\x04a\x06\x02V[a\x03\x02V[4\x80\x15a\x01-W`\0\x80\xFD[Pa\0\xFFa\x01<6`\x04a\x06\x02V[a\x03`V[4\x80\x15a\x01MW`\0\x80\xFD[Pa\0\xFFa\x03\x81V[4\x80\x15a\x01bW`\0\x80\xFD[Pa\x01\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xE3V[4\x80\x15a\x01\xAEW`\0\x80\xFD[Pa\0\xD9`\x02T\x81V[4\x80\x15a\x01\xC4W`\0\x80\xFD[Pa\0\xFFa\x01\xD36`\x04a\x06,V[a\x03\x95V[4\x80\x15a\x01\xE4W`\0\x80\xFD[Pa\0\xD9b\x01\x86\xA0\x81V[4\x80\x15a\x01\xFBW`\0\x80\xFD[P`\0T`\x01`\x01`\xA0\x1B\x03\x16a\x01\x8AV[4\x80\x15a\x02\x19W`\0\x80\xFD[Pa\0\xFFa\x02(6`\x04a\x06EV[a\x03\xA2V[4\x80\x15a\x029W`\0\x80\xFD[Pa\0\xD9a\x02H6`\x04a\x06\x02V[a\x04 V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x11\xBF,\x184\x86\x86a\x02\x89\x88\x88a\x03\x02V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`$\x83\x01R`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xF7W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\0\x81`\x02Ta\x03\x12\x91\x90a\x06vV[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x03HWc\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x01` R`@\x90 Ta\x03MV[b\x01\x86\xA0[a\x03W\x91\x90a\x06\x8DV[\x90P[\x92\x91PPV[a\x03ha\x04\xC7V[c\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R`\x01` R`@\x90 UV[a\x03\x89a\x04\xC7V[a\x03\x93`\0a\x05!V[V[a\x03\x9Da\x04\xC7V[`\x02UV[a\x03\xAAa\x04\xC7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x04\x1D\x81a\x05!V[PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA6\x92\x97\x93\x84a\x04\\\x86\x86a\x03\x02V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03W\x91\x90a\x06\xA0V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04\x0BV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\x85W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\x9CW`\0\x80\xFD[a\x03W\x82a\x05qV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x85W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05\xD2W`\0\x80\xFD[\x845\x93Pa\x05\xE2` \x86\x01a\x05qV[\x92P`@\x85\x015\x91Pa\x05\xF7``\x86\x01a\x05\xA5V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\x15W`\0\x80\xFD[a\x06\x1E\x83a\x05qV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x06>W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x06WW`\0\x80\xFD[a\x03W\x82a\x05\xA5V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03ZWa\x03Za\x06`V[\x80\x82\x01\x80\x82\x11\x15a\x03ZWa\x03Za\x06`V[`\0` \x82\x84\x03\x12\x15a\x06\xB2W`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x11?\xD0\x15\xCA@\xFD\x1F. \xDE\xF3\x10\xB4Ww\xCE'\xDC\xD7\xCB\x01w\x8A\xA1\xA1\xE5\xD9\xF2a\x92\xA9dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static GASPAYMASTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xA7W`\x005`\xE0\x1C\x80c}\x8B\xC5\x9C\x11a\0dW\x80c}\x8B\xC5\x9C\x14a\x01\xA2W\x80c\x83\xA6\xE6Y\x14a\x01\xB8W\x80c\x86<x\x0E\x14a\x01\xD8W\x80c\x8D\xA5\xCB[\x14a\x01\xEFW\x80c\xF2\xFD\xE3\x8B\x14a\x02\rW\x80c\xFD\x80,\xAC\x14a\x02-W`\0\x80\xFD[\x80c\x02l3H\x14a\0\xACW\x80c\x11\xBF,\x18\x14a\0\xECW\x80c/\xD8\x82\x92\x14a\x01\x01W\x80c2\n\x0B\xC5\x14a\x01!W\x80cqP\x18\xA6\x14a\x01AW\x80cx\xEC\xC5\xAC\x14a\x01VW[`\0\x80\xFD[4\x80\x15a\0\xB8W`\0\x80\xFD[Pa\0\xD9a\0\xC76`\x04a\x05\x8AV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xFFa\0\xFA6`\x04a\x05\xBCV[a\x02MV[\0[4\x80\x15a\x01\rW`\0\x80\xFD[Pa\0\xD9a\x01\x1C6`\x04a\x06\x02V[a\x03\x02V[4\x80\x15a\x01-W`\0\x80\xFD[Pa\0\xFFa\x01<6`\x04a\x06\x02V[a\x03`V[4\x80\x15a\x01MW`\0\x80\xFD[Pa\0\xFFa\x03\x81V[4\x80\x15a\x01bW`\0\x80\xFD[Pa\x01\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xE3V[4\x80\x15a\x01\xAEW`\0\x80\xFD[Pa\0\xD9`\x02T\x81V[4\x80\x15a\x01\xC4W`\0\x80\xFD[Pa\0\xFFa\x01\xD36`\x04a\x06,V[a\x03\x95V[4\x80\x15a\x01\xE4W`\0\x80\xFD[Pa\0\xD9b\x01\x86\xA0\x81V[4\x80\x15a\x01\xFBW`\0\x80\xFD[P`\0T`\x01`\x01`\xA0\x1B\x03\x16a\x01\x8AV[4\x80\x15a\x02\x19W`\0\x80\xFD[Pa\0\xFFa\x02(6`\x04a\x06EV[a\x03\xA2V[4\x80\x15a\x029W`\0\x80\xFD[Pa\0\xD9a\x02H6`\x04a\x06\x02V[a\x04 V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x11\xBF,\x184\x86\x86a\x02\x89\x88\x88a\x03\x02V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`$\x83\x01R`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xF7W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\0\x81`\x02Ta\x03\x12\x91\x90a\x06vV[c\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x03HWc\xFF\xFF\xFF\xFF\x84\x16`\0\x90\x81R`\x01` R`@\x90 Ta\x03MV[b\x01\x86\xA0[a\x03W\x91\x90a\x06\x8DV[\x90P[\x92\x91PPV[a\x03ha\x04\xC7V[c\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R`\x01` R`@\x90 UV[a\x03\x89a\x04\xC7V[a\x03\x93`\0a\x05!V[V[a\x03\x9Da\x04\xC7V[`\x02UV[a\x03\xAAa\x04\xC7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x04\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x04\x1D\x81a\x05!V[PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA6\x92\x97\x93\x84a\x04\\\x86\x86a\x03\x02V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03W\x91\x90a\x06\xA0V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04\x0BV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\x85W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\x9CW`\0\x80\xFD[a\x03W\x82a\x05qV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x85W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05\xD2W`\0\x80\xFD[\x845\x93Pa\x05\xE2` \x86\x01a\x05qV[\x92P`@\x85\x015\x91Pa\x05\xF7``\x86\x01a\x05\xA5V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\x15W`\0\x80\xFD[a\x06\x1E\x83a\x05qV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x06>W`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x06WW`\0\x80\xFD[a\x03W\x82a\x05\xA5V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03ZWa\x03Za\x06`V[\x80\x82\x01\x80\x82\x11\x15a\x03ZWa\x03Za\x06`V[`\0` \x82\x84\x03\x12\x15a\x06\xB2W`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x11?\xD0\x15\xCA@\xFD\x1F. \xDE\xF3\x10\xB4Ww\xCE'\xDC\xD7\xCB\x01w\x8A\xA1\xA1\xE5\xD9\xF2a\x92\xA9dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static GASPAYMASTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GasPaymaster<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GasPaymaster<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GasPaymaster<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GasPaymaster<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GasPaymaster<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GasPaymaster))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GasPaymaster<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GASPAYMASTER_ABI.clone(),
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
                GASPAYMASTER_ABI.clone(),
                GASPAYMASTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_GAS_OVERHEAD` (0x863c780e) function
        pub fn default_gas_overhead(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([134, 60, 120, 14], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `destinationGasOverhead` (0x026c3348) function
        pub fn destination_gas_overhead(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([2, 108, 51, 72], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `innerIgp` (0x78ecc5ac) function
        pub fn inner_igp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([120, 236, 197, 172], ())
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unitTokenGasOverhead` (0x7d8bc59c) function
        pub fn unit_token_gas_overhead(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([125, 139, 197, 156], ())
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
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GasPaymasterEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GasPaymaster<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum GasPaymasterEvents {
        GasPaymentFilter(GasPaymentFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for GasPaymasterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = GasPaymentFilter::decode_log(log) {
                return Ok(GasPaymasterEvents::GasPaymentFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(GasPaymasterEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GasPaymasterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GasPaymentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GasPaymentFilter> for GasPaymasterEvents {
        fn from(value: GasPaymentFilter) -> Self {
            Self::GasPaymentFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for GasPaymasterEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_GAS_OVERHEAD` function with signature `DEFAULT_GAS_OVERHEAD()` and selector `0x863c780e`
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
    #[ethcall(name = "DEFAULT_GAS_OVERHEAD", abi = "DEFAULT_GAS_OVERHEAD()")]
    pub struct DefaultGasOverheadCall;
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
    ///Container type for all input parameters for the `destinationGasOverhead` function with signature `destinationGasOverhead(uint32)` and selector `0x026c3348`
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
    #[ethcall(name = "destinationGasOverhead", abi = "destinationGasOverhead(uint32)")]
    pub struct DestinationGasOverheadCall(pub u32);
    ///Container type for all input parameters for the `innerIgp` function with signature `innerIgp()` and selector `0x78ecc5ac`
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
    #[ethcall(name = "innerIgp", abi = "innerIgp()")]
    pub struct InnerIgpCall;
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
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `unitTokenGasOverhead` function with signature `unitTokenGasOverhead()` and selector `0x7d8bc59c`
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
    #[ethcall(name = "unitTokenGasOverhead", abi = "unitTokenGasOverhead()")]
    pub struct UnitTokenGasOverheadCall;
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
    pub enum GasPaymasterCalls {
        DefaultGasOverhead(DefaultGasOverheadCall),
        DestinationGasAmount(DestinationGasAmountCall),
        DestinationGasOverhead(DestinationGasOverheadCall),
        InnerIgp(InnerIgpCall),
        Owner(OwnerCall),
        PayForGas(PayForGasCall),
        QuoteSend(QuoteSendCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetDestinationGasOverhead(SetDestinationGasOverheadCall),
        SetUnitTokenGasOverhead(SetUnitTokenGasOverheadCall),
        TransferOwnership(TransferOwnershipCall),
        UnitTokenGasOverhead(UnitTokenGasOverheadCall),
    }
    impl ::ethers::core::abi::AbiDecode for GasPaymasterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultGasOverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultGasOverhead(decoded));
            }
            if let Ok(decoded) = <DestinationGasAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DestinationGasAmount(decoded));
            }
            if let Ok(decoded) = <DestinationGasOverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DestinationGasOverhead(decoded));
            }
            if let Ok(decoded) = <InnerIgpCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InnerIgp(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
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
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
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
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnitTokenGasOverheadCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnitTokenGasOverhead(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GasPaymasterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultGasOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DestinationGasAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DestinationGasOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InnerIgp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayForGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteSend(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDestinationGasOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUnitTokenGasOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnitTokenGasOverhead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GasPaymasterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultGasOverhead(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DestinationGasAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DestinationGasOverhead(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InnerIgp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayForGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteSend(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDestinationGasOverhead(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetUnitTokenGasOverhead(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnitTokenGasOverhead(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DefaultGasOverheadCall> for GasPaymasterCalls {
        fn from(value: DefaultGasOverheadCall) -> Self {
            Self::DefaultGasOverhead(value)
        }
    }
    impl ::core::convert::From<DestinationGasAmountCall> for GasPaymasterCalls {
        fn from(value: DestinationGasAmountCall) -> Self {
            Self::DestinationGasAmount(value)
        }
    }
    impl ::core::convert::From<DestinationGasOverheadCall> for GasPaymasterCalls {
        fn from(value: DestinationGasOverheadCall) -> Self {
            Self::DestinationGasOverhead(value)
        }
    }
    impl ::core::convert::From<InnerIgpCall> for GasPaymasterCalls {
        fn from(value: InnerIgpCall) -> Self {
            Self::InnerIgp(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for GasPaymasterCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PayForGasCall> for GasPaymasterCalls {
        fn from(value: PayForGasCall) -> Self {
            Self::PayForGas(value)
        }
    }
    impl ::core::convert::From<QuoteSendCall> for GasPaymasterCalls {
        fn from(value: QuoteSendCall) -> Self {
            Self::QuoteSend(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for GasPaymasterCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetDestinationGasOverheadCall> for GasPaymasterCalls {
        fn from(value: SetDestinationGasOverheadCall) -> Self {
            Self::SetDestinationGasOverhead(value)
        }
    }
    impl ::core::convert::From<SetUnitTokenGasOverheadCall> for GasPaymasterCalls {
        fn from(value: SetUnitTokenGasOverheadCall) -> Self {
            Self::SetUnitTokenGasOverhead(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for GasPaymasterCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnitTokenGasOverheadCall> for GasPaymasterCalls {
        fn from(value: UnitTokenGasOverheadCall) -> Self {
            Self::UnitTokenGasOverhead(value)
        }
    }
    ///Container type for all return fields from the `DEFAULT_GAS_OVERHEAD` function with signature `DEFAULT_GAS_OVERHEAD()` and selector `0x863c780e`
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
    pub struct DefaultGasOverheadReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `destinationGasOverhead` function with signature `destinationGasOverhead(uint32)` and selector `0x026c3348`
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
    pub struct DestinationGasOverheadReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `innerIgp` function with signature `innerIgp()` and selector `0x78ecc5ac`
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
    pub struct InnerIgpReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `unitTokenGasOverhead` function with signature `unitTokenGasOverhead()` and selector `0x7d8bc59c`
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
    pub struct UnitTokenGasOverheadReturn(pub ::ethers::core::types::U256);
}
