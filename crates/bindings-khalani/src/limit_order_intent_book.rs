pub use limit_order_intent_book::*;
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
pub mod limit_order_intent_book {
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
                (
                    ::std::borrow::ToOwned::to_owned("LimitOrderFulfilled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LimitOrderFulfilled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LimitOrderPartialFill"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LimitOrderPartialFill",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("volumeFilled"),
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
    pub static LIMITORDERINTENTBOOK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0C\xD0\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80cJ\xF26N\x14a\0\\W\x80cr\xB8v\xEB\x14a\0\x82W\x80c{\xF8\xBB\x88\x14a\0\x97W\x80c\x90!W\x8A\x14a\0\xAAW\x80c\xD5_\x96\r\x14a\0\xE5W[`\0\x80\xFD[a\0oa\0j6`\x04a\x08hV[a\0\xF8V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x95a\0\x906`\x04a\x08\xAAV[a\x02\x1EV[\0[a\0\x95a\0\xA56`\x04a\x08\xF8V[a\x02nV[a\0\xD7a\0\xB86`\x04a\x08\xF8V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0y\x92\x91\x90a\t'V[a\0\x95a\0\xF36`\x04a\x08\xF8V[a\x03\x11V[`\0a\x01\x0Ba\x01\x06\x83a\t\xF6V[a\x03\xCCV[\x90Pa\x01\x16\x82a\x04\x06V[`\0\x81\x81R` \x81\x90R`@\x81 T`\xFF\x16`\x05\x81\x11\x15a\x019Wa\x019a\t\x11V[\x14a\x01\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x05\x81\x11\x15a\x01\xC5Wa\x01\xC5a\t\x11V[\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U\x90PPa\x01\xE1\x82\x82a\x05\xD8V[\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x02\x11\x91\x90a\n\xF2V[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\0\x82\x81R` \x81\x90R`@\x80\x82 \x80T`\xFF\x19\x16`\x03\x17\x81U`\x01\x81\x01T\x91Q\x90\x92\x82\x91\x86\x91\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x91\xA3PPPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x03\x81T`\xFF\x16`\x05\x81\x11\x15a\x02\x94Wa\x02\x94a\t\x11V[\x14a\x02\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x01zV[\x80T`\xFF\x19\x16`\x04\x17\x81U`\x01\x81\x01T`@Q\x81\x90\x84\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3PPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81T`\xFF\x16`\x05\x81\x11\x15a\x037Wa\x037a\t\x11V[\x14\x80a\x03XWP`\x02\x81T`\xFF\x16`\x05\x81\x11\x15a\x03VWa\x03Va\t\x11V[\x14[a\x03\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x01zV[\x80T`\xFF\x19\x16`\x05\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x03\xE9\x92\x91\x90a\x0BxV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0a\x04\x12\x82\x80a\x0B\x95V[\x81\x01\x90a\x04\x1F\x91\x90a\x0B\xF8V[\x90P`\0a\x05\x83\x82`@\x80Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x80\x83\x01\x91\x90\x91R\x7F\x97\xBC\x18\x0B\x88[\xB1\x106\xFACp\xBB\x14Lj\xE5U\x8B\xEE\x80\xA1\xF0\x8B\xEF\x9C\xEFq\xAE \x04\x8A\x82\x84\x01R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x80\x84\x01\x91\x90\x91RF`\x80\x80\x85\x01\x91\x90\x91R\x84Q\x80\x85\x03\x82\x01\x81R`\xA0\x85\x01\x86R\x80Q\x90\x84\x01 \x86Q\x87\x85\x01Q\x88\x88\x01Q\x94\x89\x01Q\x98\x90\x93\x01Q\x7Fs0\xFFg\xE0\x989\xF9\x8D\xC5\x8Cj\x16\xA1J\xFD[\x92\xEF\xAC\x0C\xD0\xFAv\x05\x92E(zX2\x93`\xC0\x88\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xE0\x88\x01R\x92\x81\x16a\x01\0\x87\x01Ra\x01 \x86\x01\x93\x90\x93Ra\x01@\x85\x01\x96\x90\x96R\x16a\x01`\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\x80\x83\x01\x84R\x80Q\x90\x82\x01 a\x19\x01`\xF0\x1Ba\x01\xA0\x84\x01Ra\x01\xA2\x83\x01\x94\x90\x94Ra\x01\xC2\x80\x83\x01\x94\x90\x94R\x82Q\x80\x83\x03\x90\x94\x01\x84Ra\x01\xE2\x90\x91\x01\x90\x91R\x81Q\x91\x01 \x90V[\x90Pa\x05\xD3a\x05\x95` \x85\x01\x85a\x0B\x95V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP\x85Q\x91P\x84\x90Pa\x06}V[PPPV[`\0a\x05\xE4\x83\x80a\x0B\x95V[\x81\x01\x90a\x05\xF1\x91\x90a\x0B\xF8V[\x90Pa\x06\n\x81` \x01Q\x82`@\x01Q\x83`\0\x01Qa\x07gV[`\0\x91\x82R`\x01` \x81\x81R`@\x93\x84\x90 \x83Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x83U\x92\x85\x01Q\x93\x82\x01\x80T\x84\x16\x94\x82\x16\x94\x90\x94\x17\x90\x93U\x93\x83\x01Q`\x02\x85\x01U``\x83\x01Q`\x03\x85\x01U`\x80\x90\x92\x01Q`\x04\x90\x93\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UPV[`\0\x80`\0a\x06\x8B\x86a\x07\xE6V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x89\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x06\xF1W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x01zV[PPPPPPV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x84\x90R\x84\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE0\x91\x90a\x0CxV[PPPPV[`\0\x80`\0\x83Q`A\x14a\x08LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x01zV[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[`\0` \x82\x84\x03\x12\x15a\x08zW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x91W`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x08\xA3W`\0\x80\xFD[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\xBDW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xDBW`\0\x80\xFD[\x83\x01` \x81\x86\x03\x12\x15a\x08\xEDW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\t\nW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x06\x84\x10a\tIWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\tzW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\x95Wa\t\x95a\tSV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\t\xBDWa\t\xBDa\tSV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\t\xD6W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`@\x826\x03\x12\x15a\n\x08W`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\n,Wa\n,a\tSV[\x81`@R\x845\x91P\x80\x82\x11\x15a\nAW`\0\x80\xFD[a\nM6\x83\x87\x01a\tiV[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\ncW`\0\x80\xFD[Pa\np6\x82\x86\x01a\tiV[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\n\x93W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xB3W`\0\x80\xFD[\x806\x03\x82\x13\x15a\n\xC2W`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\x0B\x02\x83\x84a\n|V[`@` \x85\x01Ra\x0B\x17``\x85\x01\x82\x84a\n\xC9V[\x91PPa\x0B'` \x85\x01\x85a\n|V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x0B>\x83\x82\x84a\n\xC9V[\x96\x95PPPPPPV[`\0\x81Q`\0[\x81\x81\x10\x15a\x0BiW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x0BOV[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x0B\x8Da\x0B\x87\x83\x86a\x0BHV[\x84a\x0BHV[\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0B\xACW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\xC7W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\n\xC2W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xF3W`\0\x80\xFD[\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a\x0C\nW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0C-Wa\x0C-a\tSV[`@Ra\x0C9\x83a\x0B\xDCV[\x81Ra\x0CG` \x84\x01a\x0B\xDCV[` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01Ra\x0Cl`\x80\x84\x01a\x0B\xDCV[`\x80\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0C\x8AW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08\xA3W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x0C\xB6#\xD0\xCEt\x8D.\x126p\x89\x84\x03\xCB\xB3\xD7\x0F\xEDp\xE9,8q\xF3x:o\xB2\xD6\xCC\xE9dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static LIMITORDERINTENTBOOK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80cJ\xF26N\x14a\0\\W\x80cr\xB8v\xEB\x14a\0\x82W\x80c{\xF8\xBB\x88\x14a\0\x97W\x80c\x90!W\x8A\x14a\0\xAAW\x80c\xD5_\x96\r\x14a\0\xE5W[`\0\x80\xFD[a\0oa\0j6`\x04a\x08hV[a\0\xF8V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x95a\0\x906`\x04a\x08\xAAV[a\x02\x1EV[\0[a\0\x95a\0\xA56`\x04a\x08\xF8V[a\x02nV[a\0\xD7a\0\xB86`\x04a\x08\xF8V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0y\x92\x91\x90a\t'V[a\0\x95a\0\xF36`\x04a\x08\xF8V[a\x03\x11V[`\0a\x01\x0Ba\x01\x06\x83a\t\xF6V[a\x03\xCCV[\x90Pa\x01\x16\x82a\x04\x06V[`\0\x81\x81R` \x81\x90R`@\x81 T`\xFF\x16`\x05\x81\x11\x15a\x019Wa\x019a\t\x11V[\x14a\x01\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x05\x81\x11\x15a\x01\xC5Wa\x01\xC5a\t\x11V[\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U\x90PPa\x01\xE1\x82\x82a\x05\xD8V[\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x02\x11\x91\x90a\n\xF2V[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\0\x82\x81R` \x81\x90R`@\x80\x82 \x80T`\xFF\x19\x16`\x03\x17\x81U`\x01\x81\x01T\x91Q\x90\x92\x82\x91\x86\x91\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x91\xA3PPPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x03\x81T`\xFF\x16`\x05\x81\x11\x15a\x02\x94Wa\x02\x94a\t\x11V[\x14a\x02\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x01zV[\x80T`\xFF\x19\x16`\x04\x17\x81U`\x01\x81\x01T`@Q\x81\x90\x84\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3PPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81T`\xFF\x16`\x05\x81\x11\x15a\x037Wa\x037a\t\x11V[\x14\x80a\x03XWP`\x02\x81T`\xFF\x16`\x05\x81\x11\x15a\x03VWa\x03Va\t\x11V[\x14[a\x03\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x01zV[\x80T`\xFF\x19\x16`\x05\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x03\xE9\x92\x91\x90a\x0BxV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0a\x04\x12\x82\x80a\x0B\x95V[\x81\x01\x90a\x04\x1F\x91\x90a\x0B\xF8V[\x90P`\0a\x05\x83\x82`@\x80Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x80\x83\x01\x91\x90\x91R\x7F\x97\xBC\x18\x0B\x88[\xB1\x106\xFACp\xBB\x14Lj\xE5U\x8B\xEE\x80\xA1\xF0\x8B\xEF\x9C\xEFq\xAE \x04\x8A\x82\x84\x01R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x80\x84\x01\x91\x90\x91RF`\x80\x80\x85\x01\x91\x90\x91R\x84Q\x80\x85\x03\x82\x01\x81R`\xA0\x85\x01\x86R\x80Q\x90\x84\x01 \x86Q\x87\x85\x01Q\x88\x88\x01Q\x94\x89\x01Q\x98\x90\x93\x01Q\x7Fs0\xFFg\xE0\x989\xF9\x8D\xC5\x8Cj\x16\xA1J\xFD[\x92\xEF\xAC\x0C\xD0\xFAv\x05\x92E(zX2\x93`\xC0\x88\x01R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xE0\x88\x01R\x92\x81\x16a\x01\0\x87\x01Ra\x01 \x86\x01\x93\x90\x93Ra\x01@\x85\x01\x96\x90\x96R\x16a\x01`\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\x80\x83\x01\x84R\x80Q\x90\x82\x01 a\x19\x01`\xF0\x1Ba\x01\xA0\x84\x01Ra\x01\xA2\x83\x01\x94\x90\x94Ra\x01\xC2\x80\x83\x01\x94\x90\x94R\x82Q\x80\x83\x03\x90\x94\x01\x84Ra\x01\xE2\x90\x91\x01\x90\x91R\x81Q\x91\x01 \x90V[\x90Pa\x05\xD3a\x05\x95` \x85\x01\x85a\x0B\x95V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP\x85Q\x91P\x84\x90Pa\x06}V[PPPV[`\0a\x05\xE4\x83\x80a\x0B\x95V[\x81\x01\x90a\x05\xF1\x91\x90a\x0B\xF8V[\x90Pa\x06\n\x81` \x01Q\x82`@\x01Q\x83`\0\x01Qa\x07gV[`\0\x91\x82R`\x01` \x81\x81R`@\x93\x84\x90 \x83Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x83U\x92\x85\x01Q\x93\x82\x01\x80T\x84\x16\x94\x82\x16\x94\x90\x94\x17\x90\x93U\x93\x83\x01Q`\x02\x85\x01U``\x83\x01Q`\x03\x85\x01U`\x80\x90\x92\x01Q`\x04\x90\x93\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UPV[`\0\x80`\0a\x06\x8B\x86a\x07\xE6V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x89\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x06\xF1W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x01zV[PPPPPPV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R0`$\x83\x01R`D\x82\x01\x84\x90R\x84\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE0\x91\x90a\x0CxV[PPPPV[`\0\x80`\0\x83Q`A\x14a\x08LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x01zV[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[`\0` \x82\x84\x03\x12\x15a\x08zW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x91W`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x08\xA3W`\0\x80\xFD[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\xBDW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xDBW`\0\x80\xFD[\x83\x01` \x81\x86\x03\x12\x15a\x08\xEDW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\t\nW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x06\x84\x10a\tIWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\tzW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\x95Wa\t\x95a\tSV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\t\xBDWa\t\xBDa\tSV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\t\xD6W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`@\x826\x03\x12\x15a\n\x08W`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\n,Wa\n,a\tSV[\x81`@R\x845\x91P\x80\x82\x11\x15a\nAW`\0\x80\xFD[a\nM6\x83\x87\x01a\tiV[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\ncW`\0\x80\xFD[Pa\np6\x82\x86\x01a\tiV[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\n\x93W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xB3W`\0\x80\xFD[\x806\x03\x82\x13\x15a\n\xC2W`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\x0B\x02\x83\x84a\n|V[`@` \x85\x01Ra\x0B\x17``\x85\x01\x82\x84a\n\xC9V[\x91PPa\x0B'` \x85\x01\x85a\n|V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x0B>\x83\x82\x84a\n\xC9V[\x96\x95PPPPPPV[`\0\x81Q`\0[\x81\x81\x10\x15a\x0BiW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x0BOV[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x0B\x8Da\x0B\x87\x83\x86a\x0BHV[\x84a\x0BHV[\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0B\xACW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\xC7W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\n\xC2W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xF3W`\0\x80\xFD[\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a\x0C\nW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0C-Wa\x0C-a\tSV[`@Ra\x0C9\x83a\x0B\xDCV[\x81Ra\x0CG` \x84\x01a\x0B\xDCV[` \x82\x01R`@\x83\x015`@\x82\x01R``\x83\x015``\x82\x01Ra\x0Cl`\x80\x84\x01a\x0B\xDCV[`\x80\x82\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0C\x8AW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08\xA3W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \x0C\xB6#\xD0\xCEt\x8D.\x126p\x89\x84\x03\xCB\xB3\xD7\x0F\xEDp\xE9,8q\xF3x:o\xB2\xD6\xCC\xE9dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static LIMITORDERINTENTBOOK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LimitOrderIntentBook<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LimitOrderIntentBook<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LimitOrderIntentBook<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LimitOrderIntentBook<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LimitOrderIntentBook<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LimitOrderIntentBook))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LimitOrderIntentBook<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIMITORDERINTENTBOOK_ABI.clone(),
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
                LIMITORDERINTENTBOOK_ABI.clone(),
                LIMITORDERINTENTBOOK_BYTECODE.clone().into(),
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
        ///Gets the contract's `LimitOrderFulfilled` event
        pub fn limit_order_fulfilled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LimitOrderFulfilledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LimitOrderPartialFill` event
        pub fn limit_order_partial_fill_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LimitOrderPartialFillFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LimitOrderIntentBookEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LimitOrderIntentBook<M> {
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
    #[ethevent(name = "LimitOrderFulfilled", abi = "LimitOrderFulfilled(bytes32)")]
    pub struct LimitOrderFulfilledFilter {
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
    #[ethevent(
        name = "LimitOrderPartialFill",
        abi = "LimitOrderPartialFill(bytes32,uint256)"
    )]
    pub struct LimitOrderPartialFillFilter {
        pub intent_id: [u8; 32],
        pub volume_filled: ::ethers::core::types::U256,
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
    pub enum LimitOrderIntentBookEvents {
        IntentBidReceivedFilter(IntentBidReceivedFilter),
        IntentCancelledFilter(IntentCancelledFilter),
        IntentCreatedFilter(IntentCreatedFilter),
        IntentMatchFilter(IntentMatchFilter),
        IntentSettledFilter(IntentSettledFilter),
        LimitOrderFulfilledFilter(LimitOrderFulfilledFilter),
        LimitOrderPartialFillFilter(LimitOrderPartialFillFilter),
    }
    impl ::ethers::contract::EthLogDecode for LimitOrderIntentBookEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = IntentBidReceivedFilter::decode_log(log) {
                return Ok(LimitOrderIntentBookEvents::IntentBidReceivedFilter(decoded));
            }
            if let Ok(decoded) = IntentCancelledFilter::decode_log(log) {
                return Ok(LimitOrderIntentBookEvents::IntentCancelledFilter(decoded));
            }
            if let Ok(decoded) = IntentCreatedFilter::decode_log(log) {
                return Ok(LimitOrderIntentBookEvents::IntentCreatedFilter(decoded));
            }
            if let Ok(decoded) = IntentMatchFilter::decode_log(log) {
                return Ok(LimitOrderIntentBookEvents::IntentMatchFilter(decoded));
            }
            if let Ok(decoded) = IntentSettledFilter::decode_log(log) {
                return Ok(LimitOrderIntentBookEvents::IntentSettledFilter(decoded));
            }
            if let Ok(decoded) = LimitOrderFulfilledFilter::decode_log(log) {
                return Ok(
                    LimitOrderIntentBookEvents::LimitOrderFulfilledFilter(decoded),
                );
            }
            if let Ok(decoded) = LimitOrderPartialFillFilter::decode_log(log) {
                return Ok(
                    LimitOrderIntentBookEvents::LimitOrderPartialFillFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LimitOrderIntentBookEvents {
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
                Self::LimitOrderFulfilledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LimitOrderPartialFillFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<IntentBidReceivedFilter> for LimitOrderIntentBookEvents {
        fn from(value: IntentBidReceivedFilter) -> Self {
            Self::IntentBidReceivedFilter(value)
        }
    }
    impl ::core::convert::From<IntentCancelledFilter> for LimitOrderIntentBookEvents {
        fn from(value: IntentCancelledFilter) -> Self {
            Self::IntentCancelledFilter(value)
        }
    }
    impl ::core::convert::From<IntentCreatedFilter> for LimitOrderIntentBookEvents {
        fn from(value: IntentCreatedFilter) -> Self {
            Self::IntentCreatedFilter(value)
        }
    }
    impl ::core::convert::From<IntentMatchFilter> for LimitOrderIntentBookEvents {
        fn from(value: IntentMatchFilter) -> Self {
            Self::IntentMatchFilter(value)
        }
    }
    impl ::core::convert::From<IntentSettledFilter> for LimitOrderIntentBookEvents {
        fn from(value: IntentSettledFilter) -> Self {
            Self::IntentSettledFilter(value)
        }
    }
    impl ::core::convert::From<LimitOrderFulfilledFilter>
    for LimitOrderIntentBookEvents {
        fn from(value: LimitOrderFulfilledFilter) -> Self {
            Self::LimitOrderFulfilledFilter(value)
        }
    }
    impl ::core::convert::From<LimitOrderPartialFillFilter>
    for LimitOrderIntentBookEvents {
        fn from(value: LimitOrderPartialFillFilter) -> Self {
            Self::LimitOrderPartialFillFilter(value)
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
    pub enum LimitOrderIntentBookCalls {
        CancelIntent(CancelIntentCall),
        Intents(IntentsCall),
        MatchIntent(MatchIntentCall),
        PlaceIntent(PlaceIntentCall),
        SettleIntent(SettleIntentCall),
    }
    impl ::ethers::core::abi::AbiDecode for LimitOrderIntentBookCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CancelIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CancelIntent(decoded));
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
    impl ::ethers::core::abi::AbiEncode for LimitOrderIntentBookCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CancelIntent(element) => {
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
    impl ::core::fmt::Display for LimitOrderIntentBookCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CancelIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::Intents(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::PlaceIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettleIntent(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CancelIntentCall> for LimitOrderIntentBookCalls {
        fn from(value: CancelIntentCall) -> Self {
            Self::CancelIntent(value)
        }
    }
    impl ::core::convert::From<IntentsCall> for LimitOrderIntentBookCalls {
        fn from(value: IntentsCall) -> Self {
            Self::Intents(value)
        }
    }
    impl ::core::convert::From<MatchIntentCall> for LimitOrderIntentBookCalls {
        fn from(value: MatchIntentCall) -> Self {
            Self::MatchIntent(value)
        }
    }
    impl ::core::convert::From<PlaceIntentCall> for LimitOrderIntentBookCalls {
        fn from(value: PlaceIntentCall) -> Self {
            Self::PlaceIntent(value)
        }
    }
    impl ::core::convert::From<SettleIntentCall> for LimitOrderIntentBookCalls {
        fn from(value: SettleIntentCall) -> Self {
            Self::SettleIntent(value)
        }
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
