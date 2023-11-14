pub use kai_liquidity_aggregator::*;
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
pub mod kai_liquidity_aggregator {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_kai"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addWhiteListedAsset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addWhiteListedAsset",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Token"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Token"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("kai"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("kai"),
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
                    ::std::borrow::ToOwned::to_owned("nominateNewOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nominateNewOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("nominatedOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nominatedOwner"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("removeWhiteListedAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeWhiteListedAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
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
                (
                    ::std::borrow::ToOwned::to_owned("OwnerChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OwnerChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnerNominated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OwnerNominated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WhiteListedTokenAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WhiteListedTokenAdded",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WhiteListedTokenRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WhiteListedTokenRemoved",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AssetNotWhiteListed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AssetNotWhiteListed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MulOverflow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("MulOverflow"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnsupportedDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UnsupportedDecimals",
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
    pub static KAILIQUIDITYAGGREGATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0E 8\x03\x80a\x0E \x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xEAV[3\x80a\0\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FOwner address cannot be 0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x7F\xB52\x07;8\xC81E\xE3\xE5\x13Sw\xA0\x8B\xF9\xAA\xB5[\xC0\xFD|\x11y\xCDO\xB9\x95\xD2\xA5\x15\x9C\x91\x01`@Q\x80\x91\x03\x90\xA1P`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\x01\x1AV[`\0` \x82\x84\x03\x12\x15a\0\xFCW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x13W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x0C\xDDa\x01C`\09`\0\x81\x81a\x018\x01R\x81\x81a\x044\x01Ra\x04\x9B\x01Ra\x0C\xDD`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xEDW\x80c\x9B}\xA9\x04\x14a\x01\0W\x80c\xC43y=\x14a\x01 W\x80c\xE8\xA2\xB1j\x14a\x013W`\0\x80\xFD[\x80c\r\xCB\xD4\x06\x14a\0\x8DW\x80c\x16'T\x0C\x14a\0\xA2W\x80cS\xA4{\xB7\x14a\0\xB5W\x80cy\xBAP\x97\x14a\0\xE5W[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\t`V[a\x01ZV[\0[a\0\xA0a\0\xB06`\x04a\t`V[a\x01\xD9V[`\x01Ta\0\xC8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\x02WV[`\0Ta\0\xC8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x13a\x01\x0E6`\x04a\t{V[a\x03AV[`@Qa\0\xDC\x91\x90a\t\xFEV[a\0\xA0a\x01.6`\x04a\t`V[a\x04\xC8V[a\0\xC8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x84\x90a\n\x1EV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F-k\xF7\xD2\x88\xB9\xA6\x82\xC7\x96T\x911z\xE7\xD7X=\x82\x8EQ\xCC\x11\xAE:\xEB\x88,Q~\x9E\xB6\x91\x90\xA2PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x84\x90a\n\x1EV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90j\x1Ck\xD7\xE3\t\x1E\xA8f\x93\xDD\x02\x9A\x83\x1C\x19\x04\x9C\xE7\x7F\x1D\xCE,\xE0\xBA\xB1\xCA\xCB\xAB\xCE\"\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FYou must be nominated before you`D\x82\x01Rt\x02\x066\x16\xE2\x06\x1666W\x07B\x06\xF7v\xE6W'6\x86\x97`\\\x1B`d\x82\x01R`\x84\x01a\x01\x84V[`\0T`\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x7F\xB52\x07;8\xC81E\xE3\xE5\x13Sw\xA0\x8B\xF9\xAA\xB5[\xC0\xFD|\x11y\xCDO\xB9\x95\xD2\xA5\x15\x9C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01\x80T`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90\x91U\x16\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x03\x9DW\x82Q`@Qc\x16\xFCB)`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x01\x84V[a\x03\xB1\x83`\0\x01Q30\x86` \x01Qa\x05;V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F2\xF9N\xE2\x9A\xFC$s(\x99\xF5\x07X\x97H\xE4J8Z[\x82m\xB3X\xCE\xFC\xDC`c\x87(\x96\x83\x85`@Qa\x03\xEC\x92\x91\x90a\nmV[`@Q\x80\x91\x03\x90\xA2a\x04\x06\x83`\0\x01Q\x84` \x01Qa\x05\x9BV[` \x84\x01\x81\x90R`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04zW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x8EW=`\0\x80>=`\0\xFD[PPP`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84RP\x82\x90P[\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x84\x90a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x7F\x8F@\x9B\x19\x84h\xB60\x90\x85I\xC5%\xE5\xF66\xB6\\\xED@\xCCC\xB2+;\xF5\xED\xD8\\\xA7\x8B\xCC\x91\x90\xA2PV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x05\x95\x90\x85\x90a\x06}V[PPPPV[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\0\x91\x90a\n\x9DV[`\xFF\x16\x90P`\0a\x06\x12\x82`\x12a\n\xD6V[\x90P`\x12\x81\x11\x15a\x066W`@Qcjx\xE9\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06C\x82`\na\x0B\xCDV[a\x06U\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\xD9V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x14a\x06sWa\x06p\x85\x82a\x07WV[\x94P[P\x92\x94\x93PPPPV[`\0a\x06\xD2\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x07\xB2\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x06\xF3WP\x80\x80` \x01\x90Q\x81\x01\x90a\x06\xF3\x91\x90a\x0B\xF0V[a\x07RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x01\x84V[PPPV[`\0\x80a\x07d\x83\x85a\x0B\xD9V[\x90P\x83\x15\x80a\x07{WP\x82a\x07y\x85\x83a\x0C\x12V[\x14[a\x07\x98W`@Qc\x06o6\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xAAg\r\xE0\xB6\xB3\xA7d\0\0\x82a\x0C\x12V[\x94\x93PPPPV[``a\x07\xC1\x84\x84`\0\x85a\x07\xCBV[\x90P[\x93\x92PPPV[``\x82G\x10\x15a\x08,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x01\x84V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x08H\x91\x90a\x0CXV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08\x85W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x8AV[``\x91P[P\x91P\x91Pa\x08\x9B\x87\x83\x83\x87a\x08\xA6V[\x97\x96PPPPPPPV[``\x83\x15a\t\x15W\x82Q`\0\x03a\t\x0EW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\t\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01\x84V[P\x81a\x07\xAAV[a\x07\xAA\x83\x83\x81Q\x15a\t*W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x84\x91\x90a\x0CtV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t[W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\trW`\0\x80\xFD[a\x07\xC4\x82a\tDV[`\0\x80\x82\x84\x03``\x81\x12\x15a\t\x8FW`\0\x80\xFD[`@\x81\x12\x15a\t\x9DW`\0\x80\xFD[P`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\t\xCFWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Ra\t\xDB\x84a\tDV[\x81R` \x84\x81\x015\x90\x82\x01R\x91Pa\t\xF5`@\x84\x01a\tDV[\x90P\x92P\x92\x90PV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x04\xC2V[` \x80\x82R`/\x90\x82\x01R\x7FOnly the contract owner may perf`@\x82\x01Rn7\xB96\x90:44\xB9\x900\xB1\xBA4\xB7\xB7`\x89\x1B``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x07\xC4` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\n\xAFW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x07\xC4W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\xC2Wa\x04\xC2a\n\xC0V[`\x01\x81\x81[\x80\x85\x11\x15a\x0B$W\x81`\0\x19\x04\x82\x11\x15a\x0B\nWa\x0B\na\n\xC0V[\x80\x85\x16\x15a\x0B\x17W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\n\xEEV[P\x92P\x92\x90PV[`\0\x82a\x0B;WP`\x01a\x04\xC2V[\x81a\x0BHWP`\0a\x04\xC2V[\x81`\x01\x81\x14a\x0B^W`\x02\x81\x14a\x0BhWa\x0B\x84V[`\x01\x91PPa\x04\xC2V[`\xFF\x84\x11\x15a\x0ByWa\x0Bya\n\xC0V[PP`\x01\x82\x1Ba\x04\xC2V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x0B\xA7WP\x81\x81\na\x04\xC2V[a\x0B\xB1\x83\x83a\n\xE9V[\x80`\0\x19\x04\x82\x11\x15a\x0B\xC5Wa\x0B\xC5a\n\xC0V[\x02\x93\x92PPPV[`\0a\x07\xC4\x83\x83a\x0B,V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xC2Wa\x04\xC2a\n\xC0V[`\0` \x82\x84\x03\x12\x15a\x0C\x02W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xC4W`\0\x80\xFD[`\0\x82a\x0C/WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0[\x83\x81\x10\x15a\x0COW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0C7V[PP`\0\x91\x01RV[`\0\x82Qa\x0Cj\x81\x84` \x87\x01a\x0C4V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C\x93\x81`@\x85\x01` \x87\x01a\x0C4V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xB08V\x05};E\xF4x\xE7\xAA\xC4\x1F\x9F\xEDA\xE3\xB4\x08\xF5\xCB\x89W\x8Cp\x9E\x1EF\xC1!C\xCDdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static KAILIQUIDITYAGGREGATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\0\xEDW\x80c\x9B}\xA9\x04\x14a\x01\0W\x80c\xC43y=\x14a\x01 W\x80c\xE8\xA2\xB1j\x14a\x013W`\0\x80\xFD[\x80c\r\xCB\xD4\x06\x14a\0\x8DW\x80c\x16'T\x0C\x14a\0\xA2W\x80cS\xA4{\xB7\x14a\0\xB5W\x80cy\xBAP\x97\x14a\0\xE5W[`\0\x80\xFD[a\0\xA0a\0\x9B6`\x04a\t`V[a\x01ZV[\0[a\0\xA0a\0\xB06`\x04a\t`V[a\x01\xD9V[`\x01Ta\0\xC8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA0a\x02WV[`\0Ta\0\xC8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x13a\x01\x0E6`\x04a\t{V[a\x03AV[`@Qa\0\xDC\x91\x90a\t\xFEV[a\0\xA0a\x01.6`\x04a\t`V[a\x04\xC8V[a\0\xC8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x84\x90a\n\x1EV[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F-k\xF7\xD2\x88\xB9\xA6\x82\xC7\x96T\x911z\xE7\xD7X=\x82\x8EQ\xCC\x11\xAE:\xEB\x88,Q~\x9E\xB6\x91\x90\xA2PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x84\x90a\n\x1EV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90j\x1Ck\xD7\xE3\t\x1E\xA8f\x93\xDD\x02\x9A\x83\x1C\x19\x04\x9C\xE7\x7F\x1D\xCE,\xE0\xBA\xB1\xCA\xCB\xAB\xCE\"\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FYou must be nominated before you`D\x82\x01Rt\x02\x066\x16\xE2\x06\x1666W\x07B\x06\xF7v\xE6W'6\x86\x97`\\\x1B`d\x82\x01R`\x84\x01a\x01\x84V[`\0T`\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x7F\xB52\x07;8\xC81E\xE3\xE5\x13Sw\xA0\x8B\xF9\xAA\xB5[\xC0\xFD|\x11y\xCDO\xB9\x95\xD2\xA5\x15\x9C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01\x80T`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90\x91U\x16\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x02` R`@\x90 T`\xFF\x16a\x03\x9DW\x82Q`@Qc\x16\xFCB)`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x01\x84V[a\x03\xB1\x83`\0\x01Q30\x86` \x01Qa\x05;V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F2\xF9N\xE2\x9A\xFC$s(\x99\xF5\x07X\x97H\xE4J8Z[\x82m\xB3X\xCE\xFC\xDC`c\x87(\x96\x83\x85`@Qa\x03\xEC\x92\x91\x90a\nmV[`@Q\x80\x91\x03\x90\xA2a\x04\x06\x83`\0\x01Q\x84` \x01Qa\x05\x9BV[` \x84\x01\x81\x90R`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04zW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x8EW=`\0\x80>=`\0\xFD[PPP`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84RP\x82\x90P[\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x84\x90a\n\x1EV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x7F\x8F@\x9B\x19\x84h\xB60\x90\x85I\xC5%\xE5\xF66\xB6\\\xED@\xCCC\xB2+;\xF5\xED\xD8\\\xA7\x8B\xCC\x91\x90\xA2PV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x05\x95\x90\x85\x90a\x06}V[PPPPV[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\0\x91\x90a\n\x9DV[`\xFF\x16\x90P`\0a\x06\x12\x82`\x12a\n\xD6V[\x90P`\x12\x81\x11\x15a\x066W`@Qcjx\xE9\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06C\x82`\na\x0B\xCDV[a\x06U\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\xD9V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x14a\x06sWa\x06p\x85\x82a\x07WV[\x94P[P\x92\x94\x93PPPPV[`\0a\x06\xD2\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x07\xB2\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x06\xF3WP\x80\x80` \x01\x90Q\x81\x01\x90a\x06\xF3\x91\x90a\x0B\xF0V[a\x07RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x01\x84V[PPPV[`\0\x80a\x07d\x83\x85a\x0B\xD9V[\x90P\x83\x15\x80a\x07{WP\x82a\x07y\x85\x83a\x0C\x12V[\x14[a\x07\x98W`@Qc\x06o6\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xAAg\r\xE0\xB6\xB3\xA7d\0\0\x82a\x0C\x12V[\x94\x93PPPPV[``a\x07\xC1\x84\x84`\0\x85a\x07\xCBV[\x90P[\x93\x92PPPV[``\x82G\x10\x15a\x08,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x01\x84V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x08H\x91\x90a\x0CXV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x08\x85W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x08\x8AV[``\x91P[P\x91P\x91Pa\x08\x9B\x87\x83\x83\x87a\x08\xA6V[\x97\x96PPPPPPPV[``\x83\x15a\t\x15W\x82Q`\0\x03a\t\x0EW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\t\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01\x84V[P\x81a\x07\xAAV[a\x07\xAA\x83\x83\x81Q\x15a\t*W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x84\x91\x90a\x0CtV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t[W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\trW`\0\x80\xFD[a\x07\xC4\x82a\tDV[`\0\x80\x82\x84\x03``\x81\x12\x15a\t\x8FW`\0\x80\xFD[`@\x81\x12\x15a\t\x9DW`\0\x80\xFD[P`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\t\xCFWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Ra\t\xDB\x84a\tDV[\x81R` \x84\x81\x015\x90\x82\x01R\x91Pa\t\xF5`@\x84\x01a\tDV[\x90P\x92P\x92\x90PV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x04\xC2V[` \x80\x82R`/\x90\x82\x01R\x7FOnly the contract owner may perf`@\x82\x01Rn7\xB96\x90:44\xB9\x900\xB1\xBA4\xB7\xB7`\x89\x1B``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x07\xC4` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\n\xAFW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x07\xC4W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x04\xC2Wa\x04\xC2a\n\xC0V[`\x01\x81\x81[\x80\x85\x11\x15a\x0B$W\x81`\0\x19\x04\x82\x11\x15a\x0B\nWa\x0B\na\n\xC0V[\x80\x85\x16\x15a\x0B\x17W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\n\xEEV[P\x92P\x92\x90PV[`\0\x82a\x0B;WP`\x01a\x04\xC2V[\x81a\x0BHWP`\0a\x04\xC2V[\x81`\x01\x81\x14a\x0B^W`\x02\x81\x14a\x0BhWa\x0B\x84V[`\x01\x91PPa\x04\xC2V[`\xFF\x84\x11\x15a\x0ByWa\x0Bya\n\xC0V[PP`\x01\x82\x1Ba\x04\xC2V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x0B\xA7WP\x81\x81\na\x04\xC2V[a\x0B\xB1\x83\x83a\n\xE9V[\x80`\0\x19\x04\x82\x11\x15a\x0B\xC5Wa\x0B\xC5a\n\xC0V[\x02\x93\x92PPPV[`\0a\x07\xC4\x83\x83a\x0B,V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xC2Wa\x04\xC2a\n\xC0V[`\0` \x82\x84\x03\x12\x15a\x0C\x02W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xC4W`\0\x80\xFD[`\0\x82a\x0C/WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0[\x83\x81\x10\x15a\x0COW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0C7V[PP`\0\x91\x01RV[`\0\x82Qa\x0Cj\x81\x84` \x87\x01a\x0C4V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C\x93\x81`@\x85\x01` \x87\x01a\x0C4V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xB08V\x05};E\xF4x\xE7\xAA\xC4\x1F\x9F\xEDA\xE3\xB4\x08\xF5\xCB\x89W\x8Cp\x9E\x1EF\xC1!C\xCDdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static KAILIQUIDITYAGGREGATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct KaiLiquidityAggregator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for KaiLiquidityAggregator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for KaiLiquidityAggregator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for KaiLiquidityAggregator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for KaiLiquidityAggregator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(KaiLiquidityAggregator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> KaiLiquidityAggregator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    KAILIQUIDITYAGGREGATOR_ABI.clone(),
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
                KAILIQUIDITYAGGREGATOR_ABI.clone(),
                KAILIQUIDITYAGGREGATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addWhiteListedAsset` (0x0dcbd406) function
        pub fn add_white_listed_asset(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 203, 212, 6], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x9b7da904) function
        pub fn deposit(
            &self,
            token: Token,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Token> {
            self.0
                .method_hash([155, 125, 169, 4], (token, receiver))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kai` (0xe8a2b16a) function
        pub fn kai(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([232, 162, 177, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nominateNewOwner` (0x1627540c) function
        pub fn nominate_new_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 39, 84, 12], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nominatedOwner` (0x53a47bb7) function
        pub fn nominated_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([83, 164, 123, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeWhiteListedAddress` (0xc433793d) function
        pub fn remove_white_listed_address(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 51, 121, 61], asset)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnerChanged` event
        pub fn owner_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnerChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnerNominated` event
        pub fn owner_nominated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnerNominatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WhiteListedTokenAdded` event
        pub fn white_listed_token_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WhiteListedTokenAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WhiteListedTokenRemoved` event
        pub fn white_listed_token_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WhiteListedTokenRemovedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            KaiLiquidityAggregatorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for KaiLiquidityAggregator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AssetNotWhiteListed` with signature `AssetNotWhiteListed(address)` and selector `0x5bf108a4`
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
    #[etherror(name = "AssetNotWhiteListed", abi = "AssetNotWhiteListed(address)")]
    pub struct AssetNotWhiteListed {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `MulOverflow` with signature `MulOverflow()` and selector `0x0cde6c26`
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
    #[etherror(name = "MulOverflow", abi = "MulOverflow()")]
    pub struct MulOverflow;
    ///Custom Error type `UnsupportedDecimals` with signature `UnsupportedDecimals()` and selector `0xd4f1d302`
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
    #[etherror(name = "UnsupportedDecimals", abi = "UnsupportedDecimals()")]
    pub struct UnsupportedDecimals;
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
    pub enum KaiLiquidityAggregatorErrors {
        AssetNotWhiteListed(AssetNotWhiteListed),
        MulOverflow(MulOverflow),
        UnsupportedDecimals(UnsupportedDecimals),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for KaiLiquidityAggregatorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AssetNotWhiteListed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssetNotWhiteListed(decoded));
            }
            if let Ok(decoded) = <MulOverflow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MulOverflow(decoded));
            }
            if let Ok(decoded) = <UnsupportedDecimals as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnsupportedDecimals(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for KaiLiquidityAggregatorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AssetNotWhiteListed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MulOverflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsupportedDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for KaiLiquidityAggregatorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AssetNotWhiteListed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MulOverflow as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <UnsupportedDecimals as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for KaiLiquidityAggregatorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetNotWhiteListed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MulOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsupportedDecimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for KaiLiquidityAggregatorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AssetNotWhiteListed> for KaiLiquidityAggregatorErrors {
        fn from(value: AssetNotWhiteListed) -> Self {
            Self::AssetNotWhiteListed(value)
        }
    }
    impl ::core::convert::From<MulOverflow> for KaiLiquidityAggregatorErrors {
        fn from(value: MulOverflow) -> Self {
            Self::MulOverflow(value)
        }
    }
    impl ::core::convert::From<UnsupportedDecimals> for KaiLiquidityAggregatorErrors {
        fn from(value: UnsupportedDecimals) -> Self {
            Self::UnsupportedDecimals(value)
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,(address,uint256))")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub token: Token,
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
    #[ethevent(name = "OwnerChanged", abi = "OwnerChanged(address,address)")]
    pub struct OwnerChangedFilter {
        pub old_owner: ::ethers::core::types::Address,
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "OwnerNominated", abi = "OwnerNominated(address)")]
    pub struct OwnerNominatedFilter {
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "WhiteListedTokenAdded", abi = "WhiteListedTokenAdded(address)")]
    pub struct WhiteListedTokenAddedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
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
        name = "WhiteListedTokenRemoved",
        abi = "WhiteListedTokenRemoved(address)"
    )]
    pub struct WhiteListedTokenRemovedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
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
    pub enum KaiLiquidityAggregatorEvents {
        DepositFilter(DepositFilter),
        OwnerChangedFilter(OwnerChangedFilter),
        OwnerNominatedFilter(OwnerNominatedFilter),
        WhiteListedTokenAddedFilter(WhiteListedTokenAddedFilter),
        WhiteListedTokenRemovedFilter(WhiteListedTokenRemovedFilter),
    }
    impl ::ethers::contract::EthLogDecode for KaiLiquidityAggregatorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(KaiLiquidityAggregatorEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = OwnerChangedFilter::decode_log(log) {
                return Ok(KaiLiquidityAggregatorEvents::OwnerChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnerNominatedFilter::decode_log(log) {
                return Ok(KaiLiquidityAggregatorEvents::OwnerNominatedFilter(decoded));
            }
            if let Ok(decoded) = WhiteListedTokenAddedFilter::decode_log(log) {
                return Ok(
                    KaiLiquidityAggregatorEvents::WhiteListedTokenAddedFilter(decoded),
                );
            }
            if let Ok(decoded) = WhiteListedTokenRemovedFilter::decode_log(log) {
                return Ok(
                    KaiLiquidityAggregatorEvents::WhiteListedTokenRemovedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for KaiLiquidityAggregatorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnerNominatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WhiteListedTokenAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WhiteListedTokenRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DepositFilter> for KaiLiquidityAggregatorEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<OwnerChangedFilter> for KaiLiquidityAggregatorEvents {
        fn from(value: OwnerChangedFilter) -> Self {
            Self::OwnerChangedFilter(value)
        }
    }
    impl ::core::convert::From<OwnerNominatedFilter> for KaiLiquidityAggregatorEvents {
        fn from(value: OwnerNominatedFilter) -> Self {
            Self::OwnerNominatedFilter(value)
        }
    }
    impl ::core::convert::From<WhiteListedTokenAddedFilter>
    for KaiLiquidityAggregatorEvents {
        fn from(value: WhiteListedTokenAddedFilter) -> Self {
            Self::WhiteListedTokenAddedFilter(value)
        }
    }
    impl ::core::convert::From<WhiteListedTokenRemovedFilter>
    for KaiLiquidityAggregatorEvents {
        fn from(value: WhiteListedTokenRemovedFilter) -> Self {
            Self::WhiteListedTokenRemovedFilter(value)
        }
    }
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
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
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `addWhiteListedAsset` function with signature `addWhiteListedAsset(address)` and selector `0x0dcbd406`
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
    #[ethcall(name = "addWhiteListedAsset", abi = "addWhiteListedAsset(address)")]
    pub struct AddWhiteListedAssetCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit((address,uint256),address)` and selector `0x9b7da904`
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
    #[ethcall(name = "deposit", abi = "deposit((address,uint256),address)")]
    pub struct DepositCall {
        pub token: Token,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `kai` function with signature `kai()` and selector `0xe8a2b16a`
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
    #[ethcall(name = "kai", abi = "kai()")]
    pub struct KaiCall;
    ///Container type for all input parameters for the `nominateNewOwner` function with signature `nominateNewOwner(address)` and selector `0x1627540c`
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
    #[ethcall(name = "nominateNewOwner", abi = "nominateNewOwner(address)")]
    pub struct NominateNewOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `nominatedOwner` function with signature `nominatedOwner()` and selector `0x53a47bb7`
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
    #[ethcall(name = "nominatedOwner", abi = "nominatedOwner()")]
    pub struct NominatedOwnerCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `removeWhiteListedAddress` function with signature `removeWhiteListedAddress(address)` and selector `0xc433793d`
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
        name = "removeWhiteListedAddress",
        abi = "removeWhiteListedAddress(address)"
    )]
    pub struct RemoveWhiteListedAddressCall {
        pub asset: ::ethers::core::types::Address,
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
    pub enum KaiLiquidityAggregatorCalls {
        AcceptOwnership(AcceptOwnershipCall),
        AddWhiteListedAsset(AddWhiteListedAssetCall),
        Deposit(DepositCall),
        Kai(KaiCall),
        NominateNewOwner(NominateNewOwnerCall),
        NominatedOwner(NominatedOwnerCall),
        Owner(OwnerCall),
        RemoveWhiteListedAddress(RemoveWhiteListedAddressCall),
    }
    impl ::ethers::core::abi::AbiDecode for KaiLiquidityAggregatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <AddWhiteListedAssetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddWhiteListedAsset(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <KaiCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Kai(decoded));
            }
            if let Ok(decoded) = <NominateNewOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NominateNewOwner(decoded));
            }
            if let Ok(decoded) = <NominatedOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NominatedOwner(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RemoveWhiteListedAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveWhiteListedAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for KaiLiquidityAggregatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddWhiteListedAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Kai(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NominateNewOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NominatedOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveWhiteListedAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for KaiLiquidityAggregatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddWhiteListedAsset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Kai(element) => ::core::fmt::Display::fmt(element, f),
                Self::NominateNewOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::NominatedOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveWhiteListedAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for KaiLiquidityAggregatorCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<AddWhiteListedAssetCall> for KaiLiquidityAggregatorCalls {
        fn from(value: AddWhiteListedAssetCall) -> Self {
            Self::AddWhiteListedAsset(value)
        }
    }
    impl ::core::convert::From<DepositCall> for KaiLiquidityAggregatorCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<KaiCall> for KaiLiquidityAggregatorCalls {
        fn from(value: KaiCall) -> Self {
            Self::Kai(value)
        }
    }
    impl ::core::convert::From<NominateNewOwnerCall> for KaiLiquidityAggregatorCalls {
        fn from(value: NominateNewOwnerCall) -> Self {
            Self::NominateNewOwner(value)
        }
    }
    impl ::core::convert::From<NominatedOwnerCall> for KaiLiquidityAggregatorCalls {
        fn from(value: NominatedOwnerCall) -> Self {
            Self::NominatedOwner(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for KaiLiquidityAggregatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RemoveWhiteListedAddressCall>
    for KaiLiquidityAggregatorCalls {
        fn from(value: RemoveWhiteListedAddressCall) -> Self {
            Self::RemoveWhiteListedAddress(value)
        }
    }
    ///Container type for all return fields from the `deposit` function with signature `deposit((address,uint256),address)` and selector `0x9b7da904`
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
    pub struct DepositReturn(pub Token);
    ///Container type for all return fields from the `kai` function with signature `kai()` and selector `0xe8a2b16a`
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
    pub struct KaiReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `nominatedOwner` function with signature `nominatedOwner()` and selector `0x53a47bb7`
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
    pub struct NominatedOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
