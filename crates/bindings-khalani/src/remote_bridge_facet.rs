pub use remote_bridge_facet::*;
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
pub mod remote_bridge_facet {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getAssetReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAssetReserves"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("send"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("send"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationChainId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approvedTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Token[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "interchainLiquidityHubPayload",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "isSwapWithAggregateToken",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("RemoteBridgeRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RemoteBridgeRequest",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationChainId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("approvedTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned(
                        "BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroTargetAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ZeroTargetAddress"),
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
    pub static REMOTEBRIDGEFACET_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x07\x01\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0)W`\x005`\xE0\x1C\x80c\x17\xECA\x99\x14a\0.W\x80c\x92\xE7\x1CT\x14a\0ZW[`\0\x80\xFD[4\x80\x15a\0:W`\0\x80\xFD[P`\x03T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0ma\0h6`\x04a\x03\xA2V[a\0oV[\0[`\x02`\x01T\x03a\0\x92W`@QcQ\xE6\x97}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\x01`\x01`\xA0\x1B\x03\x83\x16a\0\xBEW`@Qc\xEA\xF6s\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x873`\x01`\x01`\xA0\x1B\x03\x16\x7Fd\xF9\xF4\xCD2!\xB0}\x13o\xA0\x06\t\xA84\x8D\x13\xCB\xBD\xA6?\xC5\x18k\xD5\xDA\x02\x80\xF9\x940w\x89\x86`@Qa\0\xFA\x92\x91\x90a\x05eV[`@Q\x80\x91\x03\x90\xA3`\x03T`@Qc$\xA5\x14\x8F`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x92\x94R<\x90a\x014\x903\x90\x8B\x90`\x04\x01a\x05\x8FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01NW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01bW=`\0\x80>=`\0\xFD[PP`\x02T`\x05T`\x04T`\0\x94P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x93Pc\x01B\x8A\x07\x92\x16a\x01\x95\x8D\x8D\x8D\x8D\x8D\x8D\x8D\x8Da\x02}V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xB3\x93\x92\x91\x90a\x05\xBBV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF6\x91\x90a\x06\x18V[`\x02T\x89Q`@QcL\xDD\x14\x9B`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x8D\x90R`D\x81\x01\x91\x90\x91R3`d\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cL\xDD\x14\x9B\x904\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02UW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02iW=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPPPPPPPPV[``3\x89\x89\x89\x89\x89\x89\x89\x89`@Q` \x01a\x02\xA0\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x06ZV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xF6Wa\x02\xF6a\x02\xBDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03%Wa\x03%a\x02\xBDV[`@R\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03DW`\0\x80\xFD[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x03[W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03sW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x03\x8BW`\0\x80\xFD[\x92P\x92\x90PV[\x805\x80\x15\x15\x81\x14a\x03DW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15a\x03\xBEW`\0\x80\xFD[\x885\x97Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x8A\x015\x11\x15a\x03\xDBW`\0\x80\xFD[` \x89\x015\x89\x01\x8A`\x1F\x82\x01\x12a\x03\xF1W`\0\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x11\x15a\x04\nWa\x04\na\x02\xBDV[a\x04\x1A` \x825`\x05\x1B\x01a\x02\xFCV[\x815\x80\x82R` \x80\x83\x01\x92\x91`\x06\x1B\x84\x01\x01\x8D\x10\x15a\x048W`\0\x80\xFD[` \x83\x01[` \x845`\x06\x1B\x85\x01\x01\x81\x10\x15a\x04\x8BW`@\x81\x8F\x03\x12\x15a\x04^W`\0\x80\xFD[a\x04fa\x02\xD3V[a\x04o\x82a\x03-V[\x81R` \x82\x81\x015\x81\x83\x01R\x90\x84R\x92\x90\x92\x01\x91`@\x01a\x04=V[P\x98PPPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x8A\x015\x11\x15a\x04\xA9W`\0\x80\xFD[a\x04\xB9\x8A`@\x8B\x015\x8B\x01a\x03IV[\x90\x96P\x94Pa\x04\xCA``\x8A\x01a\x03\x92V[\x93Pa\x04\xD8`\x80\x8A\x01a\x03-V[\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x8A\x015\x11\x15a\x04\xF3W`\0\x80\xFD[a\x05\x03\x8A`\xA0\x8B\x015\x8B\x01a\x03IV[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x91\x94\x93PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x05ZW\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x88R\x83\x01Q\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05*V[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x05x`@\x83\x01\x85a\x05\x16V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x05\xB3\x90\x83\x01\x84a\x05\x16V[\x94\x93PPPPV[\x83\x81R`\0` \x84\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x05\xF5W\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x05\xD9V[P`\0`\x80\x82\x86\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x06*W`\0\x80\xFD[PQ\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0`\x01\x80`\xA0\x1B\x03\x80\x8C\x16\x83R\x8A` \x84\x01R`\xE0`@\x84\x01Ra\x06\x82`\xE0\x84\x01\x8Ba\x05\x16V[\x83\x81\x03``\x85\x01Ra\x06\x95\x81\x8A\x8Ca\x061V[\x90P\x87\x15\x15`\x80\x85\x01R\x81\x87\x16`\xA0\x85\x01R\x83\x81\x03`\xC0\x85\x01Ra\x06\xBA\x81\x86\x88a\x061V[\x9D\x9CPPPPPPPPPPPPPV\xFE\xA2dipfsX\"\x12 q\xAD\x86sS\x08\x8E\xC1\x97\x8D\x11\x1D\xA6\x01E\xB4Z\xB9\x01\xDF\x18\x0F#\xF8\xADAz~8\"jodsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static REMOTEBRIDGEFACET_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0)W`\x005`\xE0\x1C\x80c\x17\xECA\x99\x14a\0.W\x80c\x92\xE7\x1CT\x14a\0ZW[`\0\x80\xFD[4\x80\x15a\0:W`\0\x80\xFD[P`\x03T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[a\0ma\0h6`\x04a\x03\xA2V[a\0oV[\0[`\x02`\x01T\x03a\0\x92W`@QcQ\xE6\x97}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\x01`\x01`\xA0\x1B\x03\x83\x16a\0\xBEW`@Qc\xEA\xF6s\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x873`\x01`\x01`\xA0\x1B\x03\x16\x7Fd\xF9\xF4\xCD2!\xB0}\x13o\xA0\x06\t\xA84\x8D\x13\xCB\xBD\xA6?\xC5\x18k\xD5\xDA\x02\x80\xF9\x940w\x89\x86`@Qa\0\xFA\x92\x91\x90a\x05eV[`@Q\x80\x91\x03\x90\xA3`\x03T`@Qc$\xA5\x14\x8F`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x92\x94R<\x90a\x014\x903\x90\x8B\x90`\x04\x01a\x05\x8FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01NW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01bW=`\0\x80>=`\0\xFD[PP`\x02T`\x05T`\x04T`\0\x94P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x93Pc\x01B\x8A\x07\x92\x16a\x01\x95\x8D\x8D\x8D\x8D\x8D\x8D\x8D\x8Da\x02}V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xB3\x93\x92\x91\x90a\x05\xBBV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xF6\x91\x90a\x06\x18V[`\x02T\x89Q`@QcL\xDD\x14\x9B`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x8D\x90R`D\x81\x01\x91\x90\x91R3`d\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cL\xDD\x14\x9B\x904\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02UW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02iW=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPPPPPPPPV[``3\x89\x89\x89\x89\x89\x89\x89\x89`@Q` \x01a\x02\xA0\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x06ZV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x98\x97PPPPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xF6Wa\x02\xF6a\x02\xBDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03%Wa\x03%a\x02\xBDV[`@R\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03DW`\0\x80\xFD[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x03[W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03sW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x03\x8BW`\0\x80\xFD[\x92P\x92\x90PV[\x805\x80\x15\x15\x81\x14a\x03DW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15a\x03\xBEW`\0\x80\xFD[\x885\x97Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x8A\x015\x11\x15a\x03\xDBW`\0\x80\xFD[` \x89\x015\x89\x01\x8A`\x1F\x82\x01\x12a\x03\xF1W`\0\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x11\x15a\x04\nWa\x04\na\x02\xBDV[a\x04\x1A` \x825`\x05\x1B\x01a\x02\xFCV[\x815\x80\x82R` \x80\x83\x01\x92\x91`\x06\x1B\x84\x01\x01\x8D\x10\x15a\x048W`\0\x80\xFD[` \x83\x01[` \x845`\x06\x1B\x85\x01\x01\x81\x10\x15a\x04\x8BW`@\x81\x8F\x03\x12\x15a\x04^W`\0\x80\xFD[a\x04fa\x02\xD3V[a\x04o\x82a\x03-V[\x81R` \x82\x81\x015\x81\x83\x01R\x90\x84R\x92\x90\x92\x01\x91`@\x01a\x04=V[P\x98PPPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x8A\x015\x11\x15a\x04\xA9W`\0\x80\xFD[a\x04\xB9\x8A`@\x8B\x015\x8B\x01a\x03IV[\x90\x96P\x94Pa\x04\xCA``\x8A\x01a\x03\x92V[\x93Pa\x04\xD8`\x80\x8A\x01a\x03-V[\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x8A\x015\x11\x15a\x04\xF3W`\0\x80\xFD[a\x05\x03\x8A`\xA0\x8B\x015\x8B\x01a\x03IV[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x91\x94\x93PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x05ZW\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x88R\x83\x01Q\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05*V[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x05x`@\x83\x01\x85a\x05\x16V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x05\xB3\x90\x83\x01\x84a\x05\x16V[\x94\x93PPPPV[\x83\x81R`\0` \x84\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x05\xF5W\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x05\xD9V[P`\0`\x80\x82\x86\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x06*W`\0\x80\xFD[PQ\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0`\x01\x80`\xA0\x1B\x03\x80\x8C\x16\x83R\x8A` \x84\x01R`\xE0`@\x84\x01Ra\x06\x82`\xE0\x84\x01\x8Ba\x05\x16V[\x83\x81\x03``\x85\x01Ra\x06\x95\x81\x8A\x8Ca\x061V[\x90P\x87\x15\x15`\x80\x85\x01R\x81\x87\x16`\xA0\x85\x01R\x83\x81\x03`\xC0\x85\x01Ra\x06\xBA\x81\x86\x88a\x061V[\x9D\x9CPPPPPPPPPPPPPV\xFE\xA2dipfsX\"\x12 q\xAD\x86sS\x08\x8E\xC1\x97\x8D\x11\x1D\xA6\x01E\xB4Z\xB9\x01\xDF\x18\x0F#\xF8\xADAz~8\"jodsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static REMOTEBRIDGEFACET_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RemoteBridgeFacet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RemoteBridgeFacet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RemoteBridgeFacet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RemoteBridgeFacet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RemoteBridgeFacet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RemoteBridgeFacet))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RemoteBridgeFacet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    REMOTEBRIDGEFACET_ABI.clone(),
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
                REMOTEBRIDGEFACET_ABI.clone(),
                REMOTEBRIDGEFACET_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getAssetReserves` (0x17ec4199) function
        pub fn get_asset_reserves(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([23, 236, 65, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `send` (0x92e71c54) function
        pub fn send(
            &self,
            destination_chain_id: ::ethers::core::types::U256,
            approved_tokens: ::std::vec::Vec<Token>,
            interchain_liquidity_hub_payload: ::ethers::core::types::Bytes,
            is_swap_with_aggregate_token: bool,
            target: ::ethers::core::types::Address,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [146, 231, 28, 84],
                    (
                        destination_chain_id,
                        approved_tokens,
                        interchain_liquidity_hub_payload,
                        is_swap_with_aggregate_token,
                        target,
                        message,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `RemoteBridgeRequest` event
        pub fn remote_bridge_request_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoteBridgeRequestFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoteBridgeRequestFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RemoteBridgeFacet<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall` with signature `BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall()` and selector `0xa3cd2efa`
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
    #[etherror(
        name = "BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall",
        abi = "BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall()"
    )]
    pub struct BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall;
    ///Custom Error type `ZeroTargetAddress` with signature `ZeroTargetAddress()` and selector `0xeaf673fd`
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
    #[etherror(name = "ZeroTargetAddress", abi = "ZeroTargetAddress()")]
    pub struct ZeroTargetAddress;
    ///Container type for all of the contract's custom errors
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
    pub enum RemoteBridgeFacetErrors {
        BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall(
            BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall,
        ),
        ZeroTargetAddress(ZeroTargetAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for RemoteBridgeFacetErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall(decoded),
                );
            }
            if let Ok(decoded) = <ZeroTargetAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroTargetAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RemoteBridgeFacetErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroTargetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for RemoteBridgeFacetErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroTargetAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for RemoteBridgeFacetErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ZeroTargetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for RemoteBridgeFacetErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall>
    for RemoteBridgeFacetErrors {
        fn from(value: BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall) -> Self {
            Self::BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall(value)
        }
    }
    impl ::core::convert::From<ZeroTargetAddress> for RemoteBridgeFacetErrors {
        fn from(value: ZeroTargetAddress) -> Self {
            Self::ZeroTargetAddress(value)
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
        name = "RemoteBridgeRequest",
        abi = "RemoteBridgeRequest(address,uint256,(address,uint256)[],address)"
    )]
    pub struct RemoteBridgeRequestFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub destination_chain_id: ::ethers::core::types::U256,
        pub approved_tokens: ::std::vec::Vec<Token>,
        pub target: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getAssetReserves` function with signature `getAssetReserves()` and selector `0x17ec4199`
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
    #[ethcall(name = "getAssetReserves", abi = "getAssetReserves()")]
    pub struct GetAssetReservesCall;
    ///Container type for all input parameters for the `send` function with signature `send(uint256,(address,uint256)[],bytes,bool,address,bytes)` and selector `0x92e71c54`
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
        name = "send",
        abi = "send(uint256,(address,uint256)[],bytes,bool,address,bytes)"
    )]
    pub struct SendCall {
        pub destination_chain_id: ::ethers::core::types::U256,
        pub approved_tokens: ::std::vec::Vec<Token>,
        pub interchain_liquidity_hub_payload: ::ethers::core::types::Bytes,
        pub is_swap_with_aggregate_token: bool,
        pub target: ::ethers::core::types::Address,
        pub message: ::ethers::core::types::Bytes,
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
    pub enum RemoteBridgeFacetCalls {
        GetAssetReserves(GetAssetReservesCall),
        Send(SendCall),
    }
    impl ::ethers::core::abi::AbiDecode for RemoteBridgeFacetCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetAssetReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAssetReserves(decoded));
            }
            if let Ok(decoded) = <SendCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Send(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RemoteBridgeFacetCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetAssetReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Send(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for RemoteBridgeFacetCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetAssetReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::Send(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetAssetReservesCall> for RemoteBridgeFacetCalls {
        fn from(value: GetAssetReservesCall) -> Self {
            Self::GetAssetReserves(value)
        }
    }
    impl ::core::convert::From<SendCall> for RemoteBridgeFacetCalls {
        fn from(value: SendCall) -> Self {
            Self::Send(value)
        }
    }
    ///Container type for all return fields from the `getAssetReserves` function with signature `getAssetReserves()` and selector `0x17ec4199`
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
    pub struct GetAssetReservesReturn(pub ::ethers::core::types::Address);
}
