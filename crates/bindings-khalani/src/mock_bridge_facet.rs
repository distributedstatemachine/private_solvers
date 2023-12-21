pub use mock_bridge_facet::*;
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
pub mod mock_bridge_facet {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getLiquidityProjector"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLiquidityProjector",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("liquidityProjector"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidityProjector"),
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
                                    name: ::std::borrow::ToOwned::to_owned("rootChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rootSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Token[]"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BridgeRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BridgeRequest"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKBRIDGEFACET_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\0\x1D\x90a\0KV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\09W=`\0\x80>=`\0\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0XV[a\x04\x16\x80a\x05\x9F\x839\x01\x90V[`\x80Qa\x05 a\0\x7F`\09`\0\x81\x81`]\x01R\x81\x81`\x9E\x01Ra\x01\x1D\x01Ra\x05 `\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cr\xB9\xFF\xC0\x14a\0FW\x80c\x8B\x96\x855\x14a\0[W\x80c\xC9\r\xCF\xBB\x14a\0\x99W[`\0\x80\xFD[a\0Ya\0T6`\x04a\x02\x1BV[a\0\xC0V[\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0}\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x853`\x01`\x01`\xA0\x1B\x03\x16\x7F0\xF2o\x84\xC7u\x8Fr\x10z\xE1\x01\xD6'\xEF]z&\x88vl\xBC\xFEd6C\xC90t\x85&\x84\x87\x87\x87`@Qa\0\xFE\x93\x92\x91\x90a\x03CV[`@Q\x80\x91\x03\x90\xA3`@QcX1(\xE5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xB0bQ\xCA\x90a\x01X\x90\x89\x903\x90\x8A\x90\x8A\x90`\x04\x01a\x03oV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\x9F\x91\x90\x81\x01\x90a\x04\x14V[PPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xBFW`\0\x80\xFD[PV[\x805a\x01\xCD\x81a\x01\xAAV[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x01\xE4W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xFCW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x02\x14W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15a\x027W`\0\x80\xFD[\x885\x97P` \x89\x015a\x02I\x81a\x01\xAAV[\x96P`@\x89\x015\x95P``\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02mW`\0\x80\xFD[\x81\x8B\x01\x91P\x8B`\x1F\x83\x01\x12a\x02\x81W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\x90W`\0\x80\xFD[\x8C` \x82`\x06\x1B\x85\x01\x01\x11\x15a\x02\xA5W`\0\x80\xFD[` \x83\x01\x97P\x95Pa\x02\xB9`\x80\x8C\x01a\x01\xC2V[\x94P`\xA0\x8B\x015\x91P\x80\x82\x11\x15a\x02\xCFW`\0\x80\xFD[Pa\x02\xDC\x8B\x82\x8C\x01a\x01\xD2V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a\x038W\x815a\x03\x13\x81a\x01\xAAV[`\x01`\x01`\xA0\x1B\x03\x16\x87R\x81\x83\x015\x83\x88\x01R`@\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01a\x03\0V[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x03W`@\x83\x01\x85\x87a\x02\xF0V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x94\x93PPPPV[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x03\x9A\x90\x83\x01\x84\x86a\x02\xF0V[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xDDWa\x03\xDDa\x03\xA4V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\x0CWa\x04\x0Ca\x03\xA4V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x04'W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04?W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04SW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x04eWa\x04ea\x03\xA4V[a\x04s\x84\x82`\x05\x1B\x01a\x03\xE3V[\x81\x81R\x84\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\x04\x93W`\0\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\x04\xDFW`@\x84\x89\x03\x12\x15a\x04\xB1W`\0\x80\x81\xFD[a\x04\xB9a\x03\xBAV[\x84Qa\x04\xC4\x81a\x01\xAAV[\x81R\x84\x86\x01Q\x86\x82\x01R\x83R`@\x90\x93\x01\x92\x91\x84\x01\x91a\x04\x98V[\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xC3\xC1\xB9\xC3<w\x93\xEFc7\xA9\x8C\x7F\xED\x8E\xAC\x01k?>\x8C\xFDcf\xF3\xC2\xAB\x9By\xF35\xFDdsolcC\0\x08\x13\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\xF6\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xB0bQ\xCA\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x02\x13V[a\0YV[`@Qa\0P\x91\x90a\x03\x02V[`@Q\x80\x91\x03\x90\xF3[```\0[\x82Q\x81\x10\x15a\x01@W\x82\x81\x81Q\x81\x10a\0yWa\0ya\x03ZV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD\x850\x86\x85\x81Q\x81\x10a\0\xA7Wa\0\xA7a\x03ZV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01-\x91\x90a\x03pV[P\x80a\x018\x81a\x03\x99V[\x91PPa\0^V[P`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R\x90a\x01~V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01WW\x90P[P\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x9EW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\xDCWa\x01\xDCa\x01\xA3V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\x0BWa\x02\x0Ba\x01\xA3V[`@R\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x02(W`\0\x80\xFD[\x835\x92P` a\x029\x81\x86\x01a\x01\x87V[\x92P`@\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02WW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x02kW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02}Wa\x02}a\x01\xA3V[a\x02\x8B\x85\x82`\x05\x1B\x01a\x01\xE2V[\x81\x81R\x85\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x85\x01\x90\x8A\x82\x11\x15a\x02\xABW`\0\x80\xFD[\x92\x85\x01\x92[\x81\x84\x10\x15a\x02\xF2W\x84\x84\x8C\x03\x12\x15a\x02\xC8W`\0\x80\x81\xFD[a\x02\xD0a\x01\xB9V[a\x02\xD9\x85a\x01\x87V[\x81R\x84\x87\x015\x87\x82\x01R\x83R\x92\x84\x01\x92\x91\x85\x01\x91a\x02\xB0V[\x80\x96PPPPPPP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x03MW\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x03\x1FV[P\x91\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x03\x82W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\x92W`\0\x80\xFD[\x93\x92PPPV[`\0`\x01\x82\x01a\x03\xB9WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 y\xDF\x1C\xFA[\xD4\xD2\x94\x11\xCA\xAB\x0C\x92\xCF;\xEF4\x821\x9F\x18\x08\x9E\x88\x83\xDE\xE6\x80\xC8&i\xF4dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKBRIDGEFACET_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80cr\xB9\xFF\xC0\x14a\0FW\x80c\x8B\x96\x855\x14a\0[W\x80c\xC9\r\xCF\xBB\x14a\0\x99W[`\0\x80\xFD[a\0Ya\0T6`\x04a\x02\x1BV[a\0\xC0V[\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0}\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x853`\x01`\x01`\xA0\x1B\x03\x16\x7F0\xF2o\x84\xC7u\x8Fr\x10z\xE1\x01\xD6'\xEF]z&\x88vl\xBC\xFEd6C\xC90t\x85&\x84\x87\x87\x87`@Qa\0\xFE\x93\x92\x91\x90a\x03CV[`@Q\x80\x91\x03\x90\xA3`@QcX1(\xE5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xB0bQ\xCA\x90a\x01X\x90\x89\x903\x90\x8A\x90\x8A\x90`\x04\x01a\x03oV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01wW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01\x9F\x91\x90\x81\x01\x90a\x04\x14V[PPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xBFW`\0\x80\xFD[PV[\x805a\x01\xCD\x81a\x01\xAAV[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x01\xE4W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xFCW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x02\x14W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15a\x027W`\0\x80\xFD[\x885\x97P` \x89\x015a\x02I\x81a\x01\xAAV[\x96P`@\x89\x015\x95P``\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02mW`\0\x80\xFD[\x81\x8B\x01\x91P\x8B`\x1F\x83\x01\x12a\x02\x81W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02\x90W`\0\x80\xFD[\x8C` \x82`\x06\x1B\x85\x01\x01\x11\x15a\x02\xA5W`\0\x80\xFD[` \x83\x01\x97P\x95Pa\x02\xB9`\x80\x8C\x01a\x01\xC2V[\x94P`\xA0\x8B\x015\x91P\x80\x82\x11\x15a\x02\xCFW`\0\x80\xFD[Pa\x02\xDC\x8B\x82\x8C\x01a\x01\xD2V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a\x038W\x815a\x03\x13\x81a\x01\xAAV[`\x01`\x01`\xA0\x1B\x03\x16\x87R\x81\x83\x015\x83\x88\x01R`@\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01a\x03\0V[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x03W`@\x83\x01\x85\x87a\x02\xF0V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x94\x93PPPPV[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x03\x9A\x90\x83\x01\x84\x86a\x02\xF0V[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03\xDDWa\x03\xDDa\x03\xA4V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x04\x0CWa\x04\x0Ca\x03\xA4V[`@R\x91\x90PV[`\0` \x80\x83\x85\x03\x12\x15a\x04'W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04?W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04SW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x04eWa\x04ea\x03\xA4V[a\x04s\x84\x82`\x05\x1B\x01a\x03\xE3V[\x81\x81R\x84\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x87\x82\x11\x15a\x04\x93W`\0\x80\xFD[\x92\x84\x01\x92[\x81\x84\x10\x15a\x04\xDFW`@\x84\x89\x03\x12\x15a\x04\xB1W`\0\x80\x81\xFD[a\x04\xB9a\x03\xBAV[\x84Qa\x04\xC4\x81a\x01\xAAV[\x81R\x84\x86\x01Q\x86\x82\x01R\x83R`@\x90\x93\x01\x92\x91\x84\x01\x91a\x04\x98V[\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xC3\xC1\xB9\xC3<w\x93\xEFc7\xA9\x8C\x7F\xED\x8E\xAC\x01k?>\x8C\xFDcf\xF3\xC2\xAB\x9By\xF35\xFDdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKBRIDGEFACET_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockBridgeFacet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockBridgeFacet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockBridgeFacet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockBridgeFacet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockBridgeFacet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockBridgeFacet))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockBridgeFacet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKBRIDGEFACET_ABI.clone(),
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
                MOCKBRIDGEFACET_ABI.clone(),
                MOCKBRIDGEFACET_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getLiquidityProjector` (0x8b968535) function
        pub fn get_liquidity_projector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([139, 150, 133, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidityProjector` (0xc90dcfbb) function
        pub fn liquidity_projector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([201, 13, 207, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `send` (0x72b9ffc0) function
        pub fn send(
            &self,
            root_chain_id: ::ethers::core::types::U256,
            root_sender: ::ethers::core::types::Address,
            destination_chain_id: ::ethers::core::types::U256,
            tokens: ::std::vec::Vec<Token>,
            target: ::ethers::core::types::Address,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [114, 185, 255, 192],
                    (
                        root_chain_id,
                        root_sender,
                        destination_chain_id,
                        tokens,
                        target,
                        message,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `BridgeRequest` event
        pub fn bridge_request_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BridgeRequestFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BridgeRequestFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockBridgeFacet<M> {
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
        name = "BridgeRequest",
        abi = "BridgeRequest(address,uint256,(address,uint256)[],address)"
    )]
    pub struct BridgeRequestFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub destination_chain_id: ::ethers::core::types::U256,
        pub approved_tokens: ::std::vec::Vec<Token>,
        pub target: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getLiquidityProjector` function with signature `getLiquidityProjector()` and selector `0x8b968535`
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
    #[ethcall(name = "getLiquidityProjector", abi = "getLiquidityProjector()")]
    pub struct GetLiquidityProjectorCall;
    ///Container type for all input parameters for the `liquidityProjector` function with signature `liquidityProjector()` and selector `0xc90dcfbb`
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
    #[ethcall(name = "liquidityProjector", abi = "liquidityProjector()")]
    pub struct LiquidityProjectorCall;
    ///Container type for all input parameters for the `send` function with signature `send(uint256,address,uint256,(address,uint256)[],address,bytes)` and selector `0x72b9ffc0`
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
        abi = "send(uint256,address,uint256,(address,uint256)[],address,bytes)"
    )]
    pub struct SendCall {
        pub root_chain_id: ::ethers::core::types::U256,
        pub root_sender: ::ethers::core::types::Address,
        pub destination_chain_id: ::ethers::core::types::U256,
        pub tokens: ::std::vec::Vec<Token>,
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
    pub enum MockBridgeFacetCalls {
        GetLiquidityProjector(GetLiquidityProjectorCall),
        LiquidityProjector(LiquidityProjectorCall),
        Send(SendCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockBridgeFacetCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetLiquidityProjectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLiquidityProjector(decoded));
            }
            if let Ok(decoded) = <LiquidityProjectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidityProjector(decoded));
            }
            if let Ok(decoded) = <SendCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Send(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockBridgeFacetCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetLiquidityProjector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidityProjector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Send(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockBridgeFacetCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetLiquidityProjector(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidityProjector(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Send(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetLiquidityProjectorCall> for MockBridgeFacetCalls {
        fn from(value: GetLiquidityProjectorCall) -> Self {
            Self::GetLiquidityProjector(value)
        }
    }
    impl ::core::convert::From<LiquidityProjectorCall> for MockBridgeFacetCalls {
        fn from(value: LiquidityProjectorCall) -> Self {
            Self::LiquidityProjector(value)
        }
    }
    impl ::core::convert::From<SendCall> for MockBridgeFacetCalls {
        fn from(value: SendCall) -> Self {
            Self::Send(value)
        }
    }
    ///Container type for all return fields from the `getLiquidityProjector` function with signature `getLiquidityProjector()` and selector `0x8b968535`
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
    pub struct GetLiquidityProjectorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `liquidityProjector` function with signature `liquidityProjector()` and selector `0xc90dcfbb`
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
    pub struct LiquidityProjectorReturn(pub ::ethers::core::types::Address);
}
