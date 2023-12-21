pub use escrow::*;
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
pub mod escrow {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("permit_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IPermit2"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_intentEventProver"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract EventProver"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PERMIT2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PERMIT2"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPermit2"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("intentEventProver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("intentEventProver"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract EventProver"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lockTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lockTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
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
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentLib.SwapIntent",
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
                    ::std::borrow::ToOwned::to_owned("tokenBalancesByUser"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "tokenBalancesByUser",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("TokensLocked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokensLocked"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ESCROW_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\tM8\x03\x80a\tM\x839\x81\x01`@\x81\x90Ra\0/\x91a\0tV[`\x01`\0\x81\x90U`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x80R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90Ua\0\xAEV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0qW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\0\x87W`\0\x80\xFD[\x82Qa\0\x92\x81a\0\\V[` \x84\x01Q\x90\x92Pa\0\xA3\x81a\0\\V[\x80\x91PP\x92P\x92\x90PV[`\x80Qa\x08~a\0\xCF`\09`\0\x81\x81`V\x01Ra\x01\x8D\x01Ra\x08~`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80cj\xFD\xD8P\x14a\0QW\x80c\x86\\\xB8\xC2\x14a\0\x95W\x80c\x8D\0\xE1N\x14a\0\xA8W\x80c\xA1\xF0\n\xFC\x14a\0\xE1W[`\0\x80\xFD[a\0x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0x\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xD3a\0\xB66`\x04a\x04dV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\0\x8CV[a\0\xF4a\0\xEF6`\x04a\x04\x97V[a\0\xF6V[\0[a\0\xFEa\x03QV[`\0a\x01\x11a\x01\x0C\x83a\x05\xBBV[a\x03\xAEV[\x90P`\xC0\x82\x015`\x02`\0a\x01)` \x86\x01\x86a\x06\x9FV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x01U`\xA0\x86\x01`\x80\x87\x01a\x06\x9FV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x01\x84\x91\x90a\x06\xBAV[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c0\xF2\x8Bz`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80\x87`\x80\x01` \x81\x01\x90a\x01\xE3\x91\x90a\x06\x9FV[`\x01`\x01`\xA0\x1B\x03\x16\x81R`\xC0\x88\x015` \x91\x82\x01\x81\x90R\x91\x83Ra\x01\0\x88\x015\x83\x82\x01Ra\x01 \x88\x015`@\x93\x84\x01R\x82Q\x80\x84\x01\x90\x93R0\x83R\x82\x81\x01\x91\x90\x91Ra\x022\x90\x87\x01\x87a\x06\x9FV[a\x02?`\xE0\x88\x01\x88a\x06\xE1V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02_\x95\x94\x93\x92\x91\x90a\x07/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02yW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x8DW=`\0\x80>=`\0\xFD[PPPP`\0a\x02\xAA`@Q\x80` \x01`@R\x80\x84\x81RPa\x04\x14V[`\x01T`@Qc\x1A\xCDdY`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD6k\"\xC8\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xF1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x05W=`\0\x80>=`\0\xFD[PPPP\x7F\xB9\xEED\xDA\x07N\x91\xFD\xE0Z\xF9Q]$\xBF\n\x9C\x9A\x83E\x196\x8F\0\xEB\xFE is\x0E\xE1\x0F\x82`@Qa\x03:\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PPa\x03N`\x01`\0UV[PV[`\x02`\0T\x03a\x03\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x03\xF7\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x07\xB2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x03\xF7V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04_W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x04wW`\0\x80\xFD[a\x04\x80\x83a\x04HV[\x91Pa\x04\x8E` \x84\x01a\x04HV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x04\xA9W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xC0W`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\x04\xD3W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\x14Wa\x05\x14a\x04\xDAV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x05+W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05FWa\x05Fa\x04\xDAV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x05nWa\x05na\x04\xDAV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x05\x87W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04_W`\0\x80\xFD[`\0a\x01@\x826\x03\x12\x15a\x05\xCEW`\0\x80\xFD[a\x05\xD6a\x04\xF0V[a\x05\xDF\x83a\x04HV[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xFCW`\0\x80\xFD[a\x06\x086\x83\x87\x01a\x05\x1AV[` \x84\x01Ra\x06\x19`@\x86\x01a\x05\xA7V[`@\x84\x01Ra\x06*``\x86\x01a\x05\xA7V[``\x84\x01Ra\x06;`\x80\x86\x01a\x04HV[`\x80\x84\x01Ra\x06L`\xA0\x86\x01a\x04HV[`\xA0\x84\x01R`\xC0\x85\x015`\xC0\x84\x01R`\xE0\x85\x015\x91P\x80\x82\x11\x15a\x06oW`\0\x80\xFD[Pa\x06|6\x82\x86\x01a\x05\x1AV[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0` \x82\x84\x03\x12\x15a\x06\xB1W`\0\x80\xFD[a\x04\xD3\x82a\x04HV[\x80\x82\x01\x80\x82\x11\x15a\x06\xDBWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x06\xF8W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\x13W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x07(W`\0\x80\xFD[\x92P\x92\x90PV[\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x91\x82\x01Q\x82\x84\x01R\x81\x88\x01Q`@\x80\x85\x01\x91\x90\x91R\x88\x01Q``\x84\x01R\x86Q\x81\x16`\x80\x84\x01R\x90\x86\x01Q`\xA0\x83\x01R\x84\x16`\xC0\x82\x01Ra\x01\0`\xE0\x82\x01\x81\x90R\x81\x01\x82\x90R`\0a\x01 \x83\x85\x82\x85\x017`\0\x83\x85\x01\x82\x01R`\x1F\x90\x93\x01`\x1F\x19\x16\x90\x91\x01\x90\x91\x01\x95\x94PPPPPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Q`\0[\x81\x81\x10\x15a\x08(W` \x81\x88\x01\x81\x01Q`d\x86\x84\x01\x01R\x01a\x08\x0BV[P\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xF6\xDC\xBAL\xCD\xCD\xCF\x9A[\x1D\x01s\x0E\xD1\xE2x\x85\x95\xD7\xE5\xC3o5$\x8F\xCC\x8DDZb\xA7\xB9dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static ESCROW_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80cj\xFD\xD8P\x14a\0QW\x80c\x86\\\xB8\xC2\x14a\0\x95W\x80c\x8D\0\xE1N\x14a\0\xA8W\x80c\xA1\xF0\n\xFC\x14a\0\xE1W[`\0\x80\xFD[a\0x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0x\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xD3a\0\xB66`\x04a\x04dV[`\x02` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\0\x8CV[a\0\xF4a\0\xEF6`\x04a\x04\x97V[a\0\xF6V[\0[a\0\xFEa\x03QV[`\0a\x01\x11a\x01\x0C\x83a\x05\xBBV[a\x03\xAEV[\x90P`\xC0\x82\x015`\x02`\0a\x01)` \x86\x01\x86a\x06\x9FV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x01U`\xA0\x86\x01`\x80\x87\x01a\x06\x9FV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x01\x84\x91\x90a\x06\xBAV[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c0\xF2\x8Bz`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80\x87`\x80\x01` \x81\x01\x90a\x01\xE3\x91\x90a\x06\x9FV[`\x01`\x01`\xA0\x1B\x03\x16\x81R`\xC0\x88\x015` \x91\x82\x01\x81\x90R\x91\x83Ra\x01\0\x88\x015\x83\x82\x01Ra\x01 \x88\x015`@\x93\x84\x01R\x82Q\x80\x84\x01\x90\x93R0\x83R\x82\x81\x01\x91\x90\x91Ra\x022\x90\x87\x01\x87a\x06\x9FV[a\x02?`\xE0\x88\x01\x88a\x06\xE1V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02_\x95\x94\x93\x92\x91\x90a\x07/V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02yW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x8DW=`\0\x80>=`\0\xFD[PPPP`\0a\x02\xAA`@Q\x80` \x01`@R\x80\x84\x81RPa\x04\x14V[`\x01T`@Qc\x1A\xCDdY`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD6k\"\xC8\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xF1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x05W=`\0\x80>=`\0\xFD[PPPP\x7F\xB9\xEED\xDA\x07N\x91\xFD\xE0Z\xF9Q]$\xBF\n\x9C\x9A\x83E\x196\x8F\0\xEB\xFE is\x0E\xE1\x0F\x82`@Qa\x03:\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PPa\x03N`\x01`\0UV[PV[`\x02`\0T\x03a\x03\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x03\xF7\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x07\xB2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x03\xF7V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04_W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x04wW`\0\x80\xFD[a\x04\x80\x83a\x04HV[\x91Pa\x04\x8E` \x84\x01a\x04HV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x04\xA9W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xC0W`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\x04\xD3W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\x14Wa\x05\x14a\x04\xDAV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x05+W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05FWa\x05Fa\x04\xDAV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x05nWa\x05na\x04\xDAV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x05\x87W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04_W`\0\x80\xFD[`\0a\x01@\x826\x03\x12\x15a\x05\xCEW`\0\x80\xFD[a\x05\xD6a\x04\xF0V[a\x05\xDF\x83a\x04HV[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xFCW`\0\x80\xFD[a\x06\x086\x83\x87\x01a\x05\x1AV[` \x84\x01Ra\x06\x19`@\x86\x01a\x05\xA7V[`@\x84\x01Ra\x06*``\x86\x01a\x05\xA7V[``\x84\x01Ra\x06;`\x80\x86\x01a\x04HV[`\x80\x84\x01Ra\x06L`\xA0\x86\x01a\x04HV[`\xA0\x84\x01R`\xC0\x85\x015`\xC0\x84\x01R`\xE0\x85\x015\x91P\x80\x82\x11\x15a\x06oW`\0\x80\xFD[Pa\x06|6\x82\x86\x01a\x05\x1AV[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0` \x82\x84\x03\x12\x15a\x06\xB1W`\0\x80\xFD[a\x04\xD3\x82a\x04HV[\x80\x82\x01\x80\x82\x11\x15a\x06\xDBWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x06\xF8W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\x13W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x07(W`\0\x80\xFD[\x92P\x92\x90PV[\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x91\x82\x01Q\x82\x84\x01R\x81\x88\x01Q`@\x80\x85\x01\x91\x90\x91R\x88\x01Q``\x84\x01R\x86Q\x81\x16`\x80\x84\x01R\x90\x86\x01Q`\xA0\x83\x01R\x84\x16`\xC0\x82\x01Ra\x01\0`\xE0\x82\x01\x81\x90R\x81\x01\x82\x90R`\0a\x01 \x83\x85\x82\x85\x017`\0\x83\x85\x01\x82\x01R`\x1F\x90\x93\x01`\x1F\x19\x16\x90\x91\x01\x90\x91\x01\x95\x94PPPPPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Q`\0[\x81\x81\x10\x15a\x08(W` \x81\x88\x01\x81\x01Q`d\x86\x84\x01\x01R\x01a\x08\x0BV[P\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xF6\xDC\xBAL\xCD\xCD\xCF\x9A[\x1D\x01s\x0E\xD1\xE2x\x85\x95\xD7\xE5\xC3o5$\x8F\xCC\x8DDZb\xA7\xB9dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static ESCROW_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Escrow<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Escrow<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Escrow<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Escrow<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Escrow<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Escrow)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Escrow<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ESCROW_ABI.clone(),
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
                ESCROW_ABI.clone(),
                ESCROW_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `PERMIT2` (0x6afdd850) function
        pub fn permit2(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([106, 253, 216, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intentEventProver` (0x865cb8c2) function
        pub fn intent_event_prover(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([134, 92, 184, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lockTokens` (0xa1f00afc) function
        pub fn lock_tokens(
            &self,
            intent: SwapIntent,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 240, 10, 252], (intent,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenBalancesByUser` (0x8d00e14e) function
        pub fn token_balances_by_user(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([141, 0, 225, 78], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `TokensLocked` event
        pub fn tokens_locked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokensLockedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokensLockedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Escrow<M> {
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
    #[ethevent(name = "TokensLocked", abi = "TokensLocked(bytes32)")]
    pub struct TokensLockedFilter {
        pub intent_id: [u8; 32],
    }
    ///Container type for all input parameters for the `PERMIT2` function with signature `PERMIT2()` and selector `0x6afdd850`
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
    #[ethcall(name = "PERMIT2", abi = "PERMIT2()")]
    pub struct Permit2Call;
    ///Container type for all input parameters for the `intentEventProver` function with signature `intentEventProver()` and selector `0x865cb8c2`
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
    #[ethcall(name = "intentEventProver", abi = "intentEventProver()")]
    pub struct IntentEventProverCall;
    ///Container type for all input parameters for the `lockTokens` function with signature `lockTokens((address,bytes,uint32,uint32,address,address,uint256,bytes,uint256,uint256))` and selector `0xa1f00afc`
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
        name = "lockTokens",
        abi = "lockTokens((address,bytes,uint32,uint32,address,address,uint256,bytes,uint256,uint256))"
    )]
    pub struct LockTokensCall {
        pub intent: SwapIntent,
    }
    ///Container type for all input parameters for the `tokenBalancesByUser` function with signature `tokenBalancesByUser(address,address)` and selector `0x8d00e14e`
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
        name = "tokenBalancesByUser",
        abi = "tokenBalancesByUser(address,address)"
    )]
    pub struct TokenBalancesByUserCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
    pub enum EscrowCalls {
        Permit2(Permit2Call),
        IntentEventProver(IntentEventProverCall),
        LockTokens(LockTokensCall),
        TokenBalancesByUser(TokenBalancesByUserCall),
    }
    impl ::ethers::core::abi::AbiDecode for EscrowCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <Permit2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit2(decoded));
            }
            if let Ok(decoded) = <IntentEventProverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntentEventProver(decoded));
            }
            if let Ok(decoded) = <LockTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LockTokens(decoded));
            }
            if let Ok(decoded) = <TokenBalancesByUserCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenBalancesByUser(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EscrowCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Permit2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IntentEventProver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LockTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenBalancesByUser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for EscrowCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Permit2(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentEventProver(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenBalancesByUser(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<Permit2Call> for EscrowCalls {
        fn from(value: Permit2Call) -> Self {
            Self::Permit2(value)
        }
    }
    impl ::core::convert::From<IntentEventProverCall> for EscrowCalls {
        fn from(value: IntentEventProverCall) -> Self {
            Self::IntentEventProver(value)
        }
    }
    impl ::core::convert::From<LockTokensCall> for EscrowCalls {
        fn from(value: LockTokensCall) -> Self {
            Self::LockTokens(value)
        }
    }
    impl ::core::convert::From<TokenBalancesByUserCall> for EscrowCalls {
        fn from(value: TokenBalancesByUserCall) -> Self {
            Self::TokenBalancesByUser(value)
        }
    }
    ///Container type for all return fields from the `PERMIT2` function with signature `PERMIT2()` and selector `0x6afdd850`
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
    pub struct Permit2Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `intentEventProver` function with signature `intentEventProver()` and selector `0x865cb8c2`
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
    pub struct IntentEventProverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenBalancesByUser` function with signature `tokenBalancesByUser(address,address)` and selector `0x8d00e14e`
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
    pub struct TokenBalancesByUserReturn(pub ::ethers::core::types::U256);
}
