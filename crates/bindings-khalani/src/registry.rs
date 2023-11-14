pub use registry::*;
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
pub mod registry {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("eoaAdmin"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("assetIsUsed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetIsUsed"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("deployNewVault"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployNewVault"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_roleManager"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_profitMaxUnlockTime",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("_vault"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("factory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("factory"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract VaultFactory"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllDeployedStrategies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAllDeployedStrategies",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "allDeployedStrategies",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[][]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllDeployedVaults"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getAllDeployedVaults",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allDeployedVaults"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[][]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAssets"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDeployedStrategies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDeployedStrategies",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDeployedVaults"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDeployedVaults"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("info"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("info"),
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
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "deploymentTimeStamp",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("numAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("numAssets"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("numDeployedStrategies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "numDeployedStrategies",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("numDeployedVaults"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("numDeployedVaults"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("registerStrategy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerStrategy"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_deploymentTimestamp",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("NewDeployedVault"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewDeployedVault"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vault"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
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
                    ::std::borrow::ToOwned::to_owned("NewStrategyRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NewStrategyRegistered",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static REGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0+\xD58\x03\x80b\0+\xD5\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x011V[b\0\0?3b\0\0\xA0V[`\x01b\0\0M\x83\x82b\0\x02\xA8V[P\x80`@Qb\0\0]\x90b\0\0\xF0V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0\x8AW=`\0\x80>=`\0\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80RPb\0\x03t\x90PV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[a\x17Q\x80b\0\x14\x84\x839\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01,W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01EW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x01]W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12b\0\x01rW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x01\x87Wb\0\x01\x87b\0\0\xFEV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\x01\xB2Wb\0\x01\xB2b\0\0\xFEV[\x81`@R\x82\x81R` \x93P\x88\x84\x84\x87\x01\x01\x11\x15b\0\x01\xCFW`\0\x80\xFD[`\0\x91P[\x82\x82\x10\x15b\0\x01\xF3W\x84\x82\x01\x84\x01Q\x81\x83\x01\x85\x01R\x90\x83\x01\x90b\0\x01\xD4V[`\0\x84\x84\x83\x01\x01R\x80\x96PPPPb\0\x02\x0E\x81\x86\x01b\0\x01\x14V[\x92PPP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x02.W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x02OWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x02\xA3W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x02~WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x02\x9FW\x82\x81U`\x01\x01b\0\x02\x8AV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x02\xC4Wb\0\x02\xC4b\0\0\xFEV[b\0\x02\xDC\x81b\0\x02\xD5\x84Tb\0\x02\x19V[\x84b\0\x02UV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x03\x14W`\0\x84\x15b\0\x02\xFBWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x02\x9FV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x03EW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x03$V[P\x85\x82\x10\x15b\0\x03dW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Qa\x10\xEDb\0\x03\x97`\09`\0\x81\x81a\x02\xB2\x01Ra\t\x90\x01Ra\x10\xED`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x16W`\x005`\xE0\x1C\x80c\xA4o\xE8;\x11a\0\xA2W\x80c\xC4Z\x01U\x11a\0qW\x80c\xC4Z\x01U\x14a\x02\xADW\x80c\xCF5\xBD\xD0\x14a\x02\xD4W\x80c\xDF\x0B\x04\xAC\x14a\x02\xE7W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xFAW\x80c\xFE\xAB\x1Ck\x14a\x03\rW`\0\x80\xFD[\x80c\xA4o\xE8;\x14a\x026W\x80c\xA8$\xC8\xF6\x14a\x02>W\x80c\xA8\xDAh?\x14a\x02gW\x80c\xAC\x01v*\x14a\x02zW`\0\x80\xFD[\x80cGUV\x88\x11a\0\xE9W\x80cGUV\x88\x14a\x01\xA8W\x80cg\xE4\xAC,\x14a\x01\xC8W\x80cqP\x18\xA6\x14a\x01\xD0W\x80c\x8D\xA5\xCB[\x14a\x01\xDAW\x80c\x9F\xDA\xE3\x9F\x14a\x01\xFFW`\0\x80\xFD[\x80c\x06\xA1\x04\x87\x14a\x01\x1BW\x80c\x06\xFD\xDE\x03\x14a\x019W\x80c\n\xAEzk\x14a\x01NW\x80cF\x03\xD5n\x14a\x01\xA0W[`\0\x80\xFD[a\x01#a\x03 V[`@Qa\x010\x91\x90a\x0C\xEEV[`@Q\x80\x91\x03\x90\xF3[a\x01Aa\x04\xABV[`@Qa\x010\x91\x90a\r\xC7V[a\x01\x81a\x01\\6`\x04a\r\xF6V[`\x06` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x010V[a\x01#a\x059V[a\x01\xBBa\x01\xB66`\x04a\r\xF6V[a\x06\xBEV[`@Qa\x010\x91\x90a\x0E\x13V[a\x01\xBBa\x074V[a\x01\xD8a\x07\x96V[\0[`\0T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x010V[a\x02(a\x02\r6`\x04a\r\xF6V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x010V[`\x02Ta\x02(V[a\x02(a\x02L6`\x04a\r\xF6V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x05` R`@\x90 T\x90V[a\x01\xD8a\x02u6`\x04a\x0E`V[a\x07\xAAV[a\x02\x9Da\x02\x886`\x04a\r\xF6V[`\x03` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x010V[a\x01\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xE7a\x02\xE26`\x04a\x0E\x8CV[a\tEV[a\x01\xE7a\x02\xF56`\x04a\x0FHV[a\toV[a\x01\xD8a\x03\x086`\x04a\r\xF6V[a\n&V[a\x01\xBBa\x03\x1B6`\x04a\r\xF6V[a\n\xA4V[```\0`\x02\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03zW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\\W[PP`\x02T\x93\x94P\x83\x92PPPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x9FWa\x03\x9Fa\x0E\xA5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xD2W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\xBDW\x90P[P\x92P`\0[\x81\x81\x10\x15a\x04\xA5W`\x04`\0\x84\x83\x81Q\x81\x10a\x03\xF6Wa\x03\xF6a\x0F\xD9V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04rW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04TW[PPPPP\x84\x82\x81Q\x81\x10a\x04\x89Wa\x04\x89a\x0F\xD9V[` \x02` \x01\x01\x81\x90RP\x80a\x04\x9E\x90a\x0F\xEFV[\x90Pa\x03\xD8V[PPP\x90V[`\x01\x80Ta\x04\xB8\x90a\x10\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xE4\x90a\x10\x16V[\x80\x15a\x051W\x80`\x1F\x10a\x05\x06Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x051V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x14W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[```\0`\x02\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\x93W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05uW[PP`\x02T\x93\x94P\x83\x92PPPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xB8Wa\x05\xB8a\x0E\xA5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xEBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05\xD6W\x90P[P\x92P`\0[\x81\x81\x10\x15a\x04\xA5W`\x05`\0\x84\x83\x81Q\x81\x10a\x06\x0FWa\x06\x0Fa\x0F\xD9V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06\x8BW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06mW[PPPPP\x84\x82\x81Q\x81\x10a\x06\xA2Wa\x06\xA2a\x0F\xD9V[` \x02` \x01\x01\x81\x90RP\x80a\x06\xB7\x90a\x0F\xEFV[\x90Pa\x05\xF1V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x07(W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\nW[PPPPP\x90P\x91\x90PV[```\x02\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\x8CW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07nW[PPPPP\x90P\x90V[a\x07\x9Ea\x0B\x18V[a\x07\xA8`\0a\x0BrV[V[a\x07\xB2a\x0B\x18V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c8\xD5.\x0F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x16\x91\x90a\x10PV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x80T`\x01\x80\x82\x01\x83U\x91\x85R\x83\x85 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x8C\x89\x16\x90\x81\x17\x90\x92U\x83Q\x80\x85\x01\x85R\x87\x81R\x80\x86\x01\x8C\x81R\x92\x87R`\x06\x86R\x84\x87 \x90Q\x81T\x90\x92\x16\x91\x90\x98\x16\x17\x87UQ\x95\x01\x94\x90\x94U\x91\x81R`\x03\x90\x91R T\x90\x91P`\xFF\x16a\t\0W`\x02\x80T`\x01\x80\x82\x01\x90\x92U\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`\0\x90\x81R`\x03` R`@\x90 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\nh\x07|\x8E\xAC)\xBC\xAD\x8A\x14\x0E\t4\xB7\x0F\xE5\xE1N*\xE2<b\x14\xFF@\x9E{\x1E\xBD\x94\xD2`@Q`@Q\x80\x91\x03\x90\xA3PPPV[`\x02\x81\x81T\x81\x10a\tUW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\0a\tya\x0B\x18V[`@Qc7\xC2\xC1+`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xDF\x0B\x04\xAC\x90a\t\xCD\x90\x89\x90\x89\x90\x89\x90\x89\x90\x89\x90`\x04\x01a\x10mV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x10\x91\x90a\x10PV[\x90Pa\n\x1D\x81\x87Ba\x0B\xC2V[\x95\x94PPPPPV[a\n.a\x0B\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\n\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\n\xA1\x81a\x0BrV[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x07(W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\nWPPPPP\x90P\x91\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\n\x8FV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01\x80\x82\x01\x83U\x91\x85R\x83\x85 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x8B\x89\x16\x90\x81\x17\x90\x92U\x83Q\x80\x85\x01\x85R\x87\x81R\x80\x86\x01\x8A\x81R\x92\x87R`\x06\x86R\x84\x87 \x90Q\x81T\x90\x92\x16\x91\x90\x98\x16\x17\x87UQ\x95\x01\x94\x90\x94U\x91\x81R`\x03\x90\x91R T`\xFF\x16a\x0C\xA9W`\x02\x80T`\x01\x80\x82\x01\x90\x92U\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`\0\x90\x81R`\x03` R`@\x90 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F+\xC5B\xBE\xD0\x9D\xF2D\xBE\x94(M\xB7.k\xFF\xF8\xD6n\x84\xEA\x8D\xA6p\xDE!\xE6\xEB\xEC\x9B\t}`@Q`@Q\x80\x91\x03\x90\xA3PPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0\x80[\x83\x81\x10\x15a\rsW\x88\x86\x03`?\x19\x01\x85R\x82Q\x80Q\x80\x88R\x90\x88\x01\x90\x88\x88\x01\x90\x84[\x81\x81\x10\x15a\r]W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x8A\x01\x92\x91\x8A\x01\x91`\x01\x01a\r8V[P\x90\x97PPP\x93\x86\x01\x93\x91\x86\x01\x91`\x01\x01a\r\x16V[P\x93\x98\x97PPPPPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\r\xA7W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\r\x8BV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\r\xDA` \x83\x01\x84a\r\x81V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xA1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0E\x08W`\0\x80\xFD[\x815a\r\xDA\x81a\r\xE1V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0ETW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E/V[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0EsW`\0\x80\xFD[\x825a\x0E~\x81a\r\xE1V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x0E\x9EW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0E\xCCW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xE7Wa\x0E\xE7a\x0E\xA5V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0F\x0FWa\x0F\x0Fa\x0E\xA5V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0F(W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0F`W`\0\x80\xFD[\x855a\x0Fk\x81a\r\xE1V[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x88W`\0\x80\xFD[a\x0F\x94\x89\x83\x8A\x01a\x0E\xBBV[\x95P`@\x88\x015\x91P\x80\x82\x11\x15a\x0F\xAAW`\0\x80\xFD[Pa\x0F\xB7\x88\x82\x89\x01a\x0E\xBBV[\x93PP``\x86\x015a\x0F\xC8\x81a\r\xE1V[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x10\x0FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x10*W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10JWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10bW`\0\x80\xFD[\x81Qa\r\xDA\x81a\r\xE1V[`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x83R`\xA0` \x84\x01Ra\x10\x8F`\xA0\x84\x01\x88a\r\x81V[\x83\x81\x03`@\x85\x01Ra\x10\xA1\x81\x88a\r\x81V[\x95\x90\x91\x16``\x84\x01RPP`\x80\x01R\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x96\xF4%<\xE2\xA1\xAB\r\xEBq\xF5\xBD\xB9\xA9p\xCC\xDA\xADm2GV\x04\x16J\xBA\x8C\x80\x0F\x90)\xECdsolcC\0\x08\x13\x003`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x17Q8\x03\x80b\0\x17Q\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x02\x11V[b\0\0A`\0\x82b\0\0\xB6V[b\0\0\\`\0\x80Q` b\0\x171\x839\x81Q\x91R\x82b\0\0\xB6V[b\0\0w`\0\x80Q` b\0\x171\x839\x81Q\x91R3b\0\0\xB6V[`@Qb\0\0\x85\x90b\0\x02\x03V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0\xA2W=`\0\x80>=`\0\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80RPb\0\x02CV[b\0\0\xC2\x82\x82b\0\0\xC6V[PPV[b\0\0\xD2\x82\x82b\0\0\xF1V[`\0\x82\x81R`\x01` R`@\x90 b\0\0\xEC\x90\x82b\0\x01\x91V[PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\0\xC2W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01M3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0b\0\x01\xA8\x83`\x01`\x01`\xA0\x1B\x03\x84\x16b\0\x01\xB1V[\x90P[\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Tb\0\x01\xFAWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ub\0\x01\xABV[P`\0b\0\x01\xABV[a\x01\xE9\x80b\0\x15H\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0\x02$W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02<W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x12\xDBb\0\x02m`\09`\0\x81\x81a\x02\x15\x01R\x81\x81a\x07n\x01Ra\x08\x8D\x01Ra\x12\xDB`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80c\xA2\x17\xFD\xDF\x11a\0\x97W\x80c\xDB\x8AZ\xC8\x11a\0fW\x80c\xDB\x8AZ\xC8\x14a\x02]W\x80c\xDF\x0B\x04\xAC\x14a\x02eW\x80c\xF3\xA6\x95\\\x14a\x02xW\x80c\xFC\x0Et\xD1\x14a\x02\x8BW`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x02\x08W\x80c\xBB\xA4\x8A\x90\x14a\x02\x10W\x80c\xCA\x15\xC8s\x14a\x027W\x80c\xD5Gt\x1F\x14a\x02JW`\0\x80\xFD[\x80cU-\xE3\x8C\x11a\0\xD3W\x80cU-\xE3\x8C\x14a\x01\x86W\x80c}` \xF4\x14a\x01\xADW\x80c\x90\x10\xD0|\x14a\x01\xCAW\x80c\x91\xD1HT\x14a\x01\xF5W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x05W\x80c$\x8A\x9C\xA3\x14a\x01-W\x80c//\xF1]\x14a\x01^W\x80c6V\x8A\xBE\x14a\x01sW[`\0\x80\xFD[a\x01\x18a\x01\x136`\x04a\x0EKV[a\x02\x98V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01Pa\x01;6`\x04a\x0EuV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01$V[a\x01qa\x01l6`\x04a\x0E\xAAV[a\x02\xC3V[\0[a\x01qa\x01\x816`\x04a\x0E\xAAV[a\x02\xEDV[a\x01P\x7F\x87\xA2\x94\xB4\xB6\x0E\xBE\x1By\xE7\xCCxEt\x0Fo\"=\xC2(\x13*{'rQ%A\x16D=\xCB\x81V[a\x01\x18a\x01\xBB6`\x04a\x0E\xD6V[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[a\x01\xDDa\x01\xD86`\x04a\x0E\xF1V[a\x03pV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01$V[a\x01\x18a\x02\x036`\x04a\x0E\xAAV[a\x03\x8FV[a\x01P`\0\x81V[a\x01\xDD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01Pa\x02E6`\x04a\x0EuV[a\x03\xB8V[a\x01qa\x02X6`\x04a\x0E\xAAV[a\x03\xCFV[a\x01qa\x03\xF4V[a\x01\xDDa\x02s6`\x04a\x0F\xB6V[a\x04\xB8V[a\x01\xDDa\x02\x866`\x04a\x10CV[a\x07\x1FV[`\x02Ta\x01\x18\x90`\xFF\x16\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x02\xBDWPa\x02\xBD\x82a\x07\xE1V[\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x02\xDE\x81a\x08\x16V[a\x02\xE8\x83\x83a\x08#V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03l\x82\x82a\x08EV[PPV[`\0\x82\x81R`\x01` R`@\x81 a\x03\x88\x90\x83a\x08gV[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x81\x81R`\x01` R`@\x81 a\x02\xBD\x90a\x08sV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x03\xEA\x81a\x08\x16V[a\x02\xE8\x83\x83a\x08EV[a\x03\xFF`\x003a\x03\x8FV[a\x04\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FMust have DEFAULT_ADMIN_ROLE to `D\x82\x01Rg9\xB4:\xBA27\xBB\xB7`\xC1\x1B`d\x82\x01R`\x84\x01a\x03YV[`\x02T`\xFF\x16\x15a\x04\x80W`@QcD&\xAA\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\xFF\x19\x16`\x01\x17\x90U`@Q\x7F\xC6C\x19:\x97\xFC\x0E\x18\xD6\x9C\x95\xE1\xC04\xB9\x1FQ\xFA\x16K\xA8\xEAh\xDF\xB6\xDD\x98V\x8B\x9B\xC9k\x90`\0\x90\xA1V[`\0a\x04\xE4\x7F\x87\xA2\x94\xB4\xB6\x0E\xBE\x1By\xE7\xCCxEt\x0Fo\"=\xC2(\x13*{'rQ%A\x16D=\xCB3a\x03\x8FV[a\x05JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FMust have VAULT_DEPLOYER_ROLE to`D\x82\x01Rp\x08\x19\x19\\\x1B\x1B\xDEH\x1B\x99]\xC8\x15\x98][\x1D`z\x1B`d\x82\x01R`\x84\x01a\x03YV[\x85`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\x05\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FAsset must be a smart contract\0\0`D\x82\x01R`d\x01a\x03YV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x06\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FRole manager cannot be zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x03YV[`\0\x82\x11a\x06oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FProfit max unlock time must be g`D\x82\x01Roreater than zero`\x80\x1B`d\x82\x01R`\x84\x01a\x03YV[`\x02T`\xFF\x16\x15a\x06\x93W`@QcD&\xAA\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x86\x86\x86`@Q` \x01a\x06\xAA\x93\x92\x91\x90a\x11\x07V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x06\xD2\x88\x88\x88\x88\x88\x87a\x08}V[\x90P\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7FBA0,9<q>i\x07\x02\xC4\xA4ZW\xE9<\xEFY\xAA\x8Cn#XIXS\xB3B\x05Q\xD8`@Q`@Q\x80\x91\x03\x90\xA3\x97\x96PPPPPPPV[`\0\x80\x84\x84\x84`@Q` \x01a\x077\x93\x92\x91\x90a\x11\x07V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 0`8\x83\x01RoZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\xFF`$\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16`\x14\x83\x01Rs=`-\x80`\n=9\x81\xF36==7===6=s\x82R`X\x82\x01\x81\x90R`7`\x0C\x83\x01 `x\x83\x01R`U`C\x90\x92\x01\x91\x90\x91 \x90\x91P`\0\x90[\x96\x95PPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x02\xBDWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x02\xBDV[a\x08 \x813a\t(V[PV[a\x08-\x82\x82a\t\x81V[`\0\x82\x81R`\x01` R`@\x90 a\x02\xE8\x90\x82a\n\x05V[a\x08O\x82\x82a\n\x1AV[`\0\x82\x81R`\x01` R`@\x90 a\x02\xE8\x90\x82a\n\x7FV[`\0a\x03\x88\x83\x83a\n\x94V[`\0a\x02\xBD\x82T\x90V[`\0\x80a\x08\xB3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84a\n\xBEV[`@Qc:\xD9\x85\xF3`\xE1\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cu\xB3\x0B\xE6\x90a\x08\xEA\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01a\x11=V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\x18W=`\0\x80>=`\0\xFD[P\x92\x9A\x99PPPPPPPPPPV[a\t2\x82\x82a\x03\x8FV[a\x03lWa\t?\x81a\x0B[V[a\tJ\x83` a\x0BmV[`@Q` \x01a\t[\x92\x91\x90a\x11\x87V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03Y\x91`\x04\x01a\x11\xFCV[a\t\x8B\x82\x82a\x03\x8FV[a\x03lW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\t\xC13\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x03\x88\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\r\tV[a\n$\x82\x82a\x03\x8FV[\x15a\x03lW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x03\x88\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\rXV[`\0\x82`\0\x01\x82\x81T\x81\x10a\n\xABWa\n\xABa\x12\x0FV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x83``\x1B`\xE8\x1C\x17`\0RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x83`x\x1B\x17` R\x81`7`\t`\0\xF5\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FERC1167: create2 failed\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03YV[``a\x02\xBD`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x0B|\x83`\x02a\x12;V[a\x0B\x87\x90`\x02a\x12RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x9FWa\x0B\x9Fa\x0F\x13V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\xC9W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x0B\xE4Wa\x0B\xE4a\x12\x0FV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0C\x13Wa\x0C\x13a\x12\x0FV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0C7\x84`\x02a\x12;V[a\x0CB\x90`\x01a\x12RV[\x90P[`\x01\x81\x11\x15a\x0C\xBAWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0CvWa\x0Cva\x12\x0FV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0C\x8CWa\x0C\x8Ca\x12\x0FV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0C\xB3\x81a\x12eV[\x90Pa\x0CEV[P\x83\x15a\x03\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03YV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\rPWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02\xBDV[P`\0a\x02\xBDV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x0EAW`\0a\r|`\x01\x83a\x12|V[\x85T\x90\x91P`\0\x90a\r\x90\x90`\x01\x90a\x12|V[\x90P\x81\x81\x14a\r\xF5W`\0\x86`\0\x01\x82\x81T\x81\x10a\r\xB0Wa\r\xB0a\x12\x0FV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\r\xD3Wa\r\xD3a\x12\x0FV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x0E\x06Wa\x0E\x06a\x12\x8FV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x02\xBDV[`\0\x91PPa\x02\xBDV[`\0` \x82\x84\x03\x12\x15a\x0E]W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x03\x88W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0E\x87W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xA5W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xBDW`\0\x80\xFD[\x825\x91Pa\x0E\xCD` \x84\x01a\x0E\x8EV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0E\xE8W`\0\x80\xFD[a\x03\x88\x82a\x0E\x8EV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\x04W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0F:W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FUWa\x0FUa\x0F\x13V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0F}Wa\x0F}a\x0F\x13V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0F\x96W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0F\xCEW`\0\x80\xFD[a\x0F\xD7\x86a\x0E\x8EV[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xF4W`\0\x80\xFD[a\x10\0\x89\x83\x8A\x01a\x0F)V[\x95P`@\x88\x015\x91P\x80\x82\x11\x15a\x10\x16W`\0\x80\xFD[Pa\x10#\x88\x82\x89\x01a\x0F)V[\x93PPa\x102``\x87\x01a\x0E\x8EV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10XW`\0\x80\xFD[a\x10a\x84a\x0E\x8EV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10~W`\0\x80\xFD[a\x10\x8A\x87\x83\x88\x01a\x0F)V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x10\xA0W`\0\x80\xFD[Pa\x10\xAD\x86\x82\x87\x01a\x0F)V[\x91PP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x10\xD2W\x81\x81\x01Q\x83\x82\x01R` \x01a\x10\xBAV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x10\xF3\x81` \x86\x01` \x86\x01a\x10\xB7V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R`\0\x90a\x11+\x90\x83\x01\x85a\x10\xDBV[\x82\x81\x03`@\x84\x01Ra\x07\xD7\x81\x85a\x10\xDBV[`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x83R`\xA0` \x84\x01Ra\x11_`\xA0\x84\x01\x88a\x10\xDBV[\x83\x81\x03`@\x85\x01Ra\x11q\x81\x88a\x10\xDBV[\x95\x90\x91\x16``\x84\x01RPP`\x80\x01R\x93\x92PPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x11\xBF\x81`\x17\x85\x01` \x88\x01a\x10\xB7V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x11\xF0\x81`(\x84\x01` \x88\x01a\x10\xB7V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0a\x03\x88` \x83\x01\x84a\x10\xDBV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xBDWa\x02\xBDa\x12%V[\x80\x82\x01\x80\x82\x11\x15a\x02\xBDWa\x02\xBDa\x12%V[`\0\x81a\x12tWa\x12ta\x12%V[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x02\xBDWa\x02\xBDa\x12%V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x83a\xBB\xEA\x02N\xF6\x1F\xEA\xC23?_\xAA5\xBC\x0E/8j9\xA7\xBC#\xCC\x13tT@7\xA4adsolcC\0\x08\x13\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x01\xC9\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80cu\xB3\x0B\xE6\x14a\x000W[`\0\x80\xFD[a\0Ea\0>6`\x04a\x01\x06V[PPPPPV[\0[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0^W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\0\x8AW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xA5Wa\0\xA5a\0cV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\0\xCDWa\0\xCDa\0cV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\0\xE6W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x01\x1EW`\0\x80\xFD[a\x01'\x86a\0GV[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01DW`\0\x80\xFD[a\x01P\x89\x83\x8A\x01a\0yV[\x95P`@\x88\x015\x91P\x80\x82\x11\x15a\x01fW`\0\x80\xFD[Pa\x01s\x88\x82\x89\x01a\0yV[\x93PPa\x01\x82``\x87\x01a\0GV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV\xFE\xA2dipfsX\"\x12 J&(\x0C3#W\x9Bj#\xE0>\xEA6\t\x10\xAE\xDFM\xE3\x1B\xCA\x0B3<\xAE\xC9=\xCBf\x07WdsolcC\0\x08\x13\x003\x87\xA2\x94\xB4\xB6\x0E\xBE\x1By\xE7\xCCxEt\x0Fo\"=\xC2(\x13*{'rQ%A\x16D=\xCB";
    /// The bytecode of the contract.
    pub static REGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x16W`\x005`\xE0\x1C\x80c\xA4o\xE8;\x11a\0\xA2W\x80c\xC4Z\x01U\x11a\0qW\x80c\xC4Z\x01U\x14a\x02\xADW\x80c\xCF5\xBD\xD0\x14a\x02\xD4W\x80c\xDF\x0B\x04\xAC\x14a\x02\xE7W\x80c\xF2\xFD\xE3\x8B\x14a\x02\xFAW\x80c\xFE\xAB\x1Ck\x14a\x03\rW`\0\x80\xFD[\x80c\xA4o\xE8;\x14a\x026W\x80c\xA8$\xC8\xF6\x14a\x02>W\x80c\xA8\xDAh?\x14a\x02gW\x80c\xAC\x01v*\x14a\x02zW`\0\x80\xFD[\x80cGUV\x88\x11a\0\xE9W\x80cGUV\x88\x14a\x01\xA8W\x80cg\xE4\xAC,\x14a\x01\xC8W\x80cqP\x18\xA6\x14a\x01\xD0W\x80c\x8D\xA5\xCB[\x14a\x01\xDAW\x80c\x9F\xDA\xE3\x9F\x14a\x01\xFFW`\0\x80\xFD[\x80c\x06\xA1\x04\x87\x14a\x01\x1BW\x80c\x06\xFD\xDE\x03\x14a\x019W\x80c\n\xAEzk\x14a\x01NW\x80cF\x03\xD5n\x14a\x01\xA0W[`\0\x80\xFD[a\x01#a\x03 V[`@Qa\x010\x91\x90a\x0C\xEEV[`@Q\x80\x91\x03\x90\xF3[a\x01Aa\x04\xABV[`@Qa\x010\x91\x90a\r\xC7V[a\x01\x81a\x01\\6`\x04a\r\xF6V[`\x06` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x82V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x010V[a\x01#a\x059V[a\x01\xBBa\x01\xB66`\x04a\r\xF6V[a\x06\xBEV[`@Qa\x010\x91\x90a\x0E\x13V[a\x01\xBBa\x074V[a\x01\xD8a\x07\x96V[\0[`\0T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x010V[a\x02(a\x02\r6`\x04a\r\xF6V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x010V[`\x02Ta\x02(V[a\x02(a\x02L6`\x04a\r\xF6V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x05` R`@\x90 T\x90V[a\x01\xD8a\x02u6`\x04a\x0E`V[a\x07\xAAV[a\x02\x9Da\x02\x886`\x04a\r\xF6V[`\x03` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x010V[a\x01\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xE7a\x02\xE26`\x04a\x0E\x8CV[a\tEV[a\x01\xE7a\x02\xF56`\x04a\x0FHV[a\toV[a\x01\xD8a\x03\x086`\x04a\r\xF6V[a\n&V[a\x01\xBBa\x03\x1B6`\x04a\r\xF6V[a\n\xA4V[```\0`\x02\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x03zW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x03\\W[PP`\x02T\x93\x94P\x83\x92PPPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x9FWa\x03\x9Fa\x0E\xA5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xD2W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03\xBDW\x90P[P\x92P`\0[\x81\x81\x10\x15a\x04\xA5W`\x04`\0\x84\x83\x81Q\x81\x10a\x03\xF6Wa\x03\xF6a\x0F\xD9V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04rW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04TW[PPPPP\x84\x82\x81Q\x81\x10a\x04\x89Wa\x04\x89a\x0F\xD9V[` \x02` \x01\x01\x81\x90RP\x80a\x04\x9E\x90a\x0F\xEFV[\x90Pa\x03\xD8V[PPP\x90V[`\x01\x80Ta\x04\xB8\x90a\x10\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xE4\x90a\x10\x16V[\x80\x15a\x051W\x80`\x1F\x10a\x05\x06Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x051V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\x14W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[```\0`\x02\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\x93W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x05uW[PP`\x02T\x93\x94P\x83\x92PPPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xB8Wa\x05\xB8a\x0E\xA5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xEBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05\xD6W\x90P[P\x92P`\0[\x81\x81\x10\x15a\x04\xA5W`\x05`\0\x84\x83\x81Q\x81\x10a\x06\x0FWa\x06\x0Fa\x0F\xD9V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06\x8BW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06mW[PPPPP\x84\x82\x81Q\x81\x10a\x06\xA2Wa\x06\xA2a\x0F\xD9V[` \x02` \x01\x01\x81\x90RP\x80a\x06\xB7\x90a\x0F\xEFV[\x90Pa\x05\xF1V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x07(W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\nW[PPPPP\x90P\x91\x90PV[```\x02\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\x8CW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07nW[PPPPP\x90P\x90V[a\x07\x9Ea\x0B\x18V[a\x07\xA8`\0a\x0BrV[V[a\x07\xB2a\x0B\x18V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c8\xD5.\x0F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x16\x91\x90a\x10PV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x80T`\x01\x80\x82\x01\x83U\x91\x85R\x83\x85 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x8C\x89\x16\x90\x81\x17\x90\x92U\x83Q\x80\x85\x01\x85R\x87\x81R\x80\x86\x01\x8C\x81R\x92\x87R`\x06\x86R\x84\x87 \x90Q\x81T\x90\x92\x16\x91\x90\x98\x16\x17\x87UQ\x95\x01\x94\x90\x94U\x91\x81R`\x03\x90\x91R T\x90\x91P`\xFF\x16a\t\0W`\x02\x80T`\x01\x80\x82\x01\x90\x92U\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x17\x90\x91U`\0\x90\x81R`\x03` R`@\x90 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\nh\x07|\x8E\xAC)\xBC\xAD\x8A\x14\x0E\t4\xB7\x0F\xE5\xE1N*\xE2<b\x14\xFF@\x9E{\x1E\xBD\x94\xD2`@Q`@Q\x80\x91\x03\x90\xA3PPPV[`\x02\x81\x81T\x81\x10a\tUW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\0a\tya\x0B\x18V[`@Qc7\xC2\xC1+`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xDF\x0B\x04\xAC\x90a\t\xCD\x90\x89\x90\x89\x90\x89\x90\x89\x90\x89\x90`\x04\x01a\x10mV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x10\x91\x90a\x10PV[\x90Pa\n\x1D\x81\x87Ba\x0B\xC2V[\x95\x94PPPPPV[a\n.a\x0B\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\n\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\n\xA1\x81a\x0BrV[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x07(W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\nWPPPPP\x90P\x91\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\n\x8FV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 \x80T`\x01\x80\x82\x01\x83U\x91\x85R\x83\x85 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x8B\x89\x16\x90\x81\x17\x90\x92U\x83Q\x80\x85\x01\x85R\x87\x81R\x80\x86\x01\x8A\x81R\x92\x87R`\x06\x86R\x84\x87 \x90Q\x81T\x90\x92\x16\x91\x90\x98\x16\x17\x87UQ\x95\x01\x94\x90\x94U\x91\x81R`\x03\x90\x91R T`\xFF\x16a\x0C\xA9W`\x02\x80T`\x01\x80\x82\x01\x90\x92U\x7F@W\x87\xFA\x12\xA8#\xE0\xF2\xB7c\x1C\xC4\x1B;\xA8\x82\x8B3!\xCA\x81\x11\x11\xFAu\xCD:\xA3\xBBZ\xCE\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`\0\x90\x81R`\x03` R`@\x90 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F+\xC5B\xBE\xD0\x9D\xF2D\xBE\x94(M\xB7.k\xFF\xF8\xD6n\x84\xEA\x8D\xA6p\xDE!\xE6\xEB\xEC\x9B\t}`@Q`@Q\x80\x91\x03\x90\xA3PPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0\x80[\x83\x81\x10\x15a\rsW\x88\x86\x03`?\x19\x01\x85R\x82Q\x80Q\x80\x88R\x90\x88\x01\x90\x88\x88\x01\x90\x84[\x81\x81\x10\x15a\r]W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x8A\x01\x92\x91\x8A\x01\x91`\x01\x01a\r8V[P\x90\x97PPP\x93\x86\x01\x93\x91\x86\x01\x91`\x01\x01a\r\x16V[P\x93\x98\x97PPPPPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\r\xA7W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\r\x8BV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\r\xDA` \x83\x01\x84a\r\x81V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xA1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0E\x08W`\0\x80\xFD[\x815a\r\xDA\x81a\r\xE1V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0ETW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E/V[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0EsW`\0\x80\xFD[\x825a\x0E~\x81a\r\xE1V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x0E\x9EW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0E\xCCW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xE7Wa\x0E\xE7a\x0E\xA5V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0F\x0FWa\x0F\x0Fa\x0E\xA5V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0F(W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0F`W`\0\x80\xFD[\x855a\x0Fk\x81a\r\xE1V[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x88W`\0\x80\xFD[a\x0F\x94\x89\x83\x8A\x01a\x0E\xBBV[\x95P`@\x88\x015\x91P\x80\x82\x11\x15a\x0F\xAAW`\0\x80\xFD[Pa\x0F\xB7\x88\x82\x89\x01a\x0E\xBBV[\x93PP``\x86\x015a\x0F\xC8\x81a\r\xE1V[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x10\x0FWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x10*W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x10JWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10bW`\0\x80\xFD[\x81Qa\r\xDA\x81a\r\xE1V[`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x83R`\xA0` \x84\x01Ra\x10\x8F`\xA0\x84\x01\x88a\r\x81V[\x83\x81\x03`@\x85\x01Ra\x10\xA1\x81\x88a\r\x81V[\x95\x90\x91\x16``\x84\x01RPP`\x80\x01R\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x96\xF4%<\xE2\xA1\xAB\r\xEBq\xF5\xBD\xB9\xA9p\xCC\xDA\xADm2GV\x04\x16J\xBA\x8C\x80\x0F\x90)\xECdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static REGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Registry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Registry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Registry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Registry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Registry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Registry)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Registry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    REGISTRY_ABI.clone(),
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
                REGISTRY_ABI.clone(),
                REGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `assetIsUsed` (0xac01762a) function
        pub fn asset_is_used(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([172, 1, 118, 42], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assets` (0xcf35bdd0) function
        pub fn assets(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([207, 53, 189, 208], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployNewVault` (0xdf0b04ac) function
        pub fn deploy_new_vault(
            &self,
            asset: ::ethers::core::types::Address,
            name: ::std::string::String,
            symbol: ::std::string::String,
            role_manager: ::ethers::core::types::Address,
            profit_max_unlock_time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [223, 11, 4, 172],
                    (asset, name, symbol, role_manager, profit_max_unlock_time),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllDeployedStrategies` (0x4603d56e) function
        pub fn get_all_deployed_strategies(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
        > {
            self.0
                .method_hash([70, 3, 213, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllDeployedVaults` (0x06a10487) function
        pub fn get_all_deployed_vaults(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
        > {
            self.0
                .method_hash([6, 161, 4, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAssets` (0x67e4ac2c) function
        pub fn get_assets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([103, 228, 172, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeployedStrategies` (0xfeab1c6b) function
        pub fn get_deployed_strategies(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([254, 171, 28, 107], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeployedVaults` (0x47555688) function
        pub fn get_deployed_vaults(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([71, 85, 86, 136], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `info` (0x0aae7a6b) function
        pub fn info(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([10, 174, 122, 107], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numAssets` (0xa46fe83b) function
        pub fn num_assets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([164, 111, 232, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numDeployedStrategies` (0xa824c8f6) function
        pub fn num_deployed_strategies(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([168, 36, 200, 246], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numDeployedVaults` (0x9fdae39f) function
        pub fn num_deployed_vaults(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([159, 218, 227, 159], asset)
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
        ///Calls the contract's `registerStrategy` (0xa8da683f) function
        pub fn register_strategy(
            &self,
            strategy: ::ethers::core::types::Address,
            deployment_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([168, 218, 104, 63], (strategy, deployment_timestamp))
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `NewDeployedVault` event
        pub fn new_deployed_vault_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewDeployedVaultFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewStrategyRegistered` event
        pub fn new_strategy_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewStrategyRegisteredFilter,
        > {
            self.0.event()
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Registry<M> {
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
    #[ethevent(name = "NewDeployedVault", abi = "NewDeployedVault(address,address)")]
    pub struct NewDeployedVaultFilter {
        #[ethevent(indexed)]
        pub vault: ::ethers::core::types::Address,
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
        name = "NewStrategyRegistered",
        abi = "NewStrategyRegistered(address,address)"
    )]
    pub struct NewStrategyRegisteredFilter {
        #[ethevent(indexed)]
        pub strategy: ::ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
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
    pub enum RegistryEvents {
        NewDeployedVaultFilter(NewDeployedVaultFilter),
        NewStrategyRegisteredFilter(NewStrategyRegisteredFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for RegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = NewDeployedVaultFilter::decode_log(log) {
                return Ok(RegistryEvents::NewDeployedVaultFilter(decoded));
            }
            if let Ok(decoded) = NewStrategyRegisteredFilter::decode_log(log) {
                return Ok(RegistryEvents::NewStrategyRegisteredFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(RegistryEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for RegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NewDeployedVaultFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewStrategyRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<NewDeployedVaultFilter> for RegistryEvents {
        fn from(value: NewDeployedVaultFilter) -> Self {
            Self::NewDeployedVaultFilter(value)
        }
    }
    impl ::core::convert::From<NewStrategyRegisteredFilter> for RegistryEvents {
        fn from(value: NewStrategyRegisteredFilter) -> Self {
            Self::NewStrategyRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for RegistryEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `assetIsUsed` function with signature `assetIsUsed(address)` and selector `0xac01762a`
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
    #[ethcall(name = "assetIsUsed", abi = "assetIsUsed(address)")]
    pub struct AssetIsUsedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `assets` function with signature `assets(uint256)` and selector `0xcf35bdd0`
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
    #[ethcall(name = "assets", abi = "assets(uint256)")]
    pub struct AssetsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `deployNewVault` function with signature `deployNewVault(address,string,string,address,uint256)` and selector `0xdf0b04ac`
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
        name = "deployNewVault",
        abi = "deployNewVault(address,string,string,address,uint256)"
    )]
    pub struct DeployNewVaultCall {
        pub asset: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
        pub role_manager: ::ethers::core::types::Address,
        pub profit_max_unlock_time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `getAllDeployedStrategies` function with signature `getAllDeployedStrategies()` and selector `0x4603d56e`
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
    #[ethcall(name = "getAllDeployedStrategies", abi = "getAllDeployedStrategies()")]
    pub struct GetAllDeployedStrategiesCall;
    ///Container type for all input parameters for the `getAllDeployedVaults` function with signature `getAllDeployedVaults()` and selector `0x06a10487`
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
    #[ethcall(name = "getAllDeployedVaults", abi = "getAllDeployedVaults()")]
    pub struct GetAllDeployedVaultsCall;
    ///Container type for all input parameters for the `getAssets` function with signature `getAssets()` and selector `0x67e4ac2c`
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
    #[ethcall(name = "getAssets", abi = "getAssets()")]
    pub struct GetAssetsCall;
    ///Container type for all input parameters for the `getDeployedStrategies` function with signature `getDeployedStrategies(address)` and selector `0xfeab1c6b`
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
    #[ethcall(name = "getDeployedStrategies", abi = "getDeployedStrategies(address)")]
    pub struct GetDeployedStrategiesCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getDeployedVaults` function with signature `getDeployedVaults(address)` and selector `0x47555688`
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
    #[ethcall(name = "getDeployedVaults", abi = "getDeployedVaults(address)")]
    pub struct GetDeployedVaultsCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `info` function with signature `info(address)` and selector `0x0aae7a6b`
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
    #[ethcall(name = "info", abi = "info(address)")]
    pub struct InfoCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `numAssets` function with signature `numAssets()` and selector `0xa46fe83b`
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
    #[ethcall(name = "numAssets", abi = "numAssets()")]
    pub struct NumAssetsCall;
    ///Container type for all input parameters for the `numDeployedStrategies` function with signature `numDeployedStrategies(address)` and selector `0xa824c8f6`
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
    #[ethcall(name = "numDeployedStrategies", abi = "numDeployedStrategies(address)")]
    pub struct NumDeployedStrategiesCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `numDeployedVaults` function with signature `numDeployedVaults(address)` and selector `0x9fdae39f`
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
    #[ethcall(name = "numDeployedVaults", abi = "numDeployedVaults(address)")]
    pub struct NumDeployedVaultsCall {
        pub asset: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `registerStrategy` function with signature `registerStrategy(address,uint256)` and selector `0xa8da683f`
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
    #[ethcall(name = "registerStrategy", abi = "registerStrategy(address,uint256)")]
    pub struct RegisterStrategyCall {
        pub strategy: ::ethers::core::types::Address,
        pub deployment_timestamp: ::ethers::core::types::U256,
    }
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
    pub enum RegistryCalls {
        AssetIsUsed(AssetIsUsedCall),
        Assets(AssetsCall),
        DeployNewVault(DeployNewVaultCall),
        Factory(FactoryCall),
        GetAllDeployedStrategies(GetAllDeployedStrategiesCall),
        GetAllDeployedVaults(GetAllDeployedVaultsCall),
        GetAssets(GetAssetsCall),
        GetDeployedStrategies(GetDeployedStrategiesCall),
        GetDeployedVaults(GetDeployedVaultsCall),
        Info(InfoCall),
        Name(NameCall),
        NumAssets(NumAssetsCall),
        NumDeployedStrategies(NumDeployedStrategiesCall),
        NumDeployedVaults(NumDeployedVaultsCall),
        Owner(OwnerCall),
        RegisterStrategy(RegisterStrategyCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for RegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AssetIsUsedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssetIsUsed(decoded));
            }
            if let Ok(decoded) = <AssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Assets(decoded));
            }
            if let Ok(decoded) = <DeployNewVaultCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployNewVault(decoded));
            }
            if let Ok(decoded) = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded) = <GetAllDeployedStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAllDeployedStrategies(decoded));
            }
            if let Ok(decoded) = <GetAllDeployedVaultsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAllDeployedVaults(decoded));
            }
            if let Ok(decoded) = <GetAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAssets(decoded));
            }
            if let Ok(decoded) = <GetDeployedStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDeployedStrategies(decoded));
            }
            if let Ok(decoded) = <GetDeployedVaultsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDeployedVaults(decoded));
            }
            if let Ok(decoded) = <InfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Info(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NumAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NumAssets(decoded));
            }
            if let Ok(decoded) = <NumDeployedStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NumDeployedStrategies(decoded));
            }
            if let Ok(decoded) = <NumDeployedVaultsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NumDeployedVaults(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RegisterStrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterStrategy(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetIsUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Assets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeployNewVault(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllDeployedStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllDeployedVaults(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDeployedStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDeployedVaults(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Info(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NumAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumDeployedStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumDeployedVaults(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterStrategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetIsUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Assets(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployNewVault(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllDeployedStrategies(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAllDeployedVaults(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeployedStrategies(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDeployedVaults(element) => ::core::fmt::Display::fmt(element, f),
                Self::Info(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumDeployedStrategies(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NumDeployedVaults(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterStrategy(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssetIsUsedCall> for RegistryCalls {
        fn from(value: AssetIsUsedCall) -> Self {
            Self::AssetIsUsed(value)
        }
    }
    impl ::core::convert::From<AssetsCall> for RegistryCalls {
        fn from(value: AssetsCall) -> Self {
            Self::Assets(value)
        }
    }
    impl ::core::convert::From<DeployNewVaultCall> for RegistryCalls {
        fn from(value: DeployNewVaultCall) -> Self {
            Self::DeployNewVault(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for RegistryCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<GetAllDeployedStrategiesCall> for RegistryCalls {
        fn from(value: GetAllDeployedStrategiesCall) -> Self {
            Self::GetAllDeployedStrategies(value)
        }
    }
    impl ::core::convert::From<GetAllDeployedVaultsCall> for RegistryCalls {
        fn from(value: GetAllDeployedVaultsCall) -> Self {
            Self::GetAllDeployedVaults(value)
        }
    }
    impl ::core::convert::From<GetAssetsCall> for RegistryCalls {
        fn from(value: GetAssetsCall) -> Self {
            Self::GetAssets(value)
        }
    }
    impl ::core::convert::From<GetDeployedStrategiesCall> for RegistryCalls {
        fn from(value: GetDeployedStrategiesCall) -> Self {
            Self::GetDeployedStrategies(value)
        }
    }
    impl ::core::convert::From<GetDeployedVaultsCall> for RegistryCalls {
        fn from(value: GetDeployedVaultsCall) -> Self {
            Self::GetDeployedVaults(value)
        }
    }
    impl ::core::convert::From<InfoCall> for RegistryCalls {
        fn from(value: InfoCall) -> Self {
            Self::Info(value)
        }
    }
    impl ::core::convert::From<NameCall> for RegistryCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NumAssetsCall> for RegistryCalls {
        fn from(value: NumAssetsCall) -> Self {
            Self::NumAssets(value)
        }
    }
    impl ::core::convert::From<NumDeployedStrategiesCall> for RegistryCalls {
        fn from(value: NumDeployedStrategiesCall) -> Self {
            Self::NumDeployedStrategies(value)
        }
    }
    impl ::core::convert::From<NumDeployedVaultsCall> for RegistryCalls {
        fn from(value: NumDeployedVaultsCall) -> Self {
            Self::NumDeployedVaults(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for RegistryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RegisterStrategyCall> for RegistryCalls {
        fn from(value: RegisterStrategyCall) -> Self {
            Self::RegisterStrategy(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for RegistryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for RegistryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `assetIsUsed` function with signature `assetIsUsed(address)` and selector `0xac01762a`
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
    pub struct AssetIsUsedReturn(pub bool);
    ///Container type for all return fields from the `assets` function with signature `assets(uint256)` and selector `0xcf35bdd0`
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
    pub struct AssetsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `deployNewVault` function with signature `deployNewVault(address,string,string,address,uint256)` and selector `0xdf0b04ac`
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
    pub struct DeployNewVaultReturn {
        pub vault: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getAllDeployedStrategies` function with signature `getAllDeployedStrategies()` and selector `0x4603d56e`
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
    pub struct GetAllDeployedStrategiesReturn {
        pub all_deployed_strategies: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::Address>,
        >,
    }
    ///Container type for all return fields from the `getAllDeployedVaults` function with signature `getAllDeployedVaults()` and selector `0x06a10487`
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
    pub struct GetAllDeployedVaultsReturn {
        pub all_deployed_vaults: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::Address>,
        >,
    }
    ///Container type for all return fields from the `getAssets` function with signature `getAssets()` and selector `0x67e4ac2c`
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
    pub struct GetAssetsReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getDeployedStrategies` function with signature `getDeployedStrategies(address)` and selector `0xfeab1c6b`
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
    pub struct GetDeployedStrategiesReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getDeployedVaults` function with signature `getDeployedVaults(address)` and selector `0x47555688`
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
    pub struct GetDeployedVaultsReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `info` function with signature `info(address)` and selector `0x0aae7a6b`
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
    pub struct InfoReturn {
        pub asset: ::ethers::core::types::Address,
        pub deployment_time_stamp: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `numAssets` function with signature `numAssets()` and selector `0xa46fe83b`
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
    pub struct NumAssetsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `numDeployedStrategies` function with signature `numDeployedStrategies(address)` and selector `0xa824c8f6`
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
    pub struct NumDeployedStrategiesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `numDeployedVaults` function with signature `numDeployedVaults(address)` and selector `0x9fdae39f`
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
    pub struct NumDeployedVaultsReturn(pub ::ethers::core::types::U256);
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
