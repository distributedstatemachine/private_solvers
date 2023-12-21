pub use remote_request_processor::*;
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
pub mod remote_request_processor {
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
                    ::std::borrow::ToOwned::to_owned("ILHSwapFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ILHSwapFailed"),
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
    pub static REMOTEREQUESTPROCESSOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x12\x08\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xC2\xA1\xCB\x9D\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x0C\x9AV[a\0EV[\0[`\x02`\0T\x03a\0hW`@QcV\xA7\xF9\x91`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0\x81\x90UT`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\0\x99W`@Qc3f_\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\0\xA3\x84\x84a\x03\x94V[`\0\x80``\x80`\0\x80``a\0\xB7\x89a\x03\xDAV[\x95\x9CP\x93\x9AP\x91\x98P\x96P\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\0\xEFW`@Qc\xEA\xF6s\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86`\x01`\x01`\xA0\x1B\x03\x16\x8B\x7Fj\xEC\xE0\xB4\xD65\x1C\xB4'\xA1\x86\xCA9\xF8=\xFE\xD4H\xC1\x88S\n\xFE\xFCe\xE5<<\xCB\xF1x\xAB\x87\x89\x86`@Qa\x01-\x93\x92\x91\x90a\r\x85V[`@Q\x80\x91\x03\x90\xA3a\x01@\x8B\x860a\x04\x10V[\x94P\x82\x15a\x01UWa\x01R\x850a\x04\x94V[\x94P[\x83Q\x15a\x02\xFCW`\0a\x01h\x86\x86a\x075V[\x90PF\x87\x03a\x01\xF5Wa\x01{\x81\x84a\x07\xF3V[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x01\xF0W`@Qce\x82\x83\x85`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xCB\x05\x07\n\x90a\x01\xBD\x90\x8F\x90\x8C\x90\x86\x90\x88\x90`\x04\x01a\x0E\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xD7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xEBW=`\0\x80>=`\0\xFD[PPPP[a\x02\xF6V[`\x03Ta\x02\x0C\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16a\x08QV[`\x02T\x81Q`@Qcy#\xAF\xE7`\xE1\x1B\x81R`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xF2G_\xCE\x91a\x02H\x91\x8C\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x89\x91\x90a\x0EDV[\x90P0`\x01`\x01`\xA0\x1B\x03\x16cr\xB9\xFF\xC0\x82\x8F\x8C\x8C\x87\x8A\x8A`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xC2\x96\x95\x94\x93\x92\x91\x90a\x0E]V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02\xDBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xEFW=`\0\x80>=`\0\xFD[PPPPPP[Pa\x03\x82V[F\x86\x03a\x03\x82Wa\x03\r\x85\x83a\x07\xF3V[`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03\x82W`@Qce\x82\x83\x85`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xCB\x05\x07\n\x90a\x03O\x90\x8E\x90\x8B\x90\x8A\x90\x87\x90`\x04\x01a\x0E\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03iW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03}W=`\0\x80>=`\0\xFD[PPPP[PP`\x01`\0UPPPPPPPPPV[`\0\x82\x81R`\x06` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x14a\x03\xD6W`@Qc\x87\xED\x0F\x0F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[PPV[`\0\x80``\x80`\0\x80``\x87\x80` \x01\x90Q\x81\x01\x90a\x03\xF9\x91\x90a\x10\tV[\x95\x9E\x94\x9DP\x92\x9BP\x90\x99P\x97P\x95P\x90\x93P\x91PPV[`\x03T`@Qc\xC2\x15PG`\xE0\x1B\x81R``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC2\x15PG\x90a\x04E\x90\x87\x90\x86\x90\x88\x90`\x04\x01a\x10\xD3V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04dW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\x8C\x91\x90\x81\x01\x90a\x10\xFDV[\x94\x93PPPPV[```\0`\x02`\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xE8\xA2\xB1j`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x12\x91\x90a\x112V[\x90P`\0[\x84Q\x81\x10\x15a\x07,W\x81`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81Q\x81\x10a\x05<Wa\x05<a\x11VV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x06\x01W\x81`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x85\x87\x84\x81Q\x81\x10a\x05xWa\x05xa\x11VV[` \x02` \x01\x01Q` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xB5\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF8\x91\x90a\x11lV[P`\x01\x01a\x05\x17V[a\x06V\x85\x82\x81Q\x81\x10a\x06\x16Wa\x06\x16a\x11VV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`\x05T\x87Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x88\x90\x85\x90\x81\x10a\x06EWa\x06Ea\x11VV[` \x02` \x01\x01Q` \x01Qa\x08\x99V[`\x05T\x85Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9B}\xA9\x04\x90\x87\x90\x84\x90\x81\x10a\x06\x7FWa\x06\x7Fa\x11VV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R\x91\x90\x92\x01Q`$\x83\x01R\x87\x16`D\x82\x01R`d\x01`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xE3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x07\x91\x90a\x11\x87V[\x85\x82\x81Q\x81\x10a\x07\x19Wa\x07\x19a\x11VV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\x17V[P\x92\x93\x92PPPV[`\x04T``\x90a\x07O\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16a\x08QV[`\x04T`@Q`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a\x07q\x90\x86\x90a\x11\xA3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x07\xAEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\xB3V[``\x91P[P\x91P\x91P\x81a\x07\xD6W`@Qc'\xAD\x84]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x80` \x01\x90Q\x81\x01\x90a\x07\xEA\x91\x90a\x10\xFDV[\x95\x94PPPPPV[`\0[\x82Q\x81\x10\x15a\x08LWa\x08D\x83\x82\x81Q\x81\x10a\x08\x14Wa\x08\x14a\x11VV[` \x02` \x01\x01Q`\0\x01Q\x83\x85\x84\x81Q\x81\x10a\x083Wa\x083a\x11VV[` \x02` \x01\x01Q` \x01Qa\taV[`\x01\x01a\x07\xF6V[PPPV[`\0[\x82Q\x81\x10\x15a\x08LWa\x08\x91\x83\x82\x81Q\x81\x10a\x08rWa\x08ra\x11VV[` \x02` \x01\x01Q`\0\x01Q\x83\x85\x84\x81Q\x81\x10a\x06EWa\x06Ea\x11VV[`\x01\x01a\x08TV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x08\xEA\x84\x82a\t\x91V[a\t[W`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`\0`D\x82\x01Ra\tQ\x90\x85\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\n3V[a\t[\x84\x82a\n3V[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x08L\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\t\x1AV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\t\xAE\x91\x90a\x11\xA3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\t\xEBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xF0V[``\x91P[P\x91P\x91P\x81\x80\x15a\n\x1AWP\x80Q\x15\x80a\n\x1AWP\x80\x80` \x01\x90Q\x81\x01\x90a\n\x1A\x91\x90a\x11lV[\x80\x15a\x07\xEAWP`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15\x15a\x07\xEAV[`\0a\n\x88\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x0B\x08\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\n\xA9WP\x80\x80` \x01\x90Q\x81\x01\x90a\n\xA9\x91\x90a\x11lV[a\x08LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x03\xCDV[``a\x04\x8C\x84\x84`\0\x85\x85`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x0B/\x91\x90a\x11\xA3V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0BlW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0BqV[``\x91P[P\x91P\x91Pa\x0B\x82\x87\x83\x83\x87a\x0B\x8DV[\x97\x96PPPPPPPV[``\x83\x15a\x0B\xFCW\x82Q`\0\x03a\x0B\xF5W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x0B\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x03\xCDV[P\x81a\x04\x8CV[a\x04\x8C\x83\x83\x81Q\x15a\x0C\x11W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xCD\x91\x90a\x11\xBFV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0CjWa\x0Cja\x0C+V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\x8CWa\x0C\x8Ca\x0C+V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xAFW`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xD4W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x0C\xE5W`\0\x80\xFD[\x805a\x0C\xF8a\x0C\xF3\x82a\x0CrV[a\x0CAV[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\r\rW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\rzWa\rg\x87\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\rAV[P\x94\x95\x94PPPPPV[``\x81R`\0a\r\x98``\x83\x01\x86a\r-V[` \x83\x01\x94\x90\x94RP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`@\x90\x91\x01R\x91\x90PV[`\0[\x83\x81\x10\x15a\r\xD3W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\xBBV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\r\xF4\x81` \x86\x01` \x86\x01a\r\xB8V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`\x80`@\x82\x01\x81\x90R`\0\x90a\x0E2\x90\x83\x01\x85a\r-V[\x82\x81\x03``\x84\x01Ra\x0B\x82\x81\x85a\r\xDCV[`\0` \x82\x84\x03\x12\x15a\x0EVW`\0\x80\xFD[PQ\x91\x90PV[\x86\x81R`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16` \x84\x01R\x86`@\x84\x01R`\xC0``\x84\x01Ra\x0E\x8B`\xC0\x84\x01\x87a\r-V[\x81\x86\x16`\x80\x85\x01R\x83\x81\x03`\xA0\x85\x01Ra\x0E\xA5\x81\x86a\r\xDCV[\x9A\x99PPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xC8W`\0\x80\xFD[PV[`\0`@\x82\x84\x03\x12\x15a\x0E\xDDW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0F\0Wa\x0F\0a\x0C+V[\x80`@RP\x80\x91P\x82Qa\x0F\x13\x81a\x0E\xB3V[\x81R` \x92\x83\x01Q\x92\x01\x91\x90\x91R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x0F7W`\0\x80\xFD[\x81Q` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0FSWa\x0FSa\x0C+V[a\x0Fa\x81\x83`\x05\x1B\x01a\x0CAV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0F\x80W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0F\xA4Wa\x0F\x96\x88\x82a\x0E\xCBV[\x83R\x91\x83\x01\x91`@\x01a\x0F\x84V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x0F\xC0W`\0\x80\xFD[\x81Qa\x0F\xCEa\x0C\xF3\x82a\x0CrV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0F\xE3W`\0\x80\xFD[a\x04\x8C\x82` \x83\x01` \x87\x01a\r\xB8V[\x80Q\x80\x15\x15\x81\x14a\x10\x04W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x10$W`\0\x80\xFD[\x87Qa\x10/\x81a\x0E\xB3V[` \x89\x01Q`@\x8A\x01Q\x91\x98P\x96Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10TW`\0\x80\xFD[a\x10`\x8B\x83\x8C\x01a\x0F&V[\x96P``\x8A\x01Q\x91P\x80\x82\x11\x15a\x10vW`\0\x80\xFD[a\x10\x82\x8B\x83\x8C\x01a\x0F\xAFV[\x95Pa\x10\x90`\x80\x8B\x01a\x0F\xF4V[\x94P`\xA0\x8A\x01Q\x91Pa\x10\xA2\x82a\x0E\xB3V[`\xC0\x8A\x01Q\x91\x93P\x80\x82\x11\x15a\x10\xB7W`\0\x80\xFD[Pa\x10\xC4\x8A\x82\x8B\x01a\x0F\xAFV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x07\xEA\x90\x83\x01\x84a\r-V[`\0` \x82\x84\x03\x12\x15a\x11\x0FW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11&W`\0\x80\xFD[a\x04\x8C\x84\x82\x85\x01a\x0F&V[`\0` \x82\x84\x03\x12\x15a\x11DW`\0\x80\xFD[\x81Qa\x11O\x81a\x0E\xB3V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x11~W`\0\x80\xFD[a\x11O\x82a\x0F\xF4V[`\0`@\x82\x84\x03\x12\x15a\x11\x99W`\0\x80\xFD[a\x11O\x83\x83a\x0E\xCBV[`\0\x82Qa\x11\xB5\x81\x84` \x87\x01a\r\xB8V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x11O` \x83\x01\x84a\r\xDCV\xFE\xA2dipfsX\"\x12 \x16\x19\xFF\xB0\xD0z\x8E\xC8\x16|Z\xA7\xE9\x8C\xF8\xFA\x98{{{\x9D`\xBE\x0Ep\xA1x\xD43\x07,\x93dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static REMOTEREQUESTPROCESSOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xC2\xA1\xCB\x9D\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x0C\x9AV[a\0EV[\0[`\x02`\0T\x03a\0hW`@QcV\xA7\xF9\x91`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0\x81\x90UT`\x01`\x01`\xA0\x1B\x03\x163\x81\x14a\0\x99W`@Qc3f_\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\0\xA3\x84\x84a\x03\x94V[`\0\x80``\x80`\0\x80``a\0\xB7\x89a\x03\xDAV[\x95\x9CP\x93\x9AP\x91\x98P\x96P\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x82\x16a\0\xEFW`@Qc\xEA\xF6s\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86`\x01`\x01`\xA0\x1B\x03\x16\x8B\x7Fj\xEC\xE0\xB4\xD65\x1C\xB4'\xA1\x86\xCA9\xF8=\xFE\xD4H\xC1\x88S\n\xFE\xFCe\xE5<<\xCB\xF1x\xAB\x87\x89\x86`@Qa\x01-\x93\x92\x91\x90a\r\x85V[`@Q\x80\x91\x03\x90\xA3a\x01@\x8B\x860a\x04\x10V[\x94P\x82\x15a\x01UWa\x01R\x850a\x04\x94V[\x94P[\x83Q\x15a\x02\xFCW`\0a\x01h\x86\x86a\x075V[\x90PF\x87\x03a\x01\xF5Wa\x01{\x81\x84a\x07\xF3V[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x01\xF0W`@Qce\x82\x83\x85`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xCB\x05\x07\n\x90a\x01\xBD\x90\x8F\x90\x8C\x90\x86\x90\x88\x90`\x04\x01a\x0E\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xD7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xEBW=`\0\x80>=`\0\xFD[PPPP[a\x02\xF6V[`\x03Ta\x02\x0C\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16a\x08QV[`\x02T\x81Q`@Qcy#\xAF\xE7`\xE1\x1B\x81R`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xF2G_\xCE\x91a\x02H\x91\x8C\x91`\x04\x01\x91\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x89\x91\x90a\x0EDV[\x90P0`\x01`\x01`\xA0\x1B\x03\x16cr\xB9\xFF\xC0\x82\x8F\x8C\x8C\x87\x8A\x8A`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xC2\x96\x95\x94\x93\x92\x91\x90a\x0E]V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x02\xDBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xEFW=`\0\x80>=`\0\xFD[PPPPPP[Pa\x03\x82V[F\x86\x03a\x03\x82Wa\x03\r\x85\x83a\x07\xF3V[`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03\x82W`@Qce\x82\x83\x85`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xCB\x05\x07\n\x90a\x03O\x90\x8E\x90\x8B\x90\x8A\x90\x87\x90`\x04\x01a\x0E\x08V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03iW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03}W=`\0\x80>=`\0\xFD[PPPP[PP`\x01`\0UPPPPPPPPPV[`\0\x82\x81R`\x06` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x80\x83\x16\x91\x16\x14a\x03\xD6W`@Qc\x87\xED\x0F\x0F`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[PPV[`\0\x80``\x80`\0\x80``\x87\x80` \x01\x90Q\x81\x01\x90a\x03\xF9\x91\x90a\x10\tV[\x95\x9E\x94\x9DP\x92\x9BP\x90\x99P\x97P\x95P\x90\x93P\x91PPV[`\x03T`@Qc\xC2\x15PG`\xE0\x1B\x81R``\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC2\x15PG\x90a\x04E\x90\x87\x90\x86\x90\x88\x90`\x04\x01a\x10\xD3V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04dW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\x8C\x91\x90\x81\x01\x90a\x10\xFDV[\x94\x93PPPPV[```\0`\x02`\x01\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xE8\xA2\xB1j`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x12\x91\x90a\x112V[\x90P`\0[\x84Q\x81\x10\x15a\x07,W\x81`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81Q\x81\x10a\x05<Wa\x05<a\x11VV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x06\x01W\x81`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x85\x87\x84\x81Q\x81\x10a\x05xWa\x05xa\x11VV[` \x02` \x01\x01Q` \x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xB5\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF8\x91\x90a\x11lV[P`\x01\x01a\x05\x17V[a\x06V\x85\x82\x81Q\x81\x10a\x06\x16Wa\x06\x16a\x11VV[` \x90\x81\x02\x91\x90\x91\x01\x01QQ`\x05T\x87Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x88\x90\x85\x90\x81\x10a\x06EWa\x06Ea\x11VV[` \x02` \x01\x01Q` \x01Qa\x08\x99V[`\x05T\x85Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x9B}\xA9\x04\x90\x87\x90\x84\x90\x81\x10a\x06\x7FWa\x06\x7Fa\x11VV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R\x91\x90\x92\x01Q`$\x83\x01R\x87\x16`D\x82\x01R`d\x01`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xE3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x07\x91\x90a\x11\x87V[\x85\x82\x81Q\x81\x10a\x07\x19Wa\x07\x19a\x11VV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\x17V[P\x92\x93\x92PPPV[`\x04T``\x90a\x07O\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16a\x08QV[`\x04T`@Q`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a\x07q\x90\x86\x90a\x11\xA3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x07\xAEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\xB3V[``\x91P[P\x91P\x91P\x81a\x07\xD6W`@Qc'\xAD\x84]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x80` \x01\x90Q\x81\x01\x90a\x07\xEA\x91\x90a\x10\xFDV[\x95\x94PPPPPV[`\0[\x82Q\x81\x10\x15a\x08LWa\x08D\x83\x82\x81Q\x81\x10a\x08\x14Wa\x08\x14a\x11VV[` \x02` \x01\x01Q`\0\x01Q\x83\x85\x84\x81Q\x81\x10a\x083Wa\x083a\x11VV[` \x02` \x01\x01Q` \x01Qa\taV[`\x01\x01a\x07\xF6V[PPPV[`\0[\x82Q\x81\x10\x15a\x08LWa\x08\x91\x83\x82\x81Q\x81\x10a\x08rWa\x08ra\x11VV[` \x02` \x01\x01Q`\0\x01Q\x83\x85\x84\x81Q\x81\x10a\x06EWa\x06Ea\x11VV[`\x01\x01a\x08TV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x08\xEA\x84\x82a\t\x91V[a\t[W`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`\0`D\x82\x01Ra\tQ\x90\x85\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\n3V[a\t[\x84\x82a\n3V[PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x08L\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a\t\x1AV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\t\xAE\x91\x90a\x11\xA3V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\t\xEBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\xF0V[``\x91P[P\x91P\x91P\x81\x80\x15a\n\x1AWP\x80Q\x15\x80a\n\x1AWP\x80\x80` \x01\x90Q\x81\x01\x90a\n\x1A\x91\x90a\x11lV[\x80\x15a\x07\xEAWP`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15\x15a\x07\xEAV[`\0a\n\x88\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x0B\x08\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\n\xA9WP\x80\x80` \x01\x90Q\x81\x01\x90a\n\xA9\x91\x90a\x11lV[a\x08LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x03\xCDV[``a\x04\x8C\x84\x84`\0\x85\x85`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x0B/\x91\x90a\x11\xA3V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0BlW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0BqV[``\x91P[P\x91P\x91Pa\x0B\x82\x87\x83\x83\x87a\x0B\x8DV[\x97\x96PPPPPPPV[``\x83\x15a\x0B\xFCW\x82Q`\0\x03a\x0B\xF5W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x0B\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x03\xCDV[P\x81a\x04\x8CV[a\x04\x8C\x83\x83\x81Q\x15a\x0C\x11W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xCD\x91\x90a\x11\xBFV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0CjWa\x0Cja\x0C+V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\x8CWa\x0C\x8Ca\x0C+V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xAFW`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xD4W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x0C\xE5W`\0\x80\xFD[\x805a\x0C\xF8a\x0C\xF3\x82a\x0CrV[a\x0CAV[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\r\rW`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\rzWa\rg\x87\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\rAV[P\x94\x95\x94PPPPPV[``\x81R`\0a\r\x98``\x83\x01\x86a\r-V[` \x83\x01\x94\x90\x94RP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`@\x90\x91\x01R\x91\x90PV[`\0[\x83\x81\x10\x15a\r\xD3W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\xBBV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\r\xF4\x81` \x86\x01` \x86\x01a\r\xB8V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`\x80`@\x82\x01\x81\x90R`\0\x90a\x0E2\x90\x83\x01\x85a\r-V[\x82\x81\x03``\x84\x01Ra\x0B\x82\x81\x85a\r\xDCV[`\0` \x82\x84\x03\x12\x15a\x0EVW`\0\x80\xFD[PQ\x91\x90PV[\x86\x81R`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16` \x84\x01R\x86`@\x84\x01R`\xC0``\x84\x01Ra\x0E\x8B`\xC0\x84\x01\x87a\r-V[\x81\x86\x16`\x80\x85\x01R\x83\x81\x03`\xA0\x85\x01Ra\x0E\xA5\x81\x86a\r\xDCV[\x9A\x99PPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xC8W`\0\x80\xFD[PV[`\0`@\x82\x84\x03\x12\x15a\x0E\xDDW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0F\0Wa\x0F\0a\x0C+V[\x80`@RP\x80\x91P\x82Qa\x0F\x13\x81a\x0E\xB3V[\x81R` \x92\x83\x01Q\x92\x01\x91\x90\x91R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x0F7W`\0\x80\xFD[\x81Q` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0FSWa\x0FSa\x0C+V[a\x0Fa\x81\x83`\x05\x1B\x01a\x0CAV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0F\x80W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0F\xA4Wa\x0F\x96\x88\x82a\x0E\xCBV[\x83R\x91\x83\x01\x91`@\x01a\x0F\x84V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x0F\xC0W`\0\x80\xFD[\x81Qa\x0F\xCEa\x0C\xF3\x82a\x0CrV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0F\xE3W`\0\x80\xFD[a\x04\x8C\x82` \x83\x01` \x87\x01a\r\xB8V[\x80Q\x80\x15\x15\x81\x14a\x10\x04W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x10$W`\0\x80\xFD[\x87Qa\x10/\x81a\x0E\xB3V[` \x89\x01Q`@\x8A\x01Q\x91\x98P\x96Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10TW`\0\x80\xFD[a\x10`\x8B\x83\x8C\x01a\x0F&V[\x96P``\x8A\x01Q\x91P\x80\x82\x11\x15a\x10vW`\0\x80\xFD[a\x10\x82\x8B\x83\x8C\x01a\x0F\xAFV[\x95Pa\x10\x90`\x80\x8B\x01a\x0F\xF4V[\x94P`\xA0\x8A\x01Q\x91Pa\x10\xA2\x82a\x0E\xB3V[`\xC0\x8A\x01Q\x91\x93P\x80\x82\x11\x15a\x10\xB7W`\0\x80\xFD[Pa\x10\xC4\x8A\x82\x8B\x01a\x0F\xAFV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R`\0\x90a\x07\xEA\x90\x83\x01\x84a\r-V[`\0` \x82\x84\x03\x12\x15a\x11\x0FW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11&W`\0\x80\xFD[a\x04\x8C\x84\x82\x85\x01a\x0F&V[`\0` \x82\x84\x03\x12\x15a\x11DW`\0\x80\xFD[\x81Qa\x11O\x81a\x0E\xB3V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x11~W`\0\x80\xFD[a\x11O\x82a\x0F\xF4V[`\0`@\x82\x84\x03\x12\x15a\x11\x99W`\0\x80\xFD[a\x11O\x83\x83a\x0E\xCBV[`\0\x82Qa\x11\xB5\x81\x84` \x87\x01a\r\xB8V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x11O` \x83\x01\x84a\r\xDCV\xFE\xA2dipfsX\"\x12 \x16\x19\xFF\xB0\xD0z\x8E\xC8\x16|Z\xA7\xE9\x8C\xF8\xFA\x98{{{\x9D`\xBE\x0Ep\xA1x\xD43\x07,\x93dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static REMOTEREQUESTPROCESSOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RemoteRequestProcessor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RemoteRequestProcessor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RemoteRequestProcessor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RemoteRequestProcessor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RemoteRequestProcessor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RemoteRequestProcessor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RemoteRequestProcessor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    REMOTEREQUESTPROCESSOR_ABI.clone(),
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
                REMOTEREQUESTPROCESSOR_ABI.clone(),
                REMOTEREQUESTPROCESSOR_BYTECODE.clone().into(),
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
    for RemoteRequestProcessor<M> {
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
    ///Custom Error type `ILHSwapFailed` with signature `ILHSwapFailed()` and selector `0x27ad845d`
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
    #[etherror(name = "ILHSwapFailed", abi = "ILHSwapFailed()")]
    pub struct ILHSwapFailed;
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
    pub enum RemoteRequestProcessorErrors {
        BaseDiamondFacet__nonReentrant_reentrantCall(
            BaseDiamondFacet__nonReentrant_reentrantCall,
        ),
        ILHSwapFailed(ILHSwapFailed),
        InvalidHyperlaneAdapter(InvalidHyperlaneAdapter),
        InvalidSender(InvalidSender),
        ZeroTargetAddress(ZeroTargetAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for RemoteRequestProcessorErrors {
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
            if let Ok(decoded) = <ILHSwapFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ILHSwapFailed(decoded));
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
            if let Ok(decoded) = <ZeroTargetAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ZeroTargetAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RemoteRequestProcessorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BaseDiamondFacet__nonReentrant_reentrantCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ILHSwapFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidHyperlaneAdapter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroTargetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for RemoteRequestProcessorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BaseDiamondFacet__nonReentrant_reentrantCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ILHSwapFailed as ::ethers::contract::EthError>::selector() => {
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
                _ if selector
                    == <ZeroTargetAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for RemoteRequestProcessorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BaseDiamondFacet__nonReentrant_reentrantCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ILHSwapFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidHyperlaneAdapter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroTargetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for RemoteRequestProcessorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BaseDiamondFacet__nonReentrant_reentrantCall>
    for RemoteRequestProcessorErrors {
        fn from(value: BaseDiamondFacet__nonReentrant_reentrantCall) -> Self {
            Self::BaseDiamondFacet__nonReentrant_reentrantCall(value)
        }
    }
    impl ::core::convert::From<ILHSwapFailed> for RemoteRequestProcessorErrors {
        fn from(value: ILHSwapFailed) -> Self {
            Self::ILHSwapFailed(value)
        }
    }
    impl ::core::convert::From<InvalidHyperlaneAdapter>
    for RemoteRequestProcessorErrors {
        fn from(value: InvalidHyperlaneAdapter) -> Self {
            Self::InvalidHyperlaneAdapter(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for RemoteRequestProcessorErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<ZeroTargetAddress> for RemoteRequestProcessorErrors {
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
