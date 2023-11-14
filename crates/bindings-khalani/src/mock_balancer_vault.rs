pub use mock_balancer_vault::*;
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
pub mod mock_balancer_vault {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_bptToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("batchSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batchSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("kind"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum IVault.SwapKind"),
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
                                    name: ::std::borrow::ToOwned::to_owned("funds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct FundManagement"),
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
                                    name: ::std::borrow::ToOwned::to_owned("assetDeltas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bpt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bpt"),
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
                    ::std::borrow::ToOwned::to_owned("queryBatchSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("queryBatchSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("kind"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum IVault.SwapKind"),
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
                                    name: ::std::borrow::ToOwned::to_owned("funds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct FundManagement"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assetDeltas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPass"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPass"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_shouldPass"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKBALANCERVAULT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\0\x80T`\xFF\x19\x16`\x01\x17\x90U4\x80\x15a\0\x1DW`\0\x80\xFD[P`@Qa\r\xDA8\x03\x80a\r\xDA\x839\x81\x01`@\x81\x90Ra\0<\x91a\0gV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x97V[`\0` \x82\x84\x03\x12\x15a\0yW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x90W`\0\x80\xFD[\x93\x92PPPV[a\r4\x80a\0\xA6`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0?W`\x005`\xE0\x1C\x80cTj\xF3\xC3\x14a\0DW\x80c\x94[\xCE\xC9\x14a\0\x86W\x80c\xF3\x9E\xA9\xD0\x14a\0\xA6W\x80c\xF8M\x06n\x14a\0\xD6W[`\0\x80\xFD[4\x80\x15a\0PW`\0\x80\xFD[P`\0Ta\0i\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x99a\0\x946`\x04a\t\xFAV[a\0\xF6V[`@Qa\0}\x91\x90a\x0B\x05V[4\x80\x15a\0\xB2W`\0\x80\xFD[Pa\0\xD4a\0\xC16`\x04a\x0BIV[`\0\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[4\x80\x15a\0\xE2W`\0\x80\xFD[Pa\0\x99a\0\xF16`\x04a\x0BmV[a\x02~V[``a\x01\x02\x85\x84a\x04\x0FV[`\0T`\xFF\x16\x15a\x02\x1BW`\0[\x85Q\x81\x10\x15a\x02\x12W`\0\x84\x82\x81Q\x81\x10a\x01-Wa\x01-a\x0B\xF3V[` \x02` \x01\x01Q\x12\x15a\x02\0W\x85\x81\x81Q\x81\x10a\x01MWa\x01Ma\x0B\xF3V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x86`@\x01Q\x86\x84\x81Q\x81\x10a\x01zWa\x01za\x0B\xF3V[` \x02` \x01\x01Q`\0\x19a\x01\x8F\x91\x90a\x0C\x1FV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFE\x91\x90a\x0CUV[P[\x80a\x02\n\x81a\x0CrV[\x91PPa\x01\x10V[P\x82\x90Pa\x02tV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FMockBalancerVault: batchSwap fai`D\x82\x01Rb\x1B\x19Y`\xEA\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x96\x95PPPPPPV[``\x80\x84Q`\x01\x03a\x038W`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x84`\0\x81Q\x81\x10a\x02\xBDWa\x02\xBDa\x0B\xF3V[` \x02` \x01\x01Q``\x01Q\x81`\0\x81Q\x81\x10a\x02\xDCWa\x02\xDCa\x0B\xF3V[` \x02` \x01\x01\x81\x81RPP\x84`\0\x81Q\x81\x10a\x02\xFBWa\x02\xFBa\x0B\xF3V[` \x02` \x01\x01Q``\x01Q`\0\x19a\x03\x14\x91\x90a\x0C\x1FV[\x81`\x01\x81Q\x81\x10a\x03'Wa\x03'a\x0B\xF3V[` \x02` \x01\x01\x81\x81RPPa\x04\x04V[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x90P\x84`\0\x81Q\x81\x10a\x03lWa\x03la\x0B\xF3V[` \x02` \x01\x01Q``\x01Q\x81`\0\x81Q\x81\x10a\x03\x8BWa\x03\x8Ba\x0B\xF3V[` \x02` \x01\x01\x81\x81RPP\x84`\0\x81Q\x81\x10a\x03\xAAWa\x03\xAAa\x0B\xF3V[` \x02` \x01\x01Q``\x01Q`\0\x19a\x03\xC3\x91\x90a\x0C\x1FV[\x81`\x01\x81Q\x81\x10a\x03\xD6Wa\x03\xD6a\x0B\xF3V[` \x02` \x01\x01\x81\x81RPP`\0\x81`\x02\x81Q\x81\x10a\x03\xF7Wa\x03\xF7a\x0B\xF3V[` \x02` \x01\x01\x81\x81RPP[\x90P[\x94\x93PPPPV[`\0[\x81Q\x81\x10\x15a\x04\x90W`\0\x82\x82\x81Q\x81\x10a\x04/Wa\x04/a\x0B\xF3V[` \x02` \x01\x01Q\x13\x15a\x04\x88W`\0\x83\x82\x81Q\x81\x10a\x04QWa\x04Qa\x0B\xF3V[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x04oWa\x04oa\x0B\xF3V[` \x02` \x01\x01Q\x90Pa\x04\x85\x8230\x84a\x04\x95V[PP[`\x01\x01a\x04\x12V[PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x04\xEF\x90\x85\x90a\x04\xF5V[PPPPV[`\0a\x05J\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x05\xCA\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x05kWP\x80\x80` \x01\x90Q\x81\x01\x90a\x05k\x91\x90a\x0CUV[a\x04\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x02kV[``a\x04\x07\x84\x84`\0\x85\x85`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x05\xF1\x91\x90a\x0C\xAFV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x06.W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x063V[``\x91P[P\x91P\x91Pa\x06D\x87\x83\x83\x87a\x06OV[\x97\x96PPPPPPPV[``\x83\x15a\x06\xBEW\x82Q`\0\x03a\x06\xB7W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x06\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x02kV[P\x81a\x04\x07V[a\x04\x07\x83\x83\x81Q\x15a\x06\xD3W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02k\x91\x90a\x0C\xCBV[\x805`\x02\x81\x10a\x06\xFCW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07:Wa\x07:a\x07\x01V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07iWa\x07ia\x07\x01V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\x8BWa\x07\x8Ba\x07\x01V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\xA6W`\0\x80\xFD[\x815` a\x07\xBBa\x07\xB6\x83a\x07qV[a\x07@V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x07\xDAW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x08\xDCW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xFFW`\0\x80\x81\xFD[\x90\x88\x01\x90`\xA0`\x1F\x19\x83\x8C\x03\x81\x01\x82\x13\x15a\x08\x1AW`\0\x80\x81\xFD[a\x08\"a\x07\x17V[\x84\x89\x015\x81R`@\x80\x86\x015\x82\x8B\x01R``\x80\x87\x015\x82\x84\x01R`\x80\x80\x88\x015\x82\x85\x01R\x94\x87\x015\x94\x86\x86\x11\x15a\x08[W`\0\x91P\x81\x82\xFD[\x85\x88\x01\x97P\x8F`?\x89\x01\x12a\x08rW`\0\x95P\x85\x86\xFD[\x8B\x88\x015\x95P\x86\x86\x11\x15a\x08\x88Wa\x08\x88a\x07\x01V[a\x08\x98\x8C\x86`\x1F\x89\x01\x16\x01a\x07@V[\x96P\x85\x87R\x8F\x83\x87\x8A\x01\x01\x11\x15a\x08\xB1W`\0\x94P\x84\x85\xFD[\x85\x83\x89\x01\x8D\x89\x017`\0\x95\x87\x01\x8C\x01\x95\x90\x95RPP\x91\x82\x01\x92\x90\x92R\x85RPP\x91\x83\x01\x91\x83\x01a\x07\xDEV[P\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xFCW`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\t\x10W`\0\x80\xFD[\x815` a\t a\x07\xB6\x83a\x07qV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\t?W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x08\xDCW\x805a\tV\x81a\x08\xE7V[\x83R\x91\x83\x01\x91\x83\x01a\tCV[\x80\x15\x15\x81\x14a\x08\xFCW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a\t\x83W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\t\xA6Wa\t\xA6a\x07\x01V[`@R\x90P\x80\x825a\t\xB7\x81a\x08\xE7V[\x81R` \x83\x015a\t\xC7\x81a\tcV[` \x82\x01R`@\x83\x015a\t\xDA\x81a\x08\xE7V[`@\x82\x01R``\x83\x015a\t\xED\x81a\tcV[``\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80a\x01 \x87\x89\x03\x12\x15a\n\x14W`\0\x80\xFD[a\n\x1D\x87a\x06\xEDV[\x95P` \x80\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n;W`\0\x80\xFD[a\nG\x8B\x83\x8C\x01a\x07\x95V[\x97P`@\x8A\x015\x91P\x80\x82\x11\x15a\n]W`\0\x80\xFD[a\ni\x8B\x83\x8C\x01a\x08\xFFV[\x96Pa\nx\x8B``\x8C\x01a\tqV[\x95P`\xE0\x8A\x015\x91P\x80\x82\x11\x15a\n\x8EW`\0\x80\xFD[P\x88\x01`\x1F\x81\x01\x8A\x13a\n\xA0W`\0\x80\xFD[\x805a\n\xAEa\x07\xB6\x82a\x07qV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8C\x83\x11\x15a\n\xCDW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\n\xEBW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\n\xD2V[\x80\x96PPPPPPa\x01\0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0B=W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0B!V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0B[W`\0\x80\xFD[\x815a\x0Bf\x81a\tcV[\x93\x92PPPV[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a\x0B\x83W`\0\x80\xFD[a\x0B\x8C\x85a\x06\xEDV[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xA9W`\0\x80\xFD[a\x0B\xB5\x88\x83\x89\x01a\x07\x95V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x0B\xCBW`\0\x80\xFD[Pa\x0B\xD8\x87\x82\x88\x01a\x08\xFFV[\x92PPa\x0B\xE8\x86``\x87\x01a\tqV[\x90P\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x0C;Wa\x0C;a\x0C\tV[\x81\x81\x05\x83\x14\x82\x15\x17a\x0COWa\x0COa\x0C\tV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0CgW`\0\x80\xFD[\x81Qa\x0Bf\x81a\tcV[`\0`\x01\x82\x01a\x0C\x84Wa\x0C\x84a\x0C\tV[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15a\x0C\xA6W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0C\x8EV[PP`\0\x91\x01RV[`\0\x82Qa\x0C\xC1\x81\x84` \x87\x01a\x0C\x8BV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C\xEA\x81`@\x85\x01` \x87\x01a\x0C\x8BV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xF2K\xC7$\xB9R\x8F\x17\x1B\xB8\x90\x8B\xC2K\\\xC6\xA0\x02\xFCR\x91L<g\x92vQ\x8CX2\xBB\x10dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKBALANCERVAULT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0?W`\x005`\xE0\x1C\x80cTj\xF3\xC3\x14a\0DW\x80c\x94[\xCE\xC9\x14a\0\x86W\x80c\xF3\x9E\xA9\xD0\x14a\0\xA6W\x80c\xF8M\x06n\x14a\0\xD6W[`\0\x80\xFD[4\x80\x15a\0PW`\0\x80\xFD[P`\0Ta\0i\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\x99a\0\x946`\x04a\t\xFAV[a\0\xF6V[`@Qa\0}\x91\x90a\x0B\x05V[4\x80\x15a\0\xB2W`\0\x80\xFD[Pa\0\xD4a\0\xC16`\x04a\x0BIV[`\0\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[4\x80\x15a\0\xE2W`\0\x80\xFD[Pa\0\x99a\0\xF16`\x04a\x0BmV[a\x02~V[``a\x01\x02\x85\x84a\x04\x0FV[`\0T`\xFF\x16\x15a\x02\x1BW`\0[\x85Q\x81\x10\x15a\x02\x12W`\0\x84\x82\x81Q\x81\x10a\x01-Wa\x01-a\x0B\xF3V[` \x02` \x01\x01Q\x12\x15a\x02\0W\x85\x81\x81Q\x81\x10a\x01MWa\x01Ma\x0B\xF3V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBB\x86`@\x01Q\x86\x84\x81Q\x81\x10a\x01zWa\x01za\x0B\xF3V[` \x02` \x01\x01Q`\0\x19a\x01\x8F\x91\x90a\x0C\x1FV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xFE\x91\x90a\x0CUV[P[\x80a\x02\n\x81a\x0CrV[\x91PPa\x01\x10V[P\x82\x90Pa\x02tV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FMockBalancerVault: batchSwap fai`D\x82\x01Rb\x1B\x19Y`\xEA\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x96\x95PPPPPPV[``\x80\x84Q`\x01\x03a\x038W`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x84`\0\x81Q\x81\x10a\x02\xBDWa\x02\xBDa\x0B\xF3V[` \x02` \x01\x01Q``\x01Q\x81`\0\x81Q\x81\x10a\x02\xDCWa\x02\xDCa\x0B\xF3V[` \x02` \x01\x01\x81\x81RPP\x84`\0\x81Q\x81\x10a\x02\xFBWa\x02\xFBa\x0B\xF3V[` \x02` \x01\x01Q``\x01Q`\0\x19a\x03\x14\x91\x90a\x0C\x1FV[\x81`\x01\x81Q\x81\x10a\x03'Wa\x03'a\x0B\xF3V[` \x02` \x01\x01\x81\x81RPPa\x04\x04V[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01``\x806\x837\x01\x90PP\x90P\x84`\0\x81Q\x81\x10a\x03lWa\x03la\x0B\xF3V[` \x02` \x01\x01Q``\x01Q\x81`\0\x81Q\x81\x10a\x03\x8BWa\x03\x8Ba\x0B\xF3V[` \x02` \x01\x01\x81\x81RPP\x84`\0\x81Q\x81\x10a\x03\xAAWa\x03\xAAa\x0B\xF3V[` \x02` \x01\x01Q``\x01Q`\0\x19a\x03\xC3\x91\x90a\x0C\x1FV[\x81`\x01\x81Q\x81\x10a\x03\xD6Wa\x03\xD6a\x0B\xF3V[` \x02` \x01\x01\x81\x81RPP`\0\x81`\x02\x81Q\x81\x10a\x03\xF7Wa\x03\xF7a\x0B\xF3V[` \x02` \x01\x01\x81\x81RPP[\x90P[\x94\x93PPPPV[`\0[\x81Q\x81\x10\x15a\x04\x90W`\0\x82\x82\x81Q\x81\x10a\x04/Wa\x04/a\x0B\xF3V[` \x02` \x01\x01Q\x13\x15a\x04\x88W`\0\x83\x82\x81Q\x81\x10a\x04QWa\x04Qa\x0B\xF3V[` \x02` \x01\x01Q\x90P`\0\x83\x83\x81Q\x81\x10a\x04oWa\x04oa\x0B\xF3V[` \x02` \x01\x01Q\x90Pa\x04\x85\x8230\x84a\x04\x95V[PP[`\x01\x01a\x04\x12V[PPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x04\xEF\x90\x85\x90a\x04\xF5V[PPPPV[`\0a\x05J\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x05\xCA\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x05kWP\x80\x80` \x01\x90Q\x81\x01\x90a\x05k\x91\x90a\x0CUV[a\x04\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x02kV[``a\x04\x07\x84\x84`\0\x85\x85`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x05\xF1\x91\x90a\x0C\xAFV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x06.W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x063V[``\x91P[P\x91P\x91Pa\x06D\x87\x83\x83\x87a\x06OV[\x97\x96PPPPPPPV[``\x83\x15a\x06\xBEW\x82Q`\0\x03a\x06\xB7W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x06\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x02kV[P\x81a\x04\x07V[a\x04\x07\x83\x83\x81Q\x15a\x06\xD3W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02k\x91\x90a\x0C\xCBV[\x805`\x02\x81\x10a\x06\xFCW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07:Wa\x07:a\x07\x01V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07iWa\x07ia\x07\x01V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x07\x8BWa\x07\x8Ba\x07\x01V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x07\xA6W`\0\x80\xFD[\x815` a\x07\xBBa\x07\xB6\x83a\x07qV[a\x07@V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x07\xDAW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x08\xDCW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x07\xFFW`\0\x80\x81\xFD[\x90\x88\x01\x90`\xA0`\x1F\x19\x83\x8C\x03\x81\x01\x82\x13\x15a\x08\x1AW`\0\x80\x81\xFD[a\x08\"a\x07\x17V[\x84\x89\x015\x81R`@\x80\x86\x015\x82\x8B\x01R``\x80\x87\x015\x82\x84\x01R`\x80\x80\x88\x015\x82\x85\x01R\x94\x87\x015\x94\x86\x86\x11\x15a\x08[W`\0\x91P\x81\x82\xFD[\x85\x88\x01\x97P\x8F`?\x89\x01\x12a\x08rW`\0\x95P\x85\x86\xFD[\x8B\x88\x015\x95P\x86\x86\x11\x15a\x08\x88Wa\x08\x88a\x07\x01V[a\x08\x98\x8C\x86`\x1F\x89\x01\x16\x01a\x07@V[\x96P\x85\x87R\x8F\x83\x87\x8A\x01\x01\x11\x15a\x08\xB1W`\0\x94P\x84\x85\xFD[\x85\x83\x89\x01\x8D\x89\x017`\0\x95\x87\x01\x8C\x01\x95\x90\x95RPP\x91\x82\x01\x92\x90\x92R\x85RPP\x91\x83\x01\x91\x83\x01a\x07\xDEV[P\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xFCW`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\t\x10W`\0\x80\xFD[\x815` a\t a\x07\xB6\x83a\x07qV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\t?W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x08\xDCW\x805a\tV\x81a\x08\xE7V[\x83R\x91\x83\x01\x91\x83\x01a\tCV[\x80\x15\x15\x81\x14a\x08\xFCW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a\t\x83W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\t\xA6Wa\t\xA6a\x07\x01V[`@R\x90P\x80\x825a\t\xB7\x81a\x08\xE7V[\x81R` \x83\x015a\t\xC7\x81a\tcV[` \x82\x01R`@\x83\x015a\t\xDA\x81a\x08\xE7V[`@\x82\x01R``\x83\x015a\t\xED\x81a\tcV[``\x91\x90\x91\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80a\x01 \x87\x89\x03\x12\x15a\n\x14W`\0\x80\xFD[a\n\x1D\x87a\x06\xEDV[\x95P` \x80\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n;W`\0\x80\xFD[a\nG\x8B\x83\x8C\x01a\x07\x95V[\x97P`@\x8A\x015\x91P\x80\x82\x11\x15a\n]W`\0\x80\xFD[a\ni\x8B\x83\x8C\x01a\x08\xFFV[\x96Pa\nx\x8B``\x8C\x01a\tqV[\x95P`\xE0\x8A\x015\x91P\x80\x82\x11\x15a\n\x8EW`\0\x80\xFD[P\x88\x01`\x1F\x81\x01\x8A\x13a\n\xA0W`\0\x80\xFD[\x805a\n\xAEa\x07\xB6\x82a\x07qV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8C\x83\x11\x15a\n\xCDW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\n\xEBW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\n\xD2V[\x80\x96PPPPPPa\x01\0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0B=W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0B!V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0B[W`\0\x80\xFD[\x815a\x0Bf\x81a\tcV[\x93\x92PPPV[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a\x0B\x83W`\0\x80\xFD[a\x0B\x8C\x85a\x06\xEDV[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xA9W`\0\x80\xFD[a\x0B\xB5\x88\x83\x89\x01a\x07\x95V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x0B\xCBW`\0\x80\xFD[Pa\x0B\xD8\x87\x82\x88\x01a\x08\xFFV[\x92PPa\x0B\xE8\x86``\x87\x01a\tqV[\x90P\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x0C;Wa\x0C;a\x0C\tV[\x81\x81\x05\x83\x14\x82\x15\x17a\x0COWa\x0COa\x0C\tV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0CgW`\0\x80\xFD[\x81Qa\x0Bf\x81a\tcV[`\0`\x01\x82\x01a\x0C\x84Wa\x0C\x84a\x0C\tV[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15a\x0C\xA6W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0C\x8EV[PP`\0\x91\x01RV[`\0\x82Qa\x0C\xC1\x81\x84` \x87\x01a\x0C\x8BV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C\xEA\x81`@\x85\x01` \x87\x01a\x0C\x8BV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xF2K\xC7$\xB9R\x8F\x17\x1B\xB8\x90\x8B\xC2K\\\xC6\xA0\x02\xFCR\x91L<g\x92vQ\x8CX2\xBB\x10dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKBALANCERVAULT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockBalancerVault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockBalancerVault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockBalancerVault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockBalancerVault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockBalancerVault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockBalancerVault))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockBalancerVault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKBALANCERVAULT_ABI.clone(),
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
                MOCKBALANCERVAULT_ABI.clone(),
                MOCKBALANCERVAULT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `batchSwap` (0x945bcec9) function
        pub fn batch_swap(
            &self,
            kind: u8,
            swaps: ::std::vec::Vec<BatchSwapStep>,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            funds: FundManagement,
            limits: ::std::vec::Vec<::ethers::core::types::I256>,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::I256>,
        > {
            self.0
                .method_hash(
                    [148, 91, 206, 201],
                    (kind, swaps, assets, funds, limits, deadline),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bpt` (0x546af3c3) function
        pub fn bpt(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([84, 106, 243, 195], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryBatchSwap` (0xf84d066e) function
        pub fn query_batch_swap(
            &self,
            kind: u8,
            swaps: ::std::vec::Vec<BatchSwapStep>,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            funds: FundManagement,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::I256>,
        > {
            self.0
                .method_hash([248, 77, 6, 110], (kind, swaps, assets, funds))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPass` (0xf39ea9d0) function
        pub fn set_pass(
            &self,
            should_pass: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 158, 169, 208], should_pass)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockBalancerVault<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `batchSwap` function with signature `batchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool),int256[],uint256)` and selector `0x945bcec9`
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
        name = "batchSwap",
        abi = "batchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool),int256[],uint256)"
    )]
    pub struct BatchSwapCall {
        pub kind: u8,
        pub swaps: ::std::vec::Vec<BatchSwapStep>,
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub funds: FundManagement,
        pub limits: ::std::vec::Vec<::ethers::core::types::I256>,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `bpt` function with signature `bpt()` and selector `0x546af3c3`
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
    #[ethcall(name = "bpt", abi = "bpt()")]
    pub struct BptCall;
    ///Container type for all input parameters for the `queryBatchSwap` function with signature `queryBatchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool))` and selector `0xf84d066e`
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
        name = "queryBatchSwap",
        abi = "queryBatchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool))"
    )]
    pub struct QueryBatchSwapCall {
        pub kind: u8,
        pub swaps: ::std::vec::Vec<BatchSwapStep>,
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub funds: FundManagement,
    }
    ///Container type for all input parameters for the `setPass` function with signature `setPass(bool)` and selector `0xf39ea9d0`
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
    #[ethcall(name = "setPass", abi = "setPass(bool)")]
    pub struct SetPassCall {
        pub should_pass: bool,
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
    pub enum MockBalancerVaultCalls {
        BatchSwap(BatchSwapCall),
        Bpt(BptCall),
        QueryBatchSwap(QueryBatchSwapCall),
        SetPass(SetPassCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockBalancerVaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BatchSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BatchSwap(decoded));
            }
            if let Ok(decoded) = <BptCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Bpt(decoded));
            }
            if let Ok(decoded) = <QueryBatchSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QueryBatchSwap(decoded));
            }
            if let Ok(decoded) = <SetPassCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPass(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockBalancerVaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BatchSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bpt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryBatchSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPass(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockBalancerVaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BatchSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bpt(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryBatchSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPass(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BatchSwapCall> for MockBalancerVaultCalls {
        fn from(value: BatchSwapCall) -> Self {
            Self::BatchSwap(value)
        }
    }
    impl ::core::convert::From<BptCall> for MockBalancerVaultCalls {
        fn from(value: BptCall) -> Self {
            Self::Bpt(value)
        }
    }
    impl ::core::convert::From<QueryBatchSwapCall> for MockBalancerVaultCalls {
        fn from(value: QueryBatchSwapCall) -> Self {
            Self::QueryBatchSwap(value)
        }
    }
    impl ::core::convert::From<SetPassCall> for MockBalancerVaultCalls {
        fn from(value: SetPassCall) -> Self {
            Self::SetPass(value)
        }
    }
    ///Container type for all return fields from the `batchSwap` function with signature `batchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool),int256[],uint256)` and selector `0x945bcec9`
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
    pub struct BatchSwapReturn {
        pub asset_deltas: ::std::vec::Vec<::ethers::core::types::I256>,
    }
    ///Container type for all return fields from the `bpt` function with signature `bpt()` and selector `0x546af3c3`
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
    pub struct BptReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `queryBatchSwap` function with signature `queryBatchSwap(uint8,(bytes32,uint256,uint256,uint256,bytes)[],address[],(address,bool,address,bool))` and selector `0xf84d066e`
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
    pub struct QueryBatchSwapReturn {
        pub asset_deltas: ::std::vec::Vec<::ethers::core::types::I256>,
    }
}
