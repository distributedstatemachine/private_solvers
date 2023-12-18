pub use intent_book::*;
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
pub mod intent_book {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("bidIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bidIntent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intentBid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IntentbookLibrary.IntentBid",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
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
                                    name: ::std::borrow::ToOwned::to_owned("taker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    name: ::std::borrow::ToOwned::to_owned("bond"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IntentbookLibrary.IntentBidBond",
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
                                    name: ::std::borrow::ToOwned::to_owned("maker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reward"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IntentbookLibrary.IntentReward",
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
                                            "enum Intentbook.IntentStatus",
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
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IntentbookLibrary.Intent",
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
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
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
    pub static INTENTBOOK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0Ez\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\x90!W\x8A\x11a\0[W\x80c\x90!W\x8A\x14a\x017W\x80c\xBBaY\xDC\x14a\x01rW\x80c\xD5_\x96\r\x14a\x01\x85W\x80c\xE2V#\xE0\x14a\x01\x98W`\0\x80\xFD[\x80c\nN>z\x14a\0\x8DW\x80cY\xA8D\xB4\x14a\0\xB3W\x80c{\xF8\xBB\x88\x14a\x01\x0FW\x80c\x8A}p\x8F\x14a\x01$W[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\n\"V[a\x01\xFBV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x01a\0\xC16`\x04a\n:V[`\0` \x81\x81R\x91\x81R`@\x90\x81\x90 \x80T\x82Q\x80\x84\x01\x90\x93R`\x01\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x02\x90\x92\x01T\x93\x83\x01\x93\x90\x93R\x90\x91\x16\x90\x82V[`@Qa\0\xAA\x92\x91\x90a\nSV[a\x01\"a\x01\x1D6`\x04a\n:V[a\x03:V[\0[a\0\xA0a\x0126`\x04a\n\x83V[a\x03\xE8V[a\x01da\x01E6`\x04a\n:V[`\x02` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\xAA\x92\x91\x90a\n\xABV[a\x01\"a\x01\x806`\x04a\n:V[a\x05\"V[a\x01\"a\x01\x936`\x04a\n:V[a\x05\xA9V[a\x01\xECa\x01\xA66`\x04a\n:V[`\x01` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x80T\x92\x81\x01T\x84Q\x80\x86\x01\x90\x95R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R`\x03\x90\x92\x01T\x92\x85\x01\x92\x90\x92R\x90\x91\x16\x91\x83V[`@Qa\0\xAA\x93\x92\x91\x90a\n\xD7V[`\0a\x02\x14a\x02\x0F6\x84\x90\x03\x84\x01\x84a\x0B\x98V[a\x06FV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqBid already exists`p\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[` \x80\x83\x015`\0\x81\x81R`\x02\x90\x92R`@\x90\x91 `\x01\x81T`\xFF\x16`\x05\x81\x11\x15a\x02\xA3Wa\x02\xA3a\n\x95V[\x14a\x02\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02m\x90a\x0C\x0EV[\x80T`\xFF\x19\x16`\x02\x17\x81U`\x01\x80\x82\x01\x84\x90U`\0\x84\x81R` \x91\x90\x91R`@\x90 \x84\x90a\x02\xEE\x82\x82a\x0CwV[\x90PPa\x02\xFA\x84a\x06\x98V[\x82\x82\x7F+#Z\x06\x8B\xD9oK\xB8\x0C\x83qUT\xAE\xCD\xBF-\xD3\x1FDc:\xA1\xF7$5\xE1'\xCC\x13\xCE\x86`@Qa\x03+\x91\x90a\x0C\xC8V[`@Q\x80\x91\x03\x90\xA3PP\x91\x90PV[`\0\x81\x81R`\x02` R`@\x90 `\x03\x81T`\xFF\x16`\x05\x81\x11\x15a\x03`Wa\x03`a\n\x95V[\x14a\x03}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02m\x90a\x0C\x0EV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01\x85\x81\x01T\x85R\x90\x92R\x90\x91 a\x03\xA4\x82\x82a\x06\xC8V[\x82T`\xFF\x19\x16`\x04\x17\x83U`\x01\x83\x01T`@Q\x81\x90\x86\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3PPPPPV[`\0a\x04\x01a\x03\xFC6\x84\x90\x03\x84\x01\x84a\r\x02V[a\x07\x14V[\x90P`\0\x80\x82\x81R`\x02` R`@\x90 T`\xFF\x16`\x05\x81\x11\x15a\x04'Wa\x04'a\n\x95V[\x14a\x04lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x02mV[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R`\x02\x90\x91R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x05\x81\x11\x15a\x04\xAFWa\x04\xAFa\n\x95V[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x90\x91\x01U`\0\x82\x81R\x90\x81\x90R`@\x90 \x82\x90a\x04\xD9\x82\x82a\rBV[\x90PPa\x04\xE5\x82a\x07@V[\x80\x7Fi\xDB\xEDZ\xA8\xF5O\xB7^\x08\xB8[\xC1\xDB\xA8^\x14\xA1\xE3\x83J\x879'\x05\xC4\x19\xC5\xB8\x85\xFD\x10\x83`@Qa\x05\x15\x91\x90a\rhV[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\0\x81\x81R`\x02` \x81\x90R`@\x90\x91 \x90\x81T`\xFF\x16`\x05\x81\x11\x15a\x05JWa\x05Ja\n\x95V[\x14a\x05gW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02m\x90a\x0C\x0EV[\x80T`\xFF\x19\x16`\x03\x17\x81U`\x01\x81\x01T`@Q\x81\x90\x84\x90\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x90`\0\x90\xA3PPPV[`\0\x81\x81R`\x02` R`@\x90 `\x01\x81T`\xFF\x16`\x05\x81\x11\x15a\x05\xCFWa\x05\xCFa\n\x95V[\x14\x80a\x05\xF0WP`\x02\x81T`\xFF\x16`\x05\x81\x11\x15a\x05\xEEWa\x05\xEEa\n\x95V[\x14[a\x06\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02m\x90a\x0C\x0EV[\x80T`\xFF\x19\x16`\x05\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[` \x80\x82\x01Q\x82Q`@Q`\0\x93a\x06{\x93\x92\x91\x01\x91\x82R``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01R`4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x81\x01a\x06\xC4a\x06\xAC``\x84\x01\x83a\r\x92V[a\x06\xB9` \x85\x01\x85a\r\x92V[0\x84` \x015a\x07TV[PPV[`\x01\x82\x01T\x81T`\x02\x84\x01Ta\x06\xEE\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x920\x92\x91\x16\x90a\x07TV[`\x02\x81\x01T\x81T`\x03\x83\x01Ta\x06\xC4\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x920\x92\x91\x16\x90a\x07TV[\x80Q`@Q`\0\x91a\x06{\x91` \x01``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R`\x14\x01\x90V[` \x81\x01a\x06\xC4a\x06\xAC`@\x84\x01\x83a\r\x92V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x07\xAE\x90\x85\x90a\x07\xB4V[PPPPV[`\0a\x08\t\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x08\x8E\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x08*WP\x80\x80` \x01\x90Q\x81\x01\x90a\x08*\x91\x90a\r\xAFV[a\x08\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x02mV[PPPV[``a\x08\x9D\x84\x84`\0\x85a\x08\xA7V[\x90P[\x93\x92PPPV[``\x82G\x10\x15a\t\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x02mV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\t$\x91\x90a\r\xF5V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\taW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\tfV[``\x91P[P\x91P\x91Pa\tw\x87\x83\x83\x87a\t\x84V[\x92PPP[\x94\x93PPPPV[``\x83\x15a\t\xF3W\x82Q`\0\x03a\t\xECW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\t\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x02mV[P\x81a\t|V[a\t|\x83\x83\x81Q\x15a\n\x08W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02m\x91\x90a\x0E\x11V[`\0`\x80\x82\x84\x03\x12\x15a\n4W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\nLW`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x08\xA0` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`\0``\x82\x84\x03\x12\x15a\n4W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x06\x84\x10a\n\xCDWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R`\x80\x81\x01a\t|`@\x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B?WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BZW`\0\x80\xFD[PV[`\0`@\x82\x84\x03\x12\x15a\x0BoW`\0\x80\xFD[a\x0Bwa\x0B\x0EV[\x90P\x815a\x0B\x84\x81a\x0BEV[\x80\x82RP` \x82\x015` \x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x0B\xAAW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0B\xDBWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x825a\x0B\xE9\x81a\x0BEV[\x81R` \x83\x81\x015\x90\x82\x01Ra\x0C\x02\x84`@\x85\x01a\x0B]V[`@\x82\x01R\x93\x92PPPV[` \x80\x82R`\x0B\x90\x82\x01RjWrong state`\xA8\x1B`@\x82\x01R``\x01\x90V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815a\x0C^\x81a\x0BEV[a\x0Ch\x81\x83a\x0C3V[P` \x82\x015`\x01\x82\x01UPPV[\x815a\x0C\x82\x81a\x0BEV[a\x0C\x8C\x81\x83a\x0C3V[P` \x82\x015`\x01\x82\x01Ua\x06\xC4`@\x83\x01`\x02\x83\x01a\x0CSV[\x805a\x0C\xB2\x81a\x0BEV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x015\x91\x01RV[`\x80\x81\x01\x825a\x0C\xD7\x81a\x0BEV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x83\x81\x015\x90\x83\x01Ra\x0C\xFC`@\x80\x84\x01\x90\x85\x01a\x0C\xA7V[\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\r\x14W`\0\x80\xFD[a\r\x1Ca\x0B\x0EV[\x825a\r'\x81a\x0BEV[\x81Ra\r6\x84` \x85\x01a\x0B]V[` \x82\x01R\x93\x92PPPV[\x815a\rM\x81a\x0BEV[a\rW\x81\x83a\x0C3V[Pa\x06\xC4` \x83\x01`\x01\x83\x01a\x0CSV[``\x81\x01\x825a\rw\x81a\x0BEV[`\x01`\x01`\xA0\x1B\x03\x16\x82Ra\x0C\xFC` \x80\x84\x01\x90\x85\x01a\x0C\xA7V[`\0` \x82\x84\x03\x12\x15a\r\xA4W`\0\x80\xFD[\x815a\x08\xA0\x81a\x0BEV[`\0` \x82\x84\x03\x12\x15a\r\xC1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08\xA0W`\0\x80\xFD[`\0[\x83\x81\x10\x15a\r\xECW\x81\x81\x01Q\x83\x82\x01R` \x01a\r\xD4V[PP`\0\x91\x01RV[`\0\x82Qa\x0E\x07\x81\x84` \x87\x01a\r\xD1V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0E0\x81`@\x85\x01` \x87\x01a\r\xD1V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xD6\xA9\xEDU\xD5\xACW\x07dA\x8C\x17/\xE9\xC64\xDE\xC0\x02P\x0Be\xC9\x97s\x16lE\x8Cr\xF3MdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static INTENTBOOK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\x90!W\x8A\x11a\0[W\x80c\x90!W\x8A\x14a\x017W\x80c\xBBaY\xDC\x14a\x01rW\x80c\xD5_\x96\r\x14a\x01\x85W\x80c\xE2V#\xE0\x14a\x01\x98W`\0\x80\xFD[\x80c\nN>z\x14a\0\x8DW\x80cY\xA8D\xB4\x14a\0\xB3W\x80c{\xF8\xBB\x88\x14a\x01\x0FW\x80c\x8A}p\x8F\x14a\x01$W[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\n\"V[a\x01\xFBV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x01a\0\xC16`\x04a\n:V[`\0` \x81\x81R\x91\x81R`@\x90\x81\x90 \x80T\x82Q\x80\x84\x01\x90\x93R`\x01\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x02\x90\x92\x01T\x93\x83\x01\x93\x90\x93R\x90\x91\x16\x90\x82V[`@Qa\0\xAA\x92\x91\x90a\nSV[a\x01\"a\x01\x1D6`\x04a\n:V[a\x03:V[\0[a\0\xA0a\x0126`\x04a\n\x83V[a\x03\xE8V[a\x01da\x01E6`\x04a\n:V[`\x02` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\xAA\x92\x91\x90a\n\xABV[a\x01\"a\x01\x806`\x04a\n:V[a\x05\"V[a\x01\"a\x01\x936`\x04a\n:V[a\x05\xA9V[a\x01\xECa\x01\xA66`\x04a\n:V[`\x01` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x80T\x92\x81\x01T\x84Q\x80\x86\x01\x90\x95R`\x02\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86R`\x03\x90\x92\x01T\x92\x85\x01\x92\x90\x92R\x90\x91\x16\x91\x83V[`@Qa\0\xAA\x93\x92\x91\x90a\n\xD7V[`\0a\x02\x14a\x02\x0F6\x84\x90\x03\x84\x01\x84a\x0B\x98V[a\x06FV[`\0\x81\x81R`\x01` R`@\x90 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqBid already exists`p\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[` \x80\x83\x015`\0\x81\x81R`\x02\x90\x92R`@\x90\x91 `\x01\x81T`\xFF\x16`\x05\x81\x11\x15a\x02\xA3Wa\x02\xA3a\n\x95V[\x14a\x02\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02m\x90a\x0C\x0EV[\x80T`\xFF\x19\x16`\x02\x17\x81U`\x01\x80\x82\x01\x84\x90U`\0\x84\x81R` \x91\x90\x91R`@\x90 \x84\x90a\x02\xEE\x82\x82a\x0CwV[\x90PPa\x02\xFA\x84a\x06\x98V[\x82\x82\x7F+#Z\x06\x8B\xD9oK\xB8\x0C\x83qUT\xAE\xCD\xBF-\xD3\x1FDc:\xA1\xF7$5\xE1'\xCC\x13\xCE\x86`@Qa\x03+\x91\x90a\x0C\xC8V[`@Q\x80\x91\x03\x90\xA3PP\x91\x90PV[`\0\x81\x81R`\x02` R`@\x90 `\x03\x81T`\xFF\x16`\x05\x81\x11\x15a\x03`Wa\x03`a\n\x95V[\x14a\x03}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02m\x90a\x0C\x0EV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01\x85\x81\x01T\x85R\x90\x92R\x90\x91 a\x03\xA4\x82\x82a\x06\xC8V[\x82T`\xFF\x19\x16`\x04\x17\x83U`\x01\x83\x01T`@Q\x81\x90\x86\x90\x7F\xBF\x89u\x13\x9A\xEE\x07\x94\xECPWC<4\xFB\x93\x9E\x0FeZ\x87\xB0Q\xE3*:\xAE$\xA6U/N\x90`\0\x90\xA3PPPPPV[`\0a\x04\x01a\x03\xFC6\x84\x90\x03\x84\x01\x84a\r\x02V[a\x07\x14V[\x90P`\0\x80\x82\x81R`\x02` R`@\x90 T`\xFF\x16`\x05\x81\x11\x15a\x04'Wa\x04'a\n\x95V[\x14a\x04lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x02mV[`@\x80Q\x80\x82\x01\x90\x91R\x80`\x01\x81R`\0` \x91\x82\x01\x81\x90R\x83\x81R`\x02\x90\x91R`@\x90 \x81Q\x81T\x82\x90`\xFF\x19\x16`\x01\x83`\x05\x81\x11\x15a\x04\xAFWa\x04\xAFa\n\x95V[\x02\x17\x90UP` \x91\x82\x01Q`\x01\x90\x91\x01U`\0\x82\x81R\x90\x81\x90R`@\x90 \x82\x90a\x04\xD9\x82\x82a\rBV[\x90PPa\x04\xE5\x82a\x07@V[\x80\x7Fi\xDB\xEDZ\xA8\xF5O\xB7^\x08\xB8[\xC1\xDB\xA8^\x14\xA1\xE3\x83J\x879'\x05\xC4\x19\xC5\xB8\x85\xFD\x10\x83`@Qa\x05\x15\x91\x90a\rhV[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\0\x81\x81R`\x02` \x81\x90R`@\x90\x91 \x90\x81T`\xFF\x16`\x05\x81\x11\x15a\x05JWa\x05Ja\n\x95V[\x14a\x05gW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02m\x90a\x0C\x0EV[\x80T`\xFF\x19\x16`\x03\x17\x81U`\x01\x81\x01T`@Q\x81\x90\x84\x90\x7F\xD4\\.\x8Ek:\xE4\x1C\xA1P\xB9R\xF5\xD3b\x92\x10\x8E\xC6m\x95\x98-F\x03}\x18\\\xD9\x05\xA1\xAE\x90`\0\x90\xA3PPPV[`\0\x81\x81R`\x02` R`@\x90 `\x01\x81T`\xFF\x16`\x05\x81\x11\x15a\x05\xCFWa\x05\xCFa\n\x95V[\x14\x80a\x05\xF0WP`\x02\x81T`\xFF\x16`\x05\x81\x11\x15a\x05\xEEWa\x05\xEEa\n\x95V[\x14[a\x06\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02m\x90a\x0C\x0EV[\x80T`\xFF\x19\x16`\x05\x17\x81U`@Q\x82\x90\x7F\xC0\x8E\xB6M\xB1j9\xD2\x84\x89`\xAF\x04\xE3\xF1o\xB4\x04\xD9\xD46\xA9\xF0\xE9\xD7\xD0\xD4\x85G\x15\xC9\xDC\x90`\0\x90\xA2PPV[` \x80\x82\x01Q\x82Q`@Q`\0\x93a\x06{\x93\x92\x91\x01\x91\x82R``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16` \x82\x01R`4\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x81\x01a\x06\xC4a\x06\xAC``\x84\x01\x83a\r\x92V[a\x06\xB9` \x85\x01\x85a\r\x92V[0\x84` \x015a\x07TV[PPV[`\x01\x82\x01T\x81T`\x02\x84\x01Ta\x06\xEE\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x920\x92\x91\x16\x90a\x07TV[`\x02\x81\x01T\x81T`\x03\x83\x01Ta\x06\xC4\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x920\x92\x91\x16\x90a\x07TV[\x80Q`@Q`\0\x91a\x06{\x91` \x01``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R`\x14\x01\x90V[` \x81\x01a\x06\xC4a\x06\xAC`@\x84\x01\x83a\r\x92V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x07\xAE\x90\x85\x90a\x07\xB4V[PPPPV[`\0a\x08\t\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x08\x8E\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x08*WP\x80\x80` \x01\x90Q\x81\x01\x90a\x08*\x91\x90a\r\xAFV[a\x08\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x02mV[PPPV[``a\x08\x9D\x84\x84`\0\x85a\x08\xA7V[\x90P[\x93\x92PPPV[``\x82G\x10\x15a\t\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x02mV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\t$\x91\x90a\r\xF5V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\taW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\tfV[``\x91P[P\x91P\x91Pa\tw\x87\x83\x83\x87a\t\x84V[\x92PPP[\x94\x93PPPPV[``\x83\x15a\t\xF3W\x82Q`\0\x03a\t\xECW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\t\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x02mV[P\x81a\t|V[a\t|\x83\x83\x81Q\x15a\n\x08W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02m\x91\x90a\x0E\x11V[`\0`\x80\x82\x84\x03\x12\x15a\n4W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\nLW`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x08\xA0` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`\0``\x82\x84\x03\x12\x15a\n4W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x81\x01`\x06\x84\x10a\n\xCDWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R`\x80\x81\x01a\t|`@\x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B?WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BZW`\0\x80\xFD[PV[`\0`@\x82\x84\x03\x12\x15a\x0BoW`\0\x80\xFD[a\x0Bwa\x0B\x0EV[\x90P\x815a\x0B\x84\x81a\x0BEV[\x80\x82RP` \x82\x015` \x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x0B\xAAW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0B\xDBWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x825a\x0B\xE9\x81a\x0BEV[\x81R` \x83\x81\x015\x90\x82\x01Ra\x0C\x02\x84`@\x85\x01a\x0B]V[`@\x82\x01R\x93\x92PPPV[` \x80\x82R`\x0B\x90\x82\x01RjWrong state`\xA8\x1B`@\x82\x01R``\x01\x90V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815a\x0C^\x81a\x0BEV[a\x0Ch\x81\x83a\x0C3V[P` \x82\x015`\x01\x82\x01UPPV[\x815a\x0C\x82\x81a\x0BEV[a\x0C\x8C\x81\x83a\x0C3V[P` \x82\x015`\x01\x82\x01Ua\x06\xC4`@\x83\x01`\x02\x83\x01a\x0CSV[\x805a\x0C\xB2\x81a\x0BEV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x015\x91\x01RV[`\x80\x81\x01\x825a\x0C\xD7\x81a\x0BEV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x83\x81\x015\x90\x83\x01Ra\x0C\xFC`@\x80\x84\x01\x90\x85\x01a\x0C\xA7V[\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a\r\x14W`\0\x80\xFD[a\r\x1Ca\x0B\x0EV[\x825a\r'\x81a\x0BEV[\x81Ra\r6\x84` \x85\x01a\x0B]V[` \x82\x01R\x93\x92PPPV[\x815a\rM\x81a\x0BEV[a\rW\x81\x83a\x0C3V[Pa\x06\xC4` \x83\x01`\x01\x83\x01a\x0CSV[``\x81\x01\x825a\rw\x81a\x0BEV[`\x01`\x01`\xA0\x1B\x03\x16\x82Ra\x0C\xFC` \x80\x84\x01\x90\x85\x01a\x0C\xA7V[`\0` \x82\x84\x03\x12\x15a\r\xA4W`\0\x80\xFD[\x815a\x08\xA0\x81a\x0BEV[`\0` \x82\x84\x03\x12\x15a\r\xC1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08\xA0W`\0\x80\xFD[`\0[\x83\x81\x10\x15a\r\xECW\x81\x81\x01Q\x83\x82\x01R` \x01a\r\xD4V[PP`\0\x91\x01RV[`\0\x82Qa\x0E\x07\x81\x84` \x87\x01a\r\xD1V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0E0\x81`@\x85\x01` \x87\x01a\r\xD1V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xD6\xA9\xEDU\xD5\xACW\x07dA\x8C\x17/\xE9\xC64\xDE\xC0\x02P\x0Be\xC9\x97s\x16lE\x8Cr\xF3MdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static INTENTBOOK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct IntentBook<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IntentBook<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IntentBook<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IntentBook<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IntentBook<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IntentBook)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IntentBook<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INTENTBOOK_ABI.clone(),
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
                INTENTBOOK_ABI.clone(),
                INTENTBOOK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `bidIntent` (0x0a4e3e7a) function
        pub fn bid_intent(
            &self,
            intent_bid: IntentBid,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([10, 78, 62, 122], (intent_bid,))
                .expect("method not found (this should never happen)")
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
            (::ethers::core::types::Address, [u8; 32], IntentBidBond),
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
            (::ethers::core::types::Address, IntentReward),
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
        ///Calls the contract's `matchIntent` (0xbb6159dc) function
        pub fn match_intent(
            &self,
            intent_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 97, 89, 220], intent_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `placeIntent` (0x8a7d708f) function
        pub fn place_intent(
            &self,
            intent: Intent,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([138, 125, 112, 143], (intent,))
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
            IntentBookEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IntentBook<M> {
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
        abi = "IntentBidReceived(bytes32,bytes32,(address,bytes32,(address,uint256)))"
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
    #[ethevent(
        name = "IntentCreated",
        abi = "IntentCreated(bytes32,(address,(address,uint256)))"
    )]
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
    pub enum IntentBookEvents {
        IntentBidReceivedFilter(IntentBidReceivedFilter),
        IntentCancelledFilter(IntentCancelledFilter),
        IntentCreatedFilter(IntentCreatedFilter),
        IntentMatchFilter(IntentMatchFilter),
        IntentSettledFilter(IntentSettledFilter),
    }
    impl ::ethers::contract::EthLogDecode for IntentBookEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = IntentBidReceivedFilter::decode_log(log) {
                return Ok(IntentBookEvents::IntentBidReceivedFilter(decoded));
            }
            if let Ok(decoded) = IntentCancelledFilter::decode_log(log) {
                return Ok(IntentBookEvents::IntentCancelledFilter(decoded));
            }
            if let Ok(decoded) = IntentCreatedFilter::decode_log(log) {
                return Ok(IntentBookEvents::IntentCreatedFilter(decoded));
            }
            if let Ok(decoded) = IntentMatchFilter::decode_log(log) {
                return Ok(IntentBookEvents::IntentMatchFilter(decoded));
            }
            if let Ok(decoded) = IntentSettledFilter::decode_log(log) {
                return Ok(IntentBookEvents::IntentSettledFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IntentBookEvents {
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
    impl ::core::convert::From<IntentBidReceivedFilter> for IntentBookEvents {
        fn from(value: IntentBidReceivedFilter) -> Self {
            Self::IntentBidReceivedFilter(value)
        }
    }
    impl ::core::convert::From<IntentCancelledFilter> for IntentBookEvents {
        fn from(value: IntentCancelledFilter) -> Self {
            Self::IntentCancelledFilter(value)
        }
    }
    impl ::core::convert::From<IntentCreatedFilter> for IntentBookEvents {
        fn from(value: IntentCreatedFilter) -> Self {
            Self::IntentCreatedFilter(value)
        }
    }
    impl ::core::convert::From<IntentMatchFilter> for IntentBookEvents {
        fn from(value: IntentMatchFilter) -> Self {
            Self::IntentMatchFilter(value)
        }
    }
    impl ::core::convert::From<IntentSettledFilter> for IntentBookEvents {
        fn from(value: IntentSettledFilter) -> Self {
            Self::IntentSettledFilter(value)
        }
    }
    ///Container type for all input parameters for the `bidIntent` function with signature `bidIntent((address,bytes32,(address,uint256)))` and selector `0x0a4e3e7a`
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
        name = "bidIntent",
        abi = "bidIntent((address,bytes32,(address,uint256)))"
    )]
    pub struct BidIntentCall {
        pub intent_bid: IntentBid,
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
    ///Container type for all input parameters for the `matchIntent` function with signature `matchIntent(bytes32)` and selector `0xbb6159dc`
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
    #[ethcall(name = "matchIntent", abi = "matchIntent(bytes32)")]
    pub struct MatchIntentCall {
        pub intent_id: [u8; 32],
    }
    ///Container type for all input parameters for the `placeIntent` function with signature `placeIntent((address,(address,uint256)))` and selector `0x8a7d708f`
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
    #[ethcall(name = "placeIntent", abi = "placeIntent((address,(address,uint256)))")]
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
    pub enum IntentBookCalls {
        BidIntent(BidIntentCall),
        CancelIntent(CancelIntentCall),
        IntentBidData(IntentBidDataCall),
        IntentData(IntentDataCall),
        Intents(IntentsCall),
        MatchIntent(MatchIntentCall),
        PlaceIntent(PlaceIntentCall),
        SettleIntent(SettleIntentCall),
    }
    impl ::ethers::core::abi::AbiDecode for IntentBookCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BidIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BidIntent(decoded));
            }
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
    impl ::ethers::core::abi::AbiEncode for IntentBookCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BidIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
    impl ::core::fmt::Display for IntentBookCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BidIntent(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<BidIntentCall> for IntentBookCalls {
        fn from(value: BidIntentCall) -> Self {
            Self::BidIntent(value)
        }
    }
    impl ::core::convert::From<CancelIntentCall> for IntentBookCalls {
        fn from(value: CancelIntentCall) -> Self {
            Self::CancelIntent(value)
        }
    }
    impl ::core::convert::From<IntentBidDataCall> for IntentBookCalls {
        fn from(value: IntentBidDataCall) -> Self {
            Self::IntentBidData(value)
        }
    }
    impl ::core::convert::From<IntentDataCall> for IntentBookCalls {
        fn from(value: IntentDataCall) -> Self {
            Self::IntentData(value)
        }
    }
    impl ::core::convert::From<IntentsCall> for IntentBookCalls {
        fn from(value: IntentsCall) -> Self {
            Self::Intents(value)
        }
    }
    impl ::core::convert::From<MatchIntentCall> for IntentBookCalls {
        fn from(value: MatchIntentCall) -> Self {
            Self::MatchIntent(value)
        }
    }
    impl ::core::convert::From<PlaceIntentCall> for IntentBookCalls {
        fn from(value: PlaceIntentCall) -> Self {
            Self::PlaceIntent(value)
        }
    }
    impl ::core::convert::From<SettleIntentCall> for IntentBookCalls {
        fn from(value: SettleIntentCall) -> Self {
            Self::SettleIntent(value)
        }
    }
    ///Container type for all return fields from the `bidIntent` function with signature `bidIntent((address,bytes32,(address,uint256)))` and selector `0x0a4e3e7a`
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
    pub struct BidIntentReturn {
        pub intent_bid_id: [u8; 32],
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
        pub taker: ::ethers::core::types::Address,
        pub intent_id: [u8; 32],
        pub bond: IntentBidBond,
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
        pub maker: ::ethers::core::types::Address,
        pub reward: IntentReward,
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
    ///Container type for all return fields from the `placeIntent` function with signature `placeIntent((address,(address,uint256)))` and selector `0x8a7d708f`
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
    ///`Intent(address,(address,uint256))`
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
    pub struct Intent {
        pub maker: ::ethers::core::types::Address,
        pub reward: IntentReward,
    }
    ///`IntentBid(address,bytes32,(address,uint256))`
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
    pub struct IntentBid {
        pub taker: ::ethers::core::types::Address,
        pub intent_id: [u8; 32],
        pub bond: IntentBidBond,
    }
    ///`IntentBidBond(address,uint256)`
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
    pub struct IntentBidBond {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///`IntentReward(address,uint256)`
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
    pub struct IntentReward {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
}
