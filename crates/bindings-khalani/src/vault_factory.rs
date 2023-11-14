pub use vault_factory::*;
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
pub mod vault_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DEFAULT_ADMIN_ROLE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VAULT_DEPLOYER_ROLE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "VAULT_DEPLOYER_ROLE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
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
                                    name: ::std::borrow::ToOwned::to_owned("roleManager"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "profitMaxUnlockTime",
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleAdmin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoleMember"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMember"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRoleMemberCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("getVaultFromUnderlying"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getVaultFromUnderlying",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
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
                    ::std::borrow::ToOwned::to_owned("grantRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("grantRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("hasRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hasRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("isVaultDeployed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isVaultDeployed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vault"),
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
                    ::std::borrow::ToOwned::to_owned("renounceRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("revokeRole"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeRole"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("shutdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("shutdown"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("shutdownFactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("shutdownFactory"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
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
                    ::std::borrow::ToOwned::to_owned("vaultImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "vaultImplementation",
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FactoryShutdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FactoryShutdown"),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewVault"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewVault"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vaultAddress"),
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
                    ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleAdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdminRole"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleGranted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleGranted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RoleRevoked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
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
                    ::std::borrow::ToOwned::to_owned("Shutdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Shutdown"),
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
    pub static VAULTFACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x17Q8\x03\x80b\0\x17Q\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x02\x11V[b\0\0A`\0\x82b\0\0\xB6V[b\0\0\\`\0\x80Q` b\0\x171\x839\x81Q\x91R\x82b\0\0\xB6V[b\0\0w`\0\x80Q` b\0\x171\x839\x81Q\x91R3b\0\0\xB6V[`@Qb\0\0\x85\x90b\0\x02\x03V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0\xA2W=`\0\x80>=`\0\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80RPb\0\x02CV[b\0\0\xC2\x82\x82b\0\0\xC6V[PPV[b\0\0\xD2\x82\x82b\0\0\xF1V[`\0\x82\x81R`\x01` R`@\x90 b\0\0\xEC\x90\x82b\0\x01\x91V[PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\0\xC2W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01M3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0b\0\x01\xA8\x83`\x01`\x01`\xA0\x1B\x03\x84\x16b\0\x01\xB1V[\x90P[\x92\x91PPV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Tb\0\x01\xFAWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ub\0\x01\xABV[P`\0b\0\x01\xABV[a\x01\xE9\x80b\0\x15H\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0\x02$W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02<W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x12\xDBb\0\x02m`\09`\0\x81\x81a\x02\x15\x01R\x81\x81a\x07n\x01Ra\x08\x8D\x01Ra\x12\xDB`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80c\xA2\x17\xFD\xDF\x11a\0\x97W\x80c\xDB\x8AZ\xC8\x11a\0fW\x80c\xDB\x8AZ\xC8\x14a\x02]W\x80c\xDF\x0B\x04\xAC\x14a\x02eW\x80c\xF3\xA6\x95\\\x14a\x02xW\x80c\xFC\x0Et\xD1\x14a\x02\x8BW`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x02\x08W\x80c\xBB\xA4\x8A\x90\x14a\x02\x10W\x80c\xCA\x15\xC8s\x14a\x027W\x80c\xD5Gt\x1F\x14a\x02JW`\0\x80\xFD[\x80cU-\xE3\x8C\x11a\0\xD3W\x80cU-\xE3\x8C\x14a\x01\x86W\x80c}` \xF4\x14a\x01\xADW\x80c\x90\x10\xD0|\x14a\x01\xCAW\x80c\x91\xD1HT\x14a\x01\xF5W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x05W\x80c$\x8A\x9C\xA3\x14a\x01-W\x80c//\xF1]\x14a\x01^W\x80c6V\x8A\xBE\x14a\x01sW[`\0\x80\xFD[a\x01\x18a\x01\x136`\x04a\x0EKV[a\x02\x98V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01Pa\x01;6`\x04a\x0EuV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01$V[a\x01qa\x01l6`\x04a\x0E\xAAV[a\x02\xC3V[\0[a\x01qa\x01\x816`\x04a\x0E\xAAV[a\x02\xEDV[a\x01P\x7F\x87\xA2\x94\xB4\xB6\x0E\xBE\x1By\xE7\xCCxEt\x0Fo\"=\xC2(\x13*{'rQ%A\x16D=\xCB\x81V[a\x01\x18a\x01\xBB6`\x04a\x0E\xD6V[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[a\x01\xDDa\x01\xD86`\x04a\x0E\xF1V[a\x03pV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01$V[a\x01\x18a\x02\x036`\x04a\x0E\xAAV[a\x03\x8FV[a\x01P`\0\x81V[a\x01\xDD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01Pa\x02E6`\x04a\x0EuV[a\x03\xB8V[a\x01qa\x02X6`\x04a\x0E\xAAV[a\x03\xCFV[a\x01qa\x03\xF4V[a\x01\xDDa\x02s6`\x04a\x0F\xB6V[a\x04\xB8V[a\x01\xDDa\x02\x866`\x04a\x10CV[a\x07\x1FV[`\x02Ta\x01\x18\x90`\xFF\x16\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x02\xBDWPa\x02\xBD\x82a\x07\xE1V[\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x02\xDE\x81a\x08\x16V[a\x02\xE8\x83\x83a\x08#V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03l\x82\x82a\x08EV[PPV[`\0\x82\x81R`\x01` R`@\x81 a\x03\x88\x90\x83a\x08gV[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x81\x81R`\x01` R`@\x81 a\x02\xBD\x90a\x08sV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x03\xEA\x81a\x08\x16V[a\x02\xE8\x83\x83a\x08EV[a\x03\xFF`\x003a\x03\x8FV[a\x04\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FMust have DEFAULT_ADMIN_ROLE to `D\x82\x01Rg9\xB4:\xBA27\xBB\xB7`\xC1\x1B`d\x82\x01R`\x84\x01a\x03YV[`\x02T`\xFF\x16\x15a\x04\x80W`@QcD&\xAA\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\xFF\x19\x16`\x01\x17\x90U`@Q\x7F\xC6C\x19:\x97\xFC\x0E\x18\xD6\x9C\x95\xE1\xC04\xB9\x1FQ\xFA\x16K\xA8\xEAh\xDF\xB6\xDD\x98V\x8B\x9B\xC9k\x90`\0\x90\xA1V[`\0a\x04\xE4\x7F\x87\xA2\x94\xB4\xB6\x0E\xBE\x1By\xE7\xCCxEt\x0Fo\"=\xC2(\x13*{'rQ%A\x16D=\xCB3a\x03\x8FV[a\x05JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FMust have VAULT_DEPLOYER_ROLE to`D\x82\x01Rp\x08\x19\x19\\\x1B\x1B\xDEH\x1B\x99]\xC8\x15\x98][\x1D`z\x1B`d\x82\x01R`\x84\x01a\x03YV[\x85`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\x05\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FAsset must be a smart contract\0\0`D\x82\x01R`d\x01a\x03YV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x06\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FRole manager cannot be zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x03YV[`\0\x82\x11a\x06oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FProfit max unlock time must be g`D\x82\x01Roreater than zero`\x80\x1B`d\x82\x01R`\x84\x01a\x03YV[`\x02T`\xFF\x16\x15a\x06\x93W`@QcD&\xAA\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x86\x86\x86`@Q` \x01a\x06\xAA\x93\x92\x91\x90a\x11\x07V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x06\xD2\x88\x88\x88\x88\x88\x87a\x08}V[\x90P\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7FBA0,9<q>i\x07\x02\xC4\xA4ZW\xE9<\xEFY\xAA\x8Cn#XIXS\xB3B\x05Q\xD8`@Q`@Q\x80\x91\x03\x90\xA3\x97\x96PPPPPPPV[`\0\x80\x84\x84\x84`@Q` \x01a\x077\x93\x92\x91\x90a\x11\x07V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 0`8\x83\x01RoZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\xFF`$\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16`\x14\x83\x01Rs=`-\x80`\n=9\x81\xF36==7===6=s\x82R`X\x82\x01\x81\x90R`7`\x0C\x83\x01 `x\x83\x01R`U`C\x90\x92\x01\x91\x90\x91 \x90\x91P`\0\x90[\x96\x95PPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x02\xBDWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x02\xBDV[a\x08 \x813a\t(V[PV[a\x08-\x82\x82a\t\x81V[`\0\x82\x81R`\x01` R`@\x90 a\x02\xE8\x90\x82a\n\x05V[a\x08O\x82\x82a\n\x1AV[`\0\x82\x81R`\x01` R`@\x90 a\x02\xE8\x90\x82a\n\x7FV[`\0a\x03\x88\x83\x83a\n\x94V[`\0a\x02\xBD\x82T\x90V[`\0\x80a\x08\xB3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84a\n\xBEV[`@Qc:\xD9\x85\xF3`\xE1\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cu\xB3\x0B\xE6\x90a\x08\xEA\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01a\x11=V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\x18W=`\0\x80>=`\0\xFD[P\x92\x9A\x99PPPPPPPPPPV[a\t2\x82\x82a\x03\x8FV[a\x03lWa\t?\x81a\x0B[V[a\tJ\x83` a\x0BmV[`@Q` \x01a\t[\x92\x91\x90a\x11\x87V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03Y\x91`\x04\x01a\x11\xFCV[a\t\x8B\x82\x82a\x03\x8FV[a\x03lW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\t\xC13\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x03\x88\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\r\tV[a\n$\x82\x82a\x03\x8FV[\x15a\x03lW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x03\x88\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\rXV[`\0\x82`\0\x01\x82\x81T\x81\x10a\n\xABWa\n\xABa\x12\x0FV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x83``\x1B`\xE8\x1C\x17`\0RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x83`x\x1B\x17` R\x81`7`\t`\0\xF5\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FERC1167: create2 failed\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03YV[``a\x02\xBD`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x0B|\x83`\x02a\x12;V[a\x0B\x87\x90`\x02a\x12RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x9FWa\x0B\x9Fa\x0F\x13V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\xC9W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x0B\xE4Wa\x0B\xE4a\x12\x0FV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0C\x13Wa\x0C\x13a\x12\x0FV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0C7\x84`\x02a\x12;V[a\x0CB\x90`\x01a\x12RV[\x90P[`\x01\x81\x11\x15a\x0C\xBAWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0CvWa\x0Cva\x12\x0FV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0C\x8CWa\x0C\x8Ca\x12\x0FV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0C\xB3\x81a\x12eV[\x90Pa\x0CEV[P\x83\x15a\x03\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03YV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\rPWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02\xBDV[P`\0a\x02\xBDV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x0EAW`\0a\r|`\x01\x83a\x12|V[\x85T\x90\x91P`\0\x90a\r\x90\x90`\x01\x90a\x12|V[\x90P\x81\x81\x14a\r\xF5W`\0\x86`\0\x01\x82\x81T\x81\x10a\r\xB0Wa\r\xB0a\x12\x0FV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\r\xD3Wa\r\xD3a\x12\x0FV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x0E\x06Wa\x0E\x06a\x12\x8FV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x02\xBDV[`\0\x91PPa\x02\xBDV[`\0` \x82\x84\x03\x12\x15a\x0E]W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x03\x88W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0E\x87W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xA5W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xBDW`\0\x80\xFD[\x825\x91Pa\x0E\xCD` \x84\x01a\x0E\x8EV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0E\xE8W`\0\x80\xFD[a\x03\x88\x82a\x0E\x8EV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\x04W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0F:W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FUWa\x0FUa\x0F\x13V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0F}Wa\x0F}a\x0F\x13V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0F\x96W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0F\xCEW`\0\x80\xFD[a\x0F\xD7\x86a\x0E\x8EV[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xF4W`\0\x80\xFD[a\x10\0\x89\x83\x8A\x01a\x0F)V[\x95P`@\x88\x015\x91P\x80\x82\x11\x15a\x10\x16W`\0\x80\xFD[Pa\x10#\x88\x82\x89\x01a\x0F)V[\x93PPa\x102``\x87\x01a\x0E\x8EV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10XW`\0\x80\xFD[a\x10a\x84a\x0E\x8EV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10~W`\0\x80\xFD[a\x10\x8A\x87\x83\x88\x01a\x0F)V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x10\xA0W`\0\x80\xFD[Pa\x10\xAD\x86\x82\x87\x01a\x0F)V[\x91PP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x10\xD2W\x81\x81\x01Q\x83\x82\x01R` \x01a\x10\xBAV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x10\xF3\x81` \x86\x01` \x86\x01a\x10\xB7V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R`\0\x90a\x11+\x90\x83\x01\x85a\x10\xDBV[\x82\x81\x03`@\x84\x01Ra\x07\xD7\x81\x85a\x10\xDBV[`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x83R`\xA0` \x84\x01Ra\x11_`\xA0\x84\x01\x88a\x10\xDBV[\x83\x81\x03`@\x85\x01Ra\x11q\x81\x88a\x10\xDBV[\x95\x90\x91\x16``\x84\x01RPP`\x80\x01R\x93\x92PPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x11\xBF\x81`\x17\x85\x01` \x88\x01a\x10\xB7V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x11\xF0\x81`(\x84\x01` \x88\x01a\x10\xB7V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0a\x03\x88` \x83\x01\x84a\x10\xDBV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xBDWa\x02\xBDa\x12%V[\x80\x82\x01\x80\x82\x11\x15a\x02\xBDWa\x02\xBDa\x12%V[`\0\x81a\x12tWa\x12ta\x12%V[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x02\xBDWa\x02\xBDa\x12%V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x83a\xBB\xEA\x02N\xF6\x1F\xEA\xC23?_\xAA5\xBC\x0E/8j9\xA7\xBC#\xCC\x13tT@7\xA4adsolcC\0\x08\x13\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x01\xC9\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80cu\xB3\x0B\xE6\x14a\x000W[`\0\x80\xFD[a\0Ea\0>6`\x04a\x01\x06V[PPPPPV[\0[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0^W`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\0\x8AW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xA5Wa\0\xA5a\0cV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\0\xCDWa\0\xCDa\0cV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\0\xE6W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x01\x1EW`\0\x80\xFD[a\x01'\x86a\0GV[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01DW`\0\x80\xFD[a\x01P\x89\x83\x8A\x01a\0yV[\x95P`@\x88\x015\x91P\x80\x82\x11\x15a\x01fW`\0\x80\xFD[Pa\x01s\x88\x82\x89\x01a\0yV[\x93PPa\x01\x82``\x87\x01a\0GV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV\xFE\xA2dipfsX\"\x12 J&(\x0C3#W\x9Bj#\xE0>\xEA6\t\x10\xAE\xDFM\xE3\x1B\xCA\x0B3<\xAE\xC9=\xCBf\x07WdsolcC\0\x08\x13\x003\x87\xA2\x94\xB4\xB6\x0E\xBE\x1By\xE7\xCCxEt\x0Fo\"=\xC2(\x13*{'rQ%A\x16D=\xCB";
    /// The bytecode of the contract.
    pub static VAULTFACTORY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80c\xA2\x17\xFD\xDF\x11a\0\x97W\x80c\xDB\x8AZ\xC8\x11a\0fW\x80c\xDB\x8AZ\xC8\x14a\x02]W\x80c\xDF\x0B\x04\xAC\x14a\x02eW\x80c\xF3\xA6\x95\\\x14a\x02xW\x80c\xFC\x0Et\xD1\x14a\x02\x8BW`\0\x80\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x02\x08W\x80c\xBB\xA4\x8A\x90\x14a\x02\x10W\x80c\xCA\x15\xC8s\x14a\x027W\x80c\xD5Gt\x1F\x14a\x02JW`\0\x80\xFD[\x80cU-\xE3\x8C\x11a\0\xD3W\x80cU-\xE3\x8C\x14a\x01\x86W\x80c}` \xF4\x14a\x01\xADW\x80c\x90\x10\xD0|\x14a\x01\xCAW\x80c\x91\xD1HT\x14a\x01\xF5W`\0\x80\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x05W\x80c$\x8A\x9C\xA3\x14a\x01-W\x80c//\xF1]\x14a\x01^W\x80c6V\x8A\xBE\x14a\x01sW[`\0\x80\xFD[a\x01\x18a\x01\x136`\x04a\x0EKV[a\x02\x98V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01Pa\x01;6`\x04a\x0EuV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[`@Q\x90\x81R` \x01a\x01$V[a\x01qa\x01l6`\x04a\x0E\xAAV[a\x02\xC3V[\0[a\x01qa\x01\x816`\x04a\x0E\xAAV[a\x02\xEDV[a\x01P\x7F\x87\xA2\x94\xB4\xB6\x0E\xBE\x1By\xE7\xCCxEt\x0Fo\"=\xC2(\x13*{'rQ%A\x16D=\xCB\x81V[a\x01\x18a\x01\xBB6`\x04a\x0E\xD6V[`\x01`\x01`\xA0\x1B\x03\x16;\x15\x15\x90V[a\x01\xDDa\x01\xD86`\x04a\x0E\xF1V[a\x03pV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01$V[a\x01\x18a\x02\x036`\x04a\x0E\xAAV[a\x03\x8FV[a\x01P`\0\x81V[a\x01\xDD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01Pa\x02E6`\x04a\x0EuV[a\x03\xB8V[a\x01qa\x02X6`\x04a\x0E\xAAV[a\x03\xCFV[a\x01qa\x03\xF4V[a\x01\xDDa\x02s6`\x04a\x0F\xB6V[a\x04\xB8V[a\x01\xDDa\x02\x866`\x04a\x10CV[a\x07\x1FV[`\x02Ta\x01\x18\x90`\xFF\x16\x81V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cZ\x05\x18\x0F`\xE0\x1B\x14\x80a\x02\xBDWPa\x02\xBD\x82a\x07\xE1V[\x92\x91PPV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x02\xDE\x81a\x08\x16V[a\x02\xE8\x83\x83a\x08#V[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[a\x03l\x82\x82a\x08EV[PPV[`\0\x82\x81R`\x01` R`@\x81 a\x03\x88\x90\x83a\x08gV[\x93\x92PPPV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x81\x81R`\x01` R`@\x81 a\x02\xBD\x90a\x08sV[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x03\xEA\x81a\x08\x16V[a\x02\xE8\x83\x83a\x08EV[a\x03\xFF`\x003a\x03\x8FV[a\x04\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FMust have DEFAULT_ADMIN_ROLE to `D\x82\x01Rg9\xB4:\xBA27\xBB\xB7`\xC1\x1B`d\x82\x01R`\x84\x01a\x03YV[`\x02T`\xFF\x16\x15a\x04\x80W`@QcD&\xAA\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x80T`\xFF\x19\x16`\x01\x17\x90U`@Q\x7F\xC6C\x19:\x97\xFC\x0E\x18\xD6\x9C\x95\xE1\xC04\xB9\x1FQ\xFA\x16K\xA8\xEAh\xDF\xB6\xDD\x98V\x8B\x9B\xC9k\x90`\0\x90\xA1V[`\0a\x04\xE4\x7F\x87\xA2\x94\xB4\xB6\x0E\xBE\x1By\xE7\xCCxEt\x0Fo\"=\xC2(\x13*{'rQ%A\x16D=\xCB3a\x03\x8FV[a\x05JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FMust have VAULT_DEPLOYER_ROLE to`D\x82\x01Rp\x08\x19\x19\\\x1B\x1B\xDEH\x1B\x99]\xC8\x15\x98][\x1D`z\x1B`d\x82\x01R`\x84\x01a\x03YV[\x85`\x01`\x01`\xA0\x1B\x03\x16;`\0\x03a\x05\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FAsset must be a smart contract\0\0`D\x82\x01R`d\x01a\x03YV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x06\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FRole manager cannot be zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x03YV[`\0\x82\x11a\x06oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7FProfit max unlock time must be g`D\x82\x01Roreater than zero`\x80\x1B`d\x82\x01R`\x84\x01a\x03YV[`\x02T`\xFF\x16\x15a\x06\x93W`@QcD&\xAA\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x86\x86\x86`@Q` \x01a\x06\xAA\x93\x92\x91\x90a\x11\x07V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x06\xD2\x88\x88\x88\x88\x88\x87a\x08}V[\x90P\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7FBA0,9<q>i\x07\x02\xC4\xA4ZW\xE9<\xEFY\xAA\x8Cn#XIXS\xB3B\x05Q\xD8`@Q`@Q\x80\x91\x03\x90\xA3\x97\x96PPPPPPPV[`\0\x80\x84\x84\x84`@Q` \x01a\x077\x93\x92\x91\x90a\x11\x07V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 0`8\x83\x01RoZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\xFF`$\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16`\x14\x83\x01Rs=`-\x80`\n=9\x81\xF36==7===6=s\x82R`X\x82\x01\x81\x90R`7`\x0C\x83\x01 `x\x83\x01R`U`C\x90\x92\x01\x91\x90\x91 \x90\x91P`\0\x90[\x96\x95PPPPPPV[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x02\xBDWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x02\xBDV[a\x08 \x813a\t(V[PV[a\x08-\x82\x82a\t\x81V[`\0\x82\x81R`\x01` R`@\x90 a\x02\xE8\x90\x82a\n\x05V[a\x08O\x82\x82a\n\x1AV[`\0\x82\x81R`\x01` R`@\x90 a\x02\xE8\x90\x82a\n\x7FV[`\0a\x03\x88\x83\x83a\n\x94V[`\0a\x02\xBD\x82T\x90V[`\0\x80a\x08\xB3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84a\n\xBEV[`@Qc:\xD9\x85\xF3`\xE1\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cu\xB3\x0B\xE6\x90a\x08\xEA\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01a\x11=V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\x18W=`\0\x80>=`\0\xFD[P\x92\x9A\x99PPPPPPPPPPV[a\t2\x82\x82a\x03\x8FV[a\x03lWa\t?\x81a\x0B[V[a\tJ\x83` a\x0BmV[`@Q` \x01a\t[\x92\x91\x90a\x11\x87V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x03Y\x91`\x04\x01a\x11\xFCV[a\t\x8B\x82\x82a\x03\x8FV[a\x03lW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\t\xC13\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\0a\x03\x88\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\r\tV[a\n$\x82\x82a\x03\x8FV[\x15a\x03lW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[`\0a\x03\x88\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a\rXV[`\0\x82`\0\x01\x82\x81T\x81\x10a\n\xABWa\n\xABa\x12\x0FV[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x83``\x1B`\xE8\x1C\x17`\0RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x83`x\x1B\x17` R\x81`7`\t`\0\xF5\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FERC1167: create2 failed\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x03YV[``a\x02\xBD`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a\x0B|\x83`\x02a\x12;V[a\x0B\x87\x90`\x02a\x12RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x9FWa\x0B\x9Fa\x0F\x13V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0B\xC9W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a\x0B\xE4Wa\x0B\xE4a\x12\x0FV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a\x0C\x13Wa\x0C\x13a\x12\x0FV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a\x0C7\x84`\x02a\x12;V[a\x0CB\x90`\x01a\x12RV[\x90P[`\x01\x81\x11\x15a\x0C\xBAWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a\x0CvWa\x0Cva\x12\x0FV[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a\x0C\x8CWa\x0C\x8Ca\x12\x0FV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a\x0C\xB3\x81a\x12eV[\x90Pa\x0CEV[P\x83\x15a\x03\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x03YV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta\rPWP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x02\xBDV[P`\0a\x02\xBDV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x0EAW`\0a\r|`\x01\x83a\x12|V[\x85T\x90\x91P`\0\x90a\r\x90\x90`\x01\x90a\x12|V[\x90P\x81\x81\x14a\r\xF5W`\0\x86`\0\x01\x82\x81T\x81\x10a\r\xB0Wa\r\xB0a\x12\x0FV[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a\r\xD3Wa\r\xD3a\x12\x0FV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x0E\x06Wa\x0E\x06a\x12\x8FV[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x02\xBDV[`\0\x91PPa\x02\xBDV[`\0` \x82\x84\x03\x12\x15a\x0E]W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x03\x88W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0E\x87W`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xA5W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xBDW`\0\x80\xFD[\x825\x91Pa\x0E\xCD` \x84\x01a\x0E\x8EV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0E\xE8W`\0\x80\xFD[a\x03\x88\x82a\x0E\x8EV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\x04W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0F:W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FUWa\x0FUa\x0F\x13V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0F}Wa\x0F}a\x0F\x13V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0F\x96W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x0F\xCEW`\0\x80\xFD[a\x0F\xD7\x86a\x0E\x8EV[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xF4W`\0\x80\xFD[a\x10\0\x89\x83\x8A\x01a\x0F)V[\x95P`@\x88\x015\x91P\x80\x82\x11\x15a\x10\x16W`\0\x80\xFD[Pa\x10#\x88\x82\x89\x01a\x0F)V[\x93PPa\x102``\x87\x01a\x0E\x8EV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10XW`\0\x80\xFD[a\x10a\x84a\x0E\x8EV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10~W`\0\x80\xFD[a\x10\x8A\x87\x83\x88\x01a\x0F)V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x10\xA0W`\0\x80\xFD[Pa\x10\xAD\x86\x82\x87\x01a\x0F)V[\x91PP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x10\xD2W\x81\x81\x01Q\x83\x82\x01R` \x01a\x10\xBAV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x10\xF3\x81` \x86\x01` \x86\x01a\x10\xB7V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R`\0\x90a\x11+\x90\x83\x01\x85a\x10\xDBV[\x82\x81\x03`@\x84\x01Ra\x07\xD7\x81\x85a\x10\xDBV[`\0`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x83R`\xA0` \x84\x01Ra\x11_`\xA0\x84\x01\x88a\x10\xDBV[\x83\x81\x03`@\x85\x01Ra\x11q\x81\x88a\x10\xDBV[\x95\x90\x91\x16``\x84\x01RPP`\x80\x01R\x93\x92PPPV[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa\x11\xBF\x81`\x17\x85\x01` \x88\x01a\x10\xB7V[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa\x11\xF0\x81`(\x84\x01` \x88\x01a\x10\xB7V[\x01`(\x01\x94\x93PPPPV[` \x81R`\0a\x03\x88` \x83\x01\x84a\x10\xDBV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xBDWa\x02\xBDa\x12%V[\x80\x82\x01\x80\x82\x11\x15a\x02\xBDWa\x02\xBDa\x12%V[`\0\x81a\x12tWa\x12ta\x12%V[P`\0\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x02\xBDWa\x02\xBDa\x12%V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \x83a\xBB\xEA\x02N\xF6\x1F\xEA\xC23?_\xAA5\xBC\x0E/8j9\xA7\xBC#\xCC\x13tT@7\xA4adsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static VAULTFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct VaultFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for VaultFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for VaultFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for VaultFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for VaultFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(VaultFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> VaultFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VAULTFACTORY_ABI.clone(),
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
                VAULTFACTORY_ABI.clone(),
                VAULTFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `VAULT_DEPLOYER_ROLE` (0x552de38c) function
        pub fn vault_deployer_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([85, 45, 227, 140], ())
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
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMember` (0x9010d07c) function
        pub fn get_role_member(
            &self,
            role: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([144, 16, 208, 124], (role, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleMemberCount` (0xca15c873) function
        pub fn get_role_member_count(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 21, 200, 115], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVaultFromUnderlying` (0xf3a6955c) function
        pub fn get_vault_from_underlying(
            &self,
            asset: ::ethers::core::types::Address,
            name: ::std::string::String,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([243, 166, 149, 92], (asset, name, symbol))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isVaultDeployed` (0x7d6020f4) function
        pub fn is_vault_deployed(
            &self,
            vault: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([125, 96, 32, 244], vault)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shutdown` (0xfc0e74d1) function
        pub fn shutdown(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([252, 14, 116, 209], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shutdownFactory` (0xdb8a5ac8) function
        pub fn shutdown_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 138, 90, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vaultImplementation` (0xbba48a90) function
        pub fn vault_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([187, 164, 138, 144], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FactoryShutdown` event
        pub fn factory_shutdown_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FactoryShutdownFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewVault` event
        pub fn new_vault_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewVaultFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VaultFactoryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for VaultFactory<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Shutdown` with signature `Shutdown()` and selector `0x4426aa1f`
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
    #[etherror(name = "Shutdown", abi = "Shutdown()")]
    pub struct Shutdown;
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
    #[ethevent(name = "FactoryShutdown", abi = "FactoryShutdown()")]
    pub struct FactoryShutdownFilter;
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
    #[ethevent(name = "NewVault", abi = "NewVault(address,address)")]
    pub struct NewVaultFilter {
        #[ethevent(indexed)]
        pub vault_address: ::ethers::core::types::Address,
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
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    pub enum VaultFactoryEvents {
        FactoryShutdownFilter(FactoryShutdownFilter),
        NewVaultFilter(NewVaultFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for VaultFactoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FactoryShutdownFilter::decode_log(log) {
                return Ok(VaultFactoryEvents::FactoryShutdownFilter(decoded));
            }
            if let Ok(decoded) = NewVaultFilter::decode_log(log) {
                return Ok(VaultFactoryEvents::NewVaultFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(VaultFactoryEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(VaultFactoryEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(VaultFactoryEvents::RoleRevokedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for VaultFactoryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FactoryShutdownFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewVaultFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FactoryShutdownFilter> for VaultFactoryEvents {
        fn from(value: FactoryShutdownFilter) -> Self {
            Self::FactoryShutdownFilter(value)
        }
    }
    impl ::core::convert::From<NewVaultFilter> for VaultFactoryEvents {
        fn from(value: NewVaultFilter) -> Self {
            Self::NewVaultFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for VaultFactoryEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for VaultFactoryEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for VaultFactoryEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `VAULT_DEPLOYER_ROLE` function with signature `VAULT_DEPLOYER_ROLE()` and selector `0x552de38c`
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
    #[ethcall(name = "VAULT_DEPLOYER_ROLE", abi = "VAULT_DEPLOYER_ROLE()")]
    pub struct VaultDeployerRoleCall;
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
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    #[ethcall(name = "getRoleMember", abi = "getRoleMember(bytes32,uint256)")]
    pub struct GetRoleMemberCall {
        pub role: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    #[ethcall(name = "getRoleMemberCount", abi = "getRoleMemberCount(bytes32)")]
    pub struct GetRoleMemberCountCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `getVaultFromUnderlying` function with signature `getVaultFromUnderlying(address,string,string)` and selector `0xf3a6955c`
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
        name = "getVaultFromUnderlying",
        abi = "getVaultFromUnderlying(address,string,string)"
    )]
    pub struct GetVaultFromUnderlyingCall {
        pub asset: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isVaultDeployed` function with signature `isVaultDeployed(address)` and selector `0x7d6020f4`
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
    #[ethcall(name = "isVaultDeployed", abi = "isVaultDeployed(address)")]
    pub struct IsVaultDeployedCall {
        pub vault: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `shutdown` function with signature `shutdown()` and selector `0xfc0e74d1`
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
    #[ethcall(name = "shutdown", abi = "shutdown()")]
    pub struct ShutdownCall;
    ///Container type for all input parameters for the `shutdownFactory` function with signature `shutdownFactory()` and selector `0xdb8a5ac8`
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
    #[ethcall(name = "shutdownFactory", abi = "shutdownFactory()")]
    pub struct ShutdownFactoryCall;
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `vaultImplementation` function with signature `vaultImplementation()` and selector `0xbba48a90`
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
    #[ethcall(name = "vaultImplementation", abi = "vaultImplementation()")]
    pub struct VaultImplementationCall;
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
    pub enum VaultFactoryCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        VaultDeployerRole(VaultDeployerRoleCall),
        DeployNewVault(DeployNewVaultCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetRoleMember(GetRoleMemberCall),
        GetRoleMemberCount(GetRoleMemberCountCall),
        GetVaultFromUnderlying(GetVaultFromUnderlyingCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IsVaultDeployed(IsVaultDeployedCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        Shutdown(ShutdownCall),
        ShutdownFactory(ShutdownFactoryCall),
        SupportsInterface(SupportsInterfaceCall),
        VaultImplementation(VaultImplementationCall),
    }
    impl ::ethers::core::abi::AbiDecode for VaultFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <VaultDeployerRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VaultDeployerRole(decoded));
            }
            if let Ok(decoded) = <DeployNewVaultCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployNewVault(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMember(decoded));
            }
            if let Ok(decoded) = <GetRoleMemberCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleMemberCount(decoded));
            }
            if let Ok(decoded) = <GetVaultFromUnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetVaultFromUnderlying(decoded));
            }
            if let Ok(decoded) = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded) = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded) = <IsVaultDeployedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsVaultDeployed(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <ShutdownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Shutdown(decoded));
            }
            if let Ok(decoded) = <ShutdownFactoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ShutdownFactory(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <VaultImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VaultImplementation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VaultFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VaultDeployerRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployNewVault(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMember(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleMemberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVaultFromUnderlying(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsVaultDeployed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Shutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShutdownFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VaultImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for VaultFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::VaultDeployerRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployNewVault(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoleMemberCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVaultFromUnderlying(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsVaultDeployed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Shutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShutdownFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::VaultImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for VaultFactoryCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<VaultDeployerRoleCall> for VaultFactoryCalls {
        fn from(value: VaultDeployerRoleCall) -> Self {
            Self::VaultDeployerRole(value)
        }
    }
    impl ::core::convert::From<DeployNewVaultCall> for VaultFactoryCalls {
        fn from(value: DeployNewVaultCall) -> Self {
            Self::DeployNewVault(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for VaultFactoryCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCall> for VaultFactoryCalls {
        fn from(value: GetRoleMemberCall) -> Self {
            Self::GetRoleMember(value)
        }
    }
    impl ::core::convert::From<GetRoleMemberCountCall> for VaultFactoryCalls {
        fn from(value: GetRoleMemberCountCall) -> Self {
            Self::GetRoleMemberCount(value)
        }
    }
    impl ::core::convert::From<GetVaultFromUnderlyingCall> for VaultFactoryCalls {
        fn from(value: GetVaultFromUnderlyingCall) -> Self {
            Self::GetVaultFromUnderlying(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for VaultFactoryCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for VaultFactoryCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IsVaultDeployedCall> for VaultFactoryCalls {
        fn from(value: IsVaultDeployedCall) -> Self {
            Self::IsVaultDeployed(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for VaultFactoryCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for VaultFactoryCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<ShutdownCall> for VaultFactoryCalls {
        fn from(value: ShutdownCall) -> Self {
            Self::Shutdown(value)
        }
    }
    impl ::core::convert::From<ShutdownFactoryCall> for VaultFactoryCalls {
        fn from(value: ShutdownFactoryCall) -> Self {
            Self::ShutdownFactory(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for VaultFactoryCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<VaultImplementationCall> for VaultFactoryCalls {
        fn from(value: VaultImplementationCall) -> Self {
            Self::VaultImplementation(value)
        }
    }
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `VAULT_DEPLOYER_ROLE` function with signature `VAULT_DEPLOYER_ROLE()` and selector `0x552de38c`
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
    pub struct VaultDeployerRoleReturn(pub [u8; 32]);
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
    pub struct DeployNewVaultReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getRoleMember` function with signature `getRoleMember(bytes32,uint256)` and selector `0x9010d07c`
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
    pub struct GetRoleMemberReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRoleMemberCount` function with signature `getRoleMemberCount(bytes32)` and selector `0xca15c873`
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
    pub struct GetRoleMemberCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getVaultFromUnderlying` function with signature `getVaultFromUnderlying(address,string,string)` and selector `0xf3a6955c`
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
    pub struct GetVaultFromUnderlyingReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `isVaultDeployed` function with signature `isVaultDeployed(address)` and selector `0x7d6020f4`
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
    pub struct IsVaultDeployedReturn(pub bool);
    ///Container type for all return fields from the `shutdown` function with signature `shutdown()` and selector `0xfc0e74d1`
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
    pub struct ShutdownReturn(pub bool);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `vaultImplementation` function with signature `vaultImplementation()` and selector `0xbba48a90`
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
    pub struct VaultImplementationReturn(pub ::ethers::core::types::Address);
}
