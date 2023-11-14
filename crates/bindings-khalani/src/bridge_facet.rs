pub use bridge_facet::*;
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
pub mod bridge_facet {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
    pub static BRIDGEFACET_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x07\x8D\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0)W`\x005`\xE0\x1C\x80cr\xB9\xFF\xC0\x14a\0.W\x80c\x8B\x96\x855\x14a\0CW[`\0\x80\xFD[a\0Aa\0<6`\x04a\x03\xDAV[a\0oV[\0[4\x80\x15a\0OW`\0\x80\xFD[P`\x03T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[`\x02`\x01T\x03a\0\x92W`@QcQ\xE6\x97}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\x01`\x01`\xA0\x1B\x03\x83\x16a\0\xBEW`@Qc\xEA\xF6s\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x843`\x01`\x01`\xA0\x1B\x03\x16\x7F0\xF2o\x84\xC7u\x8Fr\x10z\xE1\x01\xD6'\xEF]z&\x88vl\xBC\xFEd6C\xC90t\x85&\x84\x86\x86`@Qa\0\xFA\x92\x91\x90a\x05^V[`@Q\x80\x91\x03\x90\xA3`\x03T`@QcX1(\xE5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB0bQ\xCA\x90a\x016\x90\x88\x903\x90\x89\x90`\x04\x01a\x05\x88V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01}\x91\x90\x81\x01\x90a\x05\xBBV[`\x02T`\0\x87\x81R`\x06` R`@\x81 T\x92\x96P\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91c\x01B\x8A\x07\x91\x89\x91\x16a\x01\xB6\x8C\x8C\x8B\x8B\x8B\x8Ba\x02\x9DV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xD4\x93\x92\x91\x90a\x06zV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x17\x91\x90a\x06\xD7V[`\x02T\x86Q`@QcL\xDD\x14\x9B`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x8A\x90R`D\x81\x01\x91\x90\x91R3`d\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cL\xDD\x14\x9B\x904\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02vW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x8AW=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPPPPPPPV[``\x86\x86\x86\x86\x86\x86`@Q` \x01a\x02\xBA\x96\x95\x94\x93\x92\x91\x90a\x06\xF0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\xEAW`\0\x80\xFD[PV[\x805a\x02\xF8\x81a\x02\xD5V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x036Wa\x036a\x02\xFDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03eWa\x03ea\x02\xFDV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\x87Wa\x03\x87a\x02\xFDV[P`\x05\x1B` \x01\x90V[`\0\x80\x83`\x1F\x84\x01\x12a\x03\xA3W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xBBW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x03\xD3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x03\xF5W`\0\x80\xFD[\x875\x96P` \x88\x015a\x04\x07\x81a\x02\xD5V[\x95P`@\x88\x015\x94P``\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04+W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x04?W`\0\x80\xFD[\x815a\x04Ra\x04M\x82a\x03mV[a\x03<V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x8D\x83\x11\x15a\x04tW`\0\x80\xFD[` \x85\x01\x94P[\x82\x85\x10\x15a\x04\xC8W`@\x85\x8F\x03\x12\x15a\x04\x93W`\0\x80\xFD[a\x04\x9Ba\x03\x13V[a\x04\xA5\x865a\x02\xD5V[\x855\x81R` \x86\x015` \x82\x01R\x80\x83RP` \x82\x01\x91P`@\x85\x01\x94Pa\x04{V[\x97Pa\x04\xD9\x91PP`\x80\x8B\x01a\x02\xEDV[\x94P`\xA0\x8A\x015\x91P\x80\x82\x11\x15a\x04\xEFW`\0\x80\xFD[Pa\x04\xFC\x8A\x82\x8B\x01a\x03\x91V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x05SW\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x88R\x83\x01Q\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05#V[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x05q`@\x83\x01\x85a\x05\x0FV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x05\xB2\x90\x83\x01\x84a\x05\x0FV[\x95\x94PPPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x05\xCEW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xE5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x05\xF6W`\0\x80\xFD[\x80Qa\x06\x04a\x04M\x82a\x03mV[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x06#W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x06oW`@\x84\x89\x03\x12\x15a\x06AW`\0\x80\x81\xFD[a\x06Ia\x03\x13V[\x84Qa\x06T\x81a\x02\xD5V[\x81R\x84\x86\x01Q\x86\x82\x01R\x82R`@\x90\x93\x01\x92\x90\x84\x01\x90a\x06(V[\x97\x96PPPPPPPV[\x83\x81R`\0` \x84\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x06\xB4W\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x06\x98V[P`\0`\x80\x82\x86\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x06\xE9W`\0\x80\xFD[PQ\x91\x90PV[\x86\x81R`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16` \x84\x01R`\xA0`@\x84\x01Ra\x07\x18`\xA0\x84\x01\x88a\x05\x0FV[\x81\x87\x16``\x85\x01R\x83\x81\x03`\x80\x85\x01R\x84\x81R\x84\x86` \x83\x017`\0` \x86\x83\x01\x01R` `\x1F\x19`\x1F\x87\x01\x16\x82\x01\x01\x92PPP\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \x83\xD8\xCC|\x07B\xC4\xFD\x94m\xFD\x0F8\xD6\xAE\xBD\xC8\xD0<I\xF0h\xCFQq\xB8\xC0\x02\xDA\x1D\xF4\x83dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static BRIDGEFACET_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0)W`\x005`\xE0\x1C\x80cr\xB9\xFF\xC0\x14a\0.W\x80c\x8B\x96\x855\x14a\0CW[`\0\x80\xFD[a\0Aa\0<6`\x04a\x03\xDAV[a\0oV[\0[4\x80\x15a\0OW`\0\x80\xFD[P`\x03T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82RQ\x90\x81\x90\x03` \x01\x90\xF3[`\x02`\x01T\x03a\0\x92W`@QcQ\xE6\x97}`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\x01`\x01`\xA0\x1B\x03\x83\x16a\0\xBEW`@Qc\xEA\xF6s\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x843`\x01`\x01`\xA0\x1B\x03\x16\x7F0\xF2o\x84\xC7u\x8Fr\x10z\xE1\x01\xD6'\xEF]z&\x88vl\xBC\xFEd6C\xC90t\x85&\x84\x86\x86`@Qa\0\xFA\x92\x91\x90a\x05^V[`@Q\x80\x91\x03\x90\xA3`\x03T`@QcX1(\xE5`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB0bQ\xCA\x90a\x016\x90\x88\x903\x90\x89\x90`\x04\x01a\x05\x88V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x01}\x91\x90\x81\x01\x90a\x05\xBBV[`\x02T`\0\x87\x81R`\x06` R`@\x81 T\x92\x96P\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91c\x01B\x8A\x07\x91\x89\x91\x16a\x01\xB6\x8C\x8C\x8B\x8B\x8B\x8Ba\x02\x9DV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xD4\x93\x92\x91\x90a\x06zV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x17\x91\x90a\x06\xD7V[`\x02T\x86Q`@QcL\xDD\x14\x9B`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x8A\x90R`D\x81\x01\x91\x90\x91R3`d\x82\x01R\x91\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90cL\xDD\x14\x9B\x904\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02vW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x8AW=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPPPPPPPV[``\x86\x86\x86\x86\x86\x86`@Q` \x01a\x02\xBA\x96\x95\x94\x93\x92\x91\x90a\x06\xF0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\xEAW`\0\x80\xFD[PV[\x805a\x02\xF8\x81a\x02\xD5V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x036Wa\x036a\x02\xFDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x03eWa\x03ea\x02\xFDV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x03\x87Wa\x03\x87a\x02\xFDV[P`\x05\x1B` \x01\x90V[`\0\x80\x83`\x1F\x84\x01\x12a\x03\xA3W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xBBW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x03\xD3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x03\xF5W`\0\x80\xFD[\x875\x96P` \x88\x015a\x04\x07\x81a\x02\xD5V[\x95P`@\x88\x015\x94P``\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04+W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x04?W`\0\x80\xFD[\x815a\x04Ra\x04M\x82a\x03mV[a\x03<V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x8D\x83\x11\x15a\x04tW`\0\x80\xFD[` \x85\x01\x94P[\x82\x85\x10\x15a\x04\xC8W`@\x85\x8F\x03\x12\x15a\x04\x93W`\0\x80\xFD[a\x04\x9Ba\x03\x13V[a\x04\xA5\x865a\x02\xD5V[\x855\x81R` \x86\x015` \x82\x01R\x80\x83RP` \x82\x01\x91P`@\x85\x01\x94Pa\x04{V[\x97Pa\x04\xD9\x91PP`\x80\x8B\x01a\x02\xEDV[\x94P`\xA0\x8A\x015\x91P\x80\x82\x11\x15a\x04\xEFW`\0\x80\xFD[Pa\x04\xFC\x8A\x82\x8B\x01a\x03\x91V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x05SW\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x88R\x83\x01Q\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x05#V[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x05q`@\x83\x01\x85a\x05\x0FV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x05\xB2\x90\x83\x01\x84a\x05\x0FV[\x95\x94PPPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x05\xCEW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xE5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x05\xF6W`\0\x80\xFD[\x80Qa\x06\x04a\x04M\x82a\x03mV[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x06#W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x06oW`@\x84\x89\x03\x12\x15a\x06AW`\0\x80\x81\xFD[a\x06Ia\x03\x13V[\x84Qa\x06T\x81a\x02\xD5V[\x81R\x84\x86\x01Q\x86\x82\x01R\x82R`@\x90\x93\x01\x92\x90\x84\x01\x90a\x06(V[\x97\x96PPPPPPPV[\x83\x81R`\0` \x84\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x06\xB4W\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x06\x98V[P`\0`\x80\x82\x86\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x06\xE9W`\0\x80\xFD[PQ\x91\x90PV[\x86\x81R`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16` \x84\x01R`\xA0`@\x84\x01Ra\x07\x18`\xA0\x84\x01\x88a\x05\x0FV[\x81\x87\x16``\x85\x01R\x83\x81\x03`\x80\x85\x01R\x84\x81R\x84\x86` \x83\x017`\0` \x86\x83\x01\x01R` `\x1F\x19`\x1F\x87\x01\x16\x82\x01\x01\x92PPP\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \x83\xD8\xCC|\x07B\xC4\xFD\x94m\xFD\x0F8\xD6\xAE\xBD\xC8\xD0<I\xF0h\xCFQq\xB8\xC0\x02\xDA\x1D\xF4\x83dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static BRIDGEFACET_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BridgeFacet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BridgeFacet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BridgeFacet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BridgeFacet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BridgeFacet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BridgeFacet))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BridgeFacet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BRIDGEFACET_ABI.clone(),
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
                BRIDGEFACET_ABI.clone(),
                BRIDGEFACET_BYTECODE.clone().into(),
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
    for BridgeFacet<M> {
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
    pub enum BridgeFacetErrors {
        BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall(
            BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall,
        ),
        ZeroTargetAddress(ZeroTargetAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BridgeFacetErrors {
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
    impl ::ethers::core::abi::AbiEncode for BridgeFacetErrors {
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
    impl ::ethers::contract::ContractRevert for BridgeFacetErrors {
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
    impl ::core::fmt::Display for BridgeFacetErrors {
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
    impl ::core::convert::From<::std::string::String> for BridgeFacetErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall>
    for BridgeFacetErrors {
        fn from(value: BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall) -> Self {
            Self::BaseDiamondFacet__bridgeCallNonReentrant_reentrantCall(value)
        }
    }
    impl ::core::convert::From<ZeroTargetAddress> for BridgeFacetErrors {
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
    pub enum BridgeFacetCalls {
        GetLiquidityProjector(GetLiquidityProjectorCall),
        Send(SendCall),
    }
    impl ::ethers::core::abi::AbiDecode for BridgeFacetCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetLiquidityProjectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLiquidityProjector(decoded));
            }
            if let Ok(decoded) = <SendCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Send(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BridgeFacetCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetLiquidityProjector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Send(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BridgeFacetCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetLiquidityProjector(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Send(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetLiquidityProjectorCall> for BridgeFacetCalls {
        fn from(value: GetLiquidityProjectorCall) -> Self {
            Self::GetLiquidityProjector(value)
        }
    }
    impl ::core::convert::From<SendCall> for BridgeFacetCalls {
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
}
