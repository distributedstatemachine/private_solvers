pub use liquidity_aggregator::*;
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
pub mod liquidity_aggregator {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("klnTokenMap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("klnTokenMap"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("registerTokenForKlnToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerTokenForKlnToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("klnToken"),
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
                    ::std::borrow::ToOwned::to_owned("AssetRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AssetRegistered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("mirrorToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("klnToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AssetNotSupported"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AssetNotSupported"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
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
    pub static LIQUIDITYAGGREGATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P3\x80a\0cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FOwner address cannot be 0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x7F\xB52\x07;8\xC81E\xE3\xE5\x13Sw\xA0\x8B\xF9\xAA\xB5[\xC0\xFD|\x11y\xCDO\xB9\x95\xD2\xA5\x15\x9C\x91\x01`@Q\x80\x91\x03\x90\xA1Pa\x0Ci\x80a\0\xCA`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cy\xBAP\x97\x11a\0[W\x80cy\xBAP\x97\x14a\0\xF0W\x80c\x8D\xA5\xCB[\x14a\0\xF8W\x80c\x9B}\xA9\x04\x14a\x01\x0BW\x80c\xFB\xEF]\x16\x14a\x01+W`\0\x80\xFD[\x80c\x16'T\x0C\x14a\0\x82W\x80c6\x9E\xF0\x03\x14a\0\x97W\x80cS\xA4{\xB7\x14a\0\xDDW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x08\xC2V[a\x01>V[\0[a\0\xC0a\0\xA56`\x04a\x08\xC2V[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\x95a\x01\xC5V[`\0Ta\0\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x1Ea\x01\x196`\x04a\x08\xDDV[a\x02\xAFV[`@Qa\0\xD4\x91\x90a\t`V[a\0\x95a\x0196`\x04a\t\x80V[a\x04\x02V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01h\x90a\t\xAAV[`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90j\x1Ck\xD7\xE3\t\x1E\xA8f\x93\xDD\x02\x9A\x83\x1C\x19\x04\x9C\xE7\x7F\x1D\xCE,\xE0\xBA\xB1\xCA\xCB\xAB\xCE\"\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FYou must be nominated before you`D\x82\x01Rt\x02\x066\x16\xE2\x06\x1666W\x07B\x06\xF7v\xE6W'6\x86\x97`\\\x1B`d\x82\x01R`\x84\x01a\x01hV[`\0T`\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x7F\xB52\x07;8\xC81E\xE3\xE5\x13Sw\xA0\x8B\xF9\xAA\xB5[\xC0\xFD|\x11y\xCDO\xB9\x95\xD2\xA5\x15\x9C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01\x80T`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90\x91U\x16\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16a\x03\x0BW\x82Q`@Qc'w\xA6\x8F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x01hV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F2\xF9N\xE2\x9A\xFC$s(\x99\xF5\x07X\x97H\xE4J8Z[\x82m\xB3X\xCE\xFC\xDC`c\x87(\x96\x83\x85`@Qa\x03F\x92\x91\x90a\t\xF9V[`@Q\x80\x91\x03\x90\xA2a\x03b\x83`\0\x01Q30\x86` \x01Qa\x04\x9DV[a\x03t\x83`\0\x01Q\x84` \x01Qa\x04\xFDV[` \x84\x81\x01\x91\x82R\x84Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x02\x90\x92R`@\x91\x82\x90 T\x81\x16\x80\x87R\x92Q\x91Qc@\xC1\x0F\x19`\xE0\x1B\x81R\x90\x85\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91Rc@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xE0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xF4W=`\0\x80>=`\0\xFD[PPPP\x82\x90P[\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04,W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01h\x90a\t\xAAV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x82R\x83\x16` \x82\x01R\x7F{\xEBX\x03\xE9]\x99z:A\xBD\xD1\x14\xAA\xD4\x94 \xE9\xB4\xEE\x9F\xD8\x0C\x96\xB9\xAC\xB7\xA8\xBF\x9E\xE0w\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x04\xF7\x90\x85\x90a\x05\xDFV[PPPPV[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05b\x91\x90a\n)V[`\xFF\x16\x90P`\0a\x05t\x82`\x12a\nbV[\x90P`\x12\x81\x11\x15a\x05\x98W`@Qcjx\xE9\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05\xA5\x82`\na\x0BYV[a\x05\xB7\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x0BeV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x14a\x05\xD5Wa\x05\xD2\x85\x82a\x06\xB9V[\x94P[P\x92\x94\x93PPPPV[`\0a\x064\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x07\x14\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x06UWP\x80\x80` \x01\x90Q\x81\x01\x90a\x06U\x91\x90a\x0B|V[a\x06\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x01hV[PPPV[`\0\x80a\x06\xC6\x83\x85a\x0BeV[\x90P\x83\x15\x80a\x06\xDDWP\x82a\x06\xDB\x85\x83a\x0B\x9EV[\x14[a\x06\xFAW`@Qc\x06o6\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x0Cg\r\xE0\xB6\xB3\xA7d\0\0\x82a\x0B\x9EV[\x94\x93PPPPV[``a\x07#\x84\x84`\0\x85a\x07-V[\x90P[\x93\x92PPPV[``\x82G\x10\x15a\x07\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x01hV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x07\xAA\x91\x90a\x0B\xE4V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x07\xE7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\xECV[``\x91P[P\x91P\x91Pa\x07\xFD\x87\x83\x83\x87a\x08\x08V[\x97\x96PPPPPPPV[``\x83\x15a\x08wW\x82Q`\0\x03a\x08pW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x08pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01hV[P\x81a\x07\x0CV[a\x07\x0C\x83\x83\x81Q\x15a\x08\x8CW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01h\x91\x90a\x0C\0V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xBDW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x08\xD4W`\0\x80\xFD[a\x07&\x82a\x08\xA6V[`\0\x80\x82\x84\x03``\x81\x12\x15a\x08\xF1W`\0\x80\xFD[`@\x81\x12\x15a\x08\xFFW`\0\x80\xFD[P`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\t1WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Ra\t=\x84a\x08\xA6V[\x81R` \x84\x81\x015\x90\x82\x01R\x91Pa\tW`@\x84\x01a\x08\xA6V[\x90P\x92P\x92\x90PV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x03\xFCV[`\0\x80`@\x83\x85\x03\x12\x15a\t\x93W`\0\x80\xFD[a\t\x9C\x83a\x08\xA6V[\x91Pa\tW` \x84\x01a\x08\xA6V[` \x80\x82R`/\x90\x82\x01R\x7FOnly the contract owner may perf`@\x82\x01Rn7\xB96\x90:44\xB9\x900\xB1\xBA4\xB7\xB7`\x89\x1B``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x07&` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\n;W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x07&W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xFCWa\x03\xFCa\nLV[`\x01\x81\x81[\x80\x85\x11\x15a\n\xB0W\x81`\0\x19\x04\x82\x11\x15a\n\x96Wa\n\x96a\nLV[\x80\x85\x16\x15a\n\xA3W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\nzV[P\x92P\x92\x90PV[`\0\x82a\n\xC7WP`\x01a\x03\xFCV[\x81a\n\xD4WP`\0a\x03\xFCV[\x81`\x01\x81\x14a\n\xEAW`\x02\x81\x14a\n\xF4Wa\x0B\x10V[`\x01\x91PPa\x03\xFCV[`\xFF\x84\x11\x15a\x0B\x05Wa\x0B\x05a\nLV[PP`\x01\x82\x1Ba\x03\xFCV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x0B3WP\x81\x81\na\x03\xFCV[a\x0B=\x83\x83a\nuV[\x80`\0\x19\x04\x82\x11\x15a\x0BQWa\x0BQa\nLV[\x02\x93\x92PPPV[`\0a\x07&\x83\x83a\n\xB8V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xFCWa\x03\xFCa\nLV[`\0` \x82\x84\x03\x12\x15a\x0B\x8EW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07&W`\0\x80\xFD[`\0\x82a\x0B\xBBWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0[\x83\x81\x10\x15a\x0B\xDBW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\xC3V[PP`\0\x91\x01RV[`\0\x82Qa\x0B\xF6\x81\x84` \x87\x01a\x0B\xC0V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C\x1F\x81`@\x85\x01` \x87\x01a\x0B\xC0V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xA2.\x18/\x1E\xED8\xC5\xCDp|\xDB\xEB\xA7\xFF\xA9!\xF9\xE9u7\x17(R\xD2J\x16\x02\xE7\x13Y<dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static LIQUIDITYAGGREGATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80cy\xBAP\x97\x11a\0[W\x80cy\xBAP\x97\x14a\0\xF0W\x80c\x8D\xA5\xCB[\x14a\0\xF8W\x80c\x9B}\xA9\x04\x14a\x01\x0BW\x80c\xFB\xEF]\x16\x14a\x01+W`\0\x80\xFD[\x80c\x16'T\x0C\x14a\0\x82W\x80c6\x9E\xF0\x03\x14a\0\x97W\x80cS\xA4{\xB7\x14a\0\xDDW[`\0\x80\xFD[a\0\x95a\0\x906`\x04a\x08\xC2V[a\x01>V[\0[a\0\xC0a\0\xA56`\x04a\x08\xC2V[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\0\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\x95a\x01\xC5V[`\0Ta\0\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x1Ea\x01\x196`\x04a\x08\xDDV[a\x02\xAFV[`@Qa\0\xD4\x91\x90a\t`V[a\0\x95a\x0196`\x04a\t\x80V[a\x04\x02V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01h\x90a\t\xAAV[`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90j\x1Ck\xD7\xE3\t\x1E\xA8f\x93\xDD\x02\x9A\x83\x1C\x19\x04\x9C\xE7\x7F\x1D\xCE,\xE0\xBA\xB1\xCA\xCB\xAB\xCE\"\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FYou must be nominated before you`D\x82\x01Rt\x02\x066\x16\xE2\x06\x1666W\x07B\x06\xF7v\xE6W'6\x86\x97`\\\x1B`d\x82\x01R`\x84\x01a\x01hV[`\0T`\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x7F\xB52\x07;8\xC81E\xE3\xE5\x13Sw\xA0\x8B\xF9\xAA\xB5[\xC0\xFD|\x11y\xCDO\xB9\x95\xD2\xA5\x15\x9C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01\x80T`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90\x91U\x16\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x16a\x03\x0BW\x82Q`@Qc'w\xA6\x8F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01a\x01hV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F2\xF9N\xE2\x9A\xFC$s(\x99\xF5\x07X\x97H\xE4J8Z[\x82m\xB3X\xCE\xFC\xDC`c\x87(\x96\x83\x85`@Qa\x03F\x92\x91\x90a\t\xF9V[`@Q\x80\x91\x03\x90\xA2a\x03b\x83`\0\x01Q30\x86` \x01Qa\x04\x9DV[a\x03t\x83`\0\x01Q\x84` \x01Qa\x04\xFDV[` \x84\x81\x01\x91\x82R\x84Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x02\x90\x92R`@\x91\x82\x90 T\x81\x16\x80\x87R\x92Q\x91Qc@\xC1\x0F\x19`\xE0\x1B\x81R\x90\x85\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91Rc@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\xE0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xF4W=`\0\x80>=`\0\xFD[PPPP\x82\x90P[\x92\x91PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04,W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01h\x90a\t\xAAV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x82R\x83\x16` \x82\x01R\x7F{\xEBX\x03\xE9]\x99z:A\xBD\xD1\x14\xAA\xD4\x94 \xE9\xB4\xEE\x9F\xD8\x0C\x96\xB9\xAC\xB7\xA8\xBF\x9E\xE0w\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90UV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x04\xF7\x90\x85\x90a\x05\xDFV[PPPPV[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05b\x91\x90a\n)V[`\xFF\x16\x90P`\0a\x05t\x82`\x12a\nbV[\x90P`\x12\x81\x11\x15a\x05\x98W`@Qcjx\xE9\x81`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05\xA5\x82`\na\x0BYV[a\x05\xB7\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x0BeV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x14a\x05\xD5Wa\x05\xD2\x85\x82a\x06\xB9V[\x94P[P\x92\x94\x93PPPPV[`\0a\x064\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x07\x14\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x06UWP\x80\x80` \x01\x90Q\x81\x01\x90a\x06U\x91\x90a\x0B|V[a\x06\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x01hV[PPPV[`\0\x80a\x06\xC6\x83\x85a\x0BeV[\x90P\x83\x15\x80a\x06\xDDWP\x82a\x06\xDB\x85\x83a\x0B\x9EV[\x14[a\x06\xFAW`@Qc\x06o6\x13`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x0Cg\r\xE0\xB6\xB3\xA7d\0\0\x82a\x0B\x9EV[\x94\x93PPPPV[``a\x07#\x84\x84`\0\x85a\x07-V[\x90P[\x93\x92PPPV[``\x82G\x10\x15a\x07\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x01hV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x07\xAA\x91\x90a\x0B\xE4V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x07\xE7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\xECV[``\x91P[P\x91P\x91Pa\x07\xFD\x87\x83\x83\x87a\x08\x08V[\x97\x96PPPPPPPV[``\x83\x15a\x08wW\x82Q`\0\x03a\x08pW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x08pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01hV[P\x81a\x07\x0CV[a\x07\x0C\x83\x83\x81Q\x15a\x08\x8CW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01h\x91\x90a\x0C\0V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xBDW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x08\xD4W`\0\x80\xFD[a\x07&\x82a\x08\xA6V[`\0\x80\x82\x84\x03``\x81\x12\x15a\x08\xF1W`\0\x80\xFD[`@\x81\x12\x15a\x08\xFFW`\0\x80\xFD[P`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\t1WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Ra\t=\x84a\x08\xA6V[\x81R` \x84\x81\x015\x90\x82\x01R\x91Pa\tW`@\x84\x01a\x08\xA6V[\x90P\x92P\x92\x90PV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x03\xFCV[`\0\x80`@\x83\x85\x03\x12\x15a\t\x93W`\0\x80\xFD[a\t\x9C\x83a\x08\xA6V[\x91Pa\tW` \x84\x01a\x08\xA6V[` \x80\x82R`/\x90\x82\x01R\x7FOnly the contract owner may perf`@\x82\x01Rn7\xB96\x90:44\xB9\x900\xB1\xBA4\xB7\xB7`\x89\x1B``\x82\x01R`\x80\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x07&` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Q\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\n;W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x07&W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xFCWa\x03\xFCa\nLV[`\x01\x81\x81[\x80\x85\x11\x15a\n\xB0W\x81`\0\x19\x04\x82\x11\x15a\n\x96Wa\n\x96a\nLV[\x80\x85\x16\x15a\n\xA3W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\nzV[P\x92P\x92\x90PV[`\0\x82a\n\xC7WP`\x01a\x03\xFCV[\x81a\n\xD4WP`\0a\x03\xFCV[\x81`\x01\x81\x14a\n\xEAW`\x02\x81\x14a\n\xF4Wa\x0B\x10V[`\x01\x91PPa\x03\xFCV[`\xFF\x84\x11\x15a\x0B\x05Wa\x0B\x05a\nLV[PP`\x01\x82\x1Ba\x03\xFCV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x0B3WP\x81\x81\na\x03\xFCV[a\x0B=\x83\x83a\nuV[\x80`\0\x19\x04\x82\x11\x15a\x0BQWa\x0BQa\nLV[\x02\x93\x92PPPV[`\0a\x07&\x83\x83a\n\xB8V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xFCWa\x03\xFCa\nLV[`\0` \x82\x84\x03\x12\x15a\x0B\x8EW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07&W`\0\x80\xFD[`\0\x82a\x0B\xBBWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0[\x83\x81\x10\x15a\x0B\xDBW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\xC3V[PP`\0\x91\x01RV[`\0\x82Qa\x0B\xF6\x81\x84` \x87\x01a\x0B\xC0V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x0C\x1F\x81`@\x85\x01` \x87\x01a\x0B\xC0V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xA2.\x18/\x1E\xED8\xC5\xCDp|\xDB\xEB\xA7\xFF\xA9!\xF9\xE9u7\x17(R\xD2J\x16\x02\xE7\x13Y<dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDITYAGGREGATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LiquidityAggregator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LiquidityAggregator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LiquidityAggregator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LiquidityAggregator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LiquidityAggregator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LiquidityAggregator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LiquidityAggregator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDITYAGGREGATOR_ABI.clone(),
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
                LIQUIDITYAGGREGATOR_ABI.clone(),
                LIQUIDITYAGGREGATOR_BYTECODE.clone().into(),
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
        ///Calls the contract's `klnTokenMap` (0x369ef003) function
        pub fn kln_token_map(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([54, 158, 240, 3], p0)
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
        ///Calls the contract's `registerTokenForKlnToken` (0xfbef5d16) function
        pub fn register_token_for_kln_token(
            &self,
            token: ::ethers::core::types::Address,
            kln_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 239, 93, 22], (token, kln_token))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AssetRegistered` event
        pub fn asset_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssetRegisteredFilter,
        > {
            self.0.event()
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidityAggregatorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LiquidityAggregator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AssetNotSupported` with signature `AssetNotSupported(address)` and selector `0x4eef4d1e`
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
    #[etherror(name = "AssetNotSupported", abi = "AssetNotSupported(address)")]
    pub struct AssetNotSupported {
        pub asset: ::ethers::core::types::Address,
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
    pub enum LiquidityAggregatorErrors {
        AssetNotSupported(AssetNotSupported),
        MulOverflow(MulOverflow),
        UnsupportedDecimals(UnsupportedDecimals),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidityAggregatorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AssetNotSupported as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssetNotSupported(decoded));
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
    impl ::ethers::core::abi::AbiEncode for LiquidityAggregatorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AssetNotSupported(element) => {
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
    impl ::ethers::contract::ContractRevert for LiquidityAggregatorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AssetNotSupported as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for LiquidityAggregatorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetNotSupported(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsupportedDecimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LiquidityAggregatorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AssetNotSupported> for LiquidityAggregatorErrors {
        fn from(value: AssetNotSupported) -> Self {
            Self::AssetNotSupported(value)
        }
    }
    impl ::core::convert::From<MulOverflow> for LiquidityAggregatorErrors {
        fn from(value: MulOverflow) -> Self {
            Self::MulOverflow(value)
        }
    }
    impl ::core::convert::From<UnsupportedDecimals> for LiquidityAggregatorErrors {
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
    #[ethevent(name = "AssetRegistered", abi = "AssetRegistered(address,address)")]
    pub struct AssetRegisteredFilter {
        pub mirror_token: ::ethers::core::types::Address,
        pub kln_token: ::ethers::core::types::Address,
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
    pub enum LiquidityAggregatorEvents {
        AssetRegisteredFilter(AssetRegisteredFilter),
        DepositFilter(DepositFilter),
        OwnerChangedFilter(OwnerChangedFilter),
        OwnerNominatedFilter(OwnerNominatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for LiquidityAggregatorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssetRegisteredFilter::decode_log(log) {
                return Ok(LiquidityAggregatorEvents::AssetRegisteredFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(LiquidityAggregatorEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = OwnerChangedFilter::decode_log(log) {
                return Ok(LiquidityAggregatorEvents::OwnerChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnerNominatedFilter::decode_log(log) {
                return Ok(LiquidityAggregatorEvents::OwnerNominatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LiquidityAggregatorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnerNominatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AssetRegisteredFilter> for LiquidityAggregatorEvents {
        fn from(value: AssetRegisteredFilter) -> Self {
            Self::AssetRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for LiquidityAggregatorEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<OwnerChangedFilter> for LiquidityAggregatorEvents {
        fn from(value: OwnerChangedFilter) -> Self {
            Self::OwnerChangedFilter(value)
        }
    }
    impl ::core::convert::From<OwnerNominatedFilter> for LiquidityAggregatorEvents {
        fn from(value: OwnerNominatedFilter) -> Self {
            Self::OwnerNominatedFilter(value)
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
    ///Container type for all input parameters for the `klnTokenMap` function with signature `klnTokenMap(address)` and selector `0x369ef003`
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
    #[ethcall(name = "klnTokenMap", abi = "klnTokenMap(address)")]
    pub struct KlnTokenMapCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `registerTokenForKlnToken` function with signature `registerTokenForKlnToken(address,address)` and selector `0xfbef5d16`
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
        name = "registerTokenForKlnToken",
        abi = "registerTokenForKlnToken(address,address)"
    )]
    pub struct RegisterTokenForKlnTokenCall {
        pub token: ::ethers::core::types::Address,
        pub kln_token: ::ethers::core::types::Address,
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
    pub enum LiquidityAggregatorCalls {
        AcceptOwnership(AcceptOwnershipCall),
        Deposit(DepositCall),
        KlnTokenMap(KlnTokenMapCall),
        NominateNewOwner(NominateNewOwnerCall),
        NominatedOwner(NominatedOwnerCall),
        Owner(OwnerCall),
        RegisterTokenForKlnToken(RegisterTokenForKlnTokenCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidityAggregatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <KlnTokenMapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KlnTokenMap(decoded));
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
            if let Ok(decoded) = <RegisterTokenForKlnTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterTokenForKlnToken(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidityAggregatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::KlnTokenMap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NominateNewOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NominatedOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterTokenForKlnToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidityAggregatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::KlnTokenMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::NominateNewOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::NominatedOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterTokenForKlnToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for LiquidityAggregatorCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<DepositCall> for LiquidityAggregatorCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<KlnTokenMapCall> for LiquidityAggregatorCalls {
        fn from(value: KlnTokenMapCall) -> Self {
            Self::KlnTokenMap(value)
        }
    }
    impl ::core::convert::From<NominateNewOwnerCall> for LiquidityAggregatorCalls {
        fn from(value: NominateNewOwnerCall) -> Self {
            Self::NominateNewOwner(value)
        }
    }
    impl ::core::convert::From<NominatedOwnerCall> for LiquidityAggregatorCalls {
        fn from(value: NominatedOwnerCall) -> Self {
            Self::NominatedOwner(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidityAggregatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RegisterTokenForKlnTokenCall>
    for LiquidityAggregatorCalls {
        fn from(value: RegisterTokenForKlnTokenCall) -> Self {
            Self::RegisterTokenForKlnToken(value)
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
    ///Container type for all return fields from the `klnTokenMap` function with signature `klnTokenMap(address)` and selector `0x369ef003`
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
    pub struct KlnTokenMapReturn(pub ::ethers::core::types::Address);
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
