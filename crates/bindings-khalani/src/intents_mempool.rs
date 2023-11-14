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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x07\x04\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x14a\0\\W\x80c\x90!W\x8A\x14a\0qW\x80c\xBC\x1BlV\x14a\x016W\x80c\xBD\x07\xF3\xC9\x14a\x01IW\x80c\xF1<F\xAA\x14a\x01|W[`\0\x80\xFD[a\0oa\0j6`\x04a\x05QV[a\x02cV[\0[a\0\xDDa\0\x7F6`\x04a\x05QV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x90\x95\x01T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x95\x93\x94c\xFF\xFF\xFF\xFF\x80\x85\x16\x95d\x01\0\0\0\0\x86\x04\x90\x91\x16\x94`\x01`@\x1B\x90\x04\x82\x16\x93\x90\x91\x16\x91\x88V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x99\x8A\x16\x81R` \x81\x01\x98\x90\x98Rc\xFF\xFF\xFF\xFF\x96\x87\x16\x90\x88\x01R\x93\x90\x94\x16``\x86\x01R\x90\x85\x16`\x80\x85\x01R\x90\x93\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x81\x01\x91\x90\x91Ra\x01\0\x01[`@Q\x80\x91\x03\x90\xF3[a\0oa\x01D6`\x04a\x05\x9AV[a\x03:V[a\x01la\x01W6`\x04a\x05QV[`\x01` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01-V[a\x02Va\x01\x8A6`\x04a\x05QV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91RP`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x83\x01T\x93\x82\x01\x93\x90\x93R`\x02\x82\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x95\x83\x01\x95\x90\x95Rd\x01\0\0\0\0\x81\x04\x90\x94\x16``\x82\x01R`\x01`@\x1B\x90\x93\x04\x82\x16`\x80\x84\x01R`\x03\x81\x01T\x90\x91\x16`\xA0\x83\x01R`\x04\x81\x01T`\xC0\x83\x01R`\x05\x01T`\xE0\x82\x01R\x90V[`@Qa\x01-\x91\x90a\x06[V[`\0\x81\x81R` \x81\x90R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x01` R`@\x90 T`\xFF\x16\x15a\x03\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x12[\x9D\x19[\x9D\x08\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`R\x1B`D\x82\x01R`d\x01a\x02\xBBV[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UV[`\0a\x03\xDD\x82\x80Q`@\x80\x83\x01Q``\x80\x85\x01Q`\x80\x86\x01Q`\xA0\x87\x01Q`\xC0\x88\x01Q`\xE0\x80\x8A\x01Q\x97Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x99\x87\x1B\x8A\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x97\x82\x1B\x88\x16`4\x82\x01R\x94\x90\x1B\x90\x95\x16`8\x84\x01R\x90\x83\x1B\x86\x16`<\x83\x01R\x90\x91\x1B\x90\x93\x16`P\x84\x01R`d\x83\x01R`\x84\x82\x01R`\0\x90`\xA4\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81\x81R` \x81\x90R`@\x90 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x02\xBBV[`\0\x81\x81R` \x81\x81R`@\x80\x83 \x85Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U\x84\x88\x01Q`\x01\x80\x85\x01\x91\x90\x91U\x84\x89\x01Q`\x02\x85\x01\x80T``\x8C\x01Q`\x80\x8D\x01Q\x87\x16`\x01`@\x1B\x02h\x01\0\0\0\0\0\0\0\0`\x01`\xE0\x1B\x03\x19c\xFF\xFF\xFF\xFF\x92\x83\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x92\x90\x95\x16\x91\x90\x91\x17\x91\x90\x91\x17\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\xA0\x89\x01Q`\x03\x85\x01\x80T\x91\x90\x94\x16\x92\x16\x91\x90\x91\x17\x90\x91U`\xC0\x87\x01Q`\x04\x83\x01U`\xE0\x87\x01Q`\x05\x90\x92\x01\x91\x90\x91U\x90\x91R\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x81\x90\x7F_fh\xF3\xAA?\xB8w\xE0\xE0i\xD7N~\x80\xB3m}A\x8A%f\x8EO\x8D\xE4\xFCr6\x81\xFA\x96\x90a\x05E\x90\x85\x90a\x06[V[`@Q\x80\x91\x03\x90\xA2PPV[`\0` \x82\x84\x03\x12\x15a\x05cW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x81W`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\x81W`\0\x80\xFD[`\0a\x01\0\x80\x83\x85\x03\x12\x15a\x05\xAEW`\0\x80\xFD[`@Q\x90\x81\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x81\x83\x10\x17\x15a\x05\xDFWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x81`@Ra\x05\xEC\x84a\x05jV[\x81R` \x84\x015` \x82\x01Ra\x06\x04`@\x85\x01a\x05\x86V[`@\x82\x01Ra\x06\x15``\x85\x01a\x05\x86V[``\x82\x01Ra\x06&`\x80\x85\x01a\x05jV[`\x80\x82\x01Ra\x067`\xA0\x85\x01a\x05jV[`\xA0\x82\x01R`\xC0\x84\x015`\xC0\x82\x01R`\xE0\x84\x015`\xE0\x82\x01R\x80\x92PPP\x92\x91PPV[`\0a\x01\0\x82\x01\x90P`\x01\x80`\xA0\x1B\x03\x80\x84Q\x16\x83R` \x84\x01Q` \x84\x01R`@\x84\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`@\x86\x01R\x80``\x87\x01Q\x16``\x86\x01RPP\x80`\x80\x85\x01Q\x16`\x80\x84\x01R\x80`\xA0\x85\x01Q\x16`\xA0\x84\x01RP`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01R\x92\x91PPV\xFE\xA2dipfsX\"\x12 Zb\xF9\xFB\xB9Y\xBF\xAA\xEF\xC8\x10\xB1\x15\xA2\x94\xD0\x9AE\x06\xA5\x92O\xB9%\x13sE\x19\xFB\x8F!\x85dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static INTENTSMEMPOOL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c{\xF8\xBB\x88\x14a\0\\W\x80c\x90!W\x8A\x14a\0qW\x80c\xBC\x1BlV\x14a\x016W\x80c\xBD\x07\xF3\xC9\x14a\x01IW\x80c\xF1<F\xAA\x14a\x01|W[`\0\x80\xFD[a\0oa\0j6`\x04a\x05QV[a\x02cV[\0[a\0\xDDa\0\x7F6`\x04a\x05QV[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x90\x95\x01T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x95\x93\x94c\xFF\xFF\xFF\xFF\x80\x85\x16\x95d\x01\0\0\0\0\x86\x04\x90\x91\x16\x94`\x01`@\x1B\x90\x04\x82\x16\x93\x90\x91\x16\x91\x88V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x99\x8A\x16\x81R` \x81\x01\x98\x90\x98Rc\xFF\xFF\xFF\xFF\x96\x87\x16\x90\x88\x01R\x93\x90\x94\x16``\x86\x01R\x90\x85\x16`\x80\x85\x01R\x90\x93\x16`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x81\x01\x91\x90\x91Ra\x01\0\x01[`@Q\x80\x91\x03\x90\xF3[a\0oa\x01D6`\x04a\x05\x9AV[a\x03:V[a\x01la\x01W6`\x04a\x05QV[`\x01` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01-V[a\x02Va\x01\x8A6`\x04a\x05QV[`@\x80Qa\x01\0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91RP`\0\x90\x81R` \x81\x81R`@\x91\x82\x90 \x82Qa\x01\0\x81\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R`\x01\x83\x01T\x93\x82\x01\x93\x90\x93R`\x02\x82\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x95\x83\x01\x95\x90\x95Rd\x01\0\0\0\0\x81\x04\x90\x94\x16``\x82\x01R`\x01`@\x1B\x90\x93\x04\x82\x16`\x80\x84\x01R`\x03\x81\x01T\x90\x91\x16`\xA0\x83\x01R`\x04\x81\x01T`\xC0\x83\x01R`\x05\x01T`\xE0\x82\x01R\x90V[`@Qa\x01-\x91\x90a\x06[V[`\0\x81\x81R` \x81\x90R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x12[\x9D\x19[\x9D\x08\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x01` R`@\x90 T`\xFF\x16\x15a\x03\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x12[\x9D\x19[\x9D\x08\x18[\x1C\x99XY\x1EH\x1C\xD9]\x1D\x1B\x19Y`R\x1B`D\x82\x01R`d\x01a\x02\xBBV[`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90UV[`\0a\x03\xDD\x82\x80Q`@\x80\x83\x01Q``\x80\x85\x01Q`\x80\x86\x01Q`\xA0\x87\x01Q`\xC0\x88\x01Q`\xE0\x80\x8A\x01Q\x97Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x99\x87\x1B\x8A\x16` \x82\x01R`\x01`\x01`\xE0\x1B\x03\x19\x97\x82\x1B\x88\x16`4\x82\x01R\x94\x90\x1B\x90\x95\x16`8\x84\x01R\x90\x83\x1B\x86\x16`<\x83\x01R\x90\x91\x1B\x90\x93\x16`P\x84\x01R`d\x83\x01R`\x84\x82\x01R`\0\x90`\xA4\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81\x81R` \x81\x90R`@\x90 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x15a\x04=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtIntent already exists`X\x1B`D\x82\x01R`d\x01a\x02\xBBV[`\0\x81\x81R` \x81\x81R`@\x80\x83 \x85Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U\x84\x88\x01Q`\x01\x80\x85\x01\x91\x90\x91U\x84\x89\x01Q`\x02\x85\x01\x80T``\x8C\x01Q`\x80\x8D\x01Q\x87\x16`\x01`@\x1B\x02h\x01\0\0\0\0\0\0\0\0`\x01`\xE0\x1B\x03\x19c\xFF\xFF\xFF\xFF\x92\x83\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x92\x90\x95\x16\x91\x90\x91\x17\x91\x90\x91\x17\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\xA0\x89\x01Q`\x03\x85\x01\x80T\x91\x90\x94\x16\x92\x16\x91\x90\x91\x17\x90\x91U`\xC0\x87\x01Q`\x04\x83\x01U`\xE0\x87\x01Q`\x05\x90\x92\x01\x91\x90\x91U\x90\x91R\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x81\x90\x7F_fh\xF3\xAA?\xB8w\xE0\xE0i\xD7N~\x80\xB3m}A\x8A%f\x8EO\x8D\xE4\xFCr6\x81\xFA\x96\x90a\x05E\x90\x85\x90a\x06[V[`@Q\x80\x91\x03\x90\xA2PPV[`\0` \x82\x84\x03\x12\x15a\x05cW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x81W`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\x81W`\0\x80\xFD[`\0a\x01\0\x80\x83\x85\x03\x12\x15a\x05\xAEW`\0\x80\xFD[`@Q\x90\x81\x01\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x81\x83\x10\x17\x15a\x05\xDFWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x81`@Ra\x05\xEC\x84a\x05jV[\x81R` \x84\x015` \x82\x01Ra\x06\x04`@\x85\x01a\x05\x86V[`@\x82\x01Ra\x06\x15``\x85\x01a\x05\x86V[``\x82\x01Ra\x06&`\x80\x85\x01a\x05jV[`\x80\x82\x01Ra\x067`\xA0\x85\x01a\x05jV[`\xA0\x82\x01R`\xC0\x84\x015`\xC0\x82\x01R`\xE0\x84\x015`\xE0\x82\x01R\x80\x92PPP\x92\x91PPV[`\0a\x01\0\x82\x01\x90P`\x01\x80`\xA0\x1B\x03\x80\x84Q\x16\x83R` \x84\x01Q` \x84\x01R`@\x84\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`@\x86\x01R\x80``\x87\x01Q\x16``\x86\x01RPP\x80`\x80\x85\x01Q\x16`\x80\x84\x01R\x80`\xA0\x85\x01Q\x16`\xA0\x84\x01RP`\xC0\x83\x01Q`\xC0\x83\x01R`\xE0\x83\x01Q`\xE0\x83\x01R\x92\x91PPV\xFE\xA2dipfsX\"\x12 Zb\xF9\xFB\xB9Y\xBF\xAA\xEF\xC8\x10\xB1\x15\xA2\x94\xD0\x9AE\x06\xA5\x92O\xB9%\x13sE\x19\xFB\x8F!\x85dsolcC\0\x08\x13\x003";
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
                ::ethers::core::types::U256,
                u32,
                u32,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
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
        ///Calls the contract's `storeIntent` (0xbc1b6c56) function
        pub fn store_intent(
            &self,
            new_intent: SwapIntent,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 27, 108, 86], (new_intent,))
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
        abi = "IntentCreated(bytes32,(address,uint256,uint32,uint32,address,address,uint256,uint256))"
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
    ///Container type for all input parameters for the `storeIntent` function with signature `storeIntent((address,uint256,uint32,uint32,address,address,uint256,uint256))` and selector `0xbc1b6c56`
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
        abi = "storeIntent((address,uint256,uint32,uint32,address,address,uint256,uint256))"
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
        pub signature: ::ethers::core::types::U256,
        pub source_chain_id: u32,
        pub destination_chain_id: u32,
        pub source_token: ::ethers::core::types::Address,
        pub destination_token: ::ethers::core::types::Address,
        pub source_amount: ::ethers::core::types::U256,
        pub source_permit_2: ::ethers::core::types::U256,
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
