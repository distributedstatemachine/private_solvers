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
                                    name: ::std::borrow::ToOwned::to_owned("filler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fillTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fillAmount"),
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
                                    name: ::std::borrow::ToOwned::to_owned("signature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationChainId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("destinationToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourcePermit2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
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
    pub static SWAPINTENTBOOK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x12\x04\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x11a\0[W\x80c{\xF8\xBB\x88\x14a\0\xE6W\x80c\x90!W\x8A\x14a\0\xF9W\x80c\xD5_\x96\r\x14a\x014W\x80c\xE2V#\xE0\x14a\x01GW`\0\x80\xFD[\x80cJ\xF26N\x14a\0\x82W\x80cY\xA8D\xB4\x14a\0\xA8W\x80cr\xB8v\xEB\x14a\0\xD1W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x0B\x04V[a\x01\xA6V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xBBa\0\xB66`\x04a\x0BFV[a\x02\xCCV[`@Qa\0\x9F\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0B\xAFV[a\0\xE4a\0\xDF6`\x04a\x0C0V[a\x04UV[\0[a\0\xE4a\0\xF46`\x04a\x0BFV[a\x04\xA5V[a\x01&a\x01\x076`\x04a\x0BFV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\x9F\x92\x91\x90a\x0C\x94V[a\0\xE4a\x01B6`\x04a\x0BFV[a\x05HV[a\x01\x81a\x01U6`\x04a\x0BFV[`\x02` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x91\x90\x92\x01T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x83V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\x9FV[`\0a\x01\xB9a\x01\xB4\x83a\r\x8DV[a\x06\x03V[\x90Pa\x01\xC4\x82a\x06=V[`\0\x81\x81R` \x81\x90R`@\x81 T`\xFF\x16`\x05\x81\x11\x15a\x01\xE7Wa\x01\xE7a\x0C~V[\x14a\x021W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x05\x81\x11\x15a\x02sWa\x02sa\x0C~V[\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U\x90PPa\x02\x8F\x82\x82a\x06\xB8V[\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x02\xBF\x91\x90a\x0E\x89V[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\x01` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T\x91\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92a\x02\xF9\x90a\x0E\xDFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03%\x90a\x0E\xDFV[\x80\x15a\x03rW\x80`\x1F\x10a\x03GWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03rV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03UW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x02\x84\x01T`\x03\x85\x01T`\x04\x86\x01T`\x05\x87\x01\x80T\x96\x97c\xFF\xFF\xFF\xFF\x80\x86\x16\x98d\x01\0\0\0\0\x87\x04\x90\x91\x16\x97P`\x01`\x01`\xA0\x1B\x03`\x01`@\x1B\x90\x96\x04\x86\x16\x96P\x93\x90\x94\x16\x93\x91\x92a\x03\xC6\x90a\x0E\xDFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xF2\x90a\x0E\xDFV[\x80\x15a\x04?W\x80`\x1F\x10a\x04\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04?V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x06\x01T\x90\x80`\x07\x01T\x90P\x8AV[`\0\x82\x81R` \x81\x90R`@\x80\x82 \x80T`\xFF\x19\x16`\x03\x17\x81U`\x01\x81\x01T\x91Q\x90\x92\x82\x91\x86\x91\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x91\xA3PPPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x03\x81T`\xFF\x16`\x05\x81\x11\x15a\x04\xCBWa\x04\xCBa\x0C~V[\x14a\x05\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x02(V[\x80T`\xFF\x19\x16`\x04\x17\x81U`\x01\x81\x01T`@Q\x81\x90\x84\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3PPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81T`\xFF\x16`\x05\x81\x11\x15a\x05nWa\x05na\x0C~V[\x14\x80a\x05\x8FWP`\x02\x81T`\xFF\x16`\x05\x81\x11\x15a\x05\x8DWa\x05\x8Da\x0C~V[\x14[a\x05\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x02(V[\x80T`\xFF\x19\x16`\x05\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x06 \x92\x91\x90a\x0F\x19V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0a\x06I\x82\x80a\x0FHV[\x81\x01\x90a\x06V\x91\x90a\x0F\xBFV[\x90P`\0a\x06c\x82a\x07\xD4V[\x90Pa\x06\xB3a\x06u` \x85\x01\x85a\x0FHV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP\x85Q\x91P\x84\x90Pa\t\x98V[PPPV[`\0a\x06\xC4\x83\x80a\x0FHV[\x81\x01\x90a\x06\xD1\x91\x90a\x0F\xBFV[`\0\x83\x81R`\x01` \x81\x81R`@\x90\x92 \x83Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x91\x83\x01Q\x92\x93P\x83\x92\x90\x82\x01\x90a\x07\x15\x90\x82a\x11\x0EV[P`@\x82\x01Q`\x02\x82\x01\x80T``\x85\x01Q`\x80\x86\x01Qc\xFF\xFF\xFF\xFF\x94\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17d\x01\0\0\0\0\x94\x90\x91\x16\x93\x90\x93\x02\x92\x90\x92\x17h\x01\0\0\0\0\0\0\0\0`\x01`\xE0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x02\x17\x90U`\xA0\x83\x01Q`\x03\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U`\xC0\x82\x01Q`\x04\x82\x01U`\xE0\x82\x01Q`\x05\x82\x01\x90a\x07\xB6\x90\x82a\x11\x0EV[Pa\x01\0\x82\x01Q`\x06\x82\x01Ua\x01 \x90\x91\x01Q`\x07\x90\x91\x01UPPPV[`@\x80\x82\x01Q\x81Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x82\x01R\x7F\xD4\xE2\xD2a\xFF\xA0\xA31\xA9>\xD4\xAE\xE4\x94\x07o\xDC\x03\xDA\x7F\x1A\x99\xDFT\xA9d\x8Ca\xCA\x1D\x85\x94\x92\x81\x01\x92\x90\x92R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\0\x90\x81\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7Fg\x8B?\x8C5\x04\xD3\x11\x9F\xA3\xD8\xC5D\xB2f\xEE\xF2\x94\x96\xC6\xAFn\xE7a|\xC0\x01\xFE\xC2\x1B{T\x84`\0\x01Q\x85``\x01Q\x86`\xA0\x01Q\x87`\xC0\x01Q\x88`@\x01Q\x89`\xE0\x01Q\x80Q\x90` \x01 \x8A`\x80\x01Q\x8Ba\x01\0\x01Q\x8Ca\x01 \x01Q`@Q` \x01a\tE\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90\x99\x8AR`\x01`\x01`\xA0\x1B\x03\x98\x89\x16` \x8B\x01Rc\xFF\xFF\xFF\xFF\x97\x88\x16`@\x8B\x01R\x95\x88\x16``\x8A\x01R`\x80\x89\x01\x94\x90\x94R\x91\x90\x94\x16`\xA0\x87\x01R`\xC0\x86\x01\x93\x90\x93R\x91\x90\x92\x16`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x01 \x82\x01Ra\x01@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 a\x19\x01`\xF0\x1B\x82\x85\x01R`\"\x84\x01\x95\x90\x95R`B\x80\x84\x01\x95\x90\x95R\x81Q\x80\x84\x03\x90\x95\x01\x85R`b\x90\x92\x01\x90R\x82Q\x92\x01\x91\x90\x91 \x93\x92PPPV[`\0\x80`\0a\t\xA6\x86a\n\x82V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x89\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\n\x0CW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\nzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x02(V[PPPPPPV[`\0\x80`\0\x83Q`A\x14a\n\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x02(V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[`\0` \x82\x84\x03\x12\x15a\x0B\x16W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B-W`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x0B?W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0BXW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x0BzW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0BbV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0B\x9B\x81` \x86\x01` \x86\x01a\x0B_V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x82Ra\x01@` \x83\x01\x81\x90R`\0\x91a\x0B\xD5\x84\x83\x01\x8Ea\x0B\x83V[c\xFF\xFF\xFF\xFF\x8D\x81\x16`@\x87\x01R\x8C\x16``\x86\x01R\x8A\x82\x16`\x80\x86\x01R\x90\x89\x16`\xA0\x85\x01R`\xC0\x84\x01\x88\x90R\x83\x81\x03`\xE0\x85\x01R\x90Pa\x0C\x14\x81\x87a\x0B\x83V[a\x01\0\x84\x01\x95\x90\x95RPPa\x01 \x01R\x98\x97PPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0CCW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CaW`\0\x80\xFD[\x83\x01` \x81\x86\x03\x12\x15a\x0CsW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x06\x84\x10a\x0C\xB6WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C\xFAWa\x0C\xFAa\x0C\xC0V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\r\x11W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r,Wa\r,a\x0C\xC0V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\rTWa\rTa\x0C\xC0V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\rmW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`@\x826\x03\x12\x15a\r\x9FW`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\r\xC3Wa\r\xC3a\x0C\xC0V[\x81`@R\x845\x91P\x80\x82\x11\x15a\r\xD8W`\0\x80\xFD[a\r\xE46\x83\x87\x01a\r\0V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\r\xFAW`\0\x80\xFD[Pa\x0E\x076\x82\x86\x01a\r\0V[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0E*W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EJW`\0\x80\xFD[\x806\x03\x82\x13\x15a\x0EYW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\x0E\x99\x83\x84a\x0E\x13V[`@` \x85\x01Ra\x0E\xAE``\x85\x01\x82\x84a\x0E`V[\x91PPa\x0E\xBE` \x85\x01\x85a\x0E\x13V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x0E\xD5\x83\x82\x84a\x0E`V[\x96\x95PPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\xF3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0F\x13WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x83Qa\x0F+\x81\x84` \x88\x01a\x0B_V[\x83Q\x90\x83\x01\x90a\x0F?\x81\x83` \x88\x01a\x0B_V[\x01\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0F_W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0FzW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0EYW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\xA6W`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xA6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0F\xD1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xE9W`\0\x80\xFD[\x90\x83\x01\x90a\x01@\x82\x86\x03\x12\x15a\x0F\xFEW`\0\x80\xFD[a\x10\x06a\x0C\xD6V[a\x10\x0F\x83a\x0F\x8FV[\x81R` \x83\x015\x82\x81\x11\x15a\x10#W`\0\x80\xFD[a\x10/\x87\x82\x86\x01a\r\0V[` \x83\x01RPa\x10A`@\x84\x01a\x0F\xABV[`@\x82\x01Ra\x10R``\x84\x01a\x0F\xABV[``\x82\x01Ra\x10c`\x80\x84\x01a\x0F\x8FV[`\x80\x82\x01Ra\x10t`\xA0\x84\x01a\x0F\x8FV[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x10\x95W`\0\x80\xFD[a\x10\xA1\x87\x82\x86\x01a\r\0V[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\x1F\x82\x11\x15a\x06\xB3W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x10\xEFWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\nzW\x82\x81U`\x01\x01a\x10\xFBV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11(Wa\x11(a\x0C\xC0V[a\x11<\x81a\x116\x84Ta\x0E\xDFV[\x84a\x10\xC8V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x11qW`\0\x84\x15a\x11YWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\nzV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x11\xA0W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x11\x81V[P\x85\x82\x10\x15a\x11\xBEW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \x10:\xD5~\xA1\xB7~\x92\\\xAA\xAF\xF8}\xCA\x80Z\xCF\xC6'X\xCCo%\xA6\xAE\x16\x05\xB3\x8A!\xFE\xEBdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SWAPINTENTBOOK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x11a\0[W\x80c{\xF8\xBB\x88\x14a\0\xE6W\x80c\x90!W\x8A\x14a\0\xF9W\x80c\xD5_\x96\r\x14a\x014W\x80c\xE2V#\xE0\x14a\x01GW`\0\x80\xFD[\x80cJ\xF26N\x14a\0\x82W\x80cY\xA8D\xB4\x14a\0\xA8W\x80cr\xB8v\xEB\x14a\0\xD1W[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x0B\x04V[a\x01\xA6V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xBBa\0\xB66`\x04a\x0BFV[a\x02\xCCV[`@Qa\0\x9F\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0B\xAFV[a\0\xE4a\0\xDF6`\x04a\x0C0V[a\x04UV[\0[a\0\xE4a\0\xF46`\x04a\x0BFV[a\x04\xA5V[a\x01&a\x01\x076`\x04a\x0BFV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\x9F\x92\x91\x90a\x0C\x94V[a\0\xE4a\x01B6`\x04a\x0BFV[a\x05HV[a\x01\x81a\x01U6`\x04a\x0BFV[`\x02` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x91\x90\x92\x01T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x83V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\x9FV[`\0a\x01\xB9a\x01\xB4\x83a\r\x8DV[a\x06\x03V[\x90Pa\x01\xC4\x82a\x06=V[`\0\x81\x81R` \x81\x90R`@\x81 T`\xFF\x16`\x05\x81\x11\x15a\x01\xE7Wa\x01\xE7a\x0C~V[\x14a\x021W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R\x90\x81\x90R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x05\x81\x11\x15a\x02sWa\x02sa\x0C~V[\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01U\x90PPa\x02\x8F\x82\x82a\x06\xB8V[\x80\x7F\\/\xF1\xA21\x9AN\xC07\x07\x9E\xD0\xFA\xCBgnj\xDE\x19\xE5\xAC\xCBR\x86F;\xF34J\xAB\xD0G\x83`@Qa\x02\xBF\x91\x90a\x0E\x89V[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\x01` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T\x91\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92a\x02\xF9\x90a\x0E\xDFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03%\x90a\x0E\xDFV[\x80\x15a\x03rW\x80`\x1F\x10a\x03GWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03rV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03UW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x02\x84\x01T`\x03\x85\x01T`\x04\x86\x01T`\x05\x87\x01\x80T\x96\x97c\xFF\xFF\xFF\xFF\x80\x86\x16\x98d\x01\0\0\0\0\x87\x04\x90\x91\x16\x97P`\x01`\x01`\xA0\x1B\x03`\x01`@\x1B\x90\x96\x04\x86\x16\x96P\x93\x90\x94\x16\x93\x91\x92a\x03\xC6\x90a\x0E\xDFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xF2\x90a\x0E\xDFV[\x80\x15a\x04?W\x80`\x1F\x10a\x04\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04?V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x06\x01T\x90\x80`\x07\x01T\x90P\x8AV[`\0\x82\x81R` \x81\x90R`@\x80\x82 \x80T`\xFF\x19\x16`\x03\x17\x81U`\x01\x81\x01T\x91Q\x90\x92\x82\x91\x86\x91\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x91\xA3PPPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x03\x81T`\xFF\x16`\x05\x81\x11\x15a\x04\xCBWa\x04\xCBa\x0C~V[\x14a\x05\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x02(V[\x80T`\xFF\x19\x16`\x04\x17\x81U`\x01\x81\x01T`@Q\x81\x90\x84\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3PPPV[`\0\x81\x81R` \x81\x90R`@\x90 `\x01\x81T`\xFF\x16`\x05\x81\x11\x15a\x05nWa\x05na\x0C~V[\x14\x80a\x05\x8FWP`\x02\x81T`\xFF\x16`\x05\x81\x11\x15a\x05\x8DWa\x05\x8Da\x0C~V[\x14[a\x05\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjWrong state`\xA8\x1B`D\x82\x01R`d\x01a\x02(V[\x80T`\xFF\x19\x16`\x05\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x06 \x92\x91\x90a\x0F\x19V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0a\x06I\x82\x80a\x0FHV[\x81\x01\x90a\x06V\x91\x90a\x0F\xBFV[\x90P`\0a\x06c\x82a\x07\xD4V[\x90Pa\x06\xB3a\x06u` \x85\x01\x85a\x0FHV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP\x85Q\x91P\x84\x90Pa\t\x98V[PPPV[`\0a\x06\xC4\x83\x80a\x0FHV[\x81\x01\x90a\x06\xD1\x91\x90a\x0F\xBFV[`\0\x83\x81R`\x01` \x81\x81R`@\x90\x92 \x83Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x91\x83\x01Q\x92\x93P\x83\x92\x90\x82\x01\x90a\x07\x15\x90\x82a\x11\x0EV[P`@\x82\x01Q`\x02\x82\x01\x80T``\x85\x01Q`\x80\x86\x01Qc\xFF\xFF\xFF\xFF\x94\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17d\x01\0\0\0\0\x94\x90\x91\x16\x93\x90\x93\x02\x92\x90\x92\x17h\x01\0\0\0\0\0\0\0\0`\x01`\xE0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x02\x17\x90U`\xA0\x83\x01Q`\x03\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U`\xC0\x82\x01Q`\x04\x82\x01U`\xE0\x82\x01Q`\x05\x82\x01\x90a\x07\xB6\x90\x82a\x11\x0EV[Pa\x01\0\x82\x01Q`\x06\x82\x01Ua\x01 \x90\x91\x01Q`\x07\x90\x91\x01UPPPV[`@\x80\x82\x01Q\x81Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x82\x01R\x7F\xD4\xE2\xD2a\xFF\xA0\xA31\xA9>\xD4\xAE\xE4\x94\x07o\xDC\x03\xDA\x7F\x1A\x99\xDFT\xA9d\x8Ca\xCA\x1D\x85\x94\x92\x81\x01\x92\x90\x92R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\0\x90\x81\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7Fg\x8B?\x8C5\x04\xD3\x11\x9F\xA3\xD8\xC5D\xB2f\xEE\xF2\x94\x96\xC6\xAFn\xE7a|\xC0\x01\xFE\xC2\x1B{T\x84`\0\x01Q\x85``\x01Q\x86`\xA0\x01Q\x87`\xC0\x01Q\x88`@\x01Q\x89`\xE0\x01Q\x80Q\x90` \x01 \x8A`\x80\x01Q\x8Ba\x01\0\x01Q\x8Ca\x01 \x01Q`@Q` \x01a\tE\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90\x99\x8AR`\x01`\x01`\xA0\x1B\x03\x98\x89\x16` \x8B\x01Rc\xFF\xFF\xFF\xFF\x97\x88\x16`@\x8B\x01R\x95\x88\x16``\x8A\x01R`\x80\x89\x01\x94\x90\x94R\x91\x90\x94\x16`\xA0\x87\x01R`\xC0\x86\x01\x93\x90\x93R\x91\x90\x92\x16`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x01 \x82\x01Ra\x01@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 a\x19\x01`\xF0\x1B\x82\x85\x01R`\"\x84\x01\x95\x90\x95R`B\x80\x84\x01\x95\x90\x95R\x81Q\x80\x84\x03\x90\x95\x01\x85R`b\x90\x92\x01\x90R\x82Q\x92\x01\x91\x90\x91 \x93\x92PPPV[`\0\x80`\0a\t\xA6\x86a\n\x82V[`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x89\x90R`\xFF\x85\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R\x92\x95P\x90\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\n\x0CW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\nzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x02(V[PPPPPPV[`\0\x80`\0\x83Q`A\x14a\n\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x02(V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[`\0` \x82\x84\x03\x12\x15a\x0B\x16W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B-W`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x0B?W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0BXW`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\x0BzW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0BbV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0B\x9B\x81` \x86\x01` \x86\x01a\x0B_V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x82Ra\x01@` \x83\x01\x81\x90R`\0\x91a\x0B\xD5\x84\x83\x01\x8Ea\x0B\x83V[c\xFF\xFF\xFF\xFF\x8D\x81\x16`@\x87\x01R\x8C\x16``\x86\x01R\x8A\x82\x16`\x80\x86\x01R\x90\x89\x16`\xA0\x85\x01R`\xC0\x84\x01\x88\x90R\x83\x81\x03`\xE0\x85\x01R\x90Pa\x0C\x14\x81\x87a\x0B\x83V[a\x01\0\x84\x01\x95\x90\x95RPPa\x01 \x01R\x98\x97PPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0CCW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0CaW`\0\x80\xFD[\x83\x01` \x81\x86\x03\x12\x15a\x0CsW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x06\x84\x10a\x0C\xB6WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C\xFAWa\x0C\xFAa\x0C\xC0V[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\r\x11W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r,Wa\r,a\x0C\xC0V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\rTWa\rTa\x0C\xC0V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\rmW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0`@\x826\x03\x12\x15a\r\x9FW`\0\x80\xFD[`@Q`@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x82\x10\x81\x83\x11\x17\x15a\r\xC3Wa\r\xC3a\x0C\xC0V[\x81`@R\x845\x91P\x80\x82\x11\x15a\r\xD8W`\0\x80\xFD[a\r\xE46\x83\x87\x01a\r\0V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a\r\xFAW`\0\x80\xFD[Pa\x0E\x076\x82\x86\x01a\r\0V[` \x83\x01RP\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0E*W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EJW`\0\x80\xFD[\x806\x03\x82\x13\x15a\x0EYW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a\x0E\x99\x83\x84a\x0E\x13V[`@` \x85\x01Ra\x0E\xAE``\x85\x01\x82\x84a\x0E`V[\x91PPa\x0E\xBE` \x85\x01\x85a\x0E\x13V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x0E\xD5\x83\x82\x84a\x0E`V[\x96\x95PPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\xF3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0F\x13WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x83Qa\x0F+\x81\x84` \x88\x01a\x0B_V[\x83Q\x90\x83\x01\x90a\x0F?\x81\x83` \x88\x01a\x0B_V[\x01\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x0F_W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0FzW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x0EYW`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\xA6W`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xA6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0F\xD1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xE9W`\0\x80\xFD[\x90\x83\x01\x90a\x01@\x82\x86\x03\x12\x15a\x0F\xFEW`\0\x80\xFD[a\x10\x06a\x0C\xD6V[a\x10\x0F\x83a\x0F\x8FV[\x81R` \x83\x015\x82\x81\x11\x15a\x10#W`\0\x80\xFD[a\x10/\x87\x82\x86\x01a\r\0V[` \x83\x01RPa\x10A`@\x84\x01a\x0F\xABV[`@\x82\x01Ra\x10R``\x84\x01a\x0F\xABV[``\x82\x01Ra\x10c`\x80\x84\x01a\x0F\x8FV[`\x80\x82\x01Ra\x10t`\xA0\x84\x01a\x0F\x8FV[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x10\x95W`\0\x80\xFD[a\x10\xA1\x87\x82\x86\x01a\r\0V[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\x1F\x82\x11\x15a\x06\xB3W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x10\xEFWP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\nzW\x82\x81U`\x01\x01a\x10\xFBV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11(Wa\x11(a\x0C\xC0V[a\x11<\x81a\x116\x84Ta\x0E\xDFV[\x84a\x10\xC8V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x11qW`\0\x84\x15a\x11YWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\nzV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x11\xA0W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x11\x81V[P\x85\x82\x10\x15a\x11\xBEW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV\xFE\xA2dipfsX\"\x12 \x10:\xD5~\xA1\xB7~\x92\\\xAA\xAF\xF8}\xCA\x80Z\xCF\xC6'X\xCCo%\xA6\xAE\x16\x05\xB3\x8A!\xFE\xEBdsolcC\0\x08\x13\x003";
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
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
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
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Bytes,
                u32,
                u32,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::Bytes,
                ::ethers::core::types::U256,
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
    pub enum SwapIntentBookEvents {
        IntentBidReceivedFilter(IntentBidReceivedFilter),
        IntentCancelledFilter(IntentCancelledFilter),
        IntentCreatedFilter(IntentCreatedFilter),
        IntentMatchFilter(IntentMatchFilter),
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
    pub enum SwapIntentBookCalls {
        CancelIntent(CancelIntentCall),
        IntentBidData(IntentBidDataCall),
        IntentData(IntentDataCall),
        Intents(IntentsCall),
        MatchIntent(MatchIntentCall),
        PlaceIntent(PlaceIntentCall),
        SettleIntent(SettleIntentCall),
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
    impl ::core::fmt::Display for SwapIntentBookCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CancelIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentBidData(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentData(element) => ::core::fmt::Display::fmt(element, f),
                Self::Intents(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::PlaceIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettleIntent(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<IntentsCall> for SwapIntentBookCalls {
        fn from(value: IntentsCall) -> Self {
            Self::Intents(value)
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
        pub filler: ::ethers::core::types::Address,
        pub fill_timestamp: ::ethers::core::types::U256,
        pub fill_amount: ::ethers::core::types::U256,
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
