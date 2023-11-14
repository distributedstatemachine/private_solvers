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
    pub static LIQUIDITYPROJECTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\r\xD78\x03\x80a\r\xD7\x839\x81\x01`@\x81\x90Ra\0/\x91a\0fV[3`\xC0R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x16`\xA0Ra\0\x99V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0aW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\0yW`\0\x80\xFD[a\0\x82\x83a\0JV[\x91Pa\0\x90` \x84\x01a\0JV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Qa\x0C\xEEa\0\xE9`\09`\0\x81\x81`\x81\x01Ra\x01L\x01R`\0\x81\x81a\x01\x1F\x01R\x81\x81a\x02\xA9\x01Ra\x05\xFB\x01R`\0\x81\x81`\xC5\x01R\x81\x81a\x02\x1C\x01Ra\x04\xA6\x01Ra\x0C\xEE`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x8CJ\xF1)\x14a\0gW\x80c\x8D\xA5\xCB[\x14a\0|W\x80c\xA3\xF5\xC1\xD2\x14a\0\xC0W\x80c\xB0bQ\xCA\x14a\0\xE7W\x80c\xC2\x15PG\x14a\x01\x07W\x80c\xE8\xA2\xB1j\x14a\x01\x1AW[`\0\x80\xFD[a\0za\0u6`\x04a\n\x13V[a\x01AV[\0[a\0\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xFAa\0\xF56`\x04a\n\xBFV[a\x02\x0FV[`@Qa\0\xB7\x91\x90a\x0B\xAEV[a\0\xFAa\x01\x156`\x04a\n\xBFV[a\x04\x99V[a\0\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x01\x8AW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x80\x86R\x91\x84R\x82\x85 \x80T\x91\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x81\x17\x90\x91U\x88\x86R`\x01\x85R\x83\x86 \x81\x87R\x90\x94R\x82\x85 \x80T\x90\x91\x16\x82\x17\x90U\x90Q\x91\x92\x90\x91\x86\x91\x7F\xDD\x1Eo\xEFi\xB2\xF4\0\x03J\xF4\xD8\x19>it\xC5\x91f\x8D9G\xD3\xE5\xE1H+\xE6\xF4\xCE\xBA\xAA\x91\xA4PPPV[``3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02ZW`@Qc\x05\xED\x9F\x1D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7Fv\n5\x8DH\n\xCDb\xE3\t\xA72K\xC2\x89n\xBA\xC8E\xFB\x1B.\xBE\xC73\x9Eg\x0B\xDC\x17A\xA5\x83`@Qa\x02\x93\x91\x90a\x0B\xAEV[`@Q\x80\x91\x03\x90\xA2`\0[\x82Q\x81\x10\x15a\x04\x90W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x02\xE3Wa\x02\xE3a\x0C\x06V[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x03HWa\x03C\x83\x82\x81Q\x81\x10a\x03\x12Wa\x03\x12a\x0C\x06V[` \x02` \x01\x01Q`\0\x01Q\x850\x86\x85\x81Q\x81\x10a\x032Wa\x032a\x0C\x06V[` \x02` \x01\x01Q` \x01Qa\x06\xECV[a\x03\xF7V[\x82\x81\x81Q\x81\x10a\x03ZWa\x03Za\x0C\x06V[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16cy\xCCg\x90\x85\x85\x84\x81Q\x81\x10a\x03\x87Wa\x03\x87a\x0C\x06V[` \x02` \x01\x01Q` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xC4\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xDEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xF2W=`\0\x80>=`\0\xFD[PPPP[`\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x84\x83\x81Q\x81\x10a\x04\x1EWa\x04\x1Ea\x0C\x06V[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x04pWa\x04pa\x0C\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90R`\x01\x01a\x02\x9EV[P\x90\x93\x92PPPV[``3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xE4W`@Qc\x05\xED\x9F\x1D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7FN8A\xFB\x84\x88\xF53\xA2wg\xAB\x10\x84\xC2\xBF\x87\x05\xFF\xF4\xBE\x10Z)o\xF1%&\x93\r\xE2\x19\x84`@Qa\x05\x1E\x91\x90a\x0B\xAEV[`@Q\x80\x91\x03\x90\xA3`\0[\x82Q\x81\x10\x15a\x04\x90W`\0\x85\x81R` \x81\x90R`@\x81 \x84Q\x82\x90\x86\x90\x85\x90\x81\x10a\x05VWa\x05Va\x0C\x06V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x90\x82\x01\x92\x90\x92R`@\x01`\0 T\x16\x90P\x80a\x05\xCEW\x83\x82\x81Q\x81\x10a\x05\x98Wa\x05\x98a\x0C\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qc\x06|x\x7F`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\x80\x84\x83\x81Q\x81\x10a\x05\xE1Wa\x05\xE1a\x0C\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x03a\x06QWa\x06L\x81\x86\x86\x85\x81Q\x81\x10a\x06;Wa\x06;a\x0C\x06V[` \x02` \x01\x01Q` \x01Qa\x07]V[a\x06\xE3V[\x80`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x19\x86\x86\x85\x81Q\x81\x10a\x06sWa\x06sa\x0C\x06V[` \x02` \x01\x01Q` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xB0\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xCAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xDEW=`\0\x80>=`\0\xFD[PPPP[P`\x01\x01a\x05)V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x07W\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x07\x92V[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x07\x8D\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\x07 V[PPPV[`\0a\x07\xE7\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x08g\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x08\x08WP\x80\x80` \x01\x90Q\x81\x01\x90a\x08\x08\x91\x90a\x0C\x1CV[a\x07\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x05\xC5V[``a\x08v\x84\x84`\0\x85a\x08~V[\x94\x93PPPPV[``\x82G\x10\x15a\x08\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x05\xC5V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x08\xFB\x91\x90a\x0CiV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\t8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t=V[``\x91P[P\x91P\x91Pa\tN\x87\x83\x83\x87a\tYV[\x97\x96PPPPPPPV[``\x83\x15a\t\xC8W\x82Q`\0\x03a\t\xC1W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\t\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05\xC5V[P\x81a\x08vV[a\x08v\x83\x83\x81Q\x15a\t\xDDW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC5\x91\x90a\x0C\x85V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\x0EW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\n(W`\0\x80\xFD[\x835\x92Pa\n8` \x85\x01a\t\xF7V[\x91Pa\nF`@\x85\x01a\t\xF7V[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\x88Wa\n\x88a\nOV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xB7Wa\n\xB7a\nOV[`@R\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\xD4W`\0\x80\xFD[\x835\x92P` a\n\xE5\x81\x86\x01a\t\xF7V[\x92P`@\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x03W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x0B\x17W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0B)Wa\x0B)a\nOV[a\x0B7\x85\x82`\x05\x1B\x01a\n\x8EV[\x81\x81R\x85\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x85\x01\x90\x8A\x82\x11\x15a\x0BWW`\0\x80\xFD[\x92\x85\x01\x92[\x81\x84\x10\x15a\x0B\x9EW\x84\x84\x8C\x03\x12\x15a\x0BtW`\0\x80\x81\xFD[a\x0B|a\neV[a\x0B\x85\x85a\t\xF7V[\x81R\x84\x87\x015\x87\x82\x01R\x83R\x92\x84\x01\x92\x91\x85\x01\x91a\x0B\\V[\x80\x96PPPPPPP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x0B\xF9W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x0B\xCBV[P\x91\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0C.W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0C>W`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x0C`W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0CHV[PP`\0\x91\x01RV[`\0\x82Qa\x0C{\x81\x84` \x87\x01a\x0CEV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C\xA4\x81`@\x85\x01` \x87\x01a\x0CEV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 -\xC5 \x9D\xECZ}\xCB\xBD\x96\xF3\xF5\xD3\xF3\xA8T\xA0\x02\xE26\xA3\xCDywO\xFE\x05\xE0<f\x19PdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static LIQUIDITYPROJECTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x8CJ\xF1)\x14a\0gW\x80c\x8D\xA5\xCB[\x14a\0|W\x80c\xA3\xF5\xC1\xD2\x14a\0\xC0W\x80c\xB0bQ\xCA\x14a\0\xE7W\x80c\xC2\x15PG\x14a\x01\x07W\x80c\xE8\xA2\xB1j\x14a\x01\x1AW[`\0\x80\xFD[a\0za\0u6`\x04a\n\x13V[a\x01AV[\0[a\0\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xFAa\0\xF56`\x04a\n\xBFV[a\x02\x0FV[`@Qa\0\xB7\x91\x90a\x0B\xAEV[a\0\xFAa\x01\x156`\x04a\n\xBFV[a\x04\x99V[a\0\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x01\x8AW`@Qc\x91.\xCC\xE7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x80\x86R\x91\x84R\x82\x85 \x80T\x91\x87\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x81\x17\x90\x91U\x88\x86R`\x01\x85R\x83\x86 \x81\x87R\x90\x94R\x82\x85 \x80T\x90\x91\x16\x82\x17\x90U\x90Q\x91\x92\x90\x91\x86\x91\x7F\xDD\x1Eo\xEFi\xB2\xF4\0\x03J\xF4\xD8\x19>it\xC5\x91f\x8D9G\xD3\xE5\xE1H+\xE6\xF4\xCE\xBA\xAA\x91\xA4PPPV[``3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02ZW`@Qc\x05\xED\x9F\x1D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7Fv\n5\x8DH\n\xCDb\xE3\t\xA72K\xC2\x89n\xBA\xC8E\xFB\x1B.\xBE\xC73\x9Eg\x0B\xDC\x17A\xA5\x83`@Qa\x02\x93\x91\x90a\x0B\xAEV[`@Q\x80\x91\x03\x90\xA2`\0[\x82Q\x81\x10\x15a\x04\x90W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x02\xE3Wa\x02\xE3a\x0C\x06V[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x03HWa\x03C\x83\x82\x81Q\x81\x10a\x03\x12Wa\x03\x12a\x0C\x06V[` \x02` \x01\x01Q`\0\x01Q\x850\x86\x85\x81Q\x81\x10a\x032Wa\x032a\x0C\x06V[` \x02` \x01\x01Q` \x01Qa\x06\xECV[a\x03\xF7V[\x82\x81\x81Q\x81\x10a\x03ZWa\x03Za\x0C\x06V[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16cy\xCCg\x90\x85\x85\x84\x81Q\x81\x10a\x03\x87Wa\x03\x87a\x0C\x06V[` \x02` \x01\x01Q` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xC4\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xDEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xF2W=`\0\x80>=`\0\xFD[PPPP[`\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x84\x83\x81Q\x81\x10a\x04\x1EWa\x04\x1Ea\x0C\x06V[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x81Q\x81\x10a\x04pWa\x04pa\x0C\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90R`\x01\x01a\x02\x9EV[P\x90\x93\x92PPPV[``3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xE4W`@Qc\x05\xED\x9F\x1D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7FN8A\xFB\x84\x88\xF53\xA2wg\xAB\x10\x84\xC2\xBF\x87\x05\xFF\xF4\xBE\x10Z)o\xF1%&\x93\r\xE2\x19\x84`@Qa\x05\x1E\x91\x90a\x0B\xAEV[`@Q\x80\x91\x03\x90\xA3`\0[\x82Q\x81\x10\x15a\x04\x90W`\0\x85\x81R` \x81\x90R`@\x81 \x84Q\x82\x90\x86\x90\x85\x90\x81\x10a\x05VWa\x05Va\x0C\x06V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R\x90\x82\x01\x92\x90\x92R`@\x01`\0 T\x16\x90P\x80a\x05\xCEW\x83\x82\x81Q\x81\x10a\x05\x98Wa\x05\x98a\x0C\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`@Qc\x06|x\x7F`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\x80\x84\x83\x81Q\x81\x10a\x05\xE1Wa\x05\xE1a\x0C\x06V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x03a\x06QWa\x06L\x81\x86\x86\x85\x81Q\x81\x10a\x06;Wa\x06;a\x0C\x06V[` \x02` \x01\x01Q` \x01Qa\x07]V[a\x06\xE3V[\x80`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x19\x86\x86\x85\x81Q\x81\x10a\x06sWa\x06sa\x0C\x06V[` \x02` \x01\x01Q` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xB0\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xCAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xDEW=`\0\x80>=`\0\xFD[PPPP[P`\x01\x01a\x05)V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x07W\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x07\x92V[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x07\x8D\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\x07 V[PPPV[`\0a\x07\xE7\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x08g\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x08\x08WP\x80\x80` \x01\x90Q\x81\x01\x90a\x08\x08\x91\x90a\x0C\x1CV[a\x07\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x05\xC5V[``a\x08v\x84\x84`\0\x85a\x08~V[\x94\x93PPPPV[``\x82G\x10\x15a\x08\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x05\xC5V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x08\xFB\x91\x90a\x0CiV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\t8W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t=V[``\x91P[P\x91P\x91Pa\tN\x87\x83\x83\x87a\tYV[\x97\x96PPPPPPPV[``\x83\x15a\t\xC8W\x82Q`\0\x03a\t\xC1W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\t\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x05\xC5V[P\x81a\x08vV[a\x08v\x83\x83\x81Q\x15a\t\xDDW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC5\x91\x90a\x0C\x85V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\x0EW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\n(W`\0\x80\xFD[\x835\x92Pa\n8` \x85\x01a\t\xF7V[\x91Pa\nF`@\x85\x01a\t\xF7V[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\x88Wa\n\x88a\nOV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xB7Wa\n\xB7a\nOV[`@R\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\xD4W`\0\x80\xFD[\x835\x92P` a\n\xE5\x81\x86\x01a\t\xF7V[\x92P`@\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x03W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x0B\x17W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0B)Wa\x0B)a\nOV[a\x0B7\x85\x82`\x05\x1B\x01a\n\x8EV[\x81\x81R\x85\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x85\x01\x90\x8A\x82\x11\x15a\x0BWW`\0\x80\xFD[\x92\x85\x01\x92[\x81\x84\x10\x15a\x0B\x9EW\x84\x84\x8C\x03\x12\x15a\x0BtW`\0\x80\x81\xFD[a\x0B|a\neV[a\x0B\x85\x85a\t\xF7V[\x81R\x84\x87\x015\x87\x82\x01R\x83R\x92\x84\x01\x92\x91\x85\x01\x91a\x0B\\V[\x80\x96PPPPPPP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x0B\xF9W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x0B\xCBV[P\x91\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x0C.W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0C>W`\0\x80\xFD[\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x0C`W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0CHV[PP`\0\x91\x01RV[`\0\x82Qa\x0C{\x81\x84` \x87\x01a\x0CEV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C\xA4\x81`@\x85\x01` \x87\x01a\x0CEV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 -\xC5 \x9D\xECZ}\xCB\xBD\x96\xF3\xF5\xD3\xF3\xA8T\xA0\x02\xE26\xA3\xCDywO\xFE\x05\xE0<f\x19PdsolcC\0\x08\x13\x003";
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
    pub enum LiquidityProjectorErrors {
        AssetNotFound(AssetNotFound),
        InvalidNexus(InvalidNexus),
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
    impl ::ethers::core::abi::AbiEncode for LiquidityProjectorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AssetNotFound(element) => {
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
    impl ::ethers::contract::ContractRevert for LiquidityProjectorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AssetNotFound as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for LiquidityProjectorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidNexus(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<InvalidNexus> for LiquidityProjectorErrors {
        fn from(value: InvalidNexus) -> Self {
            Self::InvalidNexus(value)
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
        Kai(KaiCall),
        LockOrBurn(LockOrBurnCall),
        MintOrUnlock(MintOrUnlockCall),
        Nexus(NexusCall),
        Owner(OwnerCall),
        SetMirrorToken(SetMirrorTokenCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidityProjectorCalls {
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
            if let Ok(decoded) = <SetMirrorTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetMirrorToken(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidityProjectorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Kai(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockOrBurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintOrUnlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nexus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMirrorToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidityProjectorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Kai(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockOrBurn(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintOrUnlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nexus(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMirrorToken(element) => ::core::fmt::Display::fmt(element, f),
            }
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
    impl ::core::convert::From<SetMirrorTokenCall> for LiquidityProjectorCalls {
        fn from(value: SetMirrorTokenCall) -> Self {
            Self::SetMirrorToken(value)
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
}
