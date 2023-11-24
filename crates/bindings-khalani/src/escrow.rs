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
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x08I8\x03\x80a\x08I\x839\x81\x01`@\x81\x90Ra\0/\x91a\0EV[`\x01`\0U`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0uV[`\0` \x82\x84\x03\x12\x15a\0WW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0nW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x07\xB3a\0\x96`\09`\0\x81\x81`K\x01Ra\x01o\x01Ra\x07\xB3`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cj\xFD\xD8P\x14a\0FW\x80c\x8D\0\xE1N\x14a\0\x8AW\x80c\xA1\xF0\n\xFC\x14a\0\xC3W[`\0\x80\xFD[a\0m\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB5a\0\x986`\x04a\x03\x99V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\0\x81V[a\0\xD6a\0\xD16`\x04a\x03\xCCV[a\0\xD8V[\0[a\0\xE0a\x02\xBAV[`\0a\0\xF3a\0\xEE\x83a\x04\xF0V[a\x03\x17V[\x90P`\xC0\x82\x015`\x01`\0a\x01\x0B` \x86\x01\x86a\x05\xD4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x017`\xA0\x86\x01`\x80\x87\x01a\x05\xD4V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x01f\x91\x90a\x05\xEFV[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c0\xF2\x8Bz`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80\x87`\x80\x01` \x81\x01\x90a\x01\xC5\x91\x90a\x05\xD4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R`\xC0\x88\x015` \x91\x82\x01\x81\x90R\x91\x83Ra\x01\0\x88\x015\x83\x82\x01Ra\x01 \x88\x015`@\x93\x84\x01R\x82Q\x80\x84\x01\x90\x93R0\x83R\x82\x81\x01\x91\x90\x91Ra\x02\x14\x90\x87\x01\x87a\x05\xD4V[a\x02!`\xE0\x88\x01\x88a\x06\x16V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02A\x95\x94\x93\x92\x91\x90a\x06dV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02[W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02oW=`\0\x80>=`\0\xFD[PPPP\x7F\xB9\xEED\xDA\x07N\x91\xFD\xE0Z\xF9Q]$\xBF\n\x9C\x9A\x83E\x196\x8F\0\xEB\xFE is\x0E\xE1\x0F\x81`@Qa\x02\xA4\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1Pa\x02\xB7`\x01`\0UV[PV[`\x02`\0T\x03a\x03\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x03`\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x06\xE7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x94W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03\xACW`\0\x80\xFD[a\x03\xB5\x83a\x03}V[\x91Pa\x03\xC3` \x84\x01a\x03}V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x03\xDEW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xF5W`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\x04\x08W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04IWa\x04Ia\x04\x0FV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x04`W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04{Wa\x04{a\x04\x0FV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x04\xA3Wa\x04\xA3a\x04\x0FV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x04\xBCW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\x94W`\0\x80\xFD[`\0a\x01@\x826\x03\x12\x15a\x05\x03W`\0\x80\xFD[a\x05\x0Ba\x04%V[a\x05\x14\x83a\x03}V[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x051W`\0\x80\xFD[a\x05=6\x83\x87\x01a\x04OV[` \x84\x01Ra\x05N`@\x86\x01a\x04\xDCV[`@\x84\x01Ra\x05_``\x86\x01a\x04\xDCV[``\x84\x01Ra\x05p`\x80\x86\x01a\x03}V[`\x80\x84\x01Ra\x05\x81`\xA0\x86\x01a\x03}V[`\xA0\x84\x01R`\xC0\x85\x015`\xC0\x84\x01R`\xE0\x85\x015\x91P\x80\x82\x11\x15a\x05\xA4W`\0\x80\xFD[Pa\x05\xB16\x82\x86\x01a\x04OV[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0` \x82\x84\x03\x12\x15a\x05\xE6W`\0\x80\xFD[a\x04\x08\x82a\x03}V[\x80\x82\x01\x80\x82\x11\x15a\x06\x10WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x06-W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06HW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x06]W`\0\x80\xFD[\x92P\x92\x90PV[\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x91\x82\x01Q\x82\x84\x01R\x81\x88\x01Q`@\x80\x85\x01\x91\x90\x91R\x88\x01Q``\x84\x01R\x86Q\x81\x16`\x80\x84\x01R\x90\x86\x01Q`\xA0\x83\x01R\x84\x16`\xC0\x82\x01Ra\x01\0`\xE0\x82\x01\x81\x90R\x81\x01\x82\x90R`\0a\x01 \x83\x85\x82\x85\x017`\0\x83\x85\x01\x82\x01R`\x1F\x90\x93\x01`\x1F\x19\x16\x90\x91\x01\x90\x91\x01\x95\x94PPPPPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Q`\0[\x81\x81\x10\x15a\x07]W` \x81\x88\x01\x81\x01Q`d\x86\x84\x01\x01R\x01a\x07@V[P\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xD029\xEB#?m\xBB\x1B\x85\xAC\xC6\xF1\x9CdS\x10\xBD\x1EZ\xAB\x01%\x03_\x9C\xAD\xB8n\x85\x7F\xB0dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static ESCROW_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cj\xFD\xD8P\x14a\0FW\x80c\x8D\0\xE1N\x14a\0\x8AW\x80c\xA1\xF0\n\xFC\x14a\0\xC3W[`\0\x80\xFD[a\0m\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xB5a\0\x986`\x04a\x03\x99V[`\x01` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\0\x81V[a\0\xD6a\0\xD16`\x04a\x03\xCCV[a\0\xD8V[\0[a\0\xE0a\x02\xBAV[`\0a\0\xF3a\0\xEE\x83a\x04\xF0V[a\x03\x17V[\x90P`\xC0\x82\x015`\x01`\0a\x01\x0B` \x86\x01\x86a\x05\xD4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x017`\xA0\x86\x01`\x80\x87\x01a\x05\xD4V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x82\x82Ta\x01f\x91\x90a\x05\xEFV[\x92PP\x81\x90UP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c0\xF2\x8Bz`@Q\x80``\x01`@R\x80`@Q\x80`@\x01`@R\x80\x87`\x80\x01` \x81\x01\x90a\x01\xC5\x91\x90a\x05\xD4V[`\x01`\x01`\xA0\x1B\x03\x16\x81R`\xC0\x88\x015` \x91\x82\x01\x81\x90R\x91\x83Ra\x01\0\x88\x015\x83\x82\x01Ra\x01 \x88\x015`@\x93\x84\x01R\x82Q\x80\x84\x01\x90\x93R0\x83R\x82\x81\x01\x91\x90\x91Ra\x02\x14\x90\x87\x01\x87a\x05\xD4V[a\x02!`\xE0\x88\x01\x88a\x06\x16V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02A\x95\x94\x93\x92\x91\x90a\x06dV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02[W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02oW=`\0\x80>=`\0\xFD[PPPP\x7F\xB9\xEED\xDA\x07N\x91\xFD\xE0Z\xF9Q]$\xBF\n\x9C\x9A\x83E\x196\x8F\0\xEB\xFE is\x0E\xE1\x0F\x81`@Qa\x02\xA4\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1Pa\x02\xB7`\x01`\0UV[PV[`\x02`\0T\x03a\x03\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x03`\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x06\xE7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\x94W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x03\xACW`\0\x80\xFD[a\x03\xB5\x83a\x03}V[\x91Pa\x03\xC3` \x84\x01a\x03}V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x03\xDEW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xF5W`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a\x04\x08W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04IWa\x04Ia\x04\x0FV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x04`W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04{Wa\x04{a\x04\x0FV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x04\xA3Wa\x04\xA3a\x04\x0FV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x04\xBCW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\x94W`\0\x80\xFD[`\0a\x01@\x826\x03\x12\x15a\x05\x03W`\0\x80\xFD[a\x05\x0Ba\x04%V[a\x05\x14\x83a\x03}V[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x051W`\0\x80\xFD[a\x05=6\x83\x87\x01a\x04OV[` \x84\x01Ra\x05N`@\x86\x01a\x04\xDCV[`@\x84\x01Ra\x05_``\x86\x01a\x04\xDCV[``\x84\x01Ra\x05p`\x80\x86\x01a\x03}V[`\x80\x84\x01Ra\x05\x81`\xA0\x86\x01a\x03}V[`\xA0\x84\x01R`\xC0\x85\x015`\xC0\x84\x01R`\xE0\x85\x015\x91P\x80\x82\x11\x15a\x05\xA4W`\0\x80\xFD[Pa\x05\xB16\x82\x86\x01a\x04OV[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0` \x82\x84\x03\x12\x15a\x05\xE6W`\0\x80\xFD[a\x04\x08\x82a\x03}V[\x80\x82\x01\x80\x82\x11\x15a\x06\x10WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x06-W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06HW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x06]W`\0\x80\xFD[\x92P\x92\x90PV[\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x91\x82\x01Q\x82\x84\x01R\x81\x88\x01Q`@\x80\x85\x01\x91\x90\x91R\x88\x01Q``\x84\x01R\x86Q\x81\x16`\x80\x84\x01R\x90\x86\x01Q`\xA0\x83\x01R\x84\x16`\xC0\x82\x01Ra\x01\0`\xE0\x82\x01\x81\x90R\x81\x01\x82\x90R`\0a\x01 \x83\x85\x82\x85\x017`\0\x83\x85\x01\x82\x01R`\x1F\x90\x93\x01`\x1F\x19\x16\x90\x91\x01\x90\x91\x01\x95\x94PPPPPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Q`\0[\x81\x81\x10\x15a\x07]W` \x81\x88\x01\x81\x01Q`d\x86\x84\x01\x01R\x01a\x07@V[P\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xD029\xEB#?m\xBB\x1B\x85\xAC\xC6\xF1\x9CdS\x10\xBD\x1EZ\xAB\x01%\x03_\x9C\xAD\xB8n\x85\x7F\xB0dsolcC\0\x08\x13\x003";
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
