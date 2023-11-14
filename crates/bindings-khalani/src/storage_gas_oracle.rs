pub use storage_gas_oracle::*;
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
pub mod storage_gas_oracle {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getExchangeRateAndGasPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getExchangeRateAndGasPrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_destinationDomain",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenExchangeRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
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
                    ::std::borrow::ToOwned::to_owned("remoteGasData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("remoteGasData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenExchangeRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setRemoteGasData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRemoteGasData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_config"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StorageGasOracle.RemoteGasDataConfig",
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
                    ::std::borrow::ToOwned::to_owned("setRemoteGasDataConfigs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setRemoteGasDataConfigs",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_configs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StorageGasOracle.RemoteGasDataConfig[]",
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RemoteGasDataSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RemoteGasDataSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("remoteDomain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenExchangeRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
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
    pub static STORAGEGASORACLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\0\x1A3a\0\x1FV[a\0oV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\x05\xA9\x80a\0~`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\x01\x14W\x80c\xB0\x8EV\xD0\x14a\x01/W\x80c\xF2\xFD\xE3\x8B\x14a\x01cW\x80c\xF3\xA1I_\x14a\x01vW`\0\x80\xFD[\x80c`\xFC\xEF|\x14a\0\x82W\x80ci\x8F\xAF\xFC\x14a\0\xF7W\x80cqP\x18\xA6\x14a\x01\x0CW[`\0\x80\xFD[a\0\xD2a\0\x906`\x04a\x04*V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93RT`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x85R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x90\x91\x01\x82\x90R\x91V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\na\x01\x056`\x04a\x04WV[a\x01\x89V[\0[a\x01\na\x01\xD5V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEEV[a\0\xD2a\x01=6`\x04a\x04*V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x91`\x01`\x80\x1B\x90\x04\x16\x82V[a\x01\na\x01q6`\x04a\x04\xCCV[a\x01\xE9V[a\x01\na\x01\x846`\x04a\x04\xF5V[a\x02gV[a\x01\x91a\x02xV[\x80`\0[\x81\x81\x10\x15a\x01\xCFWa\x01\xBD\x84\x84\x83\x81\x81\x10a\x01\xB2Wa\x01\xB2a\x05\rV[\x90P``\x02\x01a\x02\xD2V[\x80a\x01\xC7\x81a\x05#V[\x91PPa\x01\x95V[PPPPV[a\x01\xDDa\x02xV[a\x01\xE7`\0a\x03\xDAV[V[a\x01\xF1a\x02xV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x02d\x81a\x03\xDAV[PV[a\x02oa\x02xV[a\x02d\x81a\x02\xD2V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02RV[`@Q\x80`@\x01`@R\x80\x82` \x01` \x81\x01\x90a\x02\xF0\x91\x90a\x05JV[`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x03\x0E``\x84\x01`@\x85\x01a\x05JV[`\x01`\x01`\x80\x1B\x03\x16\x90R`\x01`\0a\x03*` \x85\x01\x85a\x04*V[c\xFF\xFF\xFF\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x82Q\x92\x82\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x93\x16\x92\x90\x92\x17\x90\x91Ua\x03m\x90\x82\x01\x82a\x04*V[c\xFF\xFF\xFF\xFF\x16\x7F\xB4\x8C\x1C\xB7\x139\x7F\xC0\xC0d\x95\x96\xC2!'\x0F\xEC\x0B=\xE3\xF8\\\xCFjsD\x11\xA2\xFEW\xA6\x94a\x03\xA4`@\x84\x01` \x85\x01a\x05JV[a\x03\xB4``\x85\x01`@\x86\x01a\x05JV[`@\x80Q`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA2PV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0` \x82\x84\x03\x12\x15a\x04<W`\0\x80\xFD[\x815c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04PW`\0\x80\xFD[\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x04jW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\x82W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04\x96W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\xA5W`\0\x80\xFD[\x86` ``\x83\x02\x85\x01\x01\x11\x15a\x04\xBAW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x04\xDEW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04PW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a\x05\x07W`\0\x80\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x05CWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x05\\W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x04PW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xAFx*\xE9f\x8C\xA9\xB5\xCE\x80n\xEF\x88\x80\xCA<\n|\x03K\x7FOM\xBE\xC1\xA4\x98\xCB\xFC\xFB\xB7\x07dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static STORAGEGASORACLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0[W\x80c\x8D\xA5\xCB[\x14a\x01\x14W\x80c\xB0\x8EV\xD0\x14a\x01/W\x80c\xF2\xFD\xE3\x8B\x14a\x01cW\x80c\xF3\xA1I_\x14a\x01vW`\0\x80\xFD[\x80c`\xFC\xEF|\x14a\0\x82W\x80ci\x8F\xAF\xFC\x14a\0\xF7W\x80cqP\x18\xA6\x14a\x01\x0CW[`\0\x80\xFD[a\0\xD2a\0\x906`\x04a\x04*V[c\xFF\xFF\xFF\xFF\x16`\0\x90\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93RT`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x80\x85R`\x01`\x80\x1B\x90\x92\x04\x16\x92\x90\x91\x01\x82\x90R\x91V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\na\x01\x056`\x04a\x04WV[a\x01\x89V[\0[a\x01\na\x01\xD5V[`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEEV[a\0\xD2a\x01=6`\x04a\x04*V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\x80\x1B\x03\x80\x82\x16\x91`\x01`\x80\x1B\x90\x04\x16\x82V[a\x01\na\x01q6`\x04a\x04\xCCV[a\x01\xE9V[a\x01\na\x01\x846`\x04a\x04\xF5V[a\x02gV[a\x01\x91a\x02xV[\x80`\0[\x81\x81\x10\x15a\x01\xCFWa\x01\xBD\x84\x84\x83\x81\x81\x10a\x01\xB2Wa\x01\xB2a\x05\rV[\x90P``\x02\x01a\x02\xD2V[\x80a\x01\xC7\x81a\x05#V[\x91PPa\x01\x95V[PPPPV[a\x01\xDDa\x02xV[a\x01\xE7`\0a\x03\xDAV[V[a\x01\xF1a\x02xV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x02d\x81a\x03\xDAV[PV[a\x02oa\x02xV[a\x02d\x81a\x02\xD2V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02RV[`@Q\x80`@\x01`@R\x80\x82` \x01` \x81\x01\x90a\x02\xF0\x91\x90a\x05JV[`\x01`\x01`\x80\x1B\x03\x16\x81R` \x01a\x03\x0E``\x84\x01`@\x85\x01a\x05JV[`\x01`\x01`\x80\x1B\x03\x16\x90R`\x01`\0a\x03*` \x85\x01\x85a\x04*V[c\xFF\xFF\xFF\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x82Q\x92\x82\x01Q`\x01`\x01`\x80\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02\x93\x16\x92\x90\x92\x17\x90\x91Ua\x03m\x90\x82\x01\x82a\x04*V[c\xFF\xFF\xFF\xFF\x16\x7F\xB4\x8C\x1C\xB7\x139\x7F\xC0\xC0d\x95\x96\xC2!'\x0F\xEC\x0B=\xE3\xF8\\\xCFjsD\x11\xA2\xFEW\xA6\x94a\x03\xA4`@\x84\x01` \x85\x01a\x05JV[a\x03\xB4``\x85\x01`@\x86\x01a\x05JV[`@\x80Q`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA2PV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\0` \x82\x84\x03\x12\x15a\x04<W`\0\x80\xFD[\x815c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04PW`\0\x80\xFD[\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x04jW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\x82W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x04\x96W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04\xA5W`\0\x80\xFD[\x86` ``\x83\x02\x85\x01\x01\x11\x15a\x04\xBAW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x04\xDEW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04PW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a\x05\x07W`\0\x80\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x05CWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x05\\W`\0\x80\xFD[\x815`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\x04PW`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xAFx*\xE9f\x8C\xA9\xB5\xCE\x80n\xEF\x88\x80\xCA<\n|\x03K\x7FOM\xBE\xC1\xA4\x98\xCB\xFC\xFB\xB7\x07dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static STORAGEGASORACLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct StorageGasOracle<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StorageGasOracle<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StorageGasOracle<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StorageGasOracle<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StorageGasOracle<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StorageGasOracle))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StorageGasOracle<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STORAGEGASORACLE_ABI.clone(),
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
                STORAGEGASORACLE_ABI.clone(),
                STORAGEGASORACLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getExchangeRateAndGasPrice` (0x60fcef7c) function
        pub fn get_exchange_rate_and_gas_price(
            &self,
            destination_domain: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([96, 252, 239, 124], destination_domain)
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
        ///Calls the contract's `remoteGasData` (0xb08e56d0) function
        pub fn remote_gas_data(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([176, 142, 86, 208], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRemoteGasData` (0xf3a1495f) function
        pub fn set_remote_gas_data(
            &self,
            config: RemoteGasDataConfig,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 161, 73, 95], (config,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRemoteGasDataConfigs` (0x698faffc) function
        pub fn set_remote_gas_data_configs(
            &self,
            configs: ::std::vec::Vec<RemoteGasDataConfig>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 143, 175, 252], configs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemoteGasDataSet` event
        pub fn remote_gas_data_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoteGasDataSetFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StorageGasOracleEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StorageGasOracle<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
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
    #[ethevent(
        name = "RemoteGasDataSet",
        abi = "RemoteGasDataSet(uint32,uint128,uint128)"
    )]
    pub struct RemoteGasDataSetFilter {
        #[ethevent(indexed)]
        pub remote_domain: u32,
        pub token_exchange_rate: u128,
        pub gas_price: u128,
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
    pub enum StorageGasOracleEvents {
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RemoteGasDataSetFilter(RemoteGasDataSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for StorageGasOracleEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(StorageGasOracleEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = RemoteGasDataSetFilter::decode_log(log) {
                return Ok(StorageGasOracleEvents::RemoteGasDataSetFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StorageGasOracleEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoteGasDataSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for StorageGasOracleEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RemoteGasDataSetFilter> for StorageGasOracleEvents {
        fn from(value: RemoteGasDataSetFilter) -> Self {
            Self::RemoteGasDataSetFilter(value)
        }
    }
    ///Container type for all input parameters for the `getExchangeRateAndGasPrice` function with signature `getExchangeRateAndGasPrice(uint32)` and selector `0x60fcef7c`
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
        name = "getExchangeRateAndGasPrice",
        abi = "getExchangeRateAndGasPrice(uint32)"
    )]
    pub struct GetExchangeRateAndGasPriceCall {
        pub destination_domain: u32,
    }
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
    ///Container type for all input parameters for the `remoteGasData` function with signature `remoteGasData(uint32)` and selector `0xb08e56d0`
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
    #[ethcall(name = "remoteGasData", abi = "remoteGasData(uint32)")]
    pub struct RemoteGasDataCall(pub u32);
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setRemoteGasData` function with signature `setRemoteGasData((uint32,uint128,uint128))` and selector `0xf3a1495f`
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
        name = "setRemoteGasData",
        abi = "setRemoteGasData((uint32,uint128,uint128))"
    )]
    pub struct SetRemoteGasDataCall {
        pub config: RemoteGasDataConfig,
    }
    ///Container type for all input parameters for the `setRemoteGasDataConfigs` function with signature `setRemoteGasDataConfigs((uint32,uint128,uint128)[])` and selector `0x698faffc`
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
        name = "setRemoteGasDataConfigs",
        abi = "setRemoteGasDataConfigs((uint32,uint128,uint128)[])"
    )]
    pub struct SetRemoteGasDataConfigsCall {
        pub configs: ::std::vec::Vec<RemoteGasDataConfig>,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum StorageGasOracleCalls {
        GetExchangeRateAndGasPrice(GetExchangeRateAndGasPriceCall),
        Owner(OwnerCall),
        RemoteGasData(RemoteGasDataCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetRemoteGasData(SetRemoteGasDataCall),
        SetRemoteGasDataConfigs(SetRemoteGasDataConfigsCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for StorageGasOracleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetExchangeRateAndGasPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetExchangeRateAndGasPrice(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RemoteGasDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoteGasData(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetRemoteGasDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetRemoteGasData(decoded));
            }
            if let Ok(decoded) = <SetRemoteGasDataConfigsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetRemoteGasDataConfigs(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StorageGasOracleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetExchangeRateAndGasPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoteGasData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRemoteGasData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRemoteGasDataConfigs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StorageGasOracleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetExchangeRateAndGasPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoteGasData(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRemoteGasData(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRemoteGasDataConfigs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetExchangeRateAndGasPriceCall>
    for StorageGasOracleCalls {
        fn from(value: GetExchangeRateAndGasPriceCall) -> Self {
            Self::GetExchangeRateAndGasPrice(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for StorageGasOracleCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RemoteGasDataCall> for StorageGasOracleCalls {
        fn from(value: RemoteGasDataCall) -> Self {
            Self::RemoteGasData(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for StorageGasOracleCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetRemoteGasDataCall> for StorageGasOracleCalls {
        fn from(value: SetRemoteGasDataCall) -> Self {
            Self::SetRemoteGasData(value)
        }
    }
    impl ::core::convert::From<SetRemoteGasDataConfigsCall> for StorageGasOracleCalls {
        fn from(value: SetRemoteGasDataConfigsCall) -> Self {
            Self::SetRemoteGasDataConfigs(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for StorageGasOracleCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `getExchangeRateAndGasPrice` function with signature `getExchangeRateAndGasPrice(uint32)` and selector `0x60fcef7c`
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
    pub struct GetExchangeRateAndGasPriceReturn {
        pub token_exchange_rate: u128,
        pub gas_price: u128,
    }
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
    ///Container type for all return fields from the `remoteGasData` function with signature `remoteGasData(uint32)` and selector `0xb08e56d0`
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
    pub struct RemoteGasDataReturn {
        pub token_exchange_rate: u128,
        pub gas_price: u128,
    }
    ///`RemoteGasDataConfig(uint32,uint128,uint128)`
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
    pub struct RemoteGasDataConfig {
        pub remote_domain: u32,
        pub token_exchange_rate: u128,
        pub gas_price: u128,
    }
}
