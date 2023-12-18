pub use spoke_chain_call_intent_book::*;
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
pub mod spoke_chain_call_intent_book {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("cancelIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cancelIntent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
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
                    ::std::borrow::ToOwned::to_owned("intentData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("intentData"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("author"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("contractToCall"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("intents"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("intents"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("status"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum BaseIntentBook.IntentStatus",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentBidId"),
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
                    ::std::borrow::ToOwned::to_owned("matchIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("matchIntent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentBid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![::ethers::core::abi::ethabi::ParamType::Bytes],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IntentBookLib.IntentBid",
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
                    ::std::borrow::ToOwned::to_owned("placeIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("placeIntent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IntentBookLib.Intent",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("settleIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("settleIntent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IntentBidReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IntentBidReceived"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentBidId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentBid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![::ethers::core::abi::ethabi::ParamType::Bytes],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IntentCancelled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IntentCancelled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
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
                    ::std::borrow::ToOwned::to_owned("IntentCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IntentCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IntentMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IntentMatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentBidId"),
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
                    ::std::borrow::ToOwned::to_owned("IntentSettled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("IntentSettled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentBidId"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SPOKECHAINCALLINTENTBOOK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0C=\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80cJ\xF26N\x14a\0gW\x80cY\xA8D\xB4\x14a\0\x8DW\x80cr\xB8v\xEB\x14a\0\xB2W\x80c{\xF8\xBB\x88\x14a\0\xC7W\x80c\x90!W\x8A\x14a\0\xDAW\x80c\xD5_\x96\r\x14a\x01\x15W[`\0\x80\xFD[a\0za\0u6`\x04a\x05\xCCV[a\x01(V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\0\x9B6`\x04a\x06\x0EV[a\x02FV[`@Qa\0\x84\x96\x95\x94\x93\x92\x91\x90a\x06KV[a\0\xC5a\0\xC06`\x04a\x06\xB5V[a\x03$V[\0[a\0\xC5a\0\xD56`\x04a\x06\x0EV[a\x03tV[a\x01\x07a\0\xE86`\x04a\x06\x0EV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\x84\x92\x91\x90a\x07\x19V[a\0\xC5a\x01#6`\x04a\x06\x0EV[a\x04\x17V[`\0a\x01;a\x016\x83a\x08\x11V[a\x04\xD2V[\x90P`\0\x80\x82\x81R` \x81\x90R`@\x90 T`\xFF\x16`\x05\x81\x11\x15a\x01aWa\x01aa\x07\x03V[\x14a\x01\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x05\x81\x11\x15a\x01\xEDWa\x01\xEDa\x07\x03V[\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U\x90PPa\x02\t\x82\x82a\x05\x0CV[\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x029\x91\x90a\t\rV[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\x01` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T\x91\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16\x93`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x92\x91\x90a\x02\x82\x90a\tcV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xAE\x90a\tcV[\x80\x15a\x02\xFBW\x80`\x1F\x10a\x02\xD0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xFBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xDEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T\x92\x93`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x91\x16\x91P\x86V[`\0\x82\x81R` \x81\x90R`@\x80\x82 \x80T`\xFF\x19\x16`\x03\x17\x81U`\x01\x81\x01T\x91Q\x90\x92\x82\x91\x86\x91\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x91\xA3PPPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x03\x81T`\xFF\x16`\x05\x81\x11\x15a\x03\x9AWa\x03\x9Aa\x07\x03V[\x14a\x03\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x01\xA2V[\x80T`\xFF\x19\x16`\x04\x17\x81U`\x01\x81\x01T`@Q\x81\x90\x84\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3PPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81T`\xFF\x16`\x05\x81\x11\x15a\x04=Wa\x04=a\x07\x03V[\x14\x80a\x04^WP`\x02\x81T`\xFF\x16`\x05\x81\x11\x15a\x04\\Wa\x04\\a\x07\x03V[\x14[a\x04\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x01\xA2V[\x80T`\xFF\x19\x16`\x05\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x04\xEF\x92\x91\x90a\t\x9DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0a\x05\x18\x83\x80a\t\xCCV[\x81\x01\x90a\x05%\x91\x90a\n/V[`\0\x83\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x84Q\x81T\x92\x86\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x90\x91\x17\x81U\x91\x83\x01Q\x92\x93P\x83\x92\x90\x82\x01\x90a\x05\x80\x90\x82a\x0BGV[P``\x82\x01Q`\x02\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x80\x84\x01Q`\x03\x84\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U`\xA0\x90\x91\x01Q`\x04\x90\x91\x01UPPPV[`\0` \x82\x84\x03\x12\x15a\x05\xDEW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xF5W`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x06\x07W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06 W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x06BW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06*V[PP`\0\x91\x01RV[`\0`\x01\x80`\xA0\x1B\x03\x80\x89\x16\x83Rc\xFF\xFF\xFF\xFF\x88\x16` \x84\x01R`\xC0`@\x84\x01R\x86Q\x80`\xC0\x85\x01Ra\x06\x85\x81`\xE0\x86\x01` \x8B\x01a\x06'V[\x95\x81\x16``\x84\x01R\x93\x90\x93\x16`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x90\x91\x01`\xE0\x01\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xC8W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xE6W`\0\x80\xFD[\x83\x01` \x81\x86\x03\x12\x15a\x06\xF8W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x06\x84\x10a\x07;WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07~Wa\x07~a\x07EV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\x95W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xB0Wa\x07\xB0a\x07EV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x07\xD8Wa\x07\xD8a\x07EV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x07\xF1W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`@\x826\x03\x12\x15a\x08#W`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x08GWa\x08Ga\x07EV[\x81`@R\x845\x91P\x80\x82\x11\x15a\x08\\W`\0\x80\xFD[a\x08h6\x83\x87\x01a\x07\x84V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x08~W`\0\x80\xFD[Pa\x08\x8B6\x82\x86\x01a\x07\x84V[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x08\xAEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xCEW`\0\x80\xFD[\x806\x03\x82\x13\x15a\x08\xDDW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\t\x1D\x83\x84a\x08\x97V[`@` \x85\x01Ra\t2``\x85\x01\x82\x84a\x08\xE4V[\x91PPa\tB` \x85\x01\x85a\x08\x97V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\tY\x83\x82\x84a\x08\xE4V[\x96\x95PPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\twW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\t\x97WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x83Qa\t\xAF\x81\x84` \x88\x01a\x06'V[\x83Q\x90\x83\x01\x90a\t\xC3\x81\x83` \x88\x01a\x06'V[\x01\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\t\xE3W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\t\xFEW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x08\xDDW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n*W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\nAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\nYW`\0\x80\xFD[\x90\x83\x01\x90`\xC0\x82\x86\x03\x12\x15a\nmW`\0\x80\xFD[a\nua\x07[V[a\n~\x83a\n\x13V[\x81R` \x83\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\n\x97W`\0\x80\xFD[` \x82\x01R`@\x83\x015\x82\x81\x11\x15a\n\xAEW`\0\x80\xFD[a\n\xBA\x87\x82\x86\x01a\x07\x84V[`@\x83\x01RPa\n\xCC``\x84\x01a\n\x13V[``\x82\x01Ra\n\xDD`\x80\x84\x01a\n\x13V[`\x80\x82\x01R`\xA0\x83\x015`\xA0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\x1F\x82\x11\x15a\x0BBW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0B\x1FWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0B>W\x82\x81U`\x01\x01a\x0B+V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BaWa\x0Baa\x07EV[a\x0Bu\x81a\x0Bo\x84Ta\tcV[\x84a\n\xF8V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0B\xAAW`\0\x84\x15a\x0B\x92WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0B>V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x0B\xD9W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0B\xBAV[P\x85\x82\x10\x15a\x0B\xF7W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 +\x17\xC7\xDE!\x897Q(R\xDF\xCC>\xA0\xBB\x17\xC2\\n\xC7\x83u|\0\x01A\x93m2\xE5\xA2\xF7dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SPOKECHAINCALLINTENTBOOK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80cJ\xF26N\x14a\0gW\x80cY\xA8D\xB4\x14a\0\x8DW\x80cr\xB8v\xEB\x14a\0\xB2W\x80c{\xF8\xBB\x88\x14a\0\xC7W\x80c\x90!W\x8A\x14a\0\xDAW\x80c\xD5_\x96\r\x14a\x01\x15W[`\0\x80\xFD[a\0za\0u6`\x04a\x05\xCCV[a\x01(V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\0\x9B6`\x04a\x06\x0EV[a\x02FV[`@Qa\0\x84\x96\x95\x94\x93\x92\x91\x90a\x06KV[a\0\xC5a\0\xC06`\x04a\x06\xB5V[a\x03$V[\0[a\0\xC5a\0\xD56`\x04a\x06\x0EV[a\x03tV[a\x01\x07a\0\xE86`\x04a\x06\x0EV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\x84\x92\x91\x90a\x07\x19V[a\0\xC5a\x01#6`\x04a\x06\x0EV[a\x04\x17V[`\0a\x01;a\x016\x83a\x08\x11V[a\x04\xD2V[\x90P`\0\x80\x82\x81R` \x81\x90R`@\x90 T`\xFF\x16`\x05\x81\x11\x15a\x01aWa\x01aa\x07\x03V[\x14a\x01\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x05\x81\x11\x15a\x01\xEDWa\x01\xEDa\x07\x03V[\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U\x90PPa\x02\t\x82\x82a\x05\x0CV[\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x029\x91\x90a\t\rV[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\x01` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T\x91\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16\x93`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x92\x91\x90a\x02\x82\x90a\tcV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xAE\x90a\tcV[\x80\x15a\x02\xFBW\x80`\x1F\x10a\x02\xD0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xFBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xDEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x02\x83\x01T`\x03\x84\x01T`\x04\x90\x94\x01T\x92\x93`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x91\x16\x91P\x86V[`\0\x82\x81R` \x81\x90R`@\x80\x82 \x80T`\xFF\x19\x16`\x03\x17\x81U`\x01\x81\x01T\x91Q\x90\x92\x82\x91\x86\x91\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x91\xA3PPPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x03\x81T`\xFF\x16`\x05\x81\x11\x15a\x03\x9AWa\x03\x9Aa\x07\x03V[\x14a\x03\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x01\xA2V[\x80T`\xFF\x19\x16`\x04\x17\x81U`\x01\x81\x01T`@Q\x81\x90\x84\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3PPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81T`\xFF\x16`\x05\x81\x11\x15a\x04=Wa\x04=a\x07\x03V[\x14\x80a\x04^WP`\x02\x81T`\xFF\x16`\x05\x81\x11\x15a\x04\\Wa\x04\\a\x07\x03V[\x14[a\x04\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x01\xA2V[\x80T`\xFF\x19\x16`\x05\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x04\xEF\x92\x91\x90a\t\x9DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0a\x05\x18\x83\x80a\t\xCCV[\x81\x01\x90a\x05%\x91\x90a\n/V[`\0\x83\x81R`\x01` \x81\x81R`@\x92\x83\x90 \x84Q\x81T\x92\x86\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x90\x91\x17\x81U\x91\x83\x01Q\x92\x93P\x83\x92\x90\x82\x01\x90a\x05\x80\x90\x82a\x0BGV[P``\x82\x01Q`\x02\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x80\x84\x01Q`\x03\x84\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U`\xA0\x90\x91\x01Q`\x04\x90\x91\x01UPPPV[`\0` \x82\x84\x03\x12\x15a\x05\xDEW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xF5W`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x06\x07W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x06 W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x06BW\x81\x81\x01Q\x83\x82\x01R` \x01a\x06*V[PP`\0\x91\x01RV[`\0`\x01\x80`\xA0\x1B\x03\x80\x89\x16\x83Rc\xFF\xFF\xFF\xFF\x88\x16` \x84\x01R`\xC0`@\x84\x01R\x86Q\x80`\xC0\x85\x01Ra\x06\x85\x81`\xE0\x86\x01` \x8B\x01a\x06'V[\x95\x81\x16``\x84\x01R\x93\x90\x93\x16`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x90\x91\x01`\xE0\x01\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xC8W`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xE6W`\0\x80\xFD[\x83\x01` \x81\x86\x03\x12\x15a\x06\xF8W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x06\x84\x10a\x07;WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07~Wa\x07~a\x07EV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\x95W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xB0Wa\x07\xB0a\x07EV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x07\xD8Wa\x07\xD8a\x07EV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x07\xF1W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`@\x826\x03\x12\x15a\x08#W`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\x08GWa\x08Ga\x07EV[\x81`@R\x845\x91P\x80\x82\x11\x15a\x08\\W`\0\x80\xFD[a\x08h6\x83\x87\x01a\x07\x84V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x08~W`\0\x80\xFD[Pa\x08\x8B6\x82\x86\x01a\x07\x84V[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x08\xAEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xCEW`\0\x80\xFD[\x806\x03\x82\x13\x15a\x08\xDDW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\t\x1D\x83\x84a\x08\x97V[`@` \x85\x01Ra\t2``\x85\x01\x82\x84a\x08\xE4V[\x91PPa\tB` \x85\x01\x85a\x08\x97V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\tY\x83\x82\x84a\x08\xE4V[\x96\x95PPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\twW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\t\x97WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x83Qa\t\xAF\x81\x84` \x88\x01a\x06'V[\x83Q\x90\x83\x01\x90a\t\xC3\x81\x83` \x88\x01a\x06'V[\x01\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\t\xE3W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\t\xFEW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x08\xDDW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n*W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\nAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\nYW`\0\x80\xFD[\x90\x83\x01\x90`\xC0\x82\x86\x03\x12\x15a\nmW`\0\x80\xFD[a\nua\x07[V[a\n~\x83a\n\x13V[\x81R` \x83\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\n\x97W`\0\x80\xFD[` \x82\x01R`@\x83\x015\x82\x81\x11\x15a\n\xAEW`\0\x80\xFD[a\n\xBA\x87\x82\x86\x01a\x07\x84V[`@\x83\x01RPa\n\xCC``\x84\x01a\n\x13V[``\x82\x01Ra\n\xDD`\x80\x84\x01a\n\x13V[`\x80\x82\x01R`\xA0\x83\x015`\xA0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\x1F\x82\x11\x15a\x0BBW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0B\x1FWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0B>W\x82\x81U`\x01\x01a\x0B+V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BaWa\x0Baa\x07EV[a\x0Bu\x81a\x0Bo\x84Ta\tcV[\x84a\n\xF8V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0B\xAAW`\0\x84\x15a\x0B\x92WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0B>V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x0B\xD9W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0B\xBAV[P\x85\x82\x10\x15a\x0B\xF7W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 +\x17\xC7\xDE!\x897Q(R\xDF\xCC>\xA0\xBB\x17\xC2\\n\xC7\x83u|\0\x01A\x93m2\xE5\xA2\xF7dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static SPOKECHAINCALLINTENTBOOK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SpokeChainCallIntentBook<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SpokeChainCallIntentBook<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SpokeChainCallIntentBook<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SpokeChainCallIntentBook<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SpokeChainCallIntentBook<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SpokeChainCallIntentBook))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SpokeChainCallIntentBook<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SPOKECHAINCALLINTENTBOOK_ABI.clone(),
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
                SPOKECHAINCALLINTENTBOOK_ABI.clone(),
                SPOKECHAINCALLINTENTBOOK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `cancelIntent` (0xd55f960d) function
        pub fn cancel_intent(
            &self,
            intent_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 95, 150, 13], intent_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intentData` (0x59a844b4) function
        pub fn intent_data(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                u32,
                ::ethers::core::types::Bytes,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([89, 168, 68, 180], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intents` (0x9021578a) function
        pub fn intents(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (u8, [u8; 32])> {
            self.0
                .method_hash([144, 33, 87, 138], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchIntent` (0x72b876eb) function
        pub fn match_intent(
            &self,
            intent_id: [u8; 32],
            intent_bid: IntentBid,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 184, 118, 235], (intent_id, intent_bid))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `placeIntent` (0x4af2364e) function
        pub fn place_intent(
            &self,
            intent: Intent,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([74, 242, 54, 78], (intent,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settleIntent` (0x7bf8bb88) function
        pub fn settle_intent(
            &self,
            intent_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 248, 187, 136], intent_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `IntentBidReceived` event
        pub fn intent_bid_received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentBidReceivedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IntentCancelled` event
        pub fn intent_cancelled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentCancelledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IntentCreated` event
        pub fn intent_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IntentMatch` event
        pub fn intent_match_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentMatchFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `IntentSettled` event
        pub fn intent_settled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentSettledFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SpokeChainCallIntentBookEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SpokeChainCallIntentBook<M> {
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
        name = "IntentBidReceived",
        abi = "IntentBidReceived(bytes32,bytes32,(bytes))"
    )]
    pub struct IntentBidReceivedFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
        #[ethevent(indexed)]
        pub intent_bid_id: [u8; 32],
        pub intent_bid: IntentBid,
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
    #[ethevent(name = "IntentCancelled", abi = "IntentCancelled(bytes32)")]
    pub struct IntentCancelledFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
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
    #[ethevent(name = "IntentCreated", abi = "IntentCreated(bytes32,(bytes,bytes))")]
    pub struct IntentCreatedFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
        pub intent: Intent,
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
    #[ethevent(name = "IntentMatch", abi = "IntentMatch(bytes32,bytes32)")]
    pub struct IntentMatchFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
        #[ethevent(indexed)]
        pub intent_bid_id: [u8; 32],
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
    #[ethevent(name = "IntentSettled", abi = "IntentSettled(bytes32,bytes32)")]
    pub struct IntentSettledFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
        #[ethevent(indexed)]
        pub intent_bid_id: [u8; 32],
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
    pub enum SpokeChainCallIntentBookEvents {
        IntentBidReceivedFilter(IntentBidReceivedFilter),
        IntentCancelledFilter(IntentCancelledFilter),
        IntentCreatedFilter(IntentCreatedFilter),
        IntentMatchFilter(IntentMatchFilter),
        IntentSettledFilter(IntentSettledFilter),
    }
    impl ::ethers::contract::EthLogDecode for SpokeChainCallIntentBookEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = IntentBidReceivedFilter::decode_log(log) {
                return Ok(
                    SpokeChainCallIntentBookEvents::IntentBidReceivedFilter(decoded),
                );
            }
            if let Ok(decoded) = IntentCancelledFilter::decode_log(log) {
                return Ok(
                    SpokeChainCallIntentBookEvents::IntentCancelledFilter(decoded),
                );
            }
            if let Ok(decoded) = IntentCreatedFilter::decode_log(log) {
                return Ok(SpokeChainCallIntentBookEvents::IntentCreatedFilter(decoded));
            }
            if let Ok(decoded) = IntentMatchFilter::decode_log(log) {
                return Ok(SpokeChainCallIntentBookEvents::IntentMatchFilter(decoded));
            }
            if let Ok(decoded) = IntentSettledFilter::decode_log(log) {
                return Ok(SpokeChainCallIntentBookEvents::IntentSettledFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SpokeChainCallIntentBookEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IntentBidReceivedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntentCancelledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntentCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntentMatchFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentSettledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<IntentBidReceivedFilter>
    for SpokeChainCallIntentBookEvents {
        fn from(value: IntentBidReceivedFilter) -> Self {
            Self::IntentBidReceivedFilter(value)
        }
    }
    impl ::core::convert::From<IntentCancelledFilter>
    for SpokeChainCallIntentBookEvents {
        fn from(value: IntentCancelledFilter) -> Self {
            Self::IntentCancelledFilter(value)
        }
    }
    impl ::core::convert::From<IntentCreatedFilter> for SpokeChainCallIntentBookEvents {
        fn from(value: IntentCreatedFilter) -> Self {
            Self::IntentCreatedFilter(value)
        }
    }
    impl ::core::convert::From<IntentMatchFilter> for SpokeChainCallIntentBookEvents {
        fn from(value: IntentMatchFilter) -> Self {
            Self::IntentMatchFilter(value)
        }
    }
    impl ::core::convert::From<IntentSettledFilter> for SpokeChainCallIntentBookEvents {
        fn from(value: IntentSettledFilter) -> Self {
            Self::IntentSettledFilter(value)
        }
    }
    ///Container type for all input parameters for the `cancelIntent` function with signature `cancelIntent(bytes32)` and selector `0xd55f960d`
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
    #[ethcall(name = "cancelIntent", abi = "cancelIntent(bytes32)")]
    pub struct CancelIntentCall {
        pub intent_id: [u8; 32],
    }
    ///Container type for all input parameters for the `intentData` function with signature `intentData(bytes32)` and selector `0x59a844b4`
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
    #[ethcall(name = "intentData", abi = "intentData(bytes32)")]
    pub struct IntentDataCall(pub [u8; 32]);
    ///Container type for all input parameters for the `intents` function with signature `intents(bytes32)` and selector `0x9021578a`
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
    #[ethcall(name = "intents", abi = "intents(bytes32)")]
    pub struct IntentsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `matchIntent` function with signature `matchIntent(bytes32,(bytes))` and selector `0x72b876eb`
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
    #[ethcall(name = "matchIntent", abi = "matchIntent(bytes32,(bytes))")]
    pub struct MatchIntentCall {
        pub intent_id: [u8; 32],
        pub intent_bid: IntentBid,
    }
    ///Container type for all input parameters for the `placeIntent` function with signature `placeIntent((bytes,bytes))` and selector `0x4af2364e`
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
    #[ethcall(name = "placeIntent", abi = "placeIntent((bytes,bytes))")]
    pub struct PlaceIntentCall {
        pub intent: Intent,
    }
    ///Container type for all input parameters for the `settleIntent` function with signature `settleIntent(bytes32)` and selector `0x7bf8bb88`
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
    #[ethcall(name = "settleIntent", abi = "settleIntent(bytes32)")]
    pub struct SettleIntentCall {
        pub intent_id: [u8; 32],
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
    pub enum SpokeChainCallIntentBookCalls {
        CancelIntent(CancelIntentCall),
        IntentData(IntentDataCall),
        Intents(IntentsCall),
        MatchIntent(MatchIntentCall),
        PlaceIntent(PlaceIntentCall),
        SettleIntent(SettleIntentCall),
    }
    impl ::ethers::core::abi::AbiDecode for SpokeChainCallIntentBookCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CancelIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CancelIntent(decoded));
            }
            if let Ok(decoded) = <IntentDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntentData(decoded));
            }
            if let Ok(decoded) = <IntentsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Intents(decoded));
            }
            if let Ok(decoded) = <MatchIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MatchIntent(decoded));
            }
            if let Ok(decoded) = <PlaceIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PlaceIntent(decoded));
            }
            if let Ok(decoded) = <SettleIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SettleIntent(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SpokeChainCallIntentBookCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CancelIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntentData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Intents(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MatchIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PlaceIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SettleIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SpokeChainCallIntentBookCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CancelIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentData(element) => ::core::fmt::Display::fmt(element, f),
                Self::Intents(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::PlaceIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettleIntent(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CancelIntentCall> for SpokeChainCallIntentBookCalls {
        fn from(value: CancelIntentCall) -> Self {
            Self::CancelIntent(value)
        }
    }
    impl ::core::convert::From<IntentDataCall> for SpokeChainCallIntentBookCalls {
        fn from(value: IntentDataCall) -> Self {
            Self::IntentData(value)
        }
    }
    impl ::core::convert::From<IntentsCall> for SpokeChainCallIntentBookCalls {
        fn from(value: IntentsCall) -> Self {
            Self::Intents(value)
        }
    }
    impl ::core::convert::From<MatchIntentCall> for SpokeChainCallIntentBookCalls {
        fn from(value: MatchIntentCall) -> Self {
            Self::MatchIntent(value)
        }
    }
    impl ::core::convert::From<PlaceIntentCall> for SpokeChainCallIntentBookCalls {
        fn from(value: PlaceIntentCall) -> Self {
            Self::PlaceIntent(value)
        }
    }
    impl ::core::convert::From<SettleIntentCall> for SpokeChainCallIntentBookCalls {
        fn from(value: SettleIntentCall) -> Self {
            Self::SettleIntent(value)
        }
    }
    ///Container type for all return fields from the `intentData` function with signature `intentData(bytes32)` and selector `0x59a844b4`
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
    pub struct IntentDataReturn {
        pub author: ::ethers::core::types::Address,
        pub chain_id: u32,
        pub call_data: ::ethers::core::types::Bytes,
        pub contract_to_call: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `intents` function with signature `intents(bytes32)` and selector `0x9021578a`
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
    pub struct IntentsReturn {
        pub status: u8,
        pub intent_bid_id: [u8; 32],
    }
    ///Container type for all return fields from the `placeIntent` function with signature `placeIntent((bytes,bytes))` and selector `0x4af2364e`
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
    pub struct PlaceIntentReturn {
        pub intent_id: [u8; 32],
    }
}
