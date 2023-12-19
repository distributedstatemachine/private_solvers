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
                                            "struct SwapIntentLib.SwapIntentOrder",
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x1F\xBF\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x11a\0fW\x80c{\xF8\xBB\x88\x14a\x01KW\x80c\x87\xF6\x17\xB6\x14a\x01^W\x80c\xD5_\x96\r\x14a\x01qW\x80c\xDB\xDF9\x03\x14a\x01\x84W\x80c\xE2V#\xE0\x14a\x01\x97W`\0\x80\xFD[\x80c\t\xC7\xB2\xF6\x14a\0\xA3W\x80cJ\xF26N\x14a\0\xB8W\x80cV\x81f\xBB\x14a\0\xDEW\x80cY\xA8D\xB4\x14a\0\xEFW\x80c_\xF8\xA6k\x14a\x01\x10W[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x14\x88V[a\x01\xB8V[\0[a\0\xCBa\0\xC66`\x04a\x14\x88V[a\x03\xFFV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\0\xEC6`\x04a\x15\x9CV[PV[a\x01\x02a\0\xFD6`\x04a\x15\xE0V[a\x05.V[`@Qa\0\xD5\x92\x91\x90a\x16IV[a\x01=a\x01\x1E6`\x04a\x15\xE0V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\xD5\x92\x91\x90a\x16\x84V[a\0\xB6a\x01Y6`\x04a\x15\xE0V[a\x06ZV[a\0\xB6a\x01l6`\x04a\x14\x88V[a\x07gV[a\0\xB6a\x01\x7F6`\x04a\x15\xE0V[a\x07zV[a\0\xB6a\x01\x926`\x04a\x17JV[a\x08\x88V[a\x01\xAAa\x01\xA56`\x04a\x15\xE0V[a\x08\xA9V[`@Qa\0\xD5\x92\x91\x90a\x18yV[\x805`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x15a\x02\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FIntent already has a bid\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81T`\xFF\x16`\x03\x81\x11\x15a\x027Wa\x027a\x16nV[\x03a\x02|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\x94Wa\x02\x94a\x16nV[\x03a\x02\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\xF9Wa\x02\xF9a\x16nV[\x03a\x03BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0a\x03Ua\x03P\x85a\x18\x92V[a\x08\xCBV[`\0\x81\x81R`\x02` R`@\x90 T\x90\x91P\x15a\x03\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqBid already exists`p\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0\x81\x81R`\x02` R`@\x90 \x84\x90a\x03\xC3\x82\x82a\x1AwV[PP`\x01\x82\x01\x81\x90U`@Q\x81\x90\x84\x90\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x90`\0\x90\xA3PPPPV[`\0a\x04\x12a\x04\r\x83a\x1BGV[a\t\x05V[\x90Pa\x04\x1D\x81a\t\"V[`\0\x81\x81R` \x81\x90R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x04@Wa\x04@a\x16nV[\x14a\x04\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x02\x16V[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\x04\xC7Wa\x04\xC7a\x16nV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x82\x01U`\0\x83\x81R\x91R`@\x90 \x82\x90a\x04\xEF\x82\x82a\x1B\xA7V[PP\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x05!\x91\x90a\x1C\xF1V[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\x01` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x05I\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05u\x90a\x19.V[\x80\x15a\x05\xC2W\x80`\x1F\x10a\x05\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xC2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xA5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x05\xD7\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x03\x90a\x19.V[\x80\x15a\x06PW\x80`\x1F\x10a\x06%Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06PV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x063W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x80a\x06\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FIntent does not have a bid\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\0a\x06\xC6\x84\x83a\n\x88V[`\0`\x01\x80\x86\x01\x82\x90U\x84\x82R`\x02` R`@\x82 \x82\x81U\x92\x93Pa\x06\xEE\x90\x83\x01\x82a\x14\"V[PP\x80\x15a\x073W\x82T`\xFF\x19\x16`\x02\x17\x83U`@Q\x82\x90\x85\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3a\x07aV[`@Q\x82\x90\x85\x90\x7F\x84oK\x93k-|\xCF_\xCB\x9F1z\xB7\x91\xF5\xEC\xE5a\x11\x1E\x890n\x99}\x88\xBB\x84*<S\x90`\0\x90\xA3[PPPPV[a\x07p\x81a\x01\xB8V[a\0\xEC\x815a\x06ZV[`\0\x81\x81R` \x81\x90R`@\x90 `\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x07\xA0Wa\x07\xA0a\x16nV[\x03a\x07\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x08\x01Wa\x08\x01a\x16nV[\x03a\x08NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[\x80T`\xFF\x19\x16`\x03\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\0a\x08\x93\x82a\x0F\xF6V[\x90Pa\x08\xA4\x83\x83`\0\x01Q\x83a\x11\xBAV[PPPV[`\x02` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T\x91\x92\x91a\x05\xD7\x90a\x19.V[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x08\xE8\x92\x91\x90a\x1DGV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x08\xE8\x92\x91\x90a\x1DmV[`\0\x81\x81R`\x01` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x82\x90\x82\x90a\tH\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tt\x90a\x19.V[\x80\x15a\t\xC1W\x80`\x1F\x10a\t\x96Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xC1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xA4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\t\xDA\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x06\x90a\x19.V[\x80\x15a\nSW\x80`\x1F\x10a\n(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nSV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\nx\x91\x90a\x1D\xF7V[\x90Pa\x08\xA4\x82` \x01Q\x82a\x08\x88V[`\0\x82\x81R`\x01` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x83\x92\x91\x90\x82\x90\x82\x90a\n\xB2\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xDE\x90a\x19.V[\x80\x15a\x0B+W\x80`\x1F\x10a\x0B\0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B+V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x0EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x0BD\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Bp\x90a\x19.V[\x80\x15a\x0B\xBDW\x80`\x1F\x10a\x0B\x92Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xBDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xA0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\x0B\xE2\x91\x90a\x1D\xF7V[\x90P`\0`\x02`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80Ta\x0C\x1E\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0CJ\x90a\x19.V[\x80\x15a\x0C\x97W\x80`\x1F\x10a\x0ClWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\x97V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0CzW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81` \x01Q\x80` \x01\x90Q\x81\x01\x90a\x0C\xBC\x91\x90a\x1E\xFFV[`\x03T`@\x85\x81\x01Q\x90Qc\xB6n\x93_`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB6n\x93_\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r5\x91\x90a\x1FCV[`\x01`\x01`\xA0\x1B\x03\x16cu\xE3f\x16a\rL\x89a\x12\xA4V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rj\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xAD\x91\x90a\x1FgV[a\r\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Ftoken not locked at source\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\x03T``\x84\x01Q`@Qc\xB6n\x93_`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB6n\x93_\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EMW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Eq\x91\x90a\x1FCV[`\x01`\x01`\xA0\x1B\x03\x16cu\xE3f\x16a\x0E\x97\x89\x84`\0\x01Q\x85` \x01Q\x86`@\x01Qa\x12\xC3V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xB5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF8\x91\x90a\x1FgV[a\x0FNW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fswap not fulfilled at destinatio`D\x82\x01R`7`\xF9\x1B`d\x82\x01R`\x84\x01a\x02\x16V[`\x04\x80T`@\x80\x86\x01Q`\x80\x87\x01Q`\xC0\x88\x01Q\x86Q\x93Qc\x18\x06\xC3C`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x95c\x18\x06\xC3C\x95a\x0F\xB7\x95\x91\x01c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x85\x01R`@\x84\x01\x91\x90\x91R\x16``\x82\x01R`\x80\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xE5W=`\0\x80>=`\0\xFD[P`\x01\x9A\x99PPPPPPPPPPV[`@\x80\x82\x01Q\x81Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x82\x01R\x7F\xD4\xE2\xD2a\xFF\xA0\xA31\xA9>\xD4\xAE\xE4\x94\x07o\xDC\x03\xDA\x7F\x1A\x99\xDFT\xA9d\x8Ca\xCA\x1D\x85\x94\x92\x81\x01\x92\x90\x92R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\0\x90\x81\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7Fg\x8B?\x8C5\x04\xD3\x11\x9F\xA3\xD8\xC5D\xB2f\xEE\xF2\x94\x96\xC6\xAFn\xE7a|\xC0\x01\xFE\xC2\x1B{T\x84`\0\x01Q\x85``\x01Q\x86`\xA0\x01Q\x87`\xC0\x01Q\x88`@\x01Q\x89`\xE0\x01Q\x80Q\x90` \x01 \x8A`\x80\x01Q\x8Ba\x01\0\x01Q\x8Ca\x01 \x01Q`@Q` \x01a\x11g\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90\x99\x8AR`\x01`\x01`\xA0\x1B\x03\x98\x89\x16` \x8B\x01Rc\xFF\xFF\xFF\xFF\x97\x88\x16`@\x8B\x01R\x95\x88\x16``\x8A\x01R`\x80\x89\x01\x94\x90\x94R\x91\x90\x94\x16`\xA0\x87\x01R`\xC0\x86\x01\x93\x90\x93R\x91\x90\x92\x16`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x01 \x82\x01Ra\x01@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 a\x19\x01`\xF0\x1B\x82\x85\x01R`\"\x84\x01\x95\x90\x95R`B\x80\x84\x01\x95\x90\x95R\x81Q\x80\x84\x03\x90\x95\x01\x85R`b\x90\x92\x01\x90R\x82Q\x92\x01\x91\x90\x91 \x93\x92PPPV[`\0\x80`\0a\x11\xC8\x86a\x13\0V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x89\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x12.W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPPPPPV[`\0a\x12\xBD`@Q\x80` \x01`@R\x80\x84\x81RPa\x13\x82V[\x92\x91PPV[`\0a\x12\xF7`@Q\x80`\x80\x01`@R\x80\x87\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x81R` \x01\x84\x81RPa\x13\xB6V[\x95\x94PPPPPV[`\0\x80`\0\x83Q`A\x14a\x13fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x08\xE8V[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x08\xE8\x94\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x94\x90\x94R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x84\x01R`D\x83\x01R`d\x82\x01R`\x84\x01\x90V[P\x80Ta\x14.\x90a\x19.V[`\0\x82U\x80`\x1F\x10a\x14>WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\0\xEC\x91\x90[\x80\x82\x11\x15a\x14lW`\0\x81U`\x01\x01a\x14XV[P\x90V[`\0`@\x82\x84\x03\x12\x15a\x14\x82W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x14\x9AW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xB0W`\0\x80\xFD[a\x14\xBC\x84\x82\x85\x01a\x14pV[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xFCWa\x14\xFCa\x14\xC4V[`@R\x90V[`@Qa\x01@\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xFCWa\x14\xFCa\x14\xC4V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xFCWa\x14\xFCa\x14\xC4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15oWa\x15oa\x14\xC4V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xECW`\0\x80\xFD[\x805a\x15\x97\x81a\x15wV[\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x15\xAEW`\0\x80\xFD[a\x15\xB6a\x14\xDAV[\x825a\x15\xC1\x81a\x15wV[\x81R` \x83\x81\x015\x90\x82\x01R`@\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x15\xF2W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x16\x14W\x81\x81\x01Q\x83\x82\x01R` \x01a\x15\xFCV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x165\x81` \x86\x01` \x86\x01a\x15\xF9V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\x16\\`@\x83\x01\x85a\x16\x1DV[\x82\x81\x03` \x84\x01Ra\x12\xF7\x81\x85a\x16\x1DV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x04\x84\x10a\x16\xA6WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x16\xC9Wa\x16\xC9a\x14\xC4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x16\xE8W`\0\x80\xFD[\x815a\x16\xFBa\x16\xF6\x82a\x16\xB0V[a\x15GV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x17\x10W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\0\xECW`\0\x80\xFD[\x805a\x15\x97\x81a\x17-V[`\0\x80`@\x83\x85\x03\x12\x15a\x17]W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17tW`\0\x80\xFD[a\x17\x80\x86\x83\x87\x01a\x16\xD7V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x17\x96W`\0\x80\xFD[\x90\x84\x01\x90a\x01@\x82\x87\x03\x12\x15a\x17\xABW`\0\x80\xFD[a\x17\xB3a\x15\x02V[a\x17\xBC\x83a\x15\x8CV[\x81R` \x83\x015\x82\x81\x11\x15a\x17\xD0W`\0\x80\xFD[a\x17\xDC\x88\x82\x86\x01a\x16\xD7V[` \x83\x01RPa\x17\xEE`@\x84\x01a\x17?V[`@\x82\x01Ra\x17\xFF``\x84\x01a\x17?V[``\x82\x01Ra\x18\x10`\x80\x84\x01a\x15\x8CV[`\x80\x82\x01Ra\x18!`\xA0\x84\x01a\x15\x8CV[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x18BW`\0\x80\xFD[a\x18N\x88\x82\x86\x01a\x16\xD7V[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x94\x91\x93P\x90\x91PPV[\x82\x81R`@` \x82\x01R`\0a\x14\xBC`@\x83\x01\x84a\x16\x1DV[`\0`@\x826\x03\x12\x15a\x18\xA4W`\0\x80\xFD[a\x18\xACa\x15%V[\x825\x81R` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xC9W`\0\x80\xFD[a\x18\xD56\x82\x86\x01a\x16\xD7V[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18\xF8W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19\x12W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x19'W`\0\x80\xFD[\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x19BW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x14\x82WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x08\xA4W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x19\x89WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x12\x9CW\x82\x81U`\x01\x01a\x19\x95V[`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17\x90V[`\x01`\x01`@\x1B\x03\x83\x11\x15a\x19\xD4Wa\x19\xD4a\x14\xC4V[a\x19\xE8\x83a\x19\xE2\x83Ta\x19.V[\x83a\x19bV[`\0`\x1F\x84\x11`\x01\x81\x14a\x1A\x16W`\0\x85\x15a\x1A\x04WP\x83\x82\x015[a\x1A\x0E\x86\x82a\x19\xA8V[\x84UPa\x1ApV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x1AGW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1A'V[P\x86\x82\x10\x15a\x1AdW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[\x815\x81U`\x01\x80\x82\x01` a\x1A\x8E\x81\x86\x01\x86a\x18\xE1V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xA5Wa\x1A\xA5a\x14\xC4V[a\x1A\xB9\x81a\x1A\xB3\x86Ta\x19.V[\x86a\x19bV[`\0`\x1F\x82\x11`\x01\x81\x14a\x1A\xE7W`\0\x83\x15a\x1A\xD5WP\x83\x82\x015[a\x1A\xDF\x84\x82a\x19\xA8V[\x87UPa\x1B<V[`\0\x86\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1B\x15W\x86\x85\x015\x82U\x93\x87\x01\x93\x90\x89\x01\x90\x87\x01a\x1A\xF8V[P\x84\x82\x10\x15a\x1B2W`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP\x86\x83\x88\x1B\x01\x86U[PPPPPPPPPV[`\0`@\x826\x03\x12\x15a\x1BYW`\0\x80\xFD[a\x1Baa\x15%V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1BxW`\0\x80\xFD[a\x1B\x846\x83\x87\x01a\x16\xD7V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x1B\x9AW`\0\x80\xFD[Pa\x18\xD56\x82\x86\x01a\x16\xD7V[a\x1B\xB1\x82\x83a\x18\xE1V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xC8Wa\x1B\xC8a\x14\xC4V[a\x1B\xDC\x81a\x1B\xD6\x85Ta\x19.V[\x85a\x19bV[`\0`\x1F\x82\x11`\x01\x81\x14a\x1C\nW`\0\x83\x15a\x1B\xF8WP\x83\x82\x015[a\x1C\x02\x84\x82a\x19\xA8V[\x86UPa\x1CdV[`\0\x85\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1C;W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1C\x1BV[P\x84\x82\x10\x15a\x1CXW`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x83`\x01\x1B\x01\x85U[PPPPa\x1Cu` \x83\x01\x83a\x18\xE1V[a\x07a\x81\x83`\x01\x86\x01a\x19\xBDV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1C\x9AW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xB9W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x19'W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\x1D\x01\x83\x84a\x1C\x83V[`@` \x85\x01Ra\x1D\x16``\x85\x01\x82\x84a\x1C\xC8V[\x91PPa\x1D&` \x85\x01\x85a\x1C\x83V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1D=\x83\x82\x84a\x1C\xC8V[\x96\x95PPPPPPV[\x82\x81R`\0\x82Qa\x1D_\x81` \x85\x01` \x87\x01a\x15\xF9V[\x91\x90\x91\x01` \x01\x93\x92PPPV[`\0\x83Qa\x1D\x7F\x81\x84` \x88\x01a\x15\xF9V[\x83Q\x90\x83\x01\x90a\x1D\x93\x81\x83` \x88\x01a\x15\xF9V[\x01\x94\x93PPPPV[\x80Qa\x15\x97\x81a\x15wV[`\0\x82`\x1F\x83\x01\x12a\x1D\xB8W`\0\x80\xFD[\x81Qa\x1D\xC6a\x16\xF6\x82a\x16\xB0V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1D\xDBW`\0\x80\xFD[a\x14\xBC\x82` \x83\x01` \x87\x01a\x15\xF9V[\x80Qa\x15\x97\x81a\x17-V[`\0` \x82\x84\x03\x12\x15a\x1E\tW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1E W`\0\x80\xFD[\x90\x83\x01\x90a\x01@\x82\x86\x03\x12\x15a\x1E5W`\0\x80\xFD[a\x1E=a\x15\x02V[a\x1EF\x83a\x1D\x9CV[\x81R` \x83\x01Q\x82\x81\x11\x15a\x1EZW`\0\x80\xFD[a\x1Ef\x87\x82\x86\x01a\x1D\xA7V[` \x83\x01RPa\x1Ex`@\x84\x01a\x1D\xECV[`@\x82\x01Ra\x1E\x89``\x84\x01a\x1D\xECV[``\x82\x01Ra\x1E\x9A`\x80\x84\x01a\x1D\x9CV[`\x80\x82\x01Ra\x1E\xAB`\xA0\x84\x01a\x1D\x9CV[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R`\xE0\x83\x01Q\x82\x81\x11\x15a\x1E\xCCW`\0\x80\xFD[a\x1E\xD8\x87\x82\x86\x01a\x1D\xA7V[`\xE0\x83\x01RPa\x01\0\x83\x81\x01Q\x90\x82\x01Ra\x01 \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a\x1F\x11W`\0\x80\xFD[a\x1F\x19a\x14\xDAV[\x82Qa\x1F$\x81a\x15wV[\x81R` \x83\x81\x01Q\x90\x82\x01R`@\x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1FUW`\0\x80\xFD[\x81Qa\x1F`\x81a\x15wV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1FyW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1F`W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 0p\t_\x9E\xEF\xC1U#;k?b\xE3\xA9\x8A,\x04>\x08\x87\xAF\xDC\xD5W\xDBzF\x88\x1Ez:dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SWAPINTENTBOOK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x11a\0fW\x80c{\xF8\xBB\x88\x14a\x01KW\x80c\x87\xF6\x17\xB6\x14a\x01^W\x80c\xD5_\x96\r\x14a\x01qW\x80c\xDB\xDF9\x03\x14a\x01\x84W\x80c\xE2V#\xE0\x14a\x01\x97W`\0\x80\xFD[\x80c\t\xC7\xB2\xF6\x14a\0\xA3W\x80cJ\xF26N\x14a\0\xB8W\x80cV\x81f\xBB\x14a\0\xDEW\x80cY\xA8D\xB4\x14a\0\xEFW\x80c_\xF8\xA6k\x14a\x01\x10W[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x14\x88V[a\x01\xB8V[\0[a\0\xCBa\0\xC66`\x04a\x14\x88V[a\x03\xFFV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB6a\0\xEC6`\x04a\x15\x9CV[PV[a\x01\x02a\0\xFD6`\x04a\x15\xE0V[a\x05.V[`@Qa\0\xD5\x92\x91\x90a\x16IV[a\x01=a\x01\x1E6`\x04a\x15\xE0V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\xD5\x92\x91\x90a\x16\x84V[a\0\xB6a\x01Y6`\x04a\x15\xE0V[a\x06ZV[a\0\xB6a\x01l6`\x04a\x14\x88V[a\x07gV[a\0\xB6a\x01\x7F6`\x04a\x15\xE0V[a\x07zV[a\0\xB6a\x01\x926`\x04a\x17JV[a\x08\x88V[a\x01\xAAa\x01\xA56`\x04a\x15\xE0V[a\x08\xA9V[`@Qa\0\xD5\x92\x91\x90a\x18yV[\x805`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x15a\x02\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FIntent already has a bid\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81T`\xFF\x16`\x03\x81\x11\x15a\x027Wa\x027a\x16nV[\x03a\x02|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\x94Wa\x02\x94a\x16nV[\x03a\x02\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x02\xF9Wa\x02\xF9a\x16nV[\x03a\x03BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0a\x03Ua\x03P\x85a\x18\x92V[a\x08\xCBV[`\0\x81\x81R`\x02` R`@\x90 T\x90\x91P\x15a\x03\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqBid already exists`p\x1B`D\x82\x01R`d\x01a\x02\x16V[`\0\x81\x81R`\x02` R`@\x90 \x84\x90a\x03\xC3\x82\x82a\x1AwV[PP`\x01\x82\x01\x81\x90U`@Q\x81\x90\x84\x90\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x90`\0\x90\xA3PPPPV[`\0a\x04\x12a\x04\r\x83a\x1BGV[a\t\x05V[\x90Pa\x04\x1D\x81a\t\"V[`\0\x81\x81R` \x81\x90R`@\x81 T`\xFF\x16`\x03\x81\x11\x15a\x04@Wa\x04@a\x16nV[\x14a\x04\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x02\x16V[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x03\x81\x11\x15a\x04\xC7Wa\x04\xC7a\x16nV[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x91\x82\x01U`\0\x83\x81R\x91R`@\x90 \x82\x90a\x04\xEF\x82\x82a\x1B\xA7V[PP\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x05!\x91\x90a\x1C\xF1V[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\x01` R`\0\x90\x81R`@\x90 \x80T\x81\x90a\x05I\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05u\x90a\x19.V[\x80\x15a\x05\xC2W\x80`\x1F\x10a\x05\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xC2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xA5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x05\xD7\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x03\x90a\x19.V[\x80\x15a\x06PW\x80`\x1F\x10a\x06%Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06PV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x063W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81\x01T\x80a\x06\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FIntent does not have a bid\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\0a\x06\xC6\x84\x83a\n\x88V[`\0`\x01\x80\x86\x01\x82\x90U\x84\x82R`\x02` R`@\x82 \x82\x81U\x92\x93Pa\x06\xEE\x90\x83\x01\x82a\x14\"V[PP\x80\x15a\x073W\x82T`\xFF\x19\x16`\x02\x17\x83U`@Q\x82\x90\x85\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3a\x07aV[`@Q\x82\x90\x85\x90\x7F\x84oK\x93k-|\xCF_\xCB\x9F1z\xB7\x91\xF5\xEC\xE5a\x11\x1E\x890n\x99}\x88\xBB\x84*<S\x90`\0\x90\xA3[PPPPV[a\x07p\x81a\x01\xB8V[a\0\xEC\x815a\x06ZV[`\0\x81\x81R` \x81\x90R`@\x90 `\x02\x81T`\xFF\x16`\x03\x81\x11\x15a\x07\xA0Wa\x07\xA0a\x16nV[\x03a\x07\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x12[\x9D\x19[\x9D\x08\x1A\\\xC8\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`:\x1B`D\x82\x01R`d\x01a\x02\x16V[`\x03\x81T`\xFF\x16`\x03\x81\x11\x15a\x08\x01Wa\x08\x01a\x16nV[\x03a\x08NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FIntent is already cancelled\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[\x80T`\xFF\x19\x16`\x03\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\0a\x08\x93\x82a\x0F\xF6V[\x90Pa\x08\xA4\x83\x83`\0\x01Q\x83a\x11\xBAV[PPPV[`\x02` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T\x91\x92\x91a\x05\xD7\x90a\x19.V[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x08\xE8\x92\x91\x90a\x1DGV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x08\xE8\x92\x91\x90a\x1DmV[`\0\x81\x81R`\x01` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x82\x90\x82\x90a\tH\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tt\x90a\x19.V[\x80\x15a\t\xC1W\x80`\x1F\x10a\t\x96Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xC1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xA4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\t\xDA\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x06\x90a\x19.V[\x80\x15a\nSW\x80`\x1F\x10a\n(Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nSV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\nx\x91\x90a\x1D\xF7V[\x90Pa\x08\xA4\x82` \x01Q\x82a\x08\x88V[`\0\x82\x81R`\x01` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x83\x92\x91\x90\x82\x90\x82\x90a\n\xB2\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xDE\x90a\x19.V[\x80\x15a\x0B+W\x80`\x1F\x10a\x0B\0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B+V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x0EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x0BD\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Bp\x90a\x19.V[\x80\x15a\x0B\xBDW\x80`\x1F\x10a\x0B\x92Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xBDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xA0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81`\0\x01Q\x80` \x01\x90Q\x81\x01\x90a\x0B\xE2\x91\x90a\x1D\xF7V[\x90P`\0`\x02`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`@\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01\x80Ta\x0C\x1E\x90a\x19.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0CJ\x90a\x19.V[\x80\x15a\x0C\x97W\x80`\x1F\x10a\x0ClWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\x97V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0CzW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`\0\x81` \x01Q\x80` \x01\x90Q\x81\x01\x90a\x0C\xBC\x91\x90a\x1E\xFFV[`\x03T`@\x85\x81\x01Q\x90Qc\xB6n\x93_`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB6n\x93_\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r5\x91\x90a\x1FCV[`\x01`\x01`\xA0\x1B\x03\x16cu\xE3f\x16a\rL\x89a\x12\xA4V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rj\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xAD\x91\x90a\x1FgV[a\r\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Ftoken not locked at source\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02\x16V[`\x03T``\x84\x01Q`@Qc\xB6n\x93_`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB6n\x93_\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EMW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Eq\x91\x90a\x1FCV[`\x01`\x01`\xA0\x1B\x03\x16cu\xE3f\x16a\x0E\x97\x89\x84`\0\x01Q\x85` \x01Q\x86`@\x01Qa\x12\xC3V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xB5\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0E\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xF8\x91\x90a\x1FgV[a\x0FNW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fswap not fulfilled at destinatio`D\x82\x01R`7`\xF9\x1B`d\x82\x01R`\x84\x01a\x02\x16V[`\x04\x80T`@\x80\x86\x01Q`\x80\x87\x01Q`\xC0\x88\x01Q\x86Q\x93Qc\x18\x06\xC3C`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x95c\x18\x06\xC3C\x95a\x0F\xB7\x95\x91\x01c\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x85\x01R`@\x84\x01\x91\x90\x91R\x16``\x82\x01R`\x80\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xE5W=`\0\x80>=`\0\xFD[P`\x01\x9A\x99PPPPPPPPPPV[`@\x80\x82\x01Q\x81Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x82\x01R\x7F\xD4\xE2\xD2a\xFF\xA0\xA31\xA9>\xD4\xAE\xE4\x94\x07o\xDC\x03\xDA\x7F\x1A\x99\xDFT\xA9d\x8Ca\xCA\x1D\x85\x94\x92\x81\x01\x92\x90\x92R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\0\x90\x81\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7Fg\x8B?\x8C5\x04\xD3\x11\x9F\xA3\xD8\xC5D\xB2f\xEE\xF2\x94\x96\xC6\xAFn\xE7a|\xC0\x01\xFE\xC2\x1B{T\x84`\0\x01Q\x85``\x01Q\x86`\xA0\x01Q\x87`\xC0\x01Q\x88`@\x01Q\x89`\xE0\x01Q\x80Q\x90` \x01 \x8A`\x80\x01Q\x8Ba\x01\0\x01Q\x8Ca\x01 \x01Q`@Q` \x01a\x11g\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90\x99\x8AR`\x01`\x01`\xA0\x1B\x03\x98\x89\x16` \x8B\x01Rc\xFF\xFF\xFF\xFF\x97\x88\x16`@\x8B\x01R\x95\x88\x16``\x8A\x01R`\x80\x89\x01\x94\x90\x94R\x91\x90\x94\x16`\xA0\x87\x01R`\xC0\x86\x01\x93\x90\x93R\x91\x90\x92\x16`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x01 \x82\x01Ra\x01@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 a\x19\x01`\xF0\x1B\x82\x85\x01R`\"\x84\x01\x95\x90\x95R`B\x80\x84\x01\x95\x90\x95R\x81Q\x80\x84\x03\x90\x95\x01\x85R`b\x90\x92\x01\x90R\x82Q\x92\x01\x91\x90\x91 \x93\x92PPPV[`\0\x80`\0a\x11\xC8\x86a\x13\0V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x89\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x12.W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPPPPPV[`\0a\x12\xBD`@Q\x80` \x01`@R\x80\x84\x81RPa\x13\x82V[\x92\x91PPV[`\0a\x12\xF7`@Q\x80`\x80\x01`@R\x80\x87\x81R` \x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x81R` \x01\x84\x81RPa\x13\xB6V[\x95\x94PPPPPV[`\0\x80`\0\x83Q`A\x14a\x13fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x02\x16V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x08\xE8V[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q`@Q` \x01a\x08\xE8\x94\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x94\x90\x94R``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x84\x01R`D\x83\x01R`d\x82\x01R`\x84\x01\x90V[P\x80Ta\x14.\x90a\x19.V[`\0\x82U\x80`\x1F\x10a\x14>WPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90a\0\xEC\x91\x90[\x80\x82\x11\x15a\x14lW`\0\x81U`\x01\x01a\x14XV[P\x90V[`\0`@\x82\x84\x03\x12\x15a\x14\x82W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x14\x9AW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xB0W`\0\x80\xFD[a\x14\xBC\x84\x82\x85\x01a\x14pV[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xFCWa\x14\xFCa\x14\xC4V[`@R\x90V[`@Qa\x01@\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xFCWa\x14\xFCa\x14\xC4V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x14\xFCWa\x14\xFCa\x14\xC4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15oWa\x15oa\x14\xC4V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xECW`\0\x80\xFD[\x805a\x15\x97\x81a\x15wV[\x91\x90PV[`\0``\x82\x84\x03\x12\x15a\x15\xAEW`\0\x80\xFD[a\x15\xB6a\x14\xDAV[\x825a\x15\xC1\x81a\x15wV[\x81R` \x83\x81\x015\x90\x82\x01R`@\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x15\xF2W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x16\x14W\x81\x81\x01Q\x83\x82\x01R` \x01a\x15\xFCV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x165\x81` \x86\x01` \x86\x01a\x15\xF9V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a\x16\\`@\x83\x01\x85a\x16\x1DV[\x82\x81\x03` \x84\x01Ra\x12\xF7\x81\x85a\x16\x1DV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x04\x84\x10a\x16\xA6WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x16\xC9Wa\x16\xC9a\x14\xC4V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x16\xE8W`\0\x80\xFD[\x815a\x16\xFBa\x16\xF6\x82a\x16\xB0V[a\x15GV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x17\x10W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\0\xECW`\0\x80\xFD[\x805a\x15\x97\x81a\x17-V[`\0\x80`@\x83\x85\x03\x12\x15a\x17]W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17tW`\0\x80\xFD[a\x17\x80\x86\x83\x87\x01a\x16\xD7V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x17\x96W`\0\x80\xFD[\x90\x84\x01\x90a\x01@\x82\x87\x03\x12\x15a\x17\xABW`\0\x80\xFD[a\x17\xB3a\x15\x02V[a\x17\xBC\x83a\x15\x8CV[\x81R` \x83\x015\x82\x81\x11\x15a\x17\xD0W`\0\x80\xFD[a\x17\xDC\x88\x82\x86\x01a\x16\xD7V[` \x83\x01RPa\x17\xEE`@\x84\x01a\x17?V[`@\x82\x01Ra\x17\xFF``\x84\x01a\x17?V[``\x82\x01Ra\x18\x10`\x80\x84\x01a\x15\x8CV[`\x80\x82\x01Ra\x18!`\xA0\x84\x01a\x15\x8CV[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x18BW`\0\x80\xFD[a\x18N\x88\x82\x86\x01a\x16\xD7V[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x94\x91\x93P\x90\x91PPV[\x82\x81R`@` \x82\x01R`\0a\x14\xBC`@\x83\x01\x84a\x16\x1DV[`\0`@\x826\x03\x12\x15a\x18\xA4W`\0\x80\xFD[a\x18\xACa\x15%V[\x825\x81R` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xC9W`\0\x80\xFD[a\x18\xD56\x82\x86\x01a\x16\xD7V[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18\xF8W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19\x12W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x19'W`\0\x80\xFD[\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x19BW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x14\x82WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x1F\x82\x11\x15a\x08\xA4W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x19\x89WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x12\x9CW\x82\x81U`\x01\x01a\x19\x95V[`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17\x90V[`\x01`\x01`@\x1B\x03\x83\x11\x15a\x19\xD4Wa\x19\xD4a\x14\xC4V[a\x19\xE8\x83a\x19\xE2\x83Ta\x19.V[\x83a\x19bV[`\0`\x1F\x84\x11`\x01\x81\x14a\x1A\x16W`\0\x85\x15a\x1A\x04WP\x83\x82\x015[a\x1A\x0E\x86\x82a\x19\xA8V[\x84UPa\x1ApV[`\0\x83\x81R` \x90 `\x1F\x19\x86\x16\x90\x83[\x82\x81\x10\x15a\x1AGW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1A'V[P\x86\x82\x10\x15a\x1AdW`\0\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83U[PPPPPV[\x815\x81U`\x01\x80\x82\x01` a\x1A\x8E\x81\x86\x01\x86a\x18\xE1V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xA5Wa\x1A\xA5a\x14\xC4V[a\x1A\xB9\x81a\x1A\xB3\x86Ta\x19.V[\x86a\x19bV[`\0`\x1F\x82\x11`\x01\x81\x14a\x1A\xE7W`\0\x83\x15a\x1A\xD5WP\x83\x82\x015[a\x1A\xDF\x84\x82a\x19\xA8V[\x87UPa\x1B<V[`\0\x86\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1B\x15W\x86\x85\x015\x82U\x93\x87\x01\x93\x90\x89\x01\x90\x87\x01a\x1A\xF8V[P\x84\x82\x10\x15a\x1B2W`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP\x86\x83\x88\x1B\x01\x86U[PPPPPPPPPV[`\0`@\x826\x03\x12\x15a\x1BYW`\0\x80\xFD[a\x1Baa\x15%V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1BxW`\0\x80\xFD[a\x1B\x846\x83\x87\x01a\x16\xD7V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\x1B\x9AW`\0\x80\xFD[Pa\x18\xD56\x82\x86\x01a\x16\xD7V[a\x1B\xB1\x82\x83a\x18\xE1V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xC8Wa\x1B\xC8a\x14\xC4V[a\x1B\xDC\x81a\x1B\xD6\x85Ta\x19.V[\x85a\x19bV[`\0`\x1F\x82\x11`\x01\x81\x14a\x1C\nW`\0\x83\x15a\x1B\xF8WP\x83\x82\x015[a\x1C\x02\x84\x82a\x19\xA8V[\x86UPa\x1CdV[`\0\x85\x81R` \x90 `\x1F\x19\x84\x16\x90\x83[\x82\x81\x10\x15a\x1C;W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1C\x1BV[P\x84\x82\x10\x15a\x1CXW`\0\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x83`\x01\x1B\x01\x85U[PPPPa\x1Cu` \x83\x01\x83a\x18\xE1V[a\x07a\x81\x83`\x01\x86\x01a\x19\xBDV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1C\x9AW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xB9W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x19'W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\x1D\x01\x83\x84a\x1C\x83V[`@` \x85\x01Ra\x1D\x16``\x85\x01\x82\x84a\x1C\xC8V[\x91PPa\x1D&` \x85\x01\x85a\x1C\x83V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1D=\x83\x82\x84a\x1C\xC8V[\x96\x95PPPPPPV[\x82\x81R`\0\x82Qa\x1D_\x81` \x85\x01` \x87\x01a\x15\xF9V[\x91\x90\x91\x01` \x01\x93\x92PPPV[`\0\x83Qa\x1D\x7F\x81\x84` \x88\x01a\x15\xF9V[\x83Q\x90\x83\x01\x90a\x1D\x93\x81\x83` \x88\x01a\x15\xF9V[\x01\x94\x93PPPPV[\x80Qa\x15\x97\x81a\x15wV[`\0\x82`\x1F\x83\x01\x12a\x1D\xB8W`\0\x80\xFD[\x81Qa\x1D\xC6a\x16\xF6\x82a\x16\xB0V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1D\xDBW`\0\x80\xFD[a\x14\xBC\x82` \x83\x01` \x87\x01a\x15\xF9V[\x80Qa\x15\x97\x81a\x17-V[`\0` \x82\x84\x03\x12\x15a\x1E\tW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1E W`\0\x80\xFD[\x90\x83\x01\x90a\x01@\x82\x86\x03\x12\x15a\x1E5W`\0\x80\xFD[a\x1E=a\x15\x02V[a\x1EF\x83a\x1D\x9CV[\x81R` \x83\x01Q\x82\x81\x11\x15a\x1EZW`\0\x80\xFD[a\x1Ef\x87\x82\x86\x01a\x1D\xA7V[` \x83\x01RPa\x1Ex`@\x84\x01a\x1D\xECV[`@\x82\x01Ra\x1E\x89``\x84\x01a\x1D\xECV[``\x82\x01Ra\x1E\x9A`\x80\x84\x01a\x1D\x9CV[`\x80\x82\x01Ra\x1E\xAB`\xA0\x84\x01a\x1D\x9CV[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R`\xE0\x83\x01Q\x82\x81\x11\x15a\x1E\xCCW`\0\x80\xFD[a\x1E\xD8\x87\x82\x86\x01a\x1D\xA7V[`\xE0\x83\x01RPa\x01\0\x83\x81\x01Q\x90\x82\x01Ra\x01 \x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a\x1F\x11W`\0\x80\xFD[a\x1F\x19a\x14\xDAV[\x82Qa\x1F$\x81a\x15wV[\x81R` \x83\x81\x01Q\x90\x82\x01R`@\x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1FUW`\0\x80\xFD[\x81Qa\x1F`\x81a\x15wV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1FyW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1F`W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 0p\t_\x9E\xEF\xC1U#;k?b\xE3\xA9\x8A,\x04>\x08\x87\xAF\xDC\xD5W\xDBzF\x88\x1Ez:dsolcC\0\x08\x13\x003";
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
        ///Calls the contract's `verify` (0x568166bb) function
        pub fn verify(
            &self,
            swap_intent_bid: SwapIntentBid,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 129, 102, 187], (swap_intent_bid,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySignature` (0xdbdf3903) function
        pub fn verify_signature(
            &self,
            signature: ::ethers::core::types::Bytes,
            swap_intent_order: SwapIntentOrder,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 223, 57, 3], (signature, swap_intent_order))
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
    #[ethevent(
        name = "IntentBidReceived",
        abi = "IntentBidReceived(bytes32,bytes32,(bytes32,bytes))"
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
        IntentBidReceivedFilter(IntentBidReceivedFilter),
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
            if let Ok(decoded) = IntentBidReceivedFilter::decode_log(log) {
                return Ok(SwapIntentBookEvents::IntentBidReceivedFilter(decoded));
            }
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
                Self::IntentPartiallySettledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntentSettledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<IntentBidReceivedFilter> for SwapIntentBookEvents {
        fn from(value: IntentBidReceivedFilter) -> Self {
            Self::IntentBidReceivedFilter(value)
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
    ///Container type for all input parameters for the `verify` function with signature `verify((address,uint256,uint256))` and selector `0x568166bb`
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
    #[ethcall(name = "verify", abi = "verify((address,uint256,uint256))")]
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
        pub swap_intent_order: SwapIntentOrder,
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
    ///`SwapIntentBid(address,uint256,uint256)`
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
        pub fill_timestamp: ::ethers::core::types::U256,
        pub fill_amount: ::ethers::core::types::U256,
    }
    ///`SwapIntentOrder(address,bytes,uint32,uint32,address,address,uint256,bytes,uint256,uint256)`
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
    pub struct SwapIntentOrder {
        pub author: ::ethers::core::types::Address,
        pub signature: ::ethers::core::types::Bytes,
        pub source_chain_id: u32,
        pub destination_chain_id: u32,
        pub source_token: ::ethers::core::types::Address,
        pub destination_token: ::ethers::core::types::Address,
        pub source_amount: ::ethers::core::types::U256,
        pub source_permit_2: ::ethers::core::types::Bytes,
        pub nonce: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
    }
}
