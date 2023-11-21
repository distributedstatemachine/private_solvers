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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0E^\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x14a\0\\W\x80c\x90!W\x8A\x14a\0qW\x80c\xBD\x07\xF3\xC9\x14a\0\xA1W\x80c\xF0<!\xD6\x14a\0\xD4W\x80c\xF1<F\xAA\x14a\0\xE7W[`\0\x80\xFD[a\0oa\0j6`\x04a\x08\xE3V[a\x01\x07V[\0[a\0\x84a\0\x7F6`\x04a\x08\xE3V[a\x01\xDEV[`@Qa\0\x98\x98\x97\x96\x95\x94\x93\x92\x91\x90a\tLV[`@Q\x80\x91\x03\x90\xF3[a\0\xC4a\0\xAF6`\x04a\x08\xE3V[`\x01` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\x98V[a\0oa\0\xE26`\x04a\n\xBDV[a\x03ZV[a\0\xFAa\0\xF56`\x04a\x08\xE3V[a\x05\x0BV[`@Qa\0\x98\x91\x90a\x0B\xAEV[`\0\x81\x81R` \x81\x90R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x01hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x01` R`@\x90 T`\xFF\x16\x15a\x01\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x12[\x9D\x19[\x9D\x08\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`R\x1B`D\x82\x01R`d\x01a\x01_V[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91a\x02\n\x90a\x0CeV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x026\x90a\x0CeV[\x80\x15a\x02\x83W\x80`\x1F\x10a\x02XWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x83V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02fW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x02\x84\x01T`\x03\x85\x01T`\x04\x86\x01T`\x05\x87\x01\x80T\x96\x97c\xFF\xFF\xFF\xFF\x80\x86\x16\x98d\x01\0\0\0\0\x87\x04\x90\x91\x16\x97P`\x01`\x01`\xA0\x1B\x03`\x01`@\x1B\x90\x96\x04\x86\x16\x96P\x93\x90\x94\x16\x93\x91\x92a\x02\xD7\x90a\x0CeV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\x03\x90a\x0CeV[\x80\x15a\x03PW\x80`\x1F\x10a\x03%Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03PV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x033W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x88V[`\0a\x03e\x82a\x06\xF1V[\x90Pa\x03u\x81\x83` \x01Qa\x07IV[`\0\x81\x81R` \x81\x90R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x03\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x01_V[`\0\x81\x81R` \x81\x81R`@\x90\x91 \x83Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x83\x01Q\x83\x91\x90`\x01\x82\x01\x90a\x04\x13\x90\x82a\x0C\xEEV[P`@\x82\x01Q`\x02\x82\x01\x80T``\x85\x01Q`\x80\x86\x01Qc\xFF\xFF\xFF\xFF\x94\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17d\x01\0\0\0\0\x94\x90\x91\x16\x93\x90\x93\x02\x92\x90\x92\x17h\x01\0\0\0\0\0\0\0\0`\x01`\xE0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x02\x17\x90U`\xA0\x83\x01Q`\x03\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U`\xC0\x82\x01Q`\x04\x82\x01U`\xE0\x82\x01Q`\x05\x82\x01\x90a\x04\xB4\x90\x82a\x0C\xEEV[PPP`\0\x81\x81R`\x01` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x81\x90\x7F\x94\x8B/\x98\xBE\xA8\xC3\xD7iS\xFC\x93\xA8l\xD8\xEE1i-,\xFFTW5\xFE\x88H\xE1!Z\xA1\xB0\x90a\x04\xFF\x90\x85\x90a\x0B\xAEV[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R``` \x83\x01\x81\x90R\x92\x82\x01\x81\x90R\x82\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01R`\xE0\x81\x01\x91\x90\x91R`\0\x82\x81R` \x81\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x91\x92\x84\x01\x91a\x05\x86\x90a\x0CeV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xB2\x90a\x0CeV[\x80\x15a\x05\xFFW\x80`\x1F\x10a\x05\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xFFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xE2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x82\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16` \x84\x01Rd\x01\0\0\0\0\x82\x04\x16`@\x83\x01R`\x01`\x01`\xA0\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16``\x83\x01R`\x03\x83\x01T\x16`\x80\x82\x01R`\x04\x82\x01T`\xA0\x82\x01R`\x05\x82\x01\x80T`\xC0\x90\x92\x01\x91a\x06h\x90a\x0CeV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x94\x90a\x0CeV[\x80\x15a\x06\xE1W\x80`\x1F\x10a\x06\xB6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xE1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xC4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q`@Q` \x01a\x07,\x97\x96\x95\x94\x93\x92\x91\x90a\r\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a\x07W\x84a\x08aV[\x7F\x19Ethereum Signed Message:\n32\0\0\0\0`\0\x90\x81R`\x1C\x89\x90R`<\x81 \x93\x96P\x91\x94P\x92P\x90`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x84\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R\x91\x92P\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x07\xE7W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x08XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x01_V[PPPPPPPV[`\0\x80`\0\x83Q`A\x14a\x08\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x01_V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[`\0` \x82\x84\x03\x12\x15a\x08\xF5W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\t\x17W\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\xFFV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\t8\x81` \x86\x01` \x86\x01a\x08\xFCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82Ra\x01\0` \x83\x01\x81\x90R`\0\x91a\tr\x84\x83\x01\x8Ca\t V[c\xFF\xFF\xFF\xFF\x8B\x81\x16`@\x87\x01R\x8A\x16``\x86\x01R\x88\x82\x16`\x80\x86\x01R\x90\x87\x16`\xA0\x85\x01R`\xC0\x84\x01\x86\x90R\x83\x81\x03`\xE0\x85\x01R\x90Pa\t\xB1\x81\x85a\t V[\x9B\x9APPPPPPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\xFAWa\t\xFAa\t\xC0V[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\x17W`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\n-W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\nHWa\nHa\t\xC0V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\npWa\npa\t\xC0V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\n\x89W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\n\x17W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\n\xCFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xE7W`\0\x80\xFD[\x90\x83\x01\x90a\x01\0\x82\x86\x03\x12\x15a\n\xFCW`\0\x80\xFD[a\x0B\x04a\t\xD6V[a\x0B\r\x83a\n\0V[\x81R` \x83\x015\x82\x81\x11\x15a\x0B!W`\0\x80\xFD[a\x0B-\x87\x82\x86\x01a\n\x1CV[` \x83\x01RPa\x0B?`@\x84\x01a\n\xA9V[`@\x82\x01Ra\x0BP``\x84\x01a\n\xA9V[``\x82\x01Ra\x0Ba`\x80\x84\x01a\n\0V[`\x80\x82\x01Ra\x0Br`\xA0\x84\x01a\n\0V[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x0B\x93W`\0\x80\xFD[a\x0B\x9F\x87\x82\x86\x01a\n\x1CV[`\xE0\x83\x01RP\x95\x94PPPPPV[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x82\x82\x01R\x82\x01Qa\x01\0`@\x83\x01\x81\x90R`\0\x91\x90a\x0B\xE1a\x01 \x85\x01\x83a\t V[\x91P`@\x85\x01Qa\x0B\xFA``\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P``\x85\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x86\x01RP`\x80\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xA0\x86\x01RP`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xC0\x86\x01RP`\xC0\x85\x01Q`\xE0\x85\x01R`\xE0\x85\x01Q`\x1F\x19\x85\x84\x03\x01\x82\x86\x01Ra\x0C[\x83\x82a\t V[\x96\x95PPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0CyW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C\x99WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x0C\xE9W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0C\xC6WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0C\xE5W\x82\x81U`\x01\x01a\x0C\xD2V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x08Wa\r\x08a\t\xC0V[a\r\x1C\x81a\r\x16\x84Ta\x0CeV[\x84a\x0C\x9FV[` \x80`\x1F\x83\x11`\x01\x81\x14a\rQW`\0\x84\x15a\r9WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0C\xE5V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\r\x80W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\raV[P\x85\x82\x10\x15a\r\x9EW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8A``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8A`\xE0\x1B\x16`\x14\x85\x01R\x80\x89`\xE0\x1B\x16`\x18\x85\x01RP\x80\x87``\x1B\x16`\x1C\x84\x01R\x80\x86``\x1B\x16`0\x84\x01RP\x83`D\x83\x01R\x82Qa\x0E\x15\x81`d\x85\x01` \x87\x01a\x08\xFCV[\x91\x90\x91\x01`d\x01\x98\x97PPPPPPPPV\xFE\xA2dipfsX\"\x12 O\x04o\xB7k\x13\xE6\xE4c_\x89.tcO\x8A\xDC\xEF\xE9_\t{\x88\xE2\x89\x02-\x0C*\x960\x9DdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static INTENTSMEMPOOL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x14a\0\\W\x80c\x90!W\x8A\x14a\0qW\x80c\xBD\x07\xF3\xC9\x14a\0\xA1W\x80c\xF0<!\xD6\x14a\0\xD4W\x80c\xF1<F\xAA\x14a\0\xE7W[`\0\x80\xFD[a\0oa\0j6`\x04a\x08\xE3V[a\x01\x07V[\0[a\0\x84a\0\x7F6`\x04a\x08\xE3V[a\x01\xDEV[`@Qa\0\x98\x98\x97\x96\x95\x94\x93\x92\x91\x90a\tLV[`@Q\x80\x91\x03\x90\xF3[a\0\xC4a\0\xAF6`\x04a\x08\xE3V[`\x01` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\x98V[a\0oa\0\xE26`\x04a\n\xBDV[a\x03ZV[a\0\xFAa\0\xF56`\x04a\x08\xE3V[a\x05\x0BV[`@Qa\0\x98\x91\x90a\x0B\xAEV[`\0\x81\x81R` \x81\x90R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x01hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x01` R`@\x90 T`\xFF\x16\x15a\x01\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x12[\x9D\x19[\x9D\x08\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`R\x1B`D\x82\x01R`d\x01a\x01_V[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91a\x02\n\x90a\x0CeV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x026\x90a\x0CeV[\x80\x15a\x02\x83W\x80`\x1F\x10a\x02XWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\x83V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02fW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP`\x02\x84\x01T`\x03\x85\x01T`\x04\x86\x01T`\x05\x87\x01\x80T\x96\x97c\xFF\xFF\xFF\xFF\x80\x86\x16\x98d\x01\0\0\0\0\x87\x04\x90\x91\x16\x97P`\x01`\x01`\xA0\x1B\x03`\x01`@\x1B\x90\x96\x04\x86\x16\x96P\x93\x90\x94\x16\x93\x91\x92a\x02\xD7\x90a\x0CeV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\x03\x90a\x0CeV[\x80\x15a\x03PW\x80`\x1F\x10a\x03%Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03PV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x033W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x88V[`\0a\x03e\x82a\x06\xF1V[\x90Pa\x03u\x81\x83` \x01Qa\x07IV[`\0\x81\x81R` \x81\x90R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x03\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x01_V[`\0\x81\x81R` \x81\x81R`@\x90\x91 \x83Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x83\x01Q\x83\x91\x90`\x01\x82\x01\x90a\x04\x13\x90\x82a\x0C\xEEV[P`@\x82\x01Q`\x02\x82\x01\x80T``\x85\x01Q`\x80\x86\x01Qc\xFF\xFF\xFF\xFF\x94\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17d\x01\0\0\0\0\x94\x90\x91\x16\x93\x90\x93\x02\x92\x90\x92\x17h\x01\0\0\0\0\0\0\0\0`\x01`\xE0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x02\x17\x90U`\xA0\x83\x01Q`\x03\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U`\xC0\x82\x01Q`\x04\x82\x01U`\xE0\x82\x01Q`\x05\x82\x01\x90a\x04\xB4\x90\x82a\x0C\xEEV[PPP`\0\x81\x81R`\x01` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x81\x90\x7F\x94\x8B/\x98\xBE\xA8\xC3\xD7iS\xFC\x93\xA8l\xD8\xEE1i-,\xFFTW5\xFE\x88H\xE1!Z\xA1\xB0\x90a\x04\xFF\x90\x85\x90a\x0B\xAEV[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R``` \x83\x01\x81\x90R\x92\x82\x01\x81\x90R\x82\x82\x01\x81\x90R`\x80\x82\x01\x81\x90R`\xA0\x82\x01\x81\x90R`\xC0\x82\x01R`\xE0\x81\x01\x91\x90\x91R`\0\x82\x81R` \x81\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x91\x92\x84\x01\x91a\x05\x86\x90a\x0CeV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\xB2\x90a\x0CeV[\x80\x15a\x05\xFFW\x80`\x1F\x10a\x05\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xFFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xE2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x02\x82\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16` \x84\x01Rd\x01\0\0\0\0\x82\x04\x16`@\x83\x01R`\x01`\x01`\xA0\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16``\x83\x01R`\x03\x83\x01T\x16`\x80\x82\x01R`\x04\x82\x01T`\xA0\x82\x01R`\x05\x82\x01\x80T`\xC0\x90\x92\x01\x91a\x06h\x90a\x0CeV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\x94\x90a\x0CeV[\x80\x15a\x06\xE1W\x80`\x1F\x10a\x06\xB6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xE1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xC4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q`@Q` \x01a\x07,\x97\x96\x95\x94\x93\x92\x91\x90a\r\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x80`\0a\x07W\x84a\x08aV[\x7F\x19Ethereum Signed Message:\n32\0\0\0\0`\0\x90\x81R`\x1C\x89\x90R`<\x81 \x93\x96P\x91\x94P\x92P\x90`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x84\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R\x91\x92P\x90`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x07\xE7W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x08XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FVerification error: Signer is in`D\x82\x01Rd\x1D\x98[\x1AY`\xDA\x1B`d\x82\x01R`\x84\x01a\x01_V[PPPPPPPV[`\0\x80`\0\x83Q`A\x14a\x08\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FThe signature length is not equa`D\x82\x01Rfl to 65`\xC8\x1B`d\x82\x01R`\x84\x01a\x01_V[PPP` \x81\x01Q`@\x82\x01Q``\x90\x92\x01Q`\0\x1A\x92\x90\x91\x90V[`\0` \x82\x84\x03\x12\x15a\x08\xF5W`\0\x80\xFD[P5\x91\x90PV[`\0[\x83\x81\x10\x15a\t\x17W\x81\x81\x01Q\x83\x82\x01R` \x01a\x08\xFFV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\t8\x81` \x86\x01` \x86\x01a\x08\xFCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82Ra\x01\0` \x83\x01\x81\x90R`\0\x91a\tr\x84\x83\x01\x8Ca\t V[c\xFF\xFF\xFF\xFF\x8B\x81\x16`@\x87\x01R\x8A\x16``\x86\x01R\x88\x82\x16`\x80\x86\x01R\x90\x87\x16`\xA0\x85\x01R`\xC0\x84\x01\x86\x90R\x83\x81\x03`\xE0\x85\x01R\x90Pa\t\xB1\x81\x85a\t V[\x9B\x9APPPPPPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\xFAWa\t\xFAa\t\xC0V[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\x17W`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\n-W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\nHWa\nHa\t\xC0V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\npWa\npa\t\xC0V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\n\x89W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\n\x17W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\n\xCFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xE7W`\0\x80\xFD[\x90\x83\x01\x90a\x01\0\x82\x86\x03\x12\x15a\n\xFCW`\0\x80\xFD[a\x0B\x04a\t\xD6V[a\x0B\r\x83a\n\0V[\x81R` \x83\x015\x82\x81\x11\x15a\x0B!W`\0\x80\xFD[a\x0B-\x87\x82\x86\x01a\n\x1CV[` \x83\x01RPa\x0B?`@\x84\x01a\n\xA9V[`@\x82\x01Ra\x0BP``\x84\x01a\n\xA9V[``\x82\x01Ra\x0Ba`\x80\x84\x01a\n\0V[`\x80\x82\x01Ra\x0Br`\xA0\x84\x01a\n\0V[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x0B\x93W`\0\x80\xFD[a\x0B\x9F\x87\x82\x86\x01a\n\x1CV[`\xE0\x83\x01RP\x95\x94PPPPPV[` \x80\x82R\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x82\x82\x01R\x82\x01Qa\x01\0`@\x83\x01\x81\x90R`\0\x91\x90a\x0B\xE1a\x01 \x85\x01\x83a\t V[\x91P`@\x85\x01Qa\x0B\xFA``\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P``\x85\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x86\x01RP`\x80\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xA0\x86\x01RP`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xC0\x86\x01RP`\xC0\x85\x01Q`\xE0\x85\x01R`\xE0\x85\x01Q`\x1F\x19\x85\x84\x03\x01\x82\x86\x01Ra\x0C[\x83\x82a\t V[\x96\x95PPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0CyW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C\x99WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x0C\xE9W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0C\xC6WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0C\xE5W\x82\x81U`\x01\x01a\x0C\xD2V[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x08Wa\r\x08a\t\xC0V[a\r\x1C\x81a\r\x16\x84Ta\x0CeV[\x84a\x0C\x9FV[` \x80`\x1F\x83\x11`\x01\x81\x14a\rQW`\0\x84\x15a\r9WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0C\xE5V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\r\x80W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\raV[P\x85\x82\x10\x15a\r\x9EW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8A``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8A`\xE0\x1B\x16`\x14\x85\x01R\x80\x89`\xE0\x1B\x16`\x18\x85\x01RP\x80\x87``\x1B\x16`\x1C\x84\x01R\x80\x86``\x1B\x16`0\x84\x01RP\x83`D\x83\x01R\x82Qa\x0E\x15\x81`d\x85\x01` \x87\x01a\x08\xFCV[\x91\x90\x91\x01`d\x01\x98\x97PPPPPPPPV\xFE\xA2dipfsX\"\x12 O\x04o\xB7k\x13\xE6\xE4c_\x89.tcO\x8A\xDC\xEF\xE9_\t{\x88\xE2\x89\x02-\x0C*\x960\x9DdsolcC\0\x08\x13\x003";
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
        ///Calls the contract's `storeIntent` (0xf03c21d6) function
        pub fn store_intent(
            &self,
            new_intent: SwapIntent,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 60, 33, 214], (new_intent,))
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
        abi = "IntentCreated(bytes32,(address,bytes,uint32,uint32,address,address,uint256,bytes))"
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
    ///Container type for all input parameters for the `storeIntent` function with signature `storeIntent((address,bytes,uint32,uint32,address,address,uint256,bytes))` and selector `0xf03c21d6`
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
        abi = "storeIntent((address,bytes,uint32,uint32,address,address,uint256,bytes))"
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
