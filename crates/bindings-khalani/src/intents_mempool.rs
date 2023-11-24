pub use intents_mempool::*;
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
pub mod intents_mempool {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getIntent"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                            "struct SwapIntentsLibrary.SwapIntent",
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
                    ::std::borrow::ToOwned::to_owned("isSettled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isSettled"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("storeIntent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("storeIntent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newIntent"),
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
                                            "struct SwapIntentsLibrary.SwapIntent",
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
            ]),
            events: ::core::convert::From::from([
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
                                    indexed: false,
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
    pub static INTENTSMEMPOOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x10\xD1\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c#\xD8u\x01\x14a\0\\W\x80c{\xF8\xBB\x88\x14a\0qW\x80c\x90!W\x8A\x14a\0\x84W\x80c\xBD\x07\xF3\xC9\x14a\0\xB6W\x80c\xF1<F\xAA\x14a\0\xE9W[`\0\x80\xFD[a\0oa\0j6`\x04a\x0C\x05V[a\x01\tV[\0[a\0oa\0\x7F6`\x04a\r\x0EV[a\x02\xD0V[a\0\x97a\0\x926`\x04a\r\x0EV[a\x03\xA2V[`@Qa\0\xAD\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\rwV[`@Q\x80\x91\x03\x90\xF3[a\0\xD9a\0\xC46`\x04a\r\x0EV[`\x01` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xADV[a\0\xFCa\0\xF76`\x04a\r\x0EV[a\x05*V[`@Qa\0\xAD\x91\x90a\r\xF8V[`\0a\x01\x14\x82a\x07aV[\x90Pa\x01\x1F\x82a\x07\xC7V[`\0\x81\x81R` \x81\x90R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x01\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R` \x81\x81R`@\x90\x91 \x83Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x83\x01Q\x83\x91\x90`\x01\x82\x01\x90a\x01\xC2\x90\x82a\x0FUV[P`@\x82\x01Q`\x02\x82\x01\x80T``\x85\x01Q`\x80\x86\x01Qc\xFF\xFF\xFF\xFF\x94\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17d\x01\0\0\0\0\x94\x90\x91\x16\x93\x90\x93\x02\x92\x90\x92\x17h\x01\0\0\0\0\0\0\0\0`\x01`\xE0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x02\x17\x90U`\xA0\x83\x01Q`\x03\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U`\xC0\x82\x01Q`\x04\x82\x01U`\xE0\x82\x01Q`\x05\x82\x01\x90a\x02c\x90\x82a\x0FUV[Pa\x01\0\x82\x01Q`\x06\x82\x01Ua\x01 \x90\x91\x01Q`\x07\x90\x91\x01U`\0\x81\x81R`\x01` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x81\x90\x7F\xB4\xA3\x10\xEE\n\xE0\xF8\x99\x99#\x05\xB2\x9F0E\x91\xFB@\xD3\x85\xA7\xA2\x8B\xCF`t\x87\x08i\xF27l\x90a\x02\xC4\x90\x85\x90a\r\xF8V[`@Q\x80\x91\x03\x90\xA2PPV[`\0\x81\x81R` \x81\x90R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x03,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01a\x01xV[`\0\x81\x81R`\x01` R`@\x90 T`\xFF\x16\x15a\x03\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x12[\x9D\x19[\x9D\x08\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`R\x1B`D\x82\x01R`d\x01a\x01xV[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91a\x03\xCE\x90a\x0E\xCCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xFA\x90a\x0E\xCCV[\x80\x15a\x04GW\x80`\x1F\x10a\x04\x1CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04GV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04*W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x02\x84\x01T`\x03\x85\x01T`\x04\x86\x01T`\x05\x87\x01\x80T\x96\x97c\xFF\xFF\xFF\xFF\x80\x86\x16\x98d\x01\0\0\0\0\x87\x04\x90\x91\x16\x97P`\x01`\x01`\xA0\x1B\x03`\x01`@\x1B\x90\x96\x04\x86\x16\x96P\x93\x90\x94\x16\x93\x91\x92a\x04\x9B\x90a\x0E\xCCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xC7\x90a\x0E\xCCV[\x80\x15a\x05\x14W\x80`\x1F\x10a\x04\xE9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x14V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xF7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x06\x01T\x90\x80`\x07\x01T\x90P\x8AV[a\x05\xA7`@Q\x80a\x01@\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x82\x81R` \x81\x81R`@\x91\x82\x90 \x82Qa\x01@\x81\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x91\x92\x84\x01\x91a\x05\xE2\x90a\x0E\xCCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x0E\x90a\x0E\xCCV[\x80\x15a\x06[W\x80`\x1F\x10a\x060Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06[V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06>W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x82\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16` \x84\x01Rd\x01\0\0\0\0\x82\x04\x16`@\x83\x01R`\x01`\x01`\xA0\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16``\x83\x01R`\x03\x83\x01T\x16`\x80\x82\x01R`\x04\x82\x01T`\xA0\x82\x01R`\x05\x82\x01\x80T`\xC0\x90\x92\x01\x91a\x06\xC4\x90a\x0E\xCCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xF0\x90a\x0E\xCCV[\x80\x15a\x07=W\x80`\x1F\x10a\x07\x12Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07=V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07 W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81RPP\x90P\x91\x90PV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x07\xAA\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x10\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a\x07\xD9\x84` \x01Qa\x08\xC2V[\x91\x94P\x92P\x90P`\0a\x07\xEB\x85a\tDV[\x85Q`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x84\x90R`\xFF\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x08MW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x01xV[PPPPPV[`\0\x80`\0\x83Q`A\x14a\t(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x01xV[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[`@\x80\x82\x01Q\x81Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x82\x01R\x7F\xD4\xE2\xD2a\xFF\xA0\xA31\xA9>\xD4\xAE\xE4\x94\x07o\xDC\x03\xDA\x7F\x1A\x99\xDFT\xA9d\x8Ca\xCA\x1D\x85\x94\x92\x81\x01\x92\x90\x92R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\0\x90\x81\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7F\x9F4\xD6E\xC3}:\xED\xB5\x03Rz\x88\xEB=\xAF\x0B\xAD\xF2*\xDE\x9B\x80C\x07\xCD\xD0\xBF2\x8A\x9B\x13\x84`\0\x01Q\x85``\x01Q\x86`\xA0\x01Q\x87`\xC0\x01Q\x88`@\x01Q\x89`\xE0\x01Q\x80Q\x90` \x01 \x8A`\x80\x01Q\x8Ba\x01\0\x01Q\x8Ca\x01 \x01Q`@Q` \x01a\n\xB5\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90\x99\x8AR`\x01`\x01`\xA0\x1B\x03\x98\x89\x16` \x8B\x01Rc\xFF\xFF\xFF\xFF\x97\x88\x16`@\x8B\x01R\x95\x88\x16``\x8A\x01R`\x80\x89\x01\x94\x90\x94R\x91\x90\x94\x16`\xA0\x87\x01R`\xC0\x86\x01\x93\x90\x93R\x91\x90\x92\x16`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x01 \x82\x01Ra\x01@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 a\x19\x01`\xF0\x1B\x82\x85\x01R`\"\x84\x01\x95\x90\x95R`B\x80\x84\x01\x95\x90\x95R\x81Q\x80\x84\x03\x90\x95\x01\x85R`b\x90\x92\x01\x90R\x82Q\x92\x01\x91\x90\x91 \x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0BBWa\x0BBa\x0B\x08V[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B_W`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x0BuW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x90Wa\x0B\x90a\x0B\x08V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0B\xB8Wa\x0B\xB8a\x0B\x08V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0B\xD1W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B_W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\x17W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C/W`\0\x80\xFD[\x90\x83\x01\x90a\x01@\x82\x86\x03\x12\x15a\x0CDW`\0\x80\xFD[a\x0CLa\x0B\x1EV[a\x0CU\x83a\x0BHV[\x81R` \x83\x015\x82\x81\x11\x15a\x0CiW`\0\x80\xFD[a\x0Cu\x87\x82\x86\x01a\x0BdV[` \x83\x01RPa\x0C\x87`@\x84\x01a\x0B\xF1V[`@\x82\x01Ra\x0C\x98``\x84\x01a\x0B\xF1V[``\x82\x01Ra\x0C\xA9`\x80\x84\x01a\x0BHV[`\x80\x82\x01Ra\x0C\xBA`\xA0\x84\x01a\x0BHV[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x0C\xDBW`\0\x80\xFD[a\x0C\xE7\x87\x82\x86\x01a\x0BdV[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\r W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\rBW\x81\x81\x01Q\x83\x82\x01R` \x01a\r*V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\rc\x81` \x86\x01` \x86\x01a\r'V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x82Ra\x01@` \x83\x01\x81\x90R`\0\x91a\r\x9D\x84\x83\x01\x8Ea\rKV[c\xFF\xFF\xFF\xFF\x8D\x81\x16`@\x87\x01R\x8C\x16``\x86\x01R\x8A\x82\x16`\x80\x86\x01R\x90\x89\x16`\xA0\x85\x01R`\xC0\x84\x01\x88\x90R\x83\x81\x03`\xE0\x85\x01R\x90Pa\r\xDC\x81\x87a\rKV[a\x01\0\x84\x01\x95\x90\x95RPPa\x01 \x01R\x98\x97PPPPPPPPV[` \x81Ra\x0E\x12` \x82\x01\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\0` \x83\x01Qa\x01@\x80`@\x85\x01Ra\x0E0a\x01`\x85\x01\x83a\rKV[\x91P`@\x85\x01Qa\x0EI``\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P``\x85\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x86\x01RP`\x80\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xA0\x86\x01RP`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xC0\x86\x01RP`\xC0\x85\x01Q`\xE0\x85\x01R`\xE0\x85\x01Qa\x01\0`\x1F\x19\x86\x85\x03\x01\x81\x87\x01Ra\x0E\xAD\x84\x83a\rKV[\x90\x87\x01Qa\x01 \x87\x81\x01\x91\x90\x91R\x90\x96\x01Q\x91\x90\x94\x01RP\x91\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\xE0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0F\0WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x0FPW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0F-WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0FLW\x82\x81U`\x01\x01a\x0F9V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FoWa\x0Foa\x0B\x08V[a\x0F\x83\x81a\x0F}\x84Ta\x0E\xCCV[\x84a\x0F\x06V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0F\xB8W`\0\x84\x15a\x0F\xA0WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0FLV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x0F\xE7W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0F\xC8V[P\x85\x82\x10\x15a\x10\x05W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Qa\x10|\x81`d\x85\x01` \x89\x01a\r'V[\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \x9C\x11\xF2\x92\xAF\x07\x91j\x15\xA5\x15\xDCw\xD4o,\xE9\xC4\x08A\x9F\x84%\x92n<10\xB0\x87\xA0\xC2dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static INTENTSMEMPOOL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c#\xD8u\x01\x14a\0\\W\x80c{\xF8\xBB\x88\x14a\0qW\x80c\x90!W\x8A\x14a\0\x84W\x80c\xBD\x07\xF3\xC9\x14a\0\xB6W\x80c\xF1<F\xAA\x14a\0\xE9W[`\0\x80\xFD[a\0oa\0j6`\x04a\x0C\x05V[a\x01\tV[\0[a\0oa\0\x7F6`\x04a\r\x0EV[a\x02\xD0V[a\0\x97a\0\x926`\x04a\r\x0EV[a\x03\xA2V[`@Qa\0\xAD\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\rwV[`@Q\x80\x91\x03\x90\xF3[a\0\xD9a\0\xC46`\x04a\r\x0EV[`\x01` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xADV[a\0\xFCa\0\xF76`\x04a\r\x0EV[a\x05*V[`@Qa\0\xAD\x91\x90a\r\xF8V[`\0a\x01\x14\x82a\x07aV[\x90Pa\x01\x1F\x82a\x07\xC7V[`\0\x81\x81R` \x81\x90R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x01\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R` \x81\x81R`@\x90\x91 \x83Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x83\x01Q\x83\x91\x90`\x01\x82\x01\x90a\x01\xC2\x90\x82a\x0FUV[P`@\x82\x01Q`\x02\x82\x01\x80T``\x85\x01Q`\x80\x86\x01Qc\xFF\xFF\xFF\xFF\x94\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17d\x01\0\0\0\0\x94\x90\x91\x16\x93\x90\x93\x02\x92\x90\x92\x17h\x01\0\0\0\0\0\0\0\0`\x01`\xE0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x02\x17\x90U`\xA0\x83\x01Q`\x03\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U`\xC0\x82\x01Q`\x04\x82\x01U`\xE0\x82\x01Q`\x05\x82\x01\x90a\x02c\x90\x82a\x0FUV[Pa\x01\0\x82\x01Q`\x06\x82\x01Ua\x01 \x90\x91\x01Q`\x07\x90\x91\x01U`\0\x81\x81R`\x01` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x81\x90\x7F\xB4\xA3\x10\xEE\n\xE0\xF8\x99\x99#\x05\xB2\x9F0E\x91\xFB@\xD3\x85\xA7\xA2\x8B\xCF`t\x87\x08i\xF27l\x90a\x02\xC4\x90\x85\x90a\r\xF8V[`@Q\x80\x91\x03\x90\xA2PPV[`\0\x81\x81R` \x81\x90R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x03,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01a\x01xV[`\0\x81\x81R`\x01` R`@\x90 T`\xFF\x16\x15a\x03\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x12[\x9D\x19[\x9D\x08\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`R\x1B`D\x82\x01R`d\x01a\x01xV[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91a\x03\xCE\x90a\x0E\xCCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xFA\x90a\x0E\xCCV[\x80\x15a\x04GW\x80`\x1F\x10a\x04\x1CWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04GV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04*W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x02\x84\x01T`\x03\x85\x01T`\x04\x86\x01T`\x05\x87\x01\x80T\x96\x97c\xFF\xFF\xFF\xFF\x80\x86\x16\x98d\x01\0\0\0\0\x87\x04\x90\x91\x16\x97P`\x01`\x01`\xA0\x1B\x03`\x01`@\x1B\x90\x96\x04\x86\x16\x96P\x93\x90\x94\x16\x93\x91\x92a\x04\x9B\x90a\x0E\xCCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xC7\x90a\x0E\xCCV[\x80\x15a\x05\x14W\x80`\x1F\x10a\x04\xE9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x14V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\xF7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x06\x01T\x90\x80`\x07\x01T\x90P\x8AV[a\x05\xA7`@Q\x80a\x01@\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x82\x81R` \x81\x81R`@\x91\x82\x90 \x82Qa\x01@\x81\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x91\x92\x84\x01\x91a\x05\xE2\x90a\x0E\xCCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x0E\x90a\x0E\xCCV[\x80\x15a\x06[W\x80`\x1F\x10a\x060Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06[V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06>W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x82\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16` \x84\x01Rd\x01\0\0\0\0\x82\x04\x16`@\x83\x01R`\x01`\x01`\xA0\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16``\x83\x01R`\x03\x83\x01T\x16`\x80\x82\x01R`\x04\x82\x01T`\xA0\x82\x01R`\x05\x82\x01\x80T`\xC0\x90\x92\x01\x91a\x06\xC4\x90a\x0E\xCCV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xF0\x90a\x0E\xCCV[\x80\x15a\x07=W\x80`\x1F\x10a\x07\x12Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07=V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07 W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x06\x82\x01T\x81R` \x01`\x07\x82\x01T\x81RPP\x90P\x91\x90PV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x07\xAA\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x10\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a\x07\xD9\x84` \x01Qa\x08\xC2V[\x91\x94P\x92P\x90P`\0a\x07\xEB\x85a\tDV[\x85Q`@\x80Q`\0\x81R` \x81\x01\x80\x83R\x84\x90R`\xFF\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x08MW=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x01xV[PPPPPV[`\0\x80`\0\x83Q`A\x14a\t(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x01xV[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[`@\x80\x82\x01Q\x81Q\x7F\xC2\xF8xqv\xB8\xACk\xF7![J\xDC\xC1\xE0i\xBFJ\xB8-\x9A\xB1\xDF\x05\xA5z\x91\xD4%\x93[n` \x82\x01R\x7F\xD4\xE2\xD2a\xFF\xA0\xA31\xA9>\xD4\xAE\xE4\x94\x07o\xDC\x03\xDA\x7F\x1A\x99\xDFT\xA9d\x8Ca\xCA\x1D\x85\x94\x92\x81\x01\x92\x90\x92R\x7F\x06\xC0\x15\xBD\"\xB4\xC6\x96\x90\x93<\x10X\x87\x8E\xBD\xFE\xF3\x1F\x9A\xAA\xE4\x0B\xBE\x86\xD8\xA0\x9F\xE1\xB2\x97,``\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\0\x90\x81\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x7F\x9F4\xD6E\xC3}:\xED\xB5\x03Rz\x88\xEB=\xAF\x0B\xAD\xF2*\xDE\x9B\x80C\x07\xCD\xD0\xBF2\x8A\x9B\x13\x84`\0\x01Q\x85``\x01Q\x86`\xA0\x01Q\x87`\xC0\x01Q\x88`@\x01Q\x89`\xE0\x01Q\x80Q\x90` \x01 \x8A`\x80\x01Q\x8Ba\x01\0\x01Q\x8Ca\x01 \x01Q`@Q` \x01a\n\xB5\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90\x99\x8AR`\x01`\x01`\xA0\x1B\x03\x98\x89\x16` \x8B\x01Rc\xFF\xFF\xFF\xFF\x97\x88\x16`@\x8B\x01R\x95\x88\x16``\x8A\x01R`\x80\x89\x01\x94\x90\x94R\x91\x90\x94\x16`\xA0\x87\x01R`\xC0\x86\x01\x93\x90\x93R\x91\x90\x92\x16`\xE0\x84\x01Ra\x01\0\x83\x01Ra\x01 \x82\x01Ra\x01@\x01\x90V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 a\x19\x01`\xF0\x1B\x82\x85\x01R`\"\x84\x01\x95\x90\x95R`B\x80\x84\x01\x95\x90\x95R\x81Q\x80\x84\x03\x90\x95\x01\x85R`b\x90\x92\x01\x90R\x82Q\x92\x01\x91\x90\x91 \x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0BBWa\x0BBa\x0B\x08V[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B_W`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x0BuW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\x90Wa\x0B\x90a\x0B\x08V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0B\xB8Wa\x0B\xB8a\x0B\x08V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0B\xD1W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B_W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\x17W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C/W`\0\x80\xFD[\x90\x83\x01\x90a\x01@\x82\x86\x03\x12\x15a\x0CDW`\0\x80\xFD[a\x0CLa\x0B\x1EV[a\x0CU\x83a\x0BHV[\x81R` \x83\x015\x82\x81\x11\x15a\x0CiW`\0\x80\xFD[a\x0Cu\x87\x82\x86\x01a\x0BdV[` \x83\x01RPa\x0C\x87`@\x84\x01a\x0B\xF1V[`@\x82\x01Ra\x0C\x98``\x84\x01a\x0B\xF1V[``\x82\x01Ra\x0C\xA9`\x80\x84\x01a\x0BHV[`\x80\x82\x01Ra\x0C\xBA`\xA0\x84\x01a\x0BHV[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x0C\xDBW`\0\x80\xFD[a\x0C\xE7\x87\x82\x86\x01a\x0BdV[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\r W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\rBW\x81\x81\x01Q\x83\x82\x01R` \x01a\r*V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\rc\x81` \x86\x01` \x86\x01a\r'V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x82Ra\x01@` \x83\x01\x81\x90R`\0\x91a\r\x9D\x84\x83\x01\x8Ea\rKV[c\xFF\xFF\xFF\xFF\x8D\x81\x16`@\x87\x01R\x8C\x16``\x86\x01R\x8A\x82\x16`\x80\x86\x01R\x90\x89\x16`\xA0\x85\x01R`\xC0\x84\x01\x88\x90R\x83\x81\x03`\xE0\x85\x01R\x90Pa\r\xDC\x81\x87a\rKV[a\x01\0\x84\x01\x95\x90\x95RPPa\x01 \x01R\x98\x97PPPPPPPPV[` \x81Ra\x0E\x12` \x82\x01\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\0` \x83\x01Qa\x01@\x80`@\x85\x01Ra\x0E0a\x01`\x85\x01\x83a\rKV[\x91P`@\x85\x01Qa\x0EI``\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P``\x85\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x86\x01RP`\x80\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xA0\x86\x01RP`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xC0\x86\x01RP`\xC0\x85\x01Q`\xE0\x85\x01R`\xE0\x85\x01Qa\x01\0`\x1F\x19\x86\x85\x03\x01\x81\x87\x01Ra\x0E\xAD\x84\x83a\rKV[\x90\x87\x01Qa\x01 \x87\x81\x01\x91\x90\x91R\x90\x96\x01Q\x91\x90\x94\x01RP\x91\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\xE0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0F\0WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x0FPW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0F-WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0FLW\x82\x81U`\x01\x01a\x0F9V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FoWa\x0Foa\x0B\x08V[a\x0F\x83\x81a\x0F}\x84Ta\x0E\xCCV[\x84a\x0F\x06V[` \x80`\x1F\x83\x11`\x01\x81\x14a\x0F\xB8W`\0\x84\x15a\x0F\xA0WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0FLV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\x0F\xE7W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\x0F\xC8V[P\x85\x82\x10\x15a\x10\x05W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Qa\x10|\x81`d\x85\x01` \x89\x01a\r'V[\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \x9C\x11\xF2\x92\xAF\x07\x91j\x15\xA5\x15\xDCw\xD4o,\xE9\xC4\x08A\x9F\x84%\x92n<10\xB0\x87\xA0\xC2dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static INTENTSMEMPOOL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct IntentsMempool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IntentsMempool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IntentsMempool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IntentsMempool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IntentsMempool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IntentsMempool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IntentsMempool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INTENTSMEMPOOL_ABI.clone(),
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
                INTENTSMEMPOOL_ABI.clone(),
                INTENTSMEMPOOL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getIntent` (0xf13c46aa) function
        pub fn get_intent(
            &self,
            intent_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, SwapIntent> {
            self.0
                .method_hash([241, 60, 70, 170], intent_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intents` (0x9021578a) function
        pub fn intents(
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
                .method_hash([144, 33, 87, 138], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSettled` (0xbd07f3c9) function
        pub fn is_settled(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([189, 7, 243, 201], p0)
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
        ///Calls the contract's `storeIntent` (0x23d87501) function
        pub fn store_intent(
            &self,
            new_intent: SwapIntent,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 216, 117, 1], (new_intent,))
                .expect("method not found (this should never happen)")
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IntentCreatedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IntentsMempool<M> {
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
        name = "IntentCreated",
        abi = "IntentCreated(bytes32,(address,bytes,uint32,uint32,address,address,uint256,bytes,uint256,uint256))"
    )]
    pub struct IntentCreatedFilter {
        #[ethevent(indexed)]
        pub intent_id: [u8; 32],
        pub intent: SwapIntent,
    }
    ///Container type for all input parameters for the `getIntent` function with signature `getIntent(bytes32)` and selector `0xf13c46aa`
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
    #[ethcall(name = "getIntent", abi = "getIntent(bytes32)")]
    pub struct GetIntentCall {
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
    ///Container type for all input parameters for the `isSettled` function with signature `isSettled(bytes32)` and selector `0xbd07f3c9`
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
    #[ethcall(name = "isSettled", abi = "isSettled(bytes32)")]
    pub struct IsSettledCall(pub [u8; 32]);
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
    ///Container type for all input parameters for the `storeIntent` function with signature `storeIntent((address,bytes,uint32,uint32,address,address,uint256,bytes,uint256,uint256))` and selector `0x23d87501`
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
        name = "storeIntent",
        abi = "storeIntent((address,bytes,uint32,uint32,address,address,uint256,bytes,uint256,uint256))"
    )]
    pub struct StoreIntentCall {
        pub new_intent: SwapIntent,
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
    pub enum IntentsMempoolCalls {
        GetIntent(GetIntentCall),
        Intents(IntentsCall),
        IsSettled(IsSettledCall),
        SettleIntent(SettleIntentCall),
        StoreIntent(StoreIntentCall),
    }
    impl ::ethers::core::abi::AbiDecode for IntentsMempoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetIntent(decoded));
            }
            if let Ok(decoded) = <IntentsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Intents(decoded));
            }
            if let Ok(decoded) = <IsSettledCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsSettled(decoded));
            }
            if let Ok(decoded) = <SettleIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SettleIntent(decoded));
            }
            if let Ok(decoded) = <StoreIntentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StoreIntent(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IntentsMempoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Intents(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsSettled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SettleIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StoreIntent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IntentsMempoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::Intents(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSettled(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettleIntent(element) => ::core::fmt::Display::fmt(element, f),
                Self::StoreIntent(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetIntentCall> for IntentsMempoolCalls {
        fn from(value: GetIntentCall) -> Self {
            Self::GetIntent(value)
        }
    }
    impl ::core::convert::From<IntentsCall> for IntentsMempoolCalls {
        fn from(value: IntentsCall) -> Self {
            Self::Intents(value)
        }
    }
    impl ::core::convert::From<IsSettledCall> for IntentsMempoolCalls {
        fn from(value: IsSettledCall) -> Self {
            Self::IsSettled(value)
        }
    }
    impl ::core::convert::From<SettleIntentCall> for IntentsMempoolCalls {
        fn from(value: SettleIntentCall) -> Self {
            Self::SettleIntent(value)
        }
    }
    impl ::core::convert::From<StoreIntentCall> for IntentsMempoolCalls {
        fn from(value: StoreIntentCall) -> Self {
            Self::StoreIntent(value)
        }
    }
    ///Container type for all return fields from the `getIntent` function with signature `getIntent(bytes32)` and selector `0xf13c46aa`
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
    pub struct GetIntentReturn(pub SwapIntent);
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
    ///Container type for all return fields from the `isSettled` function with signature `isSettled(bytes32)` and selector `0xbd07f3c9`
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
    pub struct IsSettledReturn(pub bool);
}
