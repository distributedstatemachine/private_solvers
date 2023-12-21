pub use default_request_processor::*;
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
pub mod default_request_processor {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("processRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("processRequest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_origin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("MessageProcessed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MessageProcessed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("origin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
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
                                    name: ::std::borrow::ToOwned::to_owned("destination"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
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
                        "BaseDiamondFacet__nonReentrant_reentrantCall",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BaseDiamondFacet__nonReentrant_reentrantCall",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidHyperlaneAdapter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidHyperlaneAdapter",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSender"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DEFAULTREQUESTPROCESSOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x06\xA0\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xC2\xA1\xCB\x9D\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x03\x11V[a\0EV[\0[`\x02`\0T\x03a\0hW`@QcV\xA7\xF9\x91`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0\x81\x90UT`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\0\x99W`@Qc3f_\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\0\xA3\x84\x84a\x01\x9AV[`\0\x80```\0``a\0\xB5\x87a\x01\xE2V[`@Q\x94\x99P\x92\x97P\x90\x95P\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x86\x90\x7Fj\xEC\xE0\xB4\xD65\x1C\xB4'\xA1\x86\xCA9\xF8=\xFE\xD4H\xC1\x88S\n\xFE\xFCe\xE5<<\xCB\xF1x\xAB\x90a\x01\x03\x90\x87\x90F\x90\x88\x90a\x03\xF3V[`@Q\x80\x91\x03\x90\xA3a\x01\x15\x83\x83a\x02\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x01\x8AW`@Qce\x82\x83\x85`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xCB\x05\x07\n\x90a\x01W\x90\x88\x90\x88\x90\x88\x90\x87\x90`\x04\x01a\x04JV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01qW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x85W=`\0\x80>=`\0\xFD[PPPP[PP`\x01`\0UPPPPPPPV[`\x05T\x82\x14\x80\x15a\x01\xB9WP`\x04T`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x14\x15[\x15a\x01\xDEW`@Qc\x87\xED\x0F\x0F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01`@Q\x80\x91\x03\x90\xFD[PPV[`\0\x80```\0``\x85\x80` \x01\x90Q\x81\x01\x90a\x01\xFF\x91\x90a\x05\x1CV[\x93\x9A\x92\x99P\x90\x97P\x95P\x90\x93P\x91PPV[`\x03T`@Qc3\n'g`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c3\n'g\x90a\x02C\x90\x84\x90\x86\x90`\x04\x01a\x06FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02]W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02qW=`\0\x80>=`\0\xFD[PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xB2Wa\x02\xB2a\x02yV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xE1Wa\x02\xE1a\x02yV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\x03Wa\x03\x03a\x02yV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x03&W`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03KW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x03\\W`\0\x80\xFD[\x805a\x03oa\x03j\x82a\x02\xE9V[a\x02\xB8V[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\x03\x84W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x03\xE8W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x88R\x83\x01Q\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x03\xB8V[P\x94\x95\x94PPPPPV[``\x81R`\0a\x04\x06``\x83\x01\x86a\x03\xA4V[` \x83\x01\x94\x90\x94RP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`@\x90\x91\x01R\x91\x90PV[`\0[\x83\x81\x10\x15a\x04AW\x81\x81\x01Q\x83\x82\x01R` \x01a\x04)V[PP`\0\x91\x01RV[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`\x80`@\x82\x01\x81\x90R`\0\x90a\x04t\x90\x83\x01\x85a\x03\xA4V[\x82\x81\x03``\x84\x01R\x83Q\x80\x82Ra\x04\x92\x81` \x84\x01` \x88\x01a\x04&V[`\x1F\x01`\x1F\x19\x16\x01` \x01\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xBCW`\0\x80\xFD[PV[\x80Qa\x04\xCA\x81a\x04\xA7V[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x04\xE0W`\0\x80\xFD[\x81Qa\x04\xEEa\x03j\x82a\x02\xE9V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x05\x03W`\0\x80\xFD[a\x05\x14\x82` \x83\x01` \x87\x01a\x04&V[\x94\x93PPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x054W`\0\x80\xFD[\x85Q\x94P` \x80\x87\x01Qa\x05G\x81a\x04\xA7V[\x80\x95PP`@\x80\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05gW`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x05{W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x05\x8DWa\x05\x8Da\x02yV[a\x05\x9B\x85\x82`\x05\x1B\x01a\x02\xB8V[\x81\x81R`\x06\x91\x90\x91\x1B\x83\x01\x85\x01\x90\x85\x81\x01\x90\x8D\x83\x11\x15a\x05\xBAW`\0\x80\xFD[\x93\x86\x01\x93[\x82\x85\x10\x15a\x06\x03W\x85\x85\x8F\x03\x12\x15a\x05\xD7W`\0\x80\x81\xFD[a\x05\xDFa\x02\x8FV[\x85Qa\x05\xEA\x81a\x04\xA7V[\x81R\x85\x88\x01Q\x88\x82\x01R\x82R\x93\x85\x01\x93\x90\x86\x01\x90a\x05\xBFV[\x98Pa\x06\x14\x91PP``\x8B\x01a\x04\xBFV[\x95P`\x80\x8A\x01Q\x93P\x80\x84\x11\x15a\x06*W`\0\x80\xFD[PPPa\x069\x88\x82\x89\x01a\x04\xCFV[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x05\x14\x90\x83\x01\x84a\x03\xA4V\xFE\xA2dipfsX\"\x12 G\xE3} \n\x12\x91\x9F\xD5!\x0E\xA3\xED\xE4\x11\x12\xC8\xC8\xFF8\\a\x8A\x8F\xF4\x1B\x0B\xD9{\x05\x08\xC3dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static DEFAULTREQUESTPROCESSOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xC2\xA1\xCB\x9D\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x03\x11V[a\0EV[\0[`\x02`\0T\x03a\0hW`@QcV\xA7\xF9\x91`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0\x81\x90UT`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\0\x99W`@Qc3f_\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\0\xA3\x84\x84a\x01\x9AV[`\0\x80```\0``a\0\xB5\x87a\x01\xE2V[`@Q\x94\x99P\x92\x97P\x90\x95P\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x86\x90\x7Fj\xEC\xE0\xB4\xD65\x1C\xB4'\xA1\x86\xCA9\xF8=\xFE\xD4H\xC1\x88S\n\xFE\xFCe\xE5<<\xCB\xF1x\xAB\x90a\x01\x03\x90\x87\x90F\x90\x88\x90a\x03\xF3V[`@Q\x80\x91\x03\x90\xA3a\x01\x15\x83\x83a\x02\x11V[`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x01\x8AW`@Qce\x82\x83\x85`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xCB\x05\x07\n\x90a\x01W\x90\x88\x90\x88\x90\x88\x90\x87\x90`\x04\x01a\x04JV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01qW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x85W=`\0\x80>=`\0\xFD[PPPP[PP`\x01`\0UPPPPPPPV[`\x05T\x82\x14\x80\x15a\x01\xB9WP`\x04T`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x14\x15[\x15a\x01\xDEW`@Qc\x87\xED\x0F\x0F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01`@Q\x80\x91\x03\x90\xFD[PPV[`\0\x80```\0``\x85\x80` \x01\x90Q\x81\x01\x90a\x01\xFF\x91\x90a\x05\x1CV[\x93\x9A\x92\x99P\x90\x97P\x95P\x90\x93P\x91PPV[`\x03T`@Qc3\n'g`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c3\n'g\x90a\x02C\x90\x84\x90\x86\x90`\x04\x01a\x06FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02]W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02qW=`\0\x80>=`\0\xFD[PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xB2Wa\x02\xB2a\x02yV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\xE1Wa\x02\xE1a\x02yV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\x03Wa\x03\x03a\x02yV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x03&W`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03KW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x03\\W`\0\x80\xFD[\x805a\x03oa\x03j\x82a\x02\xE9V[a\x02\xB8V[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\x03\x84W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x03\xE8W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x88R\x83\x01Q\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x03\xB8V[P\x94\x95\x94PPPPPV[``\x81R`\0a\x04\x06``\x83\x01\x86a\x03\xA4V[` \x83\x01\x94\x90\x94RP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`@\x90\x91\x01R\x91\x90PV[`\0[\x83\x81\x10\x15a\x04AW\x81\x81\x01Q\x83\x82\x01R` \x01a\x04)V[PP`\0\x91\x01RV[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`\x80`@\x82\x01\x81\x90R`\0\x90a\x04t\x90\x83\x01\x85a\x03\xA4V[\x82\x81\x03``\x84\x01R\x83Q\x80\x82Ra\x04\x92\x81` \x84\x01` \x88\x01a\x04&V[`\x1F\x01`\x1F\x19\x16\x01` \x01\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xBCW`\0\x80\xFD[PV[\x80Qa\x04\xCA\x81a\x04\xA7V[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x04\xE0W`\0\x80\xFD[\x81Qa\x04\xEEa\x03j\x82a\x02\xE9V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x05\x03W`\0\x80\xFD[a\x05\x14\x82` \x83\x01` \x87\x01a\x04&V[\x94\x93PPPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x054W`\0\x80\xFD[\x85Q\x94P` \x80\x87\x01Qa\x05G\x81a\x04\xA7V[\x80\x95PP`@\x80\x88\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05gW`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x05{W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x05\x8DWa\x05\x8Da\x02yV[a\x05\x9B\x85\x82`\x05\x1B\x01a\x02\xB8V[\x81\x81R`\x06\x91\x90\x91\x1B\x83\x01\x85\x01\x90\x85\x81\x01\x90\x8D\x83\x11\x15a\x05\xBAW`\0\x80\xFD[\x93\x86\x01\x93[\x82\x85\x10\x15a\x06\x03W\x85\x85\x8F\x03\x12\x15a\x05\xD7W`\0\x80\x81\xFD[a\x05\xDFa\x02\x8FV[\x85Qa\x05\xEA\x81a\x04\xA7V[\x81R\x85\x88\x01Q\x88\x82\x01R\x82R\x93\x85\x01\x93\x90\x86\x01\x90a\x05\xBFV[\x98Pa\x06\x14\x91PP``\x8B\x01a\x04\xBFV[\x95P`\x80\x8A\x01Q\x93P\x80\x84\x11\x15a\x06*W`\0\x80\xFD[PPPa\x069\x88\x82\x89\x01a\x04\xCFV[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x05\x14\x90\x83\x01\x84a\x03\xA4V\xFE\xA2dipfsX\"\x12 G\xE3} \n\x12\x91\x9F\xD5!\x0E\xA3\xED\xE4\x11\x12\xC8\xC8\xFF8\\a\x8A\x8F\xF4\x1B\x0B\xD9{\x05\x08\xC3dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static DEFAULTREQUESTPROCESSOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DefaultRequestProcessor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DefaultRequestProcessor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DefaultRequestProcessor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DefaultRequestProcessor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DefaultRequestProcessor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DefaultRequestProcessor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DefaultRequestProcessor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEFAULTREQUESTPROCESSOR_ABI.clone(),
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
                DEFAULTREQUESTPROCESSOR_ABI.clone(),
                DEFAULTREQUESTPROCESSOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `processRequest` (0xc2a1cb9d) function
        pub fn process_request(
            &self,
            origin: ::ethers::core::types::U256,
            sender: [u8; 32],
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 161, 203, 157], (origin, sender, message))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `MessageProcessed` event
        pub fn message_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MessageProcessedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MessageProcessedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DefaultRequestProcessor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BaseDiamondFacet__nonReentrant_reentrantCall` with signature `BaseDiamondFacet__nonReentrant_reentrantCall()` and selector `0x56a7f991`
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
        name = "BaseDiamondFacet__nonReentrant_reentrantCall",
        abi = "BaseDiamondFacet__nonReentrant_reentrantCall()"
    )]
    pub struct BaseDiamondFacet__nonReentrant_reentrantCall;
    ///Custom Error type `InvalidHyperlaneAdapter` with signature `InvalidHyperlaneAdapter()` and selector `0x33665f17`
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
    #[etherror(name = "InvalidHyperlaneAdapter", abi = "InvalidHyperlaneAdapter()")]
    pub struct InvalidHyperlaneAdapter;
    ///Custom Error type `InvalidSender` with signature `InvalidSender(bytes32)` and selector `0x87ed0f0f`
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
    #[etherror(name = "InvalidSender", abi = "InvalidSender(bytes32)")]
    pub struct InvalidSender {
        pub sender: [u8; 32],
    }
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
    pub enum DefaultRequestProcessorErrors {
        BaseDiamondFacet__nonReentrant_reentrantCall(
            BaseDiamondFacet__nonReentrant_reentrantCall,
        ),
        InvalidHyperlaneAdapter(InvalidHyperlaneAdapter),
        InvalidSender(InvalidSender),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DefaultRequestProcessorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <BaseDiamondFacet__nonReentrant_reentrantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseDiamondFacet__nonReentrant_reentrantCall(decoded));
            }
            if let Ok(decoded) = <InvalidHyperlaneAdapter as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidHyperlaneAdapter(decoded));
            }
            if let Ok(decoded) = <InvalidSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSender(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DefaultRequestProcessorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BaseDiamondFacet__nonReentrant_reentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidHyperlaneAdapter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DefaultRequestProcessorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BaseDiamondFacet__nonReentrant_reentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidHyperlaneAdapter as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DefaultRequestProcessorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BaseDiamondFacet__nonReentrant_reentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidHyperlaneAdapter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DefaultRequestProcessorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BaseDiamondFacet__nonReentrant_reentrantCall>
    for DefaultRequestProcessorErrors {
        fn from(value: BaseDiamondFacet__nonReentrant_reentrantCall) -> Self {
            Self::BaseDiamondFacet__nonReentrant_reentrantCall(value)
        }
    }
    impl ::core::convert::From<InvalidHyperlaneAdapter>
    for DefaultRequestProcessorErrors {
        fn from(value: InvalidHyperlaneAdapter) -> Self {
            Self::InvalidHyperlaneAdapter(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for DefaultRequestProcessorErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
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
        name = "MessageProcessed",
        abi = "MessageProcessed(uint256,address,(address,uint256)[],uint256,address)"
    )]
    pub struct MessageProcessedFilter {
        #[ethevent(indexed)]
        pub origin: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<Token>,
        pub destination: ::ethers::core::types::U256,
        pub target: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `processRequest` function with signature `processRequest(uint256,bytes32,bytes)` and selector `0xc2a1cb9d`
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
    #[ethcall(name = "processRequest", abi = "processRequest(uint256,bytes32,bytes)")]
    pub struct ProcessRequestCall {
        pub origin: ::ethers::core::types::U256,
        pub sender: [u8; 32],
        pub message: ::ethers::core::types::Bytes,
    }
}
