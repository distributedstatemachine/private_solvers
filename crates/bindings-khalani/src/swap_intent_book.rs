pub use swap_intent_book::*;
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
pub mod swap_intent_book {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_verifierRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract VerifierRegistry"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_rewarder"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IRewarder"),
                        ),
                    },
                ],
            }),
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
                    ::std::borrow::ToOwned::to_owned("intentBidData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("intentBidData"),
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
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                                    name: ::std::borrow::ToOwned::to_owned("intent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("intentStates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("intentStates"),
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
                    ::std::borrow::ToOwned::to_owned("matchAndSettle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("matchAndSettle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentBid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
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
                    ::std::borrow::ToOwned::to_owned("matchIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("matchIntent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentBid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
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
                (
                    ::std::borrow::ToOwned::to_owned("verify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapIntentBid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentLib.SwapIntentBid",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifySignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verifySignature"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapIntentOrder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentLib.SwapIntent",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
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
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentBid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
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
                    ::std::borrow::ToOwned::to_owned("IntentPartiallySettled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "IntentPartiallySettled",
                            ),
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
    pub static SWAPINTENTBOOK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0!Y8\x03\x80b\0!Y\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\x7FV[`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x04\x80T\x92\x90\x93\x16\x91\x16\x17\x90Ub\0\0\xBEV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0|W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\x93W`\0\x80\xFD[\x82Qb\0\0\xA0\x81b\0\0fV[` \x84\x01Q\x90\x92Pb\0\0\xB3\x81b\0\0fV[\x80\x91PP\x92P\x92\x90PV[a \x8B\x80b\0\0\xCE`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x11a\0fW\x80c{\xF8\xBB\x88\x14a\x01KW\x80c\x87\xF6\x17\xB6\x14a\x01^W\x80c\xD5_\x96\r\x14a\x01qW\x80c\xDB\xDF9\x03\x14a\x01\x84W\x80c\xE2V#\xE0\x14a\x01\x97W`\0\x80\xFD[\x80c\x03\x89\\\x91\x14a\0\xA3W\x80c\t\xC7\xB2\xF6\x14a\0\xB6W\x80cJ\xF26N\x14a\0\xC9W\x80cY\xA8D\xB4\x14a\0\xEFW\x80c_\xF8\xA6k\x14a\x01\x10W[`\0\x80\xFD[a\0\xB4a\0\xB16`\x04a\x15sV[PV[\0[a\0\xB4a\0\xC46`\x04a\x15\xC5V[a\x01\xB8V[a\0\xDCa\0\xD76`\x04a\x15\xC5V[a\x04\x0BV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\0\xFD6`\x04a\x15\xF9V[a\x05;V[`@Qa\0\xE6\x92\x91\x90a\x16bV[a\x01=a\x01\x1E6`\x04a\x15\xF9V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\xE6\x92\x91\x90a\x16\xA6V[a\0\xB4a\x01Y6`\x04a\x15\xF9V[a\x06gV[a\0\xB4a\x01l6`\x04a\x15\xC5V[a\x07tV[a\0\xB4a\x01\x7F6`\x04a\x15\xF9V[a\x07\x87V[a\0\xB4a\x01\x926`\x04a\x17lV[a\x08\x95V[a\x01\xAAa\x01\xA56`\x04a\x15\xF9V[a\x08\xB6V[`@Qa\0\xE6\x92\x91\x90a\x18\x9BV[\x805`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x15a\x02\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FIntent already has a bid\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81T`\xFF\x16`\x03\x81\x11\x15a\x027Wa\x027a\x16\x90V[\x03a\x02|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\x94Wa\x02\x94a\x16\x90V[\x03a\x02\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\xF9Wa\x02\xF9a\x16\x90V[\x03a\x03BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0a\x03Ua\x03P\x85a\x18\xB4V[a\x08\xD8V[`\0\x81\x81R`\x02` R`@\x90 T\x90\x91P\x15a\x03\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqBid already exists`p\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0\x81\x81R`\x02` R`@\x90 \x84\x90a\x03\xC3\x82\x82a\x1A\x99V[PP`\x01\x82\x01\x81\x90U\x80\x83\x7Fdi[\xEF\xF9W(\xF3\xEB5\xAC\xAF>E\x0B\xAD\xD7\xE5c\xA5\xCBXe^\x9D\xDA\xDD\xFAm\xECfI\x86`@Qa\x03\xFD\x91\x90a\x1B\xD7V[`@Q\x80\x91\x03\x90\xA3PPPPV[`\0a\x04\x1Ea\x04\x19\x83a\x1C\x05V[a\t\x12V[\x90P`\0\x80\x82\x81R` \x81\x90R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a\x04DWa\x04Da\x16\x90V[\x14a\x04\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x02\x16V[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\x04\xCBWa\x04\xCBa\x16\x90V[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x82\x01U`\0\x83\x81R\x91R`@\x90 \x82\x90a\x04\xF3\x82\x82a\x1CeV[PPa\x04\xFE\x81a\t/V[\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x05.\x91\x90a\x1DAV[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\x01` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x05V\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x82\x90a\x19PV[\x80\x15a\x05\xCFW\x80`\x1F\x10a\x05\xA4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xCFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xB2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x05\xE4\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x10\x90a\x19PV[\x80\x15a\x06]W\x80`\x1F\x10a\x062Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06]V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06@W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x80a\x06\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FIntent does not have a bid\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\0a\x06\xD3\x84\x83a\n\x95V[`\0`\x01\x80\x86\x01\x82\x90U\x84\x82R`\x02` R`@\x82 \x82\x81U\x92\x93Pa\x06\xFB\x90\x83\x01\x82a\x14oV[PP\x80\x15a\x07@W\x82T`\xFF\x19\x16`\x02\x17\x83U`@Q\x82\x90\x85\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3a\x07nV[`@Q\x82\x90\x85\x90\x7F\x84oK\x93k-|\xCF_\xCB\x9F1z\xB7\x91\xF5\xEC\xE5a\x11\x1E\x890n\x99}\x88\xBB\x84*<S\x90`\0\x90\xA3[PPPPV[a\x07}\x81a\x01\xB8V[a\0\xB1\x815a\x06gV[`\0\x81\x81R` \x81\x90R`@\x90 `\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x07\xADWa\x07\xADa\x16\x90V[\x03a\x07\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x08\x0EWa\x08\x0Ea\x16\x90V[\x03a\x08[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[\x80T`\xFF\x19\x16`\x03\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\0a\x08\xA0\x82a\x10\x0CV[\x90Pa\x08\xB1\x83\x83`\0\x01Q\x83a\x11\xD0V[PPPV[`\x02` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T\x91\x92\x91a\x05\xE4\x90a\x19PV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x08\xF5\x92\x91\x90a\x1D\x97V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x08\xF5\x92\x91\x90a\x1D\xBDV[`\0\x81\x81R`\x01` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x82\x90\x82\x90a\tU\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\x81\x90a\x19PV[\x80\x15a\t\xCEW\x80`\x1F\x10a\t\xA3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xCEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xB1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\t\xE7\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x13\x90a\x19PV[\x80\x15a\n`W\x80`\x1F\x10a\n5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n`V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\n\x85\x91\x90a\x1EGV[\x90Pa\x08\xB1\x82` \x01Q\x82a\x08\x95V[`\0\x82\x81R`\x01` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x83\x92\x91\x90\x82\x90\x82\x90a\n\xBF\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xEB\x90a\x19PV[\x80\x15a\x0B8W\x80`\x1F\x10a\x0B\rWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x1BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x0BQ\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B}\x90a\x19PV[\x80\x15a\x0B\xCAW\x80`\x1F\x10a\x0B\x9FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xCAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xADW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\x0B\xEF\x91\x90a\x1EGV[\x90P`\0`\x02`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80Ta\x0C+\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0CW\x90a\x19PV[\x80\x15a\x0C\xA4W\x80`\x1F\x10a\x0CyWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xA4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\x87W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81` \x01Q\x80` \x01\x90Q\x81\x01\x90a\x0C\xC9\x91\x90a\x1FOV[\x90P`\0a\x0C\xD6\x84a\x12\xBAV[`\x03T`@\x86\x81\x01Q\x90Qc\xB6n\x93_`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB6n\x93_\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r+W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rO\x91\x90a\x1F\x89V[`\x01`\x01`\xA0\x1B\x03\x16cu\xE3f\x16a\rf\x83a\x13\x03V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\x84\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xC7\x91\x90a\x1F\xADV[a\x0E\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Ftoken not locked at source\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\x03T``\x85\x01Q`@Qc\xB6n\x93_`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB6n\x93_\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8B\x91\x90a\x1F\x89V[`\x01`\x01`\xA0\x1B\x03\x16cu\xE3f\x16a\x0E\xAC\x83\x85`\0\x01Q\x86` \x01Qa\x13\"V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xCA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\r\x91\x90a\x1F\xADV[a\x0FcW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fswap not fulfilled at destinatio`D\x82\x01R`7`\xF9\x1B`d\x82\x01R`\x84\x01a\x02\x16V[`\x04\x80T`@\x80\x87\x01Q`\x80\x88\x01Q`\xC0\x89\x01Q\x87Q\x93Qc\x18\x06\xC3C`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x95c\x18\x06\xC3C\x95a\x0F\xCC\x95\x91\x01c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x85\x01R`@\x84\x01\x91\x90\x91R\x16``\x82\x01R`\x80\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xE6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xFAW=`\0\x80>=`\0\xFD[P`\x01\x9B\x9APPPPPPPPPPPV[`@\x80\x82\x01Q\x81Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x82\x01R\x7F\xD4\xE2\xD2a\xFF\xA0\xA31\xA9>\xD4\xAE\xE4\x94\x07o\xDC\x03\xDA\x7F\x1A\x99\xDFT\xA9d\x8Ca\xCA\x1D\x85\x94\x92\x81\x01\x92\x90\x92R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\0\x90\x81\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7F\x9F4\xD6E\xC3}:\xED\xB5\x03Rz\x88\xEB=\xAF\x0B\xAD\xF2*\xDE\x9B\x80C\x07\xCD\xD0\xBF2\x8A\x9B\x13\x84`\0\x01Q\x85``\x01Q\x86`\xA0\x01Q\x87`\xC0\x01Q\x88`@\x01Q\x89`\xE0\x01Q\x80Q\x90` \x01 \x8A`\x80\x01Q\x8Ba\x01\0\x01Q\x8Ca\x01 \x01Q`@Q` \x01a\x11}\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90\x99\x8AR`\x01`\x01`\xA0\x1B\x03\x98\x89\x16` \x8B\x01Rc\xFF\xFF\xFF\xFF\x97\x88\x16`@\x8B\x01R\x95\x88\x16``\x8A\x01R`\x80\x89\x01\x94\x90\x94R\x91\x90\x94\x16`\xA0\x87\x01R`\xC0\x86\x01\x93\x90\x93R\x91\x90\x92\x16`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x01 \x82\x01Ra\x01@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 a\x19\x01`\xF0\x1B\x82\x85\x01R`\"\x84\x01\x95\x90\x95R`B\x80\x84\x01\x95\x90\x95R\x81Q\x80\x84\x03\x90\x95\x01\x85R`b\x90\x92\x01\x90R\x82Q\x92\x01\x91\x90\x91 \x93\x92PPPV[`\0\x80`\0a\x11\xDE\x86a\x13XV[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x89\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x12DW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPPPPPV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x08\xF5\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x1F\xCFV[`\0a\x13\x1C`@Q\x80` \x01`@R\x80\x84\x81RPa\x13\xDAV[\x92\x91PPV[`\0a\x13P`@Q\x80``\x01`@R\x80\x86\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x81RPa\x14\x0EV[\x94\x93PPPPV[`\0\x80`\0\x83Q`A\x14a\x13\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x08\xF5V[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q`@Q` \x01a\x08\xF5\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x93\x90\x93R``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x83\x01R`D\x82\x01R`d\x01\x90V[P\x80Ta\x14{\x90a\x19PV[`\0\x82U\x80`\x1F\x10a\x14\x8BWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\0\xB1\x91\x90[\x80\x82\x11\x15a\x14\xB9W`\0\x81U`\x01\x01a\x14\xA5V[P\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xF5Wa\x14\xF5a\x14\xBDV[`@R\x90V[`@Qa\x01@\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xF5Wa\x14\xF5a\x14\xBDV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15FWa\x15Fa\x14\xBDV[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xB1W`\0\x80\xFD[\x805a\x15n\x81a\x15NV[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x15\x85W`\0\x80\xFD[a\x15\x8Da\x14\xD3V[\x825a\x15\x98\x81a\x15NV[\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x15\xBFW`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x15\xD7W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xEDW`\0\x80\xFD[a\x13P\x84\x82\x85\x01a\x15\xADV[`\0` \x82\x84\x03\x12\x15a\x16\x0BW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x16-W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16\x15V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x16N\x81` \x86\x01` \x86\x01a\x16\x12V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\x16u`@\x83\x01\x85a\x166V[\x82\x81\x03` \x84\x01Ra\x16\x87\x81\x85a\x166V[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x04\x84\x10a\x16\xC8WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x16\xEBWa\x16\xEBa\x14\xBDV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x17\nW`\0\x80\xFD[\x815a\x17\x1Da\x17\x18\x82a\x16\xD2V[a\x15\x1EV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x172W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\0\xB1W`\0\x80\xFD[\x805a\x15n\x81a\x17OV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\x7FW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17\x96W`\0\x80\xFD[a\x17\xA2\x86\x83\x87\x01a\x16\xF9V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x17\xB8W`\0\x80\xFD[\x90\x84\x01\x90a\x01@\x82\x87\x03\x12\x15a\x17\xCDW`\0\x80\xFD[a\x17\xD5a\x14\xFBV[a\x17\xDE\x83a\x15cV[\x81R` \x83\x015\x82\x81\x11\x15a\x17\xF2W`\0\x80\xFD[a\x17\xFE\x88\x82\x86\x01a\x16\xF9V[` \x83\x01RPa\x18\x10`@\x84\x01a\x17aV[`@\x82\x01Ra\x18!``\x84\x01a\x17aV[``\x82\x01Ra\x182`\x80\x84\x01a\x15cV[`\x80\x82\x01Ra\x18C`\xA0\x84\x01a\x15cV[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x18dW`\0\x80\xFD[a\x18p\x88\x82\x86\x01a\x16\xF9V[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x94\x91\x93P\x90\x91PPV[\x82\x81R`@` \x82\x01R`\0a\x13P`@\x83\x01\x84a\x166V[`\0`@\x826\x03\x12\x15a\x18\xC6W`\0\x80\xFD[a\x18\xCEa\x14\xD3V[\x825\x81R` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xEBW`\0\x80\xFD[a\x18\xF76\x82\x86\x01a\x16\xF9V[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19\x1AW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x194W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x19IW`\0\x80\xFD[\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x19dW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15\xBFWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x08\xB1W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x19\xABWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x12\xB2W\x82\x81U`\x01\x01a\x19\xB7V[`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17\x90V[`\x01`\x01`@\x1B\x03\x83\x11\x15a\x19\xF6Wa\x19\xF6a\x14\xBDV[a\x1A\n\x83a\x1A\x04\x83Ta\x19PV[\x83a\x19\x84V[`\0`\x1F\x84\x11`\x01\x81\x14a\x1A8W`\0\x85\x15a\x1A&WP\x83\x82\x015[a\x1A0\x86\x82a\x19\xCAV[\x84UPa\x1A\x92V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x1AiW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1AIV[P\x86\x82\x10\x15a\x1A\x86W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[\x815\x81U`\x01\x80\x82\x01` a\x1A\xB0\x81\x86\x01\x86a\x19\x03V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xC7Wa\x1A\xC7a\x14\xBDV[a\x1A\xDB\x81a\x1A\xD5\x86Ta\x19PV[\x86a\x19\x84V[`\0`\x1F\x82\x11`\x01\x81\x14a\x1B\tW`\0\x83\x15a\x1A\xF7WP\x83\x82\x015[a\x1B\x01\x84\x82a\x19\xCAV[\x87UPa\x1B^V[`\0\x86\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1B7W\x86\x85\x015\x82U\x93\x87\x01\x93\x90\x89\x01\x90\x87\x01a\x1B\x1AV[P\x84\x82\x10\x15a\x1BTW`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP\x86\x83\x88\x1B\x01\x86U[PPPPPPPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1B\x80W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\x9FW`\0\x80\xFD[\x806\x03\x82\x13\x15a\x19IW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R\x815` \x82\x01R`\0a\x1B\xF1` \x84\x01\x84a\x1BiV[`@\x80\x85\x01Ra\x16\x87``\x85\x01\x82\x84a\x1B\xAEV[`\0`@\x826\x03\x12\x15a\x1C\x17W`\0\x80\xFD[a\x1C\x1Fa\x14\xD3V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1C6W`\0\x80\xFD[a\x1CB6\x83\x87\x01a\x16\xF9V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x1CXW`\0\x80\xFD[Pa\x18\xF76\x82\x86\x01a\x16\xF9V[a\x1Co\x82\x83a\x19\x03V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\x86Wa\x1C\x86a\x14\xBDV[a\x1C\x9A\x81a\x1C\x94\x85Ta\x19PV[\x85a\x19\x84V[`\0`\x1F\x82\x11`\x01\x81\x14a\x1C\xC8W`\0\x83\x15a\x1C\xB6WP\x83\x82\x015[a\x1C\xC0\x84\x82a\x19\xCAV[\x86UPa\x1D\"V[`\0\x85\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1C\xF9W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1C\xD9V[P\x84\x82\x10\x15a\x1D\x16W`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x83`\x01\x1B\x01\x85U[PPPPa\x1D3` \x83\x01\x83a\x19\x03V[a\x07n\x81\x83`\x01\x86\x01a\x19\xDFV[` \x81R`\0a\x1DQ\x83\x84a\x1BiV[`@` \x85\x01Ra\x1Df``\x85\x01\x82\x84a\x1B\xAEV[\x91PPa\x1Dv` \x85\x01\x85a\x1BiV[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1D\x8D\x83\x82\x84a\x1B\xAEV[\x96\x95PPPPPPV[\x82\x81R`\0\x82Qa\x1D\xAF\x81` \x85\x01` \x87\x01a\x16\x12V[\x91\x90\x91\x01` \x01\x93\x92PPPV[`\0\x83Qa\x1D\xCF\x81\x84` \x88\x01a\x16\x12V[\x83Q\x90\x83\x01\x90a\x1D\xE3\x81\x83` \x88\x01a\x16\x12V[\x01\x94\x93PPPPV[\x80Qa\x15n\x81a\x15NV[`\0\x82`\x1F\x83\x01\x12a\x1E\x08W`\0\x80\xFD[\x81Qa\x1E\x16a\x17\x18\x82a\x16\xD2V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1E+W`\0\x80\xFD[a\x13P\x82` \x83\x01` \x87\x01a\x16\x12V[\x80Qa\x15n\x81a\x17OV[`\0` \x82\x84\x03\x12\x15a\x1EYW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1EpW`\0\x80\xFD[\x90\x83\x01\x90a\x01@\x82\x86\x03\x12\x15a\x1E\x85W`\0\x80\xFD[a\x1E\x8Da\x14\xFBV[a\x1E\x96\x83a\x1D\xECV[\x81R` \x83\x01Q\x82\x81\x11\x15a\x1E\xAAW`\0\x80\xFD[a\x1E\xB6\x87\x82\x86\x01a\x1D\xF7V[` \x83\x01RPa\x1E\xC8`@\x84\x01a\x1E<V[`@\x82\x01Ra\x1E\xD9``\x84\x01a\x1E<V[``\x82\x01Ra\x1E\xEA`\x80\x84\x01a\x1D\xECV[`\x80\x82\x01Ra\x1E\xFB`\xA0\x84\x01a\x1D\xECV[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R`\xE0\x83\x01Q\x82\x81\x11\x15a\x1F\x1CW`\0\x80\xFD[a\x1F(\x87\x82\x86\x01a\x1D\xF7V[`\xE0\x83\x01RPa\x01\0\x83\x81\x01Q\x90\x82\x01Ra\x01 \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a\x1FaW`\0\x80\xFD[a\x1Fia\x14\xD3V[\x82Qa\x1Ft\x81a\x15NV[\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1F\x9BW`\0\x80\xFD[\x81Qa\x1F\xA6\x81a\x15NV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1F\xBFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1F\xA6W`\0\x80\xFD[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Qa 6\x81`d\x85\x01` \x89\x01a\x16\x12V[\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xAE;\xA4\xBCB^\xA6VY_nN\xE4\x860\xB7\x010b\x1DG+\xA9=\xF5)O\xC1\x07\x846=dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SWAPINTENTBOOK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x11a\0fW\x80c{\xF8\xBB\x88\x14a\x01KW\x80c\x87\xF6\x17\xB6\x14a\x01^W\x80c\xD5_\x96\r\x14a\x01qW\x80c\xDB\xDF9\x03\x14a\x01\x84W\x80c\xE2V#\xE0\x14a\x01\x97W`\0\x80\xFD[\x80c\x03\x89\\\x91\x14a\0\xA3W\x80c\t\xC7\xB2\xF6\x14a\0\xB6W\x80cJ\xF26N\x14a\0\xC9W\x80cY\xA8D\xB4\x14a\0\xEFW\x80c_\xF8\xA6k\x14a\x01\x10W[`\0\x80\xFD[a\0\xB4a\0\xB16`\x04a\x15sV[PV[\0[a\0\xB4a\0\xC46`\x04a\x15\xC5V[a\x01\xB8V[a\0\xDCa\0\xD76`\x04a\x15\xC5V[a\x04\x0BV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\0\xFD6`\x04a\x15\xF9V[a\x05;V[`@Qa\0\xE6\x92\x91\x90a\x16bV[a\x01=a\x01\x1E6`\x04a\x15\xF9V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\xE6\x92\x91\x90a\x16\xA6V[a\0\xB4a\x01Y6`\x04a\x15\xF9V[a\x06gV[a\0\xB4a\x01l6`\x04a\x15\xC5V[a\x07tV[a\0\xB4a\x01\x7F6`\x04a\x15\xF9V[a\x07\x87V[a\0\xB4a\x01\x926`\x04a\x17lV[a\x08\x95V[a\x01\xAAa\x01\xA56`\x04a\x15\xF9V[a\x08\xB6V[`@Qa\0\xE6\x92\x91\x90a\x18\x9BV[\x805`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x15a\x02\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FIntent already has a bid\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81T`\xFF\x16`\x03\x81\x11\x15a\x027Wa\x027a\x16\x90V[\x03a\x02|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\x94Wa\x02\x94a\x16\x90V[\x03a\x02\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\xF9Wa\x02\xF9a\x16\x90V[\x03a\x03BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0a\x03Ua\x03P\x85a\x18\xB4V[a\x08\xD8V[`\0\x81\x81R`\x02` R`@\x90 T\x90\x91P\x15a\x03\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqBid already exists`p\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0\x81\x81R`\x02` R`@\x90 \x84\x90a\x03\xC3\x82\x82a\x1A\x99V[PP`\x01\x82\x01\x81\x90U\x80\x83\x7Fdi[\xEF\xF9W(\xF3\xEB5\xAC\xAF>E\x0B\xAD\xD7\xE5c\xA5\xCBXe^\x9D\xDA\xDD\xFAm\xECfI\x86`@Qa\x03\xFD\x91\x90a\x1B\xD7V[`@Q\x80\x91\x03\x90\xA3PPPPV[`\0a\x04\x1Ea\x04\x19\x83a\x1C\x05V[a\t\x12V[\x90P`\0\x80\x82\x81R` \x81\x90R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a\x04DWa\x04Da\x16\x90V[\x14a\x04\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x02\x16V[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\x04\xCBWa\x04\xCBa\x16\x90V[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x82\x01U`\0\x83\x81R\x91R`@\x90 \x82\x90a\x04\xF3\x82\x82a\x1CeV[PPa\x04\xFE\x81a\t/V[\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x05.\x91\x90a\x1DAV[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\x01` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x05V\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x82\x90a\x19PV[\x80\x15a\x05\xCFW\x80`\x1F\x10a\x05\xA4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xCFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xB2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x05\xE4\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x10\x90a\x19PV[\x80\x15a\x06]W\x80`\x1F\x10a\x062Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06]V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06@W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x80a\x06\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FIntent does not have a bid\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\0a\x06\xD3\x84\x83a\n\x95V[`\0`\x01\x80\x86\x01\x82\x90U\x84\x82R`\x02` R`@\x82 \x82\x81U\x92\x93Pa\x06\xFB\x90\x83\x01\x82a\x14oV[PP\x80\x15a\x07@W\x82T`\xFF\x19\x16`\x02\x17\x83U`@Q\x82\x90\x85\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3a\x07nV[`@Q\x82\x90\x85\x90\x7F\x84oK\x93k-|\xCF_\xCB\x9F1z\xB7\x91\xF5\xEC\xE5a\x11\x1E\x890n\x99}\x88\xBB\x84*<S\x90`\0\x90\xA3[PPPPV[a\x07}\x81a\x01\xB8V[a\0\xB1\x815a\x06gV[`\0\x81\x81R` \x81\x90R`@\x90 `\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x07\xADWa\x07\xADa\x16\x90V[\x03a\x07\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x08\x0EWa\x08\x0Ea\x16\x90V[\x03a\x08[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[\x80T`\xFF\x19\x16`\x03\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\0a\x08\xA0\x82a\x10\x0CV[\x90Pa\x08\xB1\x83\x83`\0\x01Q\x83a\x11\xD0V[PPPV[`\x02` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T\x91\x92\x91a\x05\xE4\x90a\x19PV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x08\xF5\x92\x91\x90a\x1D\x97V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x08\xF5\x92\x91\x90a\x1D\xBDV[`\0\x81\x81R`\x01` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x82\x90\x82\x90a\tU\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\x81\x90a\x19PV[\x80\x15a\t\xCEW\x80`\x1F\x10a\t\xA3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xCEV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xB1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\t\xE7\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x13\x90a\x19PV[\x80\x15a\n`W\x80`\x1F\x10a\n5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n`V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\n\x85\x91\x90a\x1EGV[\x90Pa\x08\xB1\x82` \x01Q\x82a\x08\x95V[`\0\x82\x81R`\x01` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x83\x92\x91\x90\x82\x90\x82\x90a\n\xBF\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xEB\x90a\x19PV[\x80\x15a\x0B8W\x80`\x1F\x10a\x0B\rWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x1BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x0BQ\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B}\x90a\x19PV[\x80\x15a\x0B\xCAW\x80`\x1F\x10a\x0B\x9FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xCAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xADW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\x0B\xEF\x91\x90a\x1EGV[\x90P`\0`\x02`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80Ta\x0C+\x90a\x19PV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0CW\x90a\x19PV[\x80\x15a\x0C\xA4W\x80`\x1F\x10a\x0CyWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xA4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\x87W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81` \x01Q\x80` \x01\x90Q\x81\x01\x90a\x0C\xC9\x91\x90a\x1FOV[\x90P`\0a\x0C\xD6\x84a\x12\xBAV[`\x03T`@\x86\x81\x01Q\x90Qc\xB6n\x93_`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB6n\x93_\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r+W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rO\x91\x90a\x1F\x89V[`\x01`\x01`\xA0\x1B\x03\x16cu\xE3f\x16a\rf\x83a\x13\x03V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\x84\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xC7\x91\x90a\x1F\xADV[a\x0E\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Ftoken not locked at source\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\x03T``\x85\x01Q`@Qc\xB6n\x93_`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB6n\x93_\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8B\x91\x90a\x1F\x89V[`\x01`\x01`\xA0\x1B\x03\x16cu\xE3f\x16a\x0E\xAC\x83\x85`\0\x01Q\x86` \x01Qa\x13\"V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xCA\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\r\x91\x90a\x1F\xADV[a\x0FcW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fswap not fulfilled at destinatio`D\x82\x01R`7`\xF9\x1B`d\x82\x01R`\x84\x01a\x02\x16V[`\x04\x80T`@\x80\x87\x01Q`\x80\x88\x01Q`\xC0\x89\x01Q\x87Q\x93Qc\x18\x06\xC3C`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x95c\x18\x06\xC3C\x95a\x0F\xCC\x95\x91\x01c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x85\x01R`@\x84\x01\x91\x90\x91R\x16``\x82\x01R`\x80\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xE6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xFAW=`\0\x80>=`\0\xFD[P`\x01\x9B\x9APPPPPPPPPPPV[`@\x80\x82\x01Q\x81Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x82\x01R\x7F\xD4\xE2\xD2a\xFF\xA0\xA31\xA9>\xD4\xAE\xE4\x94\x07o\xDC\x03\xDA\x7F\x1A\x99\xDFT\xA9d\x8Ca\xCA\x1D\x85\x94\x92\x81\x01\x92\x90\x92R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\0\x90\x81\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7F\x9F4\xD6E\xC3}:\xED\xB5\x03Rz\x88\xEB=\xAF\x0B\xAD\xF2*\xDE\x9B\x80C\x07\xCD\xD0\xBF2\x8A\x9B\x13\x84`\0\x01Q\x85``\x01Q\x86`\xA0\x01Q\x87`\xC0\x01Q\x88`@\x01Q\x89`\xE0\x01Q\x80Q\x90` \x01 \x8A`\x80\x01Q\x8Ba\x01\0\x01Q\x8Ca\x01 \x01Q`@Q` \x01a\x11}\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90\x99\x8AR`\x01`\x01`\xA0\x1B\x03\x98\x89\x16` \x8B\x01Rc\xFF\xFF\xFF\xFF\x97\x88\x16`@\x8B\x01R\x95\x88\x16``\x8A\x01R`\x80\x89\x01\x94\x90\x94R\x91\x90\x94\x16`\xA0\x87\x01R`\xC0\x86\x01\x93\x90\x93R\x91\x90\x92\x16`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x01 \x82\x01Ra\x01@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 a\x19\x01`\xF0\x1B\x82\x85\x01R`\"\x84\x01\x95\x90\x95R`B\x80\x84\x01\x95\x90\x95R\x81Q\x80\x84\x03\x90\x95\x01\x85R`b\x90\x92\x01\x90R\x82Q\x92\x01\x91\x90\x91 \x93\x92PPPV[`\0\x80`\0a\x11\xDE\x86a\x13XV[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x89\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x12DW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPPPPPV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x08\xF5\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x1F\xCFV[`\0a\x13\x1C`@Q\x80` \x01`@R\x80\x84\x81RPa\x13\xDAV[\x92\x91PPV[`\0a\x13P`@Q\x80``\x01`@R\x80\x86\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x81RPa\x14\x0EV[\x94\x93PPPPV[`\0\x80`\0\x83Q`A\x14a\x13\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x08\xF5V[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q`@Q` \x01a\x08\xF5\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x93\x90\x93R``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x83\x01R`D\x82\x01R`d\x01\x90V[P\x80Ta\x14{\x90a\x19PV[`\0\x82U\x80`\x1F\x10a\x14\x8BWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\0\xB1\x91\x90[\x80\x82\x11\x15a\x14\xB9W`\0\x81U`\x01\x01a\x14\xA5V[P\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xF5Wa\x14\xF5a\x14\xBDV[`@R\x90V[`@Qa\x01@\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xF5Wa\x14\xF5a\x14\xBDV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15FWa\x15Fa\x14\xBDV[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xB1W`\0\x80\xFD[\x805a\x15n\x81a\x15NV[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x15\x85W`\0\x80\xFD[a\x15\x8Da\x14\xD3V[\x825a\x15\x98\x81a\x15NV[\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x15\xBFW`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x15\xD7W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xEDW`\0\x80\xFD[a\x13P\x84\x82\x85\x01a\x15\xADV[`\0` \x82\x84\x03\x12\x15a\x16\x0BW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x16-W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16\x15V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x16N\x81` \x86\x01` \x86\x01a\x16\x12V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\x16u`@\x83\x01\x85a\x166V[\x82\x81\x03` \x84\x01Ra\x16\x87\x81\x85a\x166V[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x04\x84\x10a\x16\xC8WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x16\xEBWa\x16\xEBa\x14\xBDV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x17\nW`\0\x80\xFD[\x815a\x17\x1Da\x17\x18\x82a\x16\xD2V[a\x15\x1EV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x172W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\0\xB1W`\0\x80\xFD[\x805a\x15n\x81a\x17OV[`\0\x80`@\x83\x85\x03\x12\x15a\x17\x7FW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17\x96W`\0\x80\xFD[a\x17\xA2\x86\x83\x87\x01a\x16\xF9V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x17\xB8W`\0\x80\xFD[\x90\x84\x01\x90a\x01@\x82\x87\x03\x12\x15a\x17\xCDW`\0\x80\xFD[a\x17\xD5a\x14\xFBV[a\x17\xDE\x83a\x15cV[\x81R` \x83\x015\x82\x81\x11\x15a\x17\xF2W`\0\x80\xFD[a\x17\xFE\x88\x82\x86\x01a\x16\xF9V[` \x83\x01RPa\x18\x10`@\x84\x01a\x17aV[`@\x82\x01Ra\x18!``\x84\x01a\x17aV[``\x82\x01Ra\x182`\x80\x84\x01a\x15cV[`\x80\x82\x01Ra\x18C`\xA0\x84\x01a\x15cV[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x18dW`\0\x80\xFD[a\x18p\x88\x82\x86\x01a\x16\xF9V[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x94\x91\x93P\x90\x91PPV[\x82\x81R`@` \x82\x01R`\0a\x13P`@\x83\x01\x84a\x166V[`\0`@\x826\x03\x12\x15a\x18\xC6W`\0\x80\xFD[a\x18\xCEa\x14\xD3V[\x825\x81R` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xEBW`\0\x80\xFD[a\x18\xF76\x82\x86\x01a\x16\xF9V[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19\x1AW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x194W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x19IW`\0\x80\xFD[\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x19dW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x15\xBFWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x08\xB1W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x19\xABWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x12\xB2W\x82\x81U`\x01\x01a\x19\xB7V[`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17\x90V[`\x01`\x01`@\x1B\x03\x83\x11\x15a\x19\xF6Wa\x19\xF6a\x14\xBDV[a\x1A\n\x83a\x1A\x04\x83Ta\x19PV[\x83a\x19\x84V[`\0`\x1F\x84\x11`\x01\x81\x14a\x1A8W`\0\x85\x15a\x1A&WP\x83\x82\x015[a\x1A0\x86\x82a\x19\xCAV[\x84UPa\x1A\x92V[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x1AiW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1AIV[P\x86\x82\x10\x15a\x1A\x86W`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[\x815\x81U`\x01\x80\x82\x01` a\x1A\xB0\x81\x86\x01\x86a\x19\x03V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xC7Wa\x1A\xC7a\x14\xBDV[a\x1A\xDB\x81a\x1A\xD5\x86Ta\x19PV[\x86a\x19\x84V[`\0`\x1F\x82\x11`\x01\x81\x14a\x1B\tW`\0\x83\x15a\x1A\xF7WP\x83\x82\x015[a\x1B\x01\x84\x82a\x19\xCAV[\x87UPa\x1B^V[`\0\x86\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1B7W\x86\x85\x015\x82U\x93\x87\x01\x93\x90\x89\x01\x90\x87\x01a\x1B\x1AV[P\x84\x82\x10\x15a\x1BTW`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP\x86\x83\x88\x1B\x01\x86U[PPPPPPPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1B\x80W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\x9FW`\0\x80\xFD[\x806\x03\x82\x13\x15a\x19IW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R\x815` \x82\x01R`\0a\x1B\xF1` \x84\x01\x84a\x1BiV[`@\x80\x85\x01Ra\x16\x87``\x85\x01\x82\x84a\x1B\xAEV[`\0`@\x826\x03\x12\x15a\x1C\x17W`\0\x80\xFD[a\x1C\x1Fa\x14\xD3V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1C6W`\0\x80\xFD[a\x1CB6\x83\x87\x01a\x16\xF9V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x1CXW`\0\x80\xFD[Pa\x18\xF76\x82\x86\x01a\x16\xF9V[a\x1Co\x82\x83a\x19\x03V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\x86Wa\x1C\x86a\x14\xBDV[a\x1C\x9A\x81a\x1C\x94\x85Ta\x19PV[\x85a\x19\x84V[`\0`\x1F\x82\x11`\x01\x81\x14a\x1C\xC8W`\0\x83\x15a\x1C\xB6WP\x83\x82\x015[a\x1C\xC0\x84\x82a\x19\xCAV[\x86UPa\x1D\"V[`\0\x85\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1C\xF9W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1C\xD9V[P\x84\x82\x10\x15a\x1D\x16W`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x83`\x01\x1B\x01\x85U[PPPPa\x1D3` \x83\x01\x83a\x19\x03V[a\x07n\x81\x83`\x01\x86\x01a\x19\xDFV[` \x81R`\0a\x1DQ\x83\x84a\x1BiV[`@` \x85\x01Ra\x1Df``\x85\x01\x82\x84a\x1B\xAEV[\x91PPa\x1Dv` \x85\x01\x85a\x1BiV[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1D\x8D\x83\x82\x84a\x1B\xAEV[\x96\x95PPPPPPV[\x82\x81R`\0\x82Qa\x1D\xAF\x81` \x85\x01` \x87\x01a\x16\x12V[\x91\x90\x91\x01` \x01\x93\x92PPPV[`\0\x83Qa\x1D\xCF\x81\x84` \x88\x01a\x16\x12V[\x83Q\x90\x83\x01\x90a\x1D\xE3\x81\x83` \x88\x01a\x16\x12V[\x01\x94\x93PPPPV[\x80Qa\x15n\x81a\x15NV[`\0\x82`\x1F\x83\x01\x12a\x1E\x08W`\0\x80\xFD[\x81Qa\x1E\x16a\x17\x18\x82a\x16\xD2V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1E+W`\0\x80\xFD[a\x13P\x82` \x83\x01` \x87\x01a\x16\x12V[\x80Qa\x15n\x81a\x17OV[`\0` \x82\x84\x03\x12\x15a\x1EYW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1EpW`\0\x80\xFD[\x90\x83\x01\x90a\x01@\x82\x86\x03\x12\x15a\x1E\x85W`\0\x80\xFD[a\x1E\x8Da\x14\xFBV[a\x1E\x96\x83a\x1D\xECV[\x81R` \x83\x01Q\x82\x81\x11\x15a\x1E\xAAW`\0\x80\xFD[a\x1E\xB6\x87\x82\x86\x01a\x1D\xF7V[` \x83\x01RPa\x1E\xC8`@\x84\x01a\x1E<V[`@\x82\x01Ra\x1E\xD9``\x84\x01a\x1E<V[``\x82\x01Ra\x1E\xEA`\x80\x84\x01a\x1D\xECV[`\x80\x82\x01Ra\x1E\xFB`\xA0\x84\x01a\x1D\xECV[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R`\xE0\x83\x01Q\x82\x81\x11\x15a\x1F\x1CW`\0\x80\xFD[a\x1F(\x87\x82\x86\x01a\x1D\xF7V[`\xE0\x83\x01RPa\x01\0\x83\x81\x01Q\x90\x82\x01Ra\x01 \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a\x1FaW`\0\x80\xFD[a\x1Fia\x14\xD3V[\x82Qa\x1Ft\x81a\x15NV[\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1F\x9BW`\0\x80\xFD[\x81Qa\x1F\xA6\x81a\x15NV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1F\xBFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1F\xA6W`\0\x80\xFD[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Qa 6\x81`d\x85\x01` \x89\x01a\x16\x12V[\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xAE;\xA4\xBCB^\xA6VY_nN\xE4\x860\xB7\x010b\x1DG+\xA9=\xF5)O\xC1\x07\x846=dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static SWAPINTENTBOOK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SwapIntentBook<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SwapIntentBook<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SwapIntentBook<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SwapIntentBook<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SwapIntentBook<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SwapIntentBook))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SwapIntentBook<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SWAPINTENTBOOK_ABI.clone(),
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
                SWAPINTENTBOOK_ABI.clone(),
                SWAPINTENTBOOK_BYTECODE.clone().into(),
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
        ///Calls the contract's `intentBidData` (0xe25623e0) function
        pub fn intent_bid_data(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ([u8; 32], ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([226, 86, 35, 224], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intentData` (0x59a844b4) function
        pub fn intent_data(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Bytes, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([89, 168, 68, 180], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intentStates` (0x5ff8a66b) function
        pub fn intent_states(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, (u8, [u8; 32])> {
            self.0
                .method_hash([95, 248, 166, 107], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchAndSettle` (0x87f617b6) function
        pub fn match_and_settle(
            &self,
            intent_bid: IntentBid,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 246, 23, 182], (intent_bid,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchIntent` (0x09c7b2f6) function
        pub fn match_intent(
            &self,
            intent_bid: IntentBid,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 199, 178, 246], (intent_bid,))
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
        ///Calls the contract's `verify` (0x03895c91) function
        pub fn verify(
            &self,
            swap_intent_bid: SwapIntentBid,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 137, 92, 145], (swap_intent_bid,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySignature` (0xdbdf3903) function
        pub fn verify_signature(
            &self,
            signature: ::ethers::core::types::Bytes,
            swap_intent_order: SwapIntent,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 223, 57, 3], (signature, swap_intent_order))
                .expect("method not found (this should never happen)")
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
        ///Gets the contract's `IntentPartiallySettled` event
        pub fn intent_partially_settled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentPartiallySettledFilter,
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
            SwapIntentBookEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SwapIntentBook<M> {
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
    #[ethevent(
        name = "IntentMatch",
        abi = "IntentMatch(bytes32,bytes32,(bytes32,bytes))"
    )]
    pub struct IntentMatchFilter {
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
    #[ethevent(
        name = "IntentPartiallySettled",
        abi = "IntentPartiallySettled(bytes32,bytes32)"
    )]
    pub struct IntentPartiallySettledFilter {
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
    pub enum SwapIntentBookEvents {
        IntentCancelledFilter(IntentCancelledFilter),
        IntentCreatedFilter(IntentCreatedFilter),
        IntentMatchFilter(IntentMatchFilter),
        IntentPartiallySettledFilter(IntentPartiallySettledFilter),
        IntentSettledFilter(IntentSettledFilter),
    }
    impl ::ethers::contract::EthLogDecode for SwapIntentBookEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = IntentCancelledFilter::decode_log(log) {
                return Ok(SwapIntentBookEvents::IntentCancelledFilter(decoded));
            }
            if let Ok(decoded) = IntentCreatedFilter::decode_log(log) {
                return Ok(SwapIntentBookEvents::IntentCreatedFilter(decoded));
            }
            if let Ok(decoded) = IntentMatchFilter::decode_log(log) {
                return Ok(SwapIntentBookEvents::IntentMatchFilter(decoded));
            }
            if let Ok(decoded) = IntentPartiallySettledFilter::decode_log(log) {
                return Ok(SwapIntentBookEvents::IntentPartiallySettledFilter(decoded));
            }
            if let Ok(decoded) = IntentSettledFilter::decode_log(log) {
                return Ok(SwapIntentBookEvents::IntentSettledFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SwapIntentBookEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IntentCancelledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntentCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntentMatchFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentPartiallySettledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntentSettledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<IntentCancelledFilter> for SwapIntentBookEvents {
        fn from(value: IntentCancelledFilter) -> Self {
            Self::IntentCancelledFilter(value)
        }
    }
    impl ::core::convert::From<IntentCreatedFilter> for SwapIntentBookEvents {
        fn from(value: IntentCreatedFilter) -> Self {
            Self::IntentCreatedFilter(value)
        }
    }
    impl ::core::convert::From<IntentMatchFilter> for SwapIntentBookEvents {
        fn from(value: IntentMatchFilter) -> Self {
            Self::IntentMatchFilter(value)
        }
    }
    impl ::core::convert::From<IntentPartiallySettledFilter> for SwapIntentBookEvents {
        fn from(value: IntentPartiallySettledFilter) -> Self {
            Self::IntentPartiallySettledFilter(value)
        }
    }
    impl ::core::convert::From<IntentSettledFilter> for SwapIntentBookEvents {
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
    ///Container type for all input parameters for the `intentBidData` function with signature `intentBidData(bytes32)` and selector `0xe25623e0`
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
    #[ethcall(name = "intentBidData", abi = "intentBidData(bytes32)")]
    pub struct IntentBidDataCall(pub [u8; 32]);
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
    ///Container type for all input parameters for the `intentStates` function with signature `intentStates(bytes32)` and selector `0x5ff8a66b`
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
    #[ethcall(name = "intentStates", abi = "intentStates(bytes32)")]
    pub struct IntentStatesCall(pub [u8; 32]);
    ///Container type for all input parameters for the `matchAndSettle` function with signature `matchAndSettle((bytes32,bytes))` and selector `0x87f617b6`
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
    #[ethcall(name = "matchAndSettle", abi = "matchAndSettle((bytes32,bytes))")]
    pub struct MatchAndSettleCall {
        pub intent_bid: IntentBid,
    }
    ///Container type for all input parameters for the `matchIntent` function with signature `matchIntent((bytes32,bytes))` and selector `0x09c7b2f6`
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
    #[ethcall(name = "matchIntent", abi = "matchIntent((bytes32,bytes))")]
    pub struct MatchIntentCall {
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
    ///Container type for all input parameters for the `verify` function with signature `verify((address,uint256))` and selector `0x03895c91`
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
    #[ethcall(name = "verify", abi = "verify((address,uint256))")]
    pub struct VerifyCall {
        pub swap_intent_bid: SwapIntentBid,
    }
    ///Container type for all input parameters for the `verifySignature` function with signature `verifySignature(bytes,(address,bytes,uint32,uint32,address,address,uint256,bytes,uint256,uint256))` and selector `0xdbdf3903`
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
        name = "verifySignature",
        abi = "verifySignature(bytes,(address,bytes,uint32,uint32,address,address,uint256,bytes,uint256,uint256))"
    )]
    pub struct VerifySignatureCall {
        pub signature: ::ethers::core::types::Bytes,
        pub swap_intent_order: SwapIntent,
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
    pub enum SwapIntentBookCalls {
        CancelIntent(CancelIntentCall),
        IntentBidData(IntentBidDataCall),
        IntentData(IntentDataCall),
        IntentStates(IntentStatesCall),
        MatchAndSettle(MatchAndSettleCall),
        MatchIntent(MatchIntentCall),
        PlaceIntent(PlaceIntentCall),
        SettleIntent(SettleIntentCall),
        Verify(VerifyCall),
        VerifySignature(VerifySignatureCall),
    }
    impl ::ethers::core::abi::AbiDecode for SwapIntentBookCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CancelIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CancelIntent(decoded));
            }
            if let Ok(decoded) = <IntentBidDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntentBidData(decoded));
            }
            if let Ok(decoded) = <IntentDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntentData(decoded));
            }
            if let Ok(decoded) = <IntentStatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntentStates(decoded));
            }
            if let Ok(decoded) = <MatchAndSettleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MatchAndSettle(decoded));
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
            if let Ok(decoded) = <VerifyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Verify(decoded));
            }
            if let Ok(decoded) = <VerifySignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifySignature(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SwapIntentBookCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CancelIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntentBidData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntentData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntentStates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MatchAndSettle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MatchIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PlaceIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SettleIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Verify(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifySignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SwapIntentBookCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CancelIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentBidData(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentData(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentStates(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchAndSettle(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::PlaceIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettleIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::Verify(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifySignature(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CancelIntentCall> for SwapIntentBookCalls {
        fn from(value: CancelIntentCall) -> Self {
            Self::CancelIntent(value)
        }
    }
    impl ::core::convert::From<IntentBidDataCall> for SwapIntentBookCalls {
        fn from(value: IntentBidDataCall) -> Self {
            Self::IntentBidData(value)
        }
    }
    impl ::core::convert::From<IntentDataCall> for SwapIntentBookCalls {
        fn from(value: IntentDataCall) -> Self {
            Self::IntentData(value)
        }
    }
    impl ::core::convert::From<IntentStatesCall> for SwapIntentBookCalls {
        fn from(value: IntentStatesCall) -> Self {
            Self::IntentStates(value)
        }
    }
    impl ::core::convert::From<MatchAndSettleCall> for SwapIntentBookCalls {
        fn from(value: MatchAndSettleCall) -> Self {
            Self::MatchAndSettle(value)
        }
    }
    impl ::core::convert::From<MatchIntentCall> for SwapIntentBookCalls {
        fn from(value: MatchIntentCall) -> Self {
            Self::MatchIntent(value)
        }
    }
    impl ::core::convert::From<PlaceIntentCall> for SwapIntentBookCalls {
        fn from(value: PlaceIntentCall) -> Self {
            Self::PlaceIntent(value)
        }
    }
    impl ::core::convert::From<SettleIntentCall> for SwapIntentBookCalls {
        fn from(value: SettleIntentCall) -> Self {
            Self::SettleIntent(value)
        }
    }
    impl ::core::convert::From<VerifyCall> for SwapIntentBookCalls {
        fn from(value: VerifyCall) -> Self {
            Self::Verify(value)
        }
    }
    impl ::core::convert::From<VerifySignatureCall> for SwapIntentBookCalls {
        fn from(value: VerifySignatureCall) -> Self {
            Self::VerifySignature(value)
        }
    }
    ///Container type for all return fields from the `intentBidData` function with signature `intentBidData(bytes32)` and selector `0xe25623e0`
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
    pub struct IntentBidDataReturn {
        pub intent_id: [u8; 32],
        pub bid: ::ethers::core::types::Bytes,
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
        pub intent: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `intentStates` function with signature `intentStates(bytes32)` and selector `0x5ff8a66b`
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
    pub struct IntentStatesReturn {
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
    ///`SwapIntentBid(address,uint256)`
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
    pub struct SwapIntentBid {
        pub filler: ::ethers::core::types::Address,
        pub fill_amount: ::ethers::core::types::U256,
    }
}
