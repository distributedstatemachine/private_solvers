pub use interchain_liquidity_hub_wrapper::*;
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
pub mod interchain_liquidity_hub_wrapper {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_balancerVault"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_interchainMessagingGateway",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("balancerVault"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balancerVault"),
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
                    ::std::borrow::ToOwned::to_owned("executeSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swaps"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapCount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IAsset[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("limits"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
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
                            outputs: ::std::vec![
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("interchainMessagingGateway"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "interchainMessagingGateway",
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
                    ::std::borrow::ToOwned::to_owned("withdrawLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swaps"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BatchSwapStep[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IAsset[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("limits"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
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
                            outputs: ::std::vec![
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("LiquidityWithdrawn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("LiquidityWithdrawn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapExecuted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SwapExecuted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
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
                        "InterchainLiquidityHubWrapper__NoAssetsToSwap",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InterchainLiquidityHubWrapper__NoAssetsToSwap",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "InterchainLiquidityHubWrapper__NoSwapsToExecute",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InterchainLiquidityHubWrapper__NoSwapsToExecute",
                            ),
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
    pub static INTERCHAINLIQUIDITYHUBWRAPPER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qb\0\x18\t8\x03\x80b\0\x18\t\x839\x81\x01`@\x81\x90Ra\x001\x91a\0iV[`\x01`\0U`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0R\x16`\x80Ra\0\x9CV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0dW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\0|W`\0\x80\xFD[a\0\x85\x83a\0MV[\x91Pa\0\x93` \x84\x01a\0MV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Qa\x17&b\0\0\xE3`\09`\0\x81\x81`V\x01R\x81\x81a\x02z\x01Ra\x03\x01\x01R`\0\x81\x81`\xC7\x01R\x81\x81a\x01\x8F\x01R\x81\x81a\x04\xC2\x01Ra\x06\xBD\x01Ra\x17&`\0\xF3\xFE`\x80`@R`\x046\x10a\0?W`\x005`\xE0\x1C\x80c\t!}\xA9\x14a\0DW\x80c\x13D\x97\x04\x14a\0\x95W\x80c\x15\x82t\xA5\x14a\0\xB5W\x80cd!\x82\xF3\x14a\0\xE9W[`\0\x80\xFD[4\x80\x15a\0PW`\0\x80\xFD[Pa\0x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA8a\0\xA36`\x04a\x0F~V[a\0\xFCV[`@Qa\0\x8C\x91\x90a\x10\xB2V[4\x80\x15a\0\xC1W`\0\x80\xFD[Pa\0x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xA8a\0\xF76`\x04a\x10\xCCV[a\x03\xFAV[``a\x01\x06a\x05\xD2V[`\0\x85\x90\x03a\x01(W`@Qc=\x8F\x03[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q`\0\x03a\x01JW`@Qcwp\xE6\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01T\x84\x84a\x060V[`@\x80Q`\x80\x81\x01\x82R`\0` \x82\x01\x81\x90R``\x82\x01\x81\x90R0\x80\x83R\x82\x84\x01R\x91Qc\x94[\xCE\xC9`\xE0\x1B\x81R\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x94[\xCE\xC9\x90a\x01\xD0\x90\x84\x90\x8C\x90\x8C\x90\x8C\x90\x89\x90\x8D\x90\x8D\x90`\x04\x01a\x12KV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\x17\x91\x90\x81\x01\x90a\x13\xB5V[\x90P\x81`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB4\xBE.\xE7O6V\xD3\xA0/\xB5Q\xDE\xE6\xD3\xBE{\xE9<\x02\xBD\x1C\xB8\x01u\t\x1B_\xC7\xBD\xCD\xCF\x88`@Qa\x02`\x91\x90a\x14;V[`@Q\x80\x91\x03\x90\xA3a\x02r\x86\x82a\x07JV[\x92Pa\x02\xFF\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8B\x96\x855`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xFA\x91\x90a\x14NV[a\x08\xBEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cr\xB9\xFF\xC04F3\x8E\x88\x8F`@Q\x80` \x01`@R\x80`\0\x81RP`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03e\x96\x95\x94\x93\x92\x91\x90a\x14\xBBV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x03~W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x92W=`\0\x80>=`\0\xFD[PPPPP\x88`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x8B\x7F+\xBD*\x07;\xA3\xD7\xFB\xF0;t\x1B\xB6bnV\x84\xDD\xC17\xCA\xABH\xE4|a\x02\x076\xD8 \xB8\x86`@Qa\x03\xDB\x91\x90a\x10\xB2V[`@Q\x80\x91\x03\x90\xA4PPa\x03\xEF`\x01`\0UV[\x97\x96PPPPPPPV[``a\x04\x04a\x05\xD2V[\x85\x15\x80a\x04\x0FWP\x84\x15[\x15a\x04-W`@Qc=\x8F\x03[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q`\0\x03a\x04OW`@Qcwp\xE6\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04Y\x84\x84a\x060V[`@\x80Q`\x80\x81\x01\x82R`\0` \x80\x83\x01\x82\x90R``\x83\x01\x82\x90R0\x83R3\x83\x85\x01R\x83Q`\x1F\x8B\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x90\x94R\x89\x84R\x91\x92\x90\x91a\x04\xBC\x91\x90\x8B\x90\x8B\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8B\x92Pa\t\x17\x91PPV[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x94[\xCE\xC9`\0\x84\x8A\x87\x8B\x8B`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x17\x96\x95\x94\x93\x92\x91\x90a\x15\x11V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x056W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05^\x91\x90\x81\x01\x90a\x13\xB5V[\x90P\x82`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB4\xBE.\xE7O6V\xD3\xA0/\xB5Q\xDE\xE6\xD3\xBE{\xE9<\x02\xBD\x1C\xB8\x01u\t\x1B_\xC7\xBD\xCD\xCF\x89`@Qa\x05\xA7\x91\x90a\x14;V[`@Q\x80\x91\x03\x90\xA3a\x05\xB9\x87\x82a\x07JV[\x93PPPPa\x05\xC8`\x01`\0UV[\x96\x95PPPPPPV[`\x02`\0T\x03a\x06)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0[\x81Q\x81\x10\x15a\x07EW`\0\x82\x82\x81Q\x81\x10a\x06PWa\x06Pa\x16*V[` \x02` \x01\x01Q\x13\x15a\x07=W`\0\x83\x82\x81Q\x81\x10a\x06rWa\x06ra\x16*V[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x06\x90Wa\x06\x90a\x16*V[` \x02` \x01\x01Q\x90Pa\x06\xA6\x8230\x84a\n4V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x83\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x079\x91\x90a\x16@V[PPP[`\x01\x01a\x063V[PPPV[```\0\x80[\x83Q\x81\x10\x15a\x07\x91W`\0\x84\x82\x81Q\x81\x10a\x07mWa\x07ma\x16*V[` \x02` \x01\x01Q\x12\x15a\x07\x89W\x81a\x07\x85\x81a\x16xV[\x92PP[`\x01\x01a\x07PV[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xADWa\x07\xADa\x0EDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xF2W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\xCBW\x90P[P\x90P`\0\x80[\x85Q\x81\x10\x15a\x08\xB1W`\0\x86\x82\x81Q\x81\x10a\x08\x16Wa\x08\x16a\x16*V[` \x02` \x01\x01Q\x12\x15a\x08\xA9W`@Q\x80`@\x01`@R\x80\x88\x83\x81Q\x81\x10a\x08AWa\x08Aa\x16*V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87\x83\x81Q\x81\x10a\x08iWa\x08ia\x16*V[` \x02` \x01\x01Q`\0\x19a\x08~\x91\x90a\x16\x91V[\x90R\x83\x83a\x08\x8B\x81a\x16xV[\x94P\x81Q\x81\x10a\x08\x9DWa\x08\x9Da\x16*V[` \x02` \x01\x01\x81\x90RP[`\x01\x01a\x07\xF9V[P\x90\x92PPP[\x92\x91PPV[`\0[\x82Q\x81\x10\x15a\x07EWa\t\x0F\x83\x82\x81Q\x81\x10a\x08\xDFWa\x08\xDFa\x16*V[` \x02` \x01\x01Q`\0\x01Q\x83\x85\x84\x81Q\x81\x10a\x08\xFEWa\x08\xFEa\x16*V[` \x02` \x01\x01Q` \x01Qa\n\xA5V[`\x01\x01a\x08\xC1V[```\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t4Wa\t4a\x0EDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x9AW\x81` \x01[a\t\x87`@Q\x80`\xA0\x01`@R\x80`\0\x80\x19\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\tRW\x90P[P\x90P`\x80`\0[\x84\x81\x10\x15a\n*W` \x82\x82\x02\x87\x01\x81\x81\x01Q`@\x80\x83\x01Q``\x80\x85\x01Q`\x80\x95\x86\x01Q\x84Q`\xA0\x81\x01\x86R\x86\x81R\x80\x89\x01\x85\x90R\x80\x86\x01\x83\x90R\x92\x83\x01\x81\x90R\x84Q\x97\x88\x01\x90\x94R`\0\x87R\x94\x81\x01\x95\x90\x95R\x87Q\x92\x94\x90\x93\x92\x88\x90\x87\x90\x81\x10a\n\x10Wa\n\x10a\x16*V[` \x02` \x01\x01\x81\x90RP\x84`\x01\x01\x94PPPPPa\t\xA2V[P\x90\x94\x93PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\n\x9F\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x0B0V[PPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\n\xF6\x84\x82a\x0C\x05V[a\n\x9FW`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`\0`D\x82\x01Ra\x0B*\x90\x85\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01a\nhV[a\n\x9F\x84\x82[`\0a\x0B\x85\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xAC\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x0B\xA6WP\x80\x80` \x01\x90Q\x81\x01\x90a\x0B\xA6\x91\x90a\x16@V[a\x07EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x06 V[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x0C\"\x91\x90a\x16\xC1V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0C_W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0CdV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0C\x8EWP\x80Q\x15\x80a\x0C\x8EWP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\x8E\x91\x90a\x16@V[\x80\x15a\x0C\xA3WP`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15\x15[\x95\x94PPPPPV[``a\x0C\xBB\x84\x84`\0\x85a\x0C\xC3V[\x94\x93PPPPV[``\x82G\x10\x15a\r$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x06 V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\r@\x91\x90a\x16\xC1V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\r}W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\x82V[``\x91P[P\x91P\x91Pa\x03\xEF\x87\x83\x83\x87``\x83\x15a\r\xFDW\x82Q`\0\x03a\r\xF6W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\r\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x06 V[P\x81a\x0C\xBBV[a\x0C\xBB\x83\x83\x81Q\x15a\x0E\x12W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06 \x91\x90a\x16\xDDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0EAW`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\x83Wa\x0E\x83a\x0EDV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xA5Wa\x0E\xA5a\x0EDV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0E\xC0W`\0\x80\xFD[\x815` a\x0E\xD5a\x0E\xD0\x83a\x0E\x8BV[a\x0EZV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0E\xF4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0F\x18W\x805a\x0F\x0B\x81a\x0E,V[\x83R\x91\x83\x01\x91\x83\x01a\x0E\xF8V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x0F4W`\0\x80\xFD[\x815` a\x0FDa\x0E\xD0\x83a\x0E\x8BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0FcW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0F\x18W\x805\x83R\x91\x83\x01\x91\x83\x01a\x0FgV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x0F\x99W`\0\x80\xFD[\x875\x96P` \x88\x015a\x0F\xAB\x81a\x0E,V[\x95P`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xC8W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x0F\xDCW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\xEBW`\0\x80\xFD[\x8B` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x10\0W`\0\x80\xFD[` \x83\x01\x97P\x80\x96PP``\x8A\x015\x91P\x80\x82\x11\x15a\x10\x1EW`\0\x80\xFD[a\x10*\x8B\x83\x8C\x01a\x0E\xAFV[\x94P`\x80\x8A\x015\x91P\x80\x82\x11\x15a\x10@W`\0\x80\xFD[Pa\x10M\x8A\x82\x8B\x01a\x0F#V[\x92PP`\xA0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x10\xA7W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x88R\x83\x01Q\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x10wV[P\x94\x95\x94PPPPPV[` \x81R`\0a\x10\xC5` \x83\x01\x84a\x10cV[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x10\xE5W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\xFDW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x11\x11W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11 W`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x112W`\0\x80\xFD[` \x92\x83\x01\x98P\x96P\x90\x88\x015\x94P`@\x88\x015\x90\x80\x82\x11\x15a\x11TW`\0\x80\xFD[a\x11`\x8A\x83\x8B\x01a\x0E\xAFV[\x94P``\x89\x015\x91P\x80\x82\x11\x15a\x11vW`\0\x80\xFD[Pa\x11\x83\x89\x82\x8A\x01a\x0F#V[\x92PP`\x80\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\x02\x81\x10a\x11\xB5WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x10\xA7W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x11\xF6V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x10\xA7W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x12/V[`\0a\x01 \x80\x83\x01a\x12]\x84\x8Ca\x11\x97V[` \x84\x81\x01\x92\x90\x92R\x88\x90Ra\x01@\x80\x84\x01\x91`\x05\x8A\x90\x1B\x85\x01\x90\x91\x01\x90\x8A`\0[\x8B\x81\x10\x15a\x138W\x86\x84\x03a\x01?\x19\x01\x85R\x8156\x8E\x90\x03`\x9E\x19\x01\x81\x12a\x12\xA6W`\0\x80\xFD[\x8D\x01\x805\x85R\x83\x81\x015\x84\x86\x01R`@\x80\x82\x015\x90\x86\x01R``\x80\x82\x015\x90\x86\x01R`\xA0`\x80\x80\x83\x0156\x84\x90\x03`\x1E\x19\x01\x81\x12a\x12\xE3W`\0\x80\xFD[\x90\x92\x01\x85\x81\x01\x92\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x01W`\0\x80\xFD[\x806\x03\x84\x13\x15a\x13\x10W`\0\x80\xFD[\x82\x82\x89\x01Ra\x13\"\x83\x89\x01\x82\x86a\x11\xB9V[\x98\x87\x01\x98\x97PPP\x92\x84\x01\x92PP`\x01\x01a\x12\x7FV[PPP\x83\x81\x03`@\x85\x01Ra\x13M\x81\x89a\x11\xE2V[\x91PPa\x13\x8D``\x84\x01\x87\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x15\x15\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x90\x81\x01Q\x15\x15\x91\x01RV[\x82\x81\x03`\xE0\x84\x01Ra\x13\x9F\x81\x86a\x12\x1BV[\x91PP\x82a\x01\0\x83\x01R\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x13\xC8W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xDFW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x13\xF0W`\0\x80\xFD[\x80Qa\x13\xFEa\x0E\xD0\x82a\x0E\x8BV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x14\x1DW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x03\xEFW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x14\"V[` \x81R`\0a\x10\xC5` \x83\x01\x84a\x11\xE2V[`\0` \x82\x84\x03\x12\x15a\x14`W`\0\x80\xFD[\x81Qa\x10\xC5\x81a\x0E,V[`\0[\x83\x81\x10\x15a\x14\x86W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14nV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x14\xA7\x81` \x86\x01` \x86\x01a\x14kV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x86\x81R`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16` \x84\x01R\x86`@\x84\x01R`\xC0``\x84\x01Ra\x14\xE9`\xC0\x84\x01\x87a\x10cV[\x81\x86\x16`\x80\x85\x01R\x83\x81\x03`\xA0\x85\x01Ra\x15\x03\x81\x86a\x14\x8FV[\x9A\x99PPPPPPPPPPV[`\0a\x01 \x80\x83\x01a\x15#\x84\x8Ba\x11\x97V[` \x84\x81\x01\x92\x90\x92R\x88Q\x90\x81\x90Ra\x01@\x80\x85\x01\x92`\x05\x83\x90\x1B\x86\x01\x90\x91\x01\x91\x8A\x82\x01`\0[\x82\x81\x10\x15a\x15\xADW\x87\x85\x03a\x01?\x19\x01\x86R\x81Q\x80Q\x86R\x84\x81\x01Q\x85\x87\x01R`@\x80\x82\x01Q\x90\x87\x01R``\x80\x82\x01Q\x90\x87\x01R`\x80\x90\x81\x01Q`\xA0\x91\x87\x01\x82\x90R\x90a\x15\x99\x81\x88\x01\x83a\x14\x8FV[\x97\x86\x01\x97\x96PPP\x90\x83\x01\x90`\x01\x01a\x15JV[PPPP\x83\x81\x03`@\x85\x01Ra\x15\xC3\x81\x89a\x11\xE2V[\x91PPa\x16\x03``\x84\x01\x87\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x15\x15\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x90\x81\x01Q\x15\x15\x91\x01RV[\x82\x81\x03`\xE0\x84\x01Ra\x16\x15\x81\x86a\x12\x1BV[\x91PP\x82a\x01\0\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x16RW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x10\xC5W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x16\x8AWa\x16\x8Aa\x16bV[P`\x01\x01\x90V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x16\xADWa\x16\xADa\x16bV[\x81\x81\x05\x83\x14\x82\x15\x17a\x08\xB8Wa\x08\xB8a\x16bV[`\0\x82Qa\x16\xD3\x81\x84` \x87\x01a\x14kV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x10\xC5` \x83\x01\x84a\x14\x8FV\xFE\xA2dipfsX\"\x12 \xF0XR\xCCL\x94(\xBB\xC0w\xEC#\xF5,\x8F\xECB\xE1\xE0]\x93\xCC\xBF\xFF\xFC>\xAA\xD0\xFFjP5dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static INTERCHAINLIQUIDITYHUBWRAPPER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0?W`\x005`\xE0\x1C\x80c\t!}\xA9\x14a\0DW\x80c\x13D\x97\x04\x14a\0\x95W\x80c\x15\x82t\xA5\x14a\0\xB5W\x80cd!\x82\xF3\x14a\0\xE9W[`\0\x80\xFD[4\x80\x15a\0PW`\0\x80\xFD[Pa\0x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA8a\0\xA36`\x04a\x0F~V[a\0\xFCV[`@Qa\0\x8C\x91\x90a\x10\xB2V[4\x80\x15a\0\xC1W`\0\x80\xFD[Pa\0x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xA8a\0\xF76`\x04a\x10\xCCV[a\x03\xFAV[``a\x01\x06a\x05\xD2V[`\0\x85\x90\x03a\x01(W`@Qc=\x8F\x03[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q`\0\x03a\x01JW`@Qcwp\xE6\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01T\x84\x84a\x060V[`@\x80Q`\x80\x81\x01\x82R`\0` \x82\x01\x81\x90R``\x82\x01\x81\x90R0\x80\x83R\x82\x84\x01R\x91Qc\x94[\xCE\xC9`\xE0\x1B\x81R\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x94[\xCE\xC9\x90a\x01\xD0\x90\x84\x90\x8C\x90\x8C\x90\x8C\x90\x89\x90\x8D\x90\x8D\x90`\x04\x01a\x12KV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\x17\x91\x90\x81\x01\x90a\x13\xB5V[\x90P\x81`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB4\xBE.\xE7O6V\xD3\xA0/\xB5Q\xDE\xE6\xD3\xBE{\xE9<\x02\xBD\x1C\xB8\x01u\t\x1B_\xC7\xBD\xCD\xCF\x88`@Qa\x02`\x91\x90a\x14;V[`@Q\x80\x91\x03\x90\xA3a\x02r\x86\x82a\x07JV[\x92Pa\x02\xFF\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8B\x96\x855`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xFA\x91\x90a\x14NV[a\x08\xBEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cr\xB9\xFF\xC04F3\x8E\x88\x8F`@Q\x80` \x01`@R\x80`\0\x81RP`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03e\x96\x95\x94\x93\x92\x91\x90a\x14\xBBV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x03~W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x92W=`\0\x80>=`\0\xFD[PPPPP\x88`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x8B\x7F+\xBD*\x07;\xA3\xD7\xFB\xF0;t\x1B\xB6bnV\x84\xDD\xC17\xCA\xABH\xE4|a\x02\x076\xD8 \xB8\x86`@Qa\x03\xDB\x91\x90a\x10\xB2V[`@Q\x80\x91\x03\x90\xA4PPa\x03\xEF`\x01`\0UV[\x97\x96PPPPPPPV[``a\x04\x04a\x05\xD2V[\x85\x15\x80a\x04\x0FWP\x84\x15[\x15a\x04-W`@Qc=\x8F\x03[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q`\0\x03a\x04OW`@Qcwp\xE6\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04Y\x84\x84a\x060V[`@\x80Q`\x80\x81\x01\x82R`\0` \x80\x83\x01\x82\x90R``\x83\x01\x82\x90R0\x83R3\x83\x85\x01R\x83Q`\x1F\x8B\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x90\x94R\x89\x84R\x91\x92\x90\x91a\x04\xBC\x91\x90\x8B\x90\x8B\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8B\x92Pa\t\x17\x91PPV[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x94[\xCE\xC9`\0\x84\x8A\x87\x8B\x8B`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\x17\x96\x95\x94\x93\x92\x91\x90a\x15\x11V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x056W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05^\x91\x90\x81\x01\x90a\x13\xB5V[\x90P\x82`@\x01Q`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB4\xBE.\xE7O6V\xD3\xA0/\xB5Q\xDE\xE6\xD3\xBE{\xE9<\x02\xBD\x1C\xB8\x01u\t\x1B_\xC7\xBD\xCD\xCF\x89`@Qa\x05\xA7\x91\x90a\x14;V[`@Q\x80\x91\x03\x90\xA3a\x05\xB9\x87\x82a\x07JV[\x93PPPPa\x05\xC8`\x01`\0UV[\x96\x95PPPPPPV[`\x02`\0T\x03a\x06)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0[\x81Q\x81\x10\x15a\x07EW`\0\x82\x82\x81Q\x81\x10a\x06PWa\x06Pa\x16*V[` \x02` \x01\x01Q\x13\x15a\x07=W`\0\x83\x82\x81Q\x81\x10a\x06rWa\x06ra\x16*V[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x06\x90Wa\x06\x90a\x16*V[` \x02` \x01\x01Q\x90Pa\x06\xA6\x8230\x84a\n4V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x83\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x079\x91\x90a\x16@V[PPP[`\x01\x01a\x063V[PPPV[```\0\x80[\x83Q\x81\x10\x15a\x07\x91W`\0\x84\x82\x81Q\x81\x10a\x07mWa\x07ma\x16*V[` \x02` \x01\x01Q\x12\x15a\x07\x89W\x81a\x07\x85\x81a\x16xV[\x92PP[`\x01\x01a\x07PV[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xADWa\x07\xADa\x0EDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xF2W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\xCBW\x90P[P\x90P`\0\x80[\x85Q\x81\x10\x15a\x08\xB1W`\0\x86\x82\x81Q\x81\x10a\x08\x16Wa\x08\x16a\x16*V[` \x02` \x01\x01Q\x12\x15a\x08\xA9W`@Q\x80`@\x01`@R\x80\x88\x83\x81Q\x81\x10a\x08AWa\x08Aa\x16*V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87\x83\x81Q\x81\x10a\x08iWa\x08ia\x16*V[` \x02` \x01\x01Q`\0\x19a\x08~\x91\x90a\x16\x91V[\x90R\x83\x83a\x08\x8B\x81a\x16xV[\x94P\x81Q\x81\x10a\x08\x9DWa\x08\x9Da\x16*V[` \x02` \x01\x01\x81\x90RP[`\x01\x01a\x07\xF9V[P\x90\x92PPP[\x92\x91PPV[`\0[\x82Q\x81\x10\x15a\x07EWa\t\x0F\x83\x82\x81Q\x81\x10a\x08\xDFWa\x08\xDFa\x16*V[` \x02` \x01\x01Q`\0\x01Q\x83\x85\x84\x81Q\x81\x10a\x08\xFEWa\x08\xFEa\x16*V[` \x02` \x01\x01Q` \x01Qa\n\xA5V[`\x01\x01a\x08\xC1V[```\0\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t4Wa\t4a\x0EDV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x9AW\x81` \x01[a\t\x87`@Q\x80`\xA0\x01`@R\x80`\0\x80\x19\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\tRW\x90P[P\x90P`\x80`\0[\x84\x81\x10\x15a\n*W` \x82\x82\x02\x87\x01\x81\x81\x01Q`@\x80\x83\x01Q``\x80\x85\x01Q`\x80\x95\x86\x01Q\x84Q`\xA0\x81\x01\x86R\x86\x81R\x80\x89\x01\x85\x90R\x80\x86\x01\x83\x90R\x92\x83\x01\x81\x90R\x84Q\x97\x88\x01\x90\x94R`\0\x87R\x94\x81\x01\x95\x90\x95R\x87Q\x92\x94\x90\x93\x92\x88\x90\x87\x90\x81\x10a\n\x10Wa\n\x10a\x16*V[` \x02` \x01\x01\x81\x90RP\x84`\x01\x01\x94PPPPPa\t\xA2V[P\x90\x94\x93PPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\n\x9F\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x0B0V[PPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\n\xF6\x84\x82a\x0C\x05V[a\n\x9FW`@Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`\0`D\x82\x01Ra\x0B*\x90\x85\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01a\nhV[a\n\x9F\x84\x82[`\0a\x0B\x85\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xAC\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x0B\xA6WP\x80\x80` \x01\x90Q\x81\x01\x90a\x0B\xA6\x91\x90a\x16@V[a\x07EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x06 V[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x0C\"\x91\x90a\x16\xC1V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0C_W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0CdV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0C\x8EWP\x80Q\x15\x80a\x0C\x8EWP\x80\x80` \x01\x90Q\x81\x01\x90a\x0C\x8E\x91\x90a\x16@V[\x80\x15a\x0C\xA3WP`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15\x15[\x95\x94PPPPPV[``a\x0C\xBB\x84\x84`\0\x85a\x0C\xC3V[\x94\x93PPPPV[``\x82G\x10\x15a\r$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x06 V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\r@\x91\x90a\x16\xC1V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\r}W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\x82V[``\x91P[P\x91P\x91Pa\x03\xEF\x87\x83\x83\x87``\x83\x15a\r\xFDW\x82Q`\0\x03a\r\xF6W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\r\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x06 V[P\x81a\x0C\xBBV[a\x0C\xBB\x83\x83\x81Q\x15a\x0E\x12W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06 \x91\x90a\x16\xDDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0EAW`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\x83Wa\x0E\x83a\x0EDV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xA5Wa\x0E\xA5a\x0EDV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0E\xC0W`\0\x80\xFD[\x815` a\x0E\xD5a\x0E\xD0\x83a\x0E\x8BV[a\x0EZV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0E\xF4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0F\x18W\x805a\x0F\x0B\x81a\x0E,V[\x83R\x91\x83\x01\x91\x83\x01a\x0E\xF8V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x0F4W`\0\x80\xFD[\x815` a\x0FDa\x0E\xD0\x83a\x0E\x8BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0FcW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0F\x18W\x805\x83R\x91\x83\x01\x91\x83\x01a\x0FgV[`\0\x80`\0\x80`\0\x80`\0`\xC0\x88\x8A\x03\x12\x15a\x0F\x99W`\0\x80\xFD[\x875\x96P` \x88\x015a\x0F\xAB\x81a\x0E,V[\x95P`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xC8W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x0F\xDCW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\xEBW`\0\x80\xFD[\x8B` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x10\0W`\0\x80\xFD[` \x83\x01\x97P\x80\x96PP``\x8A\x015\x91P\x80\x82\x11\x15a\x10\x1EW`\0\x80\xFD[a\x10*\x8B\x83\x8C\x01a\x0E\xAFV[\x94P`\x80\x8A\x015\x91P\x80\x82\x11\x15a\x10@W`\0\x80\xFD[Pa\x10M\x8A\x82\x8B\x01a\x0F#V[\x92PP`\xA0\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x10\xA7W\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x88R\x83\x01Q\x83\x88\x01R`@\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01a\x10wV[P\x94\x95\x94PPPPPV[` \x81R`\0a\x10\xC5` \x83\x01\x84a\x10cV[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x10\xE5W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\xFDW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x11\x11W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11 W`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x112W`\0\x80\xFD[` \x92\x83\x01\x98P\x96P\x90\x88\x015\x94P`@\x88\x015\x90\x80\x82\x11\x15a\x11TW`\0\x80\xFD[a\x11`\x8A\x83\x8B\x01a\x0E\xAFV[\x94P``\x89\x015\x91P\x80\x82\x11\x15a\x11vW`\0\x80\xFD[Pa\x11\x83\x89\x82\x8A\x01a\x0F#V[\x92PP`\x80\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\x02\x81\x10a\x11\xB5WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x10\xA7W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x11\xF6V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x10\xA7W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x12/V[`\0a\x01 \x80\x83\x01a\x12]\x84\x8Ca\x11\x97V[` \x84\x81\x01\x92\x90\x92R\x88\x90Ra\x01@\x80\x84\x01\x91`\x05\x8A\x90\x1B\x85\x01\x90\x91\x01\x90\x8A`\0[\x8B\x81\x10\x15a\x138W\x86\x84\x03a\x01?\x19\x01\x85R\x8156\x8E\x90\x03`\x9E\x19\x01\x81\x12a\x12\xA6W`\0\x80\xFD[\x8D\x01\x805\x85R\x83\x81\x015\x84\x86\x01R`@\x80\x82\x015\x90\x86\x01R``\x80\x82\x015\x90\x86\x01R`\xA0`\x80\x80\x83\x0156\x84\x90\x03`\x1E\x19\x01\x81\x12a\x12\xE3W`\0\x80\xFD[\x90\x92\x01\x85\x81\x01\x92\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x01W`\0\x80\xFD[\x806\x03\x84\x13\x15a\x13\x10W`\0\x80\xFD[\x82\x82\x89\x01Ra\x13\"\x83\x89\x01\x82\x86a\x11\xB9V[\x98\x87\x01\x98\x97PPP\x92\x84\x01\x92PP`\x01\x01a\x12\x7FV[PPP\x83\x81\x03`@\x85\x01Ra\x13M\x81\x89a\x11\xE2V[\x91PPa\x13\x8D``\x84\x01\x87\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x15\x15\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x90\x81\x01Q\x15\x15\x91\x01RV[\x82\x81\x03`\xE0\x84\x01Ra\x13\x9F\x81\x86a\x12\x1BV[\x91PP\x82a\x01\0\x83\x01R\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x13\xC8W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xDFW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x13\xF0W`\0\x80\xFD[\x80Qa\x13\xFEa\x0E\xD0\x82a\x0E\x8BV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x14\x1DW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x03\xEFW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x14\"V[` \x81R`\0a\x10\xC5` \x83\x01\x84a\x11\xE2V[`\0` \x82\x84\x03\x12\x15a\x14`W`\0\x80\xFD[\x81Qa\x10\xC5\x81a\x0E,V[`\0[\x83\x81\x10\x15a\x14\x86W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14nV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x14\xA7\x81` \x86\x01` \x86\x01a\x14kV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x86\x81R`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16` \x84\x01R\x86`@\x84\x01R`\xC0``\x84\x01Ra\x14\xE9`\xC0\x84\x01\x87a\x10cV[\x81\x86\x16`\x80\x85\x01R\x83\x81\x03`\xA0\x85\x01Ra\x15\x03\x81\x86a\x14\x8FV[\x9A\x99PPPPPPPPPPV[`\0a\x01 \x80\x83\x01a\x15#\x84\x8Ba\x11\x97V[` \x84\x81\x01\x92\x90\x92R\x88Q\x90\x81\x90Ra\x01@\x80\x85\x01\x92`\x05\x83\x90\x1B\x86\x01\x90\x91\x01\x91\x8A\x82\x01`\0[\x82\x81\x10\x15a\x15\xADW\x87\x85\x03a\x01?\x19\x01\x86R\x81Q\x80Q\x86R\x84\x81\x01Q\x85\x87\x01R`@\x80\x82\x01Q\x90\x87\x01R``\x80\x82\x01Q\x90\x87\x01R`\x80\x90\x81\x01Q`\xA0\x91\x87\x01\x82\x90R\x90a\x15\x99\x81\x88\x01\x83a\x14\x8FV[\x97\x86\x01\x97\x96PPP\x90\x83\x01\x90`\x01\x01a\x15JV[PPPP\x83\x81\x03`@\x85\x01Ra\x15\xC3\x81\x89a\x11\xE2V[\x91PPa\x16\x03``\x84\x01\x87\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x15\x15\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x90\x81\x01Q\x15\x15\x91\x01RV[\x82\x81\x03`\xE0\x84\x01Ra\x16\x15\x81\x86a\x12\x1BV[\x91PP\x82a\x01\0\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x16RW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x10\xC5W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x16\x8AWa\x16\x8Aa\x16bV[P`\x01\x01\x90V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x16\xADWa\x16\xADa\x16bV[\x81\x81\x05\x83\x14\x82\x15\x17a\x08\xB8Wa\x08\xB8a\x16bV[`\0\x82Qa\x16\xD3\x81\x84` \x87\x01a\x14kV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x10\xC5` \x83\x01\x84a\x14\x8FV\xFE\xA2dipfsX\"\x12 \xF0XR\xCCL\x94(\xBB\xC0w\xEC#\xF5,\x8F\xECB\xE1\xE0]\x93\xCC\xBF\xFF\xFC>\xAA\xD0\xFFjP5dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static INTERCHAINLIQUIDITYHUBWRAPPER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct InterchainLiquidityHubWrapper<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for InterchainLiquidityHubWrapper<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for InterchainLiquidityHubWrapper<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for InterchainLiquidityHubWrapper<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for InterchainLiquidityHubWrapper<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(InterchainLiquidityHubWrapper))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> InterchainLiquidityHubWrapper<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INTERCHAINLIQUIDITYHUBWRAPPER_ABI.clone(),
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
                INTERCHAINLIQUIDITYHUBWRAPPER_ABI.clone(),
                INTERCHAINLIQUIDITYHUBWRAPPER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `balancerVault` (0x158274a5) function
        pub fn balancer_vault(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([21, 130, 116, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeSwap` (0x642182f3) function
        pub fn execute_swap(
            &self,
            swaps: ::ethers::core::types::Bytes,
            swap_count: ::ethers::core::types::U256,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            limits: ::std::vec::Vec<::ethers::core::types::I256>,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Token>> {
            self.0
                .method_hash(
                    [100, 33, 130, 243],
                    (swaps, swap_count, assets, limits, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `interchainMessagingGateway` (0x09217da9) function
        pub fn interchain_messaging_gateway(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([9, 33, 125, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawLiquidity` (0x13449704) function
        pub fn withdraw_liquidity(
            &self,
            chain_id: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            swaps: ::std::vec::Vec<BatchSwapStep>,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            limits: ::std::vec::Vec<::ethers::core::types::I256>,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Token>> {
            self.0
                .method_hash(
                    [19, 68, 151, 4],
                    (chain_id, receiver, swaps, assets, limits, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `LiquidityWithdrawn` event
        pub fn liquidity_withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidityWithdrawnFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SwapExecuted` event
        pub fn swap_executed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SwapExecutedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InterchainLiquidityHubWrapperEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for InterchainLiquidityHubWrapper<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InterchainLiquidityHubWrapper__NoAssetsToSwap` with signature `InterchainLiquidityHubWrapper__NoAssetsToSwap()` and selector `0x7770e6a3`
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
        name = "InterchainLiquidityHubWrapper__NoAssetsToSwap",
        abi = "InterchainLiquidityHubWrapper__NoAssetsToSwap()"
    )]
    pub struct InterchainLiquidityHubWrapper__NoAssetsToSwap;
    ///Custom Error type `InterchainLiquidityHubWrapper__NoSwapsToExecute` with signature `InterchainLiquidityHubWrapper__NoSwapsToExecute()` and selector `0x7b1e06b6`
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
        name = "InterchainLiquidityHubWrapper__NoSwapsToExecute",
        abi = "InterchainLiquidityHubWrapper__NoSwapsToExecute()"
    )]
    pub struct InterchainLiquidityHubWrapper__NoSwapsToExecute;
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
    pub enum InterchainLiquidityHubWrapperErrors {
        InterchainLiquidityHubWrapper__NoAssetsToSwap(
            InterchainLiquidityHubWrapper__NoAssetsToSwap,
        ),
        InterchainLiquidityHubWrapper__NoSwapsToExecute(
            InterchainLiquidityHubWrapper__NoSwapsToExecute,
        ),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for InterchainLiquidityHubWrapperErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InterchainLiquidityHubWrapper__NoAssetsToSwap as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InterchainLiquidityHubWrapper__NoAssetsToSwap(decoded));
            }
            if let Ok(decoded) = <InterchainLiquidityHubWrapper__NoSwapsToExecute as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::InterchainLiquidityHubWrapper__NoSwapsToExecute(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for InterchainLiquidityHubWrapperErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InterchainLiquidityHubWrapper__NoAssetsToSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InterchainLiquidityHubWrapper__NoSwapsToExecute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for InterchainLiquidityHubWrapperErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InterchainLiquidityHubWrapper__NoAssetsToSwap as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InterchainLiquidityHubWrapper__NoSwapsToExecute as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for InterchainLiquidityHubWrapperErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InterchainLiquidityHubWrapper__NoAssetsToSwap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InterchainLiquidityHubWrapper__NoSwapsToExecute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for InterchainLiquidityHubWrapperErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InterchainLiquidityHubWrapper__NoAssetsToSwap>
    for InterchainLiquidityHubWrapperErrors {
        fn from(value: InterchainLiquidityHubWrapper__NoAssetsToSwap) -> Self {
            Self::InterchainLiquidityHubWrapper__NoAssetsToSwap(value)
        }
    }
    impl ::core::convert::From<InterchainLiquidityHubWrapper__NoSwapsToExecute>
    for InterchainLiquidityHubWrapperErrors {
        fn from(value: InterchainLiquidityHubWrapper__NoSwapsToExecute) -> Self {
            Self::InterchainLiquidityHubWrapper__NoSwapsToExecute(value)
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
        name = "LiquidityWithdrawn",
        abi = "LiquidityWithdrawn(uint256,address,address,(address,uint256)[])"
    )]
    pub struct LiquidityWithdrawnFilter {
        #[ethevent(indexed)]
        pub chain_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<Token>,
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
    #[ethevent(name = "SwapExecuted", abi = "SwapExecuted(address,address,address[])")]
    pub struct SwapExecutedFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all of the contract's events
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
    pub enum InterchainLiquidityHubWrapperEvents {
        LiquidityWithdrawnFilter(LiquidityWithdrawnFilter),
        SwapExecutedFilter(SwapExecutedFilter),
    }
    impl ::ethers::contract::EthLogDecode for InterchainLiquidityHubWrapperEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LiquidityWithdrawnFilter::decode_log(log) {
                return Ok(
                    InterchainLiquidityHubWrapperEvents::LiquidityWithdrawnFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = SwapExecutedFilter::decode_log(log) {
                return Ok(
                    InterchainLiquidityHubWrapperEvents::SwapExecutedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for InterchainLiquidityHubWrapperEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LiquidityWithdrawnFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapExecutedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<LiquidityWithdrawnFilter>
    for InterchainLiquidityHubWrapperEvents {
        fn from(value: LiquidityWithdrawnFilter) -> Self {
            Self::LiquidityWithdrawnFilter(value)
        }
    }
    impl ::core::convert::From<SwapExecutedFilter>
    for InterchainLiquidityHubWrapperEvents {
        fn from(value: SwapExecutedFilter) -> Self {
            Self::SwapExecutedFilter(value)
        }
    }
    ///Container type for all input parameters for the `balancerVault` function with signature `balancerVault()` and selector `0x158274a5`
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
    #[ethcall(name = "balancerVault", abi = "balancerVault()")]
    pub struct BalancerVaultCall;
    ///Container type for all input parameters for the `executeSwap` function with signature `executeSwap(bytes,uint256,address[],int256[],uint256)` and selector `0x642182f3`
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
        name = "executeSwap",
        abi = "executeSwap(bytes,uint256,address[],int256[],uint256)"
    )]
    pub struct ExecuteSwapCall {
        pub swaps: ::ethers::core::types::Bytes,
        pub swap_count: ::ethers::core::types::U256,
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub limits: ::std::vec::Vec<::ethers::core::types::I256>,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `interchainMessagingGateway` function with signature `interchainMessagingGateway()` and selector `0x09217da9`
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
    #[ethcall(name = "interchainMessagingGateway", abi = "interchainMessagingGateway()")]
    pub struct InterchainMessagingGatewayCall;
    ///Container type for all input parameters for the `withdrawLiquidity` function with signature `withdrawLiquidity(uint256,address,(bytes32,uint256,uint256,uint256,bytes)[],address[],int256[],uint256)` and selector `0x13449704`
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
        name = "withdrawLiquidity",
        abi = "withdrawLiquidity(uint256,address,(bytes32,uint256,uint256,uint256,bytes)[],address[],int256[],uint256)"
    )]
    pub struct WithdrawLiquidityCall {
        pub chain_id: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub swaps: ::std::vec::Vec<BatchSwapStep>,
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub limits: ::std::vec::Vec<::ethers::core::types::I256>,
        pub deadline: ::ethers::core::types::U256,
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
    pub enum InterchainLiquidityHubWrapperCalls {
        BalancerVault(BalancerVaultCall),
        ExecuteSwap(ExecuteSwapCall),
        InterchainMessagingGateway(InterchainMessagingGatewayCall),
        WithdrawLiquidity(WithdrawLiquidityCall),
    }
    impl ::ethers::core::abi::AbiDecode for InterchainLiquidityHubWrapperCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BalancerVaultCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalancerVault(decoded));
            }
            if let Ok(decoded) = <ExecuteSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteSwap(decoded));
            }
            if let Ok(decoded) = <InterchainMessagingGatewayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InterchainMessagingGateway(decoded));
            }
            if let Ok(decoded) = <WithdrawLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawLiquidity(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for InterchainLiquidityHubWrapperCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BalancerVault(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InterchainMessagingGateway(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for InterchainLiquidityHubWrapperCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BalancerVault(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::InterchainMessagingGateway(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawLiquidity(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BalancerVaultCall>
    for InterchainLiquidityHubWrapperCalls {
        fn from(value: BalancerVaultCall) -> Self {
            Self::BalancerVault(value)
        }
    }
    impl ::core::convert::From<ExecuteSwapCall> for InterchainLiquidityHubWrapperCalls {
        fn from(value: ExecuteSwapCall) -> Self {
            Self::ExecuteSwap(value)
        }
    }
    impl ::core::convert::From<InterchainMessagingGatewayCall>
    for InterchainLiquidityHubWrapperCalls {
        fn from(value: InterchainMessagingGatewayCall) -> Self {
            Self::InterchainMessagingGateway(value)
        }
    }
    impl ::core::convert::From<WithdrawLiquidityCall>
    for InterchainLiquidityHubWrapperCalls {
        fn from(value: WithdrawLiquidityCall) -> Self {
            Self::WithdrawLiquidity(value)
        }
    }
    ///Container type for all return fields from the `balancerVault` function with signature `balancerVault()` and selector `0x158274a5`
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
    pub struct BalancerVaultReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `executeSwap` function with signature `executeSwap(bytes,uint256,address[],int256[],uint256)` and selector `0x642182f3`
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
    pub struct ExecuteSwapReturn {
        pub tokens: ::std::vec::Vec<Token>,
    }
    ///Container type for all return fields from the `interchainMessagingGateway` function with signature `interchainMessagingGateway()` and selector `0x09217da9`
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
    pub struct InterchainMessagingGatewayReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `withdrawLiquidity` function with signature `withdrawLiquidity(uint256,address,(bytes32,uint256,uint256,uint256,bytes)[],address[],int256[],uint256)` and selector `0x13449704`
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
    pub struct WithdrawLiquidityReturn {
        pub tokens: ::std::vec::Vec<Token>,
    }
}
