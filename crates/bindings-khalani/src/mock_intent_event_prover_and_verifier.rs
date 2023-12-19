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
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SpokeChainCallEventLibrary.SpokeCalled",
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
                    ::std::borrow::ToOwned::to_owned("verifySpokeCalledEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verifySpokeCalledEvent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("event_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SpokeChainCallEventLibrary.SpokeCalled",
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x07\xB8\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\xA8\xE8\xFEY\x11a\0fW\x80c\xA8\xE8\xFEY\x14a\x01(W\x80c\xB2\r\xC0\x07\x14a\x01;W\x80c\xB8l\x94\xB2\x14a\x01NW\x80c\xD6k\"\xC8\x14a\x01aW\x80c\xEB\x8Eq-\x14a\x01\x8AW`\0\x80\xFD[\x80c\x13\xD8\xB9T\x14a\0\xA3W\x80cLi\x85K\x14a\0\xB8W\x80c]\x06\x80\xDC\x14a\0\xCBW\x80cu\xE3f\x16\x14a\0\xDEW\x80c~\xB6 \xA7\x14a\x01\x15W[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x04\x0FV[a\x01\x9DV[\0[a\0\xB6a\0\xC66`\x04a\x04+V[a\x01\xDBV[a\0\xB6a\0\xD96`\x04a\x04\x0FV[a\x01\xEEV[a\x01\x01a\0\xEC6`\x04a\x04fV[`\0\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\x01#6`\x04a\x04\x7FV[a\x02\x07V[a\x01\x01a\x0166`\x04a\x04+V[a\x02 V[a\x01\x01a\x01I6`\x04a\x04\x0FV[a\x02\x9BV[a\x01\x01a\x01\\6`\x04a\x04\x0FV[a\x02\xB0V[a\0\xB6a\x01o6`\x04a\x04fV[`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x01\x01a\x01\x986`\x04a\x04\x7FV[a\x02\xC5V[`\0a\x01\xB6a\x01\xB16\x84\x90\x03\x84\x01\x84a\x05CV[a\x02\xDAV[\x90Pa\x01\xD7\x81`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[PPV[`\0a\x01\xB6a\x01\xE9\x83a\x05{V[a\x03'V[`\0a\x01\xB6a\x02\x026\x84\x90\x03\x84\x01\x84a\x05CV[a\x03WV[`\0a\x01\xB6a\x02\x1B6\x84\x90\x03\x84\x01\x84a\x06_V[a\x03\x8BV[`\0\x80a\x02/a\x01\xE9\x84a\x05{V[`@Qc:\xF1\xB3\x0B`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P0\x90cu\xE3f\x16\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x94\x91\x90a\x06\xCAV[\x93\x92PPPV[`\0\x80a\x02/a\x02\x026\x85\x90\x03\x85\x01\x85a\x05CV[`\0\x80a\x02/a\x01\xB16\x85\x90\x03\x85\x01\x85a\x05CV[`\0\x80a\x02/a\x02\x1B6\x85\x90\x03\x85\x01\x85a\x06_V[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Q`\0\x97a\x03\n\x97\x90\x96\x95\x91\x01a\x06\xECV[\x80Q`@Qr)\xBB\xB0\xB8$\xB7:2\xB7:*7\xB5\xB2\xB7!:\xB97`i\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x03\nV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x03\n\x94\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x94\x90\x94R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x84\x01R`D\x83\x01R`d\x82\x01R`\x84\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x04\tW`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x04!W`\0\x80\xFD[a\x02\x94\x83\x83a\x03\xF7V[`\0` \x82\x84\x03\x12\x15a\x04=W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04TW`\0\x80\xFD[\x82\x01`\xC0\x81\x85\x03\x12\x15a\x02\x94W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x04xW`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x04\tW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xCAWa\x04\xCAa\x04\x91V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xF9Wa\x04\xF9a\x04\x91V[`@R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\x13W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x056Wa\x056a\x04\x91V[`@R\x915\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05UW`\0\x80\xFD[a\x02\x94\x83\x83a\x05\x01V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05vW`\0\x80\xFD[\x91\x90PV[`\0`\xC0\x826\x03\x12\x15a\x05\x8DW`\0\x80\xFD[a\x05\x95a\x04\xA7V[a\x05\x9E\x83a\x05_V[\x81R` \x80\x84\x015\x81\x83\x01Ra\x05\xB6`@\x85\x01a\x05_V[`@\x83\x01R``\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xD6W`\0\x80\xFD[\x90\x85\x01\x906`\x1F\x83\x01\x12a\x05\xE9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\xFBWa\x05\xFBa\x04\x91V[a\x06\r`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x04\xD0V[\x91P\x80\x82R6\x84\x82\x85\x01\x01\x11\x15a\x06#W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80``\x85\x01RPPPa\x06I`\x80\x84\x01a\x05_V[`\x80\x82\x01R`\xA0\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0`\x80\x82\x84\x03\x12\x15a\x06qW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\x94Wa\x06\x94a\x04\x91V[`@R\x825\x81Ra\x06\xA7` \x84\x01a\x05_V[` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x06\xDCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02\x94W`\0\x80\xFD[j\x14\xDC\x1B\xDA\xD9P\xD8[\x1B\x19Y`\xAA\x1B\x81R`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x89``\x1B\x16`\x0B\x84\x01R\x87`\x1F\x84\x01R\x80\x87``\x1B\x16`?\x84\x01RP\x84Q`\0[\x81\x81\x10\x15a\x07LW` \x81\x88\x01\x81\x01Q`S\x86\x84\x01\x01R\x01a\x07/V[P``\x94\x90\x94\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x93\x01`S\x81\x01\x91\x90\x91R`g\x81\x01\x91\x90\x91R`\x87\x01\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \x93\xC0\xBA\x0CK\x0Cc\x83\xDD|\\]p\x94\x9C&~\xDA:;B\x1D\xFF\xE3y0%\x90\x86\nx\xF4dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKINTENTEVENTPROVERANDVERIFIER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\xA8\xE8\xFEY\x11a\0fW\x80c\xA8\xE8\xFEY\x14a\x01(W\x80c\xB2\r\xC0\x07\x14a\x01;W\x80c\xB8l\x94\xB2\x14a\x01NW\x80c\xD6k\"\xC8\x14a\x01aW\x80c\xEB\x8Eq-\x14a\x01\x8AW`\0\x80\xFD[\x80c\x13\xD8\xB9T\x14a\0\xA3W\x80cLi\x85K\x14a\0\xB8W\x80c]\x06\x80\xDC\x14a\0\xCBW\x80cu\xE3f\x16\x14a\0\xDEW\x80c~\xB6 \xA7\x14a\x01\x15W[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x04\x0FV[a\x01\x9DV[\0[a\0\xB6a\0\xC66`\x04a\x04+V[a\x01\xDBV[a\0\xB6a\0\xD96`\x04a\x04\x0FV[a\x01\xEEV[a\x01\x01a\0\xEC6`\x04a\x04fV[`\0\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\x01#6`\x04a\x04\x7FV[a\x02\x07V[a\x01\x01a\x0166`\x04a\x04+V[a\x02 V[a\x01\x01a\x01I6`\x04a\x04\x0FV[a\x02\x9BV[a\x01\x01a\x01\\6`\x04a\x04\x0FV[a\x02\xB0V[a\0\xB6a\x01o6`\x04a\x04fV[`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x01\x01a\x01\x986`\x04a\x04\x7FV[a\x02\xC5V[`\0a\x01\xB6a\x01\xB16\x84\x90\x03\x84\x01\x84a\x05CV[a\x02\xDAV[\x90Pa\x01\xD7\x81`\0\x90\x81R` \x81\x90R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[PPV[`\0a\x01\xB6a\x01\xE9\x83a\x05{V[a\x03'V[`\0a\x01\xB6a\x02\x026\x84\x90\x03\x84\x01\x84a\x05CV[a\x03WV[`\0a\x01\xB6a\x02\x1B6\x84\x90\x03\x84\x01\x84a\x06_V[a\x03\x8BV[`\0\x80a\x02/a\x01\xE9\x84a\x05{V[`@Qc:\xF1\xB3\x0B`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P0\x90cu\xE3f\x16\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02pW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x94\x91\x90a\x06\xCAV[\x93\x92PPPV[`\0\x80a\x02/a\x02\x026\x85\x90\x03\x85\x01\x85a\x05CV[`\0\x80a\x02/a\x01\xB16\x85\x90\x03\x85\x01\x85a\x05CV[`\0\x80a\x02/a\x02\x1B6\x85\x90\x03\x85\x01\x85a\x06_V[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80Q` \x80\x83\x01Q`@\x80\x85\x01Q``\x86\x01Q`\x80\x87\x01Q`\xA0\x88\x01Q\x93Q`\0\x97a\x03\n\x97\x90\x96\x95\x91\x01a\x06\xECV[\x80Q`@Qr)\xBB\xB0\xB8$\xB7:2\xB7:*7\xB5\xB2\xB7!:\xB97`i\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x03\nV[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x03\n\x94\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x94\x90\x94R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x84\x01R`D\x83\x01R`d\x82\x01R`\x84\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x04\tW`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x04!W`\0\x80\xFD[a\x02\x94\x83\x83a\x03\xF7V[`\0` \x82\x84\x03\x12\x15a\x04=W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04TW`\0\x80\xFD[\x82\x01`\xC0\x81\x85\x03\x12\x15a\x02\x94W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x04xW`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x84\x03\x12\x15a\x04\tW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xCAWa\x04\xCAa\x04\x91V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\xF9Wa\x04\xF9a\x04\x91V[`@R\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\x13W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x056Wa\x056a\x04\x91V[`@R\x915\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05UW`\0\x80\xFD[a\x02\x94\x83\x83a\x05\x01V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05vW`\0\x80\xFD[\x91\x90PV[`\0`\xC0\x826\x03\x12\x15a\x05\x8DW`\0\x80\xFD[a\x05\x95a\x04\xA7V[a\x05\x9E\x83a\x05_V[\x81R` \x80\x84\x015\x81\x83\x01Ra\x05\xB6`@\x85\x01a\x05_V[`@\x83\x01R``\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xD6W`\0\x80\xFD[\x90\x85\x01\x906`\x1F\x83\x01\x12a\x05\xE9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x05\xFBWa\x05\xFBa\x04\x91V[a\x06\r`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x04\xD0V[\x91P\x80\x82R6\x84\x82\x85\x01\x01\x11\x15a\x06#W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80``\x85\x01RPPPa\x06I`\x80\x84\x01a\x05_V[`\x80\x82\x01R`\xA0\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0`\x80\x82\x84\x03\x12\x15a\x06qW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x06\x94Wa\x06\x94a\x04\x91V[`@R\x825\x81Ra\x06\xA7` \x84\x01a\x05_V[` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x06\xDCW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02\x94W`\0\x80\xFD[j\x14\xDC\x1B\xDA\xD9P\xD8[\x1B\x19Y`\xAA\x1B\x81R`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x89``\x1B\x16`\x0B\x84\x01R\x87`\x1F\x84\x01R\x80\x87``\x1B\x16`?\x84\x01RP\x84Q`\0[\x81\x81\x10\x15a\x07LW` \x81\x88\x01\x81\x01Q`S\x86\x84\x01\x01R\x01a\x07/V[P``\x94\x90\x94\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x91\x90\x93\x01`S\x81\x01\x91\x90\x91R`g\x81\x01\x91\x90\x91R`\x87\x01\x95\x94PPPPPV\xFE\xA2dipfsX\"\x12 \x93\xC0\xBA\x0CK\x0Cc\x83\xDD|\\]p\x94\x9C&~\xDA:;B\x1D\xFF\xE3y0%\x90\x86\nx\xF4dsolcC\0\x08\x13\x003";
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
        ///Calls the contract's `registerSpokeCalledEvent` (0x4c69854b) function
        pub fn register_spoke_called_event(
            &self,
            event: SpokeCalled,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 105, 133, 75], (event,))
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
        ///Calls the contract's `verifySpokeCalledEvent` (0xa8e8fe59) function
        pub fn verify_spoke_called_event(
            &self,
            event: SpokeCalled,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([168, 232, 254, 89], (event,))
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
    ///Container type for all input parameters for the `registerSpokeCalledEvent` function with signature `registerSpokeCalledEvent((address,bytes32,address,bytes,address,uint256))` and selector `0x4c69854b`
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
        abi = "registerSpokeCalledEvent((address,bytes32,address,bytes,address,uint256))"
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
    ///Container type for all input parameters for the `verifySpokeCalledEvent` function with signature `verifySpokeCalledEvent((address,bytes32,address,bytes,address,uint256))` and selector `0xa8e8fe59`
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
        name = "verifySpokeCalledEvent",
        abi = "verifySpokeCalledEvent((address,bytes32,address,bytes,address,uint256))"
    )]
    pub struct VerifySpokeCalledEventCall {
        pub event: SpokeCalled,
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
        RegisterSpokeCalledEvent(RegisterSpokeCalledEventCall),
        RegisterSwapIntentFilledEvent(RegisterSwapIntentFilledEventCall),
        RegisterSwapIntentTokenBurnEvent(RegisterSwapIntentTokenBurnEventCall),
        RegisterSwapIntentTokenLockEvent(RegisterSwapIntentTokenLockEventCall),
        Verify(VerifyCall),
        VerifySpokeCalledEvent(VerifySpokeCalledEventCall),
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
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Verify(decoded));
            }
            if let Ok(decoded) = <VerifySpokeCalledEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifySpokeCalledEvent(decoded));
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
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifySpokeCalledEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifySpokeCalledEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<RegisterSpokeCalledEventCall>
    for MockIntentEventProverAndVerifierCalls {
        fn from(value: RegisterSpokeCalledEventCall) -> Self {
            Self::RegisterSpokeCalledEvent(value)
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
    impl ::core::convert::From<VerifySpokeCalledEventCall>
    for MockIntentEventProverAndVerifierCalls {
        fn from(value: VerifySpokeCalledEventCall) -> Self {
            Self::VerifySpokeCalledEvent(value)
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
    ///Container type for all return fields from the `verifySpokeCalledEvent` function with signature `verifySpokeCalledEvent((address,bytes32,address,bytes,address,uint256))` and selector `0xa8e8fe59`
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
    pub struct VerifySpokeCalledEventReturn(pub bool);
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
