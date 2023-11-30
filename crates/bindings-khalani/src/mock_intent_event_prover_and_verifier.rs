pub use mock_intent_event_prover_and_verifier::*;
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
pub mod mock_intent_event_prover_and_verifier {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("verify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifySwapIntentFilledEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verifySwapIntentFilledEvent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("event_"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifySwapIntentTokenBurnEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verifySwapIntentTokenBurnEvent",
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifySwapIntentTokenLockEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verifySwapIntentTokenLockEvent",
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
    pub static MOCKINTENTEVENTPROVERANDVERIFIER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x05\x12\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\xB2\r\xC0\x07\x11a\0[W\x80c\xB2\r\xC0\x07\x14a\0\xFFW\x80c\xB8l\x94\xB2\x14a\x01\x12W\x80c\xD6k\"\xC8\x14a\x01%W\x80c\xEB\x8Eq-\x14a\x01NW`\0\x80\xFD[\x80c\x13\xD8\xB9T\x14a\0\x8DW\x80c]\x06\x80\xDC\x14a\0\xA2W\x80cu\xE3f\x16\x14a\0\xB5W\x80c~\xB6 \xA7\x14a\0\xECW[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\x03\x81V[a\x01aV[\0[a\0\xA0a\0\xB06`\x04a\x03\x81V[a\x01\x9FV[a\0\xD8a\0\xC36`\x04a\x03\x9DV[`\0\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\0\xFA6`\x04a\x03\xB6V[a\x01\xB8V[a\0\xD8a\x01\r6`\x04a\x03\x81V[a\x01\xD1V[a\0\xD8a\x01 6`\x04a\x03\x81V[a\x02RV[a\0\xA0a\x0136`\x04a\x03\x9DV[`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\0\xD8a\x01\\6`\x04a\x03\xB6V[a\x02gV[`\0a\x01za\x01u6\x84\x90\x03\x84\x01\x84a\x04\x18V[a\x02|V[\x90Pa\x01\x9B\x81`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[PPV[`\0a\x01za\x01\xB36\x84\x90\x03\x84\x01\x84a\x04\x18V[a\x02\xC9V[`\0a\x01za\x01\xCC6\x84\x90\x03\x84\x01\x84a\x044V[a\x02\xFDV[`\0\x80a\x01\xE6a\x01\xB36\x85\x90\x03\x85\x01\x85a\x04\x18V[`@Qc:\xF1\xB3\x0B`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P0\x90cu\xE3f\x16\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02K\x91\x90a\x04\xBAV[\x93\x92PPPV[`\0\x80a\x01\xE6a\x01u6\x85\x90\x03\x85\x01\x85a\x04\x18V[`\0\x80a\x01\xE6a\x01\xCC6\x85\x90\x03\x85\x01\x85a\x044V[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80Q`@Qr)\xBB\xB0\xB8$\xB7:2\xB7:*7\xB5\xB2\xB7!:\xB97`i\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x02\xACV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x02\xAC\x94\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x94\x90\x94R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x84\x01R`D\x83\x01R`d\x82\x01R`\x84\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x03{W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x03\x93W`\0\x80\xFD[a\x02K\x83\x83a\x03iV[`\0` \x82\x84\x03\x12\x15a\x03\xAFW`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x03{W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xDAW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04\x0BWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x915\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x04*W`\0\x80\xFD[a\x02K\x83\x83a\x03\xC8V[`\0`\x80\x82\x84\x03\x12\x15a\x04FW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04wWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x825\x81R` \x83\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x98W`\0\x80\xFD[` \x82\x01R`@\x83\x81\x015\x90\x82\x01R``\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x04\xCCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02KW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 =\x15\x8CP\xAF\\#\x991\x80o:>\x8F,\xE9\xF5\x88\xAD\x98o\xB5gIA@\xA9N\xB8\xCA\x8C.dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKINTENTEVENTPROVERANDVERIFIER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\xB2\r\xC0\x07\x11a\0[W\x80c\xB2\r\xC0\x07\x14a\0\xFFW\x80c\xB8l\x94\xB2\x14a\x01\x12W\x80c\xD6k\"\xC8\x14a\x01%W\x80c\xEB\x8Eq-\x14a\x01NW`\0\x80\xFD[\x80c\x13\xD8\xB9T\x14a\0\x8DW\x80c]\x06\x80\xDC\x14a\0\xA2W\x80cu\xE3f\x16\x14a\0\xB5W\x80c~\xB6 \xA7\x14a\0\xECW[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\x03\x81V[a\x01aV[\0[a\0\xA0a\0\xB06`\x04a\x03\x81V[a\x01\x9FV[a\0\xD8a\0\xC36`\x04a\x03\x9DV[`\0\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\0\xFA6`\x04a\x03\xB6V[a\x01\xB8V[a\0\xD8a\x01\r6`\x04a\x03\x81V[a\x01\xD1V[a\0\xD8a\x01 6`\x04a\x03\x81V[a\x02RV[a\0\xA0a\x0136`\x04a\x03\x9DV[`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\0\xD8a\x01\\6`\x04a\x03\xB6V[a\x02gV[`\0a\x01za\x01u6\x84\x90\x03\x84\x01\x84a\x04\x18V[a\x02|V[\x90Pa\x01\x9B\x81`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[PPV[`\0a\x01za\x01\xB36\x84\x90\x03\x84\x01\x84a\x04\x18V[a\x02\xC9V[`\0a\x01za\x01\xCC6\x84\x90\x03\x84\x01\x84a\x044V[a\x02\xFDV[`\0\x80a\x01\xE6a\x01\xB36\x85\x90\x03\x85\x01\x85a\x04\x18V[`@Qc:\xF1\xB3\x0B`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P0\x90cu\xE3f\x16\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02K\x91\x90a\x04\xBAV[\x93\x92PPPV[`\0\x80a\x01\xE6a\x01u6\x85\x90\x03\x85\x01\x85a\x04\x18V[`\0\x80a\x01\xE6a\x01\xCC6\x85\x90\x03\x85\x01\x85a\x044V[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80Q`@Qr)\xBB\xB0\xB8$\xB7:2\xB7:*7\xB5\xB2\xB7!:\xB97`i\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x02\xACV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x02\xAC\x94\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x94\x90\x94R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x84\x01R`D\x83\x01R`d\x82\x01R`\x84\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x03{W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x03\x93W`\0\x80\xFD[a\x02K\x83\x83a\x03iV[`\0` \x82\x84\x03\x12\x15a\x03\xAFW`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x03{W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xDAW`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04\x0BWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x915\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x04*W`\0\x80\xFD[a\x02K\x83\x83a\x03\xC8V[`\0`\x80\x82\x84\x03\x12\x15a\x04FW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x04wWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x825\x81R` \x83\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\x98W`\0\x80\xFD[` \x82\x01R`@\x83\x81\x015\x90\x82\x01R``\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x04\xCCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02KW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 =\x15\x8CP\xAF\\#\x991\x80o:>\x8F,\xE9\xF5\x88\xAD\x98o\xB5gIA@\xA9N\xB8\xCA\x8C.dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKINTENTEVENTPROVERANDVERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockIntentEventProverAndVerifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockIntentEventProverAndVerifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockIntentEventProverAndVerifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockIntentEventProverAndVerifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockIntentEventProverAndVerifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockIntentEventProverAndVerifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockIntentEventProverAndVerifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKINTENTEVENTPROVERANDVERIFIER_ABI.clone(),
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
                MOCKINTENTEVENTPROVERANDVERIFIER_ABI.clone(),
                MOCKINTENTEVENTPROVERANDVERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `verify` (0x75e36616) function
        pub fn verify(
            &self,
            event_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([117, 227, 102, 22], event_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySwapIntentFilledEvent` (0xeb8e712d) function
        pub fn verify_swap_intent_filled_event(
            &self,
            event: SwapIntentFilled,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([235, 142, 113, 45], (event,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySwapIntentTokenBurnEvent` (0xb20dc007) function
        pub fn verify_swap_intent_token_burn_event(
            &self,
            event: SwapIntentTokenBurn,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([178, 13, 192, 7], (event,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySwapIntentTokenLockEvent` (0xb86c94b2) function
        pub fn verify_swap_intent_token_lock_event(
            &self,
            event: SwapIntentTokenLock,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([184, 108, 148, 178], (event,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockIntentEventProverAndVerifier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all input parameters for the `verify` function with signature `verify(bytes32)` and selector `0x75e36616`
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
    #[ethcall(name = "verify", abi = "verify(bytes32)")]
    pub struct VerifyCall {
        pub event_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `verifySwapIntentFilledEvent` function with signature `verifySwapIntentFilledEvent((bytes32,address,uint256,uint256))` and selector `0xeb8e712d`
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
        name = "verifySwapIntentFilledEvent",
        abi = "verifySwapIntentFilledEvent((bytes32,address,uint256,uint256))"
    )]
    pub struct VerifySwapIntentFilledEventCall {
        pub event: SwapIntentFilled,
    }
    ///Container type for all input parameters for the `verifySwapIntentTokenBurnEvent` function with signature `verifySwapIntentTokenBurnEvent((bytes32))` and selector `0xb20dc007`
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
        name = "verifySwapIntentTokenBurnEvent",
        abi = "verifySwapIntentTokenBurnEvent((bytes32))"
    )]
    pub struct VerifySwapIntentTokenBurnEventCall {
        pub event: SwapIntentTokenBurn,
    }
    ///Container type for all input parameters for the `verifySwapIntentTokenLockEvent` function with signature `verifySwapIntentTokenLockEvent((bytes32))` and selector `0xb86c94b2`
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
        name = "verifySwapIntentTokenLockEvent",
        abi = "verifySwapIntentTokenLockEvent((bytes32))"
    )]
    pub struct VerifySwapIntentTokenLockEventCall {
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
    pub enum MockIntentEventProverAndVerifierCalls {
        RegisterEvent(RegisterEventCall),
        RegisterSwapIntentFilledEvent(RegisterSwapIntentFilledEventCall),
        RegisterSwapIntentTokenBurnEvent(RegisterSwapIntentTokenBurnEventCall),
        RegisterSwapIntentTokenLockEvent(RegisterSwapIntentTokenLockEventCall),
        Verify(VerifyCall),
        VerifySwapIntentFilledEvent(VerifySwapIntentFilledEventCall),
        VerifySwapIntentTokenBurnEvent(VerifySwapIntentTokenBurnEventCall),
        VerifySwapIntentTokenLockEvent(VerifySwapIntentTokenLockEventCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockIntentEventProverAndVerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <RegisterEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterEvent(decoded));
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
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Verify(decoded));
            }
            if let Ok(decoded) = <VerifySwapIntentFilledEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifySwapIntentFilledEvent(decoded));
            }
            if let Ok(decoded) = <VerifySwapIntentTokenBurnEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifySwapIntentTokenBurnEvent(decoded));
            }
            if let Ok(decoded) = <VerifySwapIntentTokenLockEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifySwapIntentTokenLockEvent(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockIntentEventProverAndVerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::RegisterEvent(element) => {
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
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifySwapIntentFilledEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifySwapIntentTokenBurnEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifySwapIntentTokenLockEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockIntentEventProverAndVerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RegisterEvent(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterSwapIntentFilledEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterSwapIntentTokenBurnEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterSwapIntentTokenLockEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifySwapIntentFilledEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifySwapIntentTokenBurnEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifySwapIntentTokenLockEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<RegisterEventCall>
    for MockIntentEventProverAndVerifierCalls {
        fn from(value: RegisterEventCall) -> Self {
            Self::RegisterEvent(value)
        }
    }
    impl ::core::convert::From<RegisterSwapIntentFilledEventCall>
    for MockIntentEventProverAndVerifierCalls {
        fn from(value: RegisterSwapIntentFilledEventCall) -> Self {
            Self::RegisterSwapIntentFilledEvent(value)
        }
    }
    impl ::core::convert::From<RegisterSwapIntentTokenBurnEventCall>
    for MockIntentEventProverAndVerifierCalls {
        fn from(value: RegisterSwapIntentTokenBurnEventCall) -> Self {
            Self::RegisterSwapIntentTokenBurnEvent(value)
        }
    }
    impl ::core::convert::From<RegisterSwapIntentTokenLockEventCall>
    for MockIntentEventProverAndVerifierCalls {
        fn from(value: RegisterSwapIntentTokenLockEventCall) -> Self {
            Self::RegisterSwapIntentTokenLockEvent(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for MockIntentEventProverAndVerifierCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifySwapIntentFilledEventCall>
    for MockIntentEventProverAndVerifierCalls {
        fn from(value: VerifySwapIntentFilledEventCall) -> Self {
            Self::VerifySwapIntentFilledEvent(value)
        }
    }
    impl ::core::convert::From<VerifySwapIntentTokenBurnEventCall>
    for MockIntentEventProverAndVerifierCalls {
        fn from(value: VerifySwapIntentTokenBurnEventCall) -> Self {
            Self::VerifySwapIntentTokenBurnEvent(value)
        }
    }
    impl ::core::convert::From<VerifySwapIntentTokenLockEventCall>
    for MockIntentEventProverAndVerifierCalls {
        fn from(value: VerifySwapIntentTokenLockEventCall) -> Self {
            Self::VerifySwapIntentTokenLockEvent(value)
        }
    }
    ///Container type for all return fields from the `verify` function with signature `verify(bytes32)` and selector `0x75e36616`
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
    pub struct VerifyReturn(pub bool);
    ///Container type for all return fields from the `verifySwapIntentFilledEvent` function with signature `verifySwapIntentFilledEvent((bytes32,address,uint256,uint256))` and selector `0xeb8e712d`
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
    pub struct VerifySwapIntentFilledEventReturn(pub bool);
    ///Container type for all return fields from the `verifySwapIntentTokenBurnEvent` function with signature `verifySwapIntentTokenBurnEvent((bytes32))` and selector `0xb20dc007`
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
    pub struct VerifySwapIntentTokenBurnEventReturn(pub bool);
    ///Container type for all return fields from the `verifySwapIntentTokenLockEvent` function with signature `verifySwapIntentTokenLockEvent((bytes32))` and selector `0xb86c94b2`
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
    pub struct VerifySwapIntentTokenLockEventReturn(pub bool);
}
