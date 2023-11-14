pub use aave_v3_strategy::*;
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
pub mod aave_v3_strategy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_asset"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IERC20"),
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
                        name: ::std::borrow::ToOwned::to_owned("_lendingPool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IPool"),
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
                    ::std::borrow::ToOwned::to_owned("aToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("aToken"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("asset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("asset"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("convertToAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("convertToAssets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("convertToShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("convertToShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subtractedValue"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
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
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fullProfitUnlockDate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "fullProfitUnlockDate",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addedValue"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastReport"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastReport"),
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
                    ::std::borrow::ToOwned::to_owned("lendingPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lendingPool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxDeposit"),
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
                    ::std::borrow::ToOwned::to_owned("maxMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxMint"),
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
                    ::std::borrow::ToOwned::to_owned("maxRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxRedeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("maxWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxWithdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("previewDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewDeposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("previewMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewMint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("previewRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewRedeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("previewWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewWithdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("pricePerShare"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pricePerShare"),
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
                    ::std::borrow::ToOwned::to_owned("profitMaxUnlockTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "profitMaxUnlockTime",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("profitUnlockingRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "profitUnlockingRate",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("redeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxLoss"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("report"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("report"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("profit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("loss"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("setKeeper"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setKeeper"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_keeper"),
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
                    ::std::borrow::ToOwned::to_owned("setManagement"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setManagement"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_management"),
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
                    ::std::borrow::ToOwned::to_owned("setProfitMaxUnlockTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setProfitMaxUnlockTime",
                            ),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                    ::std::borrow::ToOwned::to_owned("totalAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalAssets"),
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
                    ::std::borrow::ToOwned::to_owned("totalDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalDebt"),
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
                    ::std::borrow::ToOwned::to_owned("totalIdle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalIdle"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxLoss"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Reported"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Reported"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("profit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("loss"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
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
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UpdateProfitMaxUnlockTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UpdateProfitMaxUnlockTime",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newProfitMaxUnlockTime",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
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
    pub static AAVEV3STRATEGY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0:\xFF8\x03\x80b\0:\xFF\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x08eV[`\x01\x80U\x83\x83\x83\x82\x82\x82`\x05b\0\0M\x83\x82b\0\t\x87V[P`\x06b\0\0\\\x82\x82b\0\t\x87V[PPP`\0\x80b\0\0s\x83b\0\x01\xF7` \x1B` \x1CV[\x91P\x91P\x81b\0\0\x85W`\x12b\0\0\x87V[\x80[`\xFF\x16`\xA0RPP`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\0\xA8`\x003b\0\x02\xDCV[b\0\0\xD4\x7F\xB4\xD5\xC1a\x80\xAA\x1FB\xA7\x9CC\x01\xE7\xBDt\xB2J\xA5VV<f-\xB1\tW\xE0\xE1\x95\xB5\xBER3b\0\x02\xDCV[b\0\x01\0\x7Fq\xA9\x85\x9D}\xD2\x1B$PJo0`w\xFF\xC2\xD5\x10\xB4\xD4\xB6\x11(\xE91\xFE\x93tA\xAD\x1863b\0\x02\xDCV[PPb\t:\x80`\x0CUPB`\x0BU`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\xC0\x81\x90R`@Qc5\xEAju`\xE0\x1B\x81R\x91\x86\x16`\x04\x83\x01R\x90c5\xEAju\x90`$\x01a\x01\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\x82\x91\x90b\0\n\xEBV[a\x01\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x81\x90Rb\0\x01\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x07`$\x82\x01Rf\x10\xB0\xAA7\xB5\xB2\xB7`\xC9\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\xC0Qb\0\x01\xED\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90`\0\x19b\0\x02\xECV[PPPPb\0\x0C\xC7V[`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1<\xE5g`\xE0\x1B\x17\x90R\x90Q`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x87\x16\x91b\0\x02@\x91b\0\x0C/V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14b\0\x02}W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x02\x82V[``\x91P[P\x91P\x91P\x81\x80\x15b\0\x02\x97WP` \x81Q\x10\x15[\x15b\0\x02\xCFW`\0\x81\x80` \x01\x90Q\x81\x01\x90b\0\x02\xB5\x91\x90b\0\x0CMV[\x90P`\xFF\x81\x11b\0\x02\xCDW`\x01\x96\x90\x95P\x93PPPPV[P[P`\0\x94\x85\x94P\x92PPPV[b\0\x02\xE8\x82\x82b\0\x04;V[PPV[\x80\x15\x80b\0\x03jWP`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x03BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x03h\x91\x90b\0\x0CMV[\x15[b\0\x03\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FSafeERC20: approve from non-zero`D\x82\x01R\x7F to non-zero allowance\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01b\0\x01\xC8V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x90\x81\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90\x91Rb\0\x046\x91\x85\x91b\0\x04\xDB\x16V[PPPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\x02\xE8W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x04\x973\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`@\x80Q\x80\x82\x01\x90\x91R` \x80\x82R\x7FSafeERC20: low-level call failed\x90\x82\x01R`\0\x90b\0\x05*\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x84\x90b\0\x05\xAFV[\x90P\x80Q`\0\x14\x80b\0\x05NWP\x80\x80` \x01\x90Q\x81\x01\x90b\0\x05N\x91\x90b\0\x0CgV[b\0\x046W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01b\0\x01\xC8V[``b\0\x05\xC0\x84\x84`\0\x85b\0\x05\xC8V[\x94\x93PPPPV[``\x82G\x10\x15b\0\x06+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01b\0\x01\xC8V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qb\0\x06I\x91\x90b\0\x0C/V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14b\0\x06\x88W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x06\x8DV[``\x91P[P\x90\x92P\x90Pb\0\x06\xA1\x87\x83\x83\x87b\0\x06\xACV[\x97\x96PPPPPPPV[``\x83\x15b\0\x07 W\x82Q`\0\x03b\0\x07\x18W`\x01`\x01`\xA0\x1B\x03\x85\x16;b\0\x07\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01b\0\x01\xC8V[P\x81b\0\x05\xC0V[b\0\x05\xC0\x83\x83\x81Q\x15b\0\x077W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x01\xC8\x91\x90b\0\x0C\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x07iW`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x07\xA8Wb\0\x07\xA8b\0\x07lV[`@R\x90V[`\0[\x83\x81\x10\x15b\0\x07\xCBW\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x07\xB1V[PP`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12b\0\x07\xE6W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x08\x03Wb\0\x08\x03b\0\x07lV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x08.Wb\0\x08.b\0\x07lV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x08HW`\0\x80\xFD[b\0\x08[\x84` \x83\x01` \x89\x01b\0\x07\xAEV[\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x08|W`\0\x80\xFD[\x84Qb\0\x08\x89\x81b\0\x07SV[` \x86\x01Q\x90\x94P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x08\xA7W`\0\x80\xFD[b\0\x08\xB5\x88\x83\x89\x01b\0\x07\xD4V[\x94P`@\x87\x01Q\x91P\x80\x82\x11\x15b\0\x08\xCCW`\0\x80\xFD[Pb\0\x08\xDB\x87\x82\x88\x01b\0\x07\xD4V[\x92PP``\x85\x01Qb\0\x08\xEE\x81b\0\x07SV[\x93\x96\x92\x95P\x90\x93PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\t\x0EW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\t/WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x046W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\t^WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\t\x7FW\x82\x81U`\x01\x01b\0\tjV[PPPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\t\xA3Wb\0\t\xA3b\0\x07lV[b\0\t\xBB\x81b\0\t\xB4\x84Tb\0\x08\xF9V[\x84b\0\t5V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\t\xF3W`\0\x84\x15b\0\t\xDAWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\t\x7FV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\n$W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\n\x03V[P\x85\x82\x10\x15b\0\nCW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15b\0\nfW`\0\x80\xFD[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\n\x8BWb\0\n\x8Bb\0\x07lV[`@R\x91Q\x82RP\x91\x90PV[\x80Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14b\0\n\xB0W`\0\x80\xFD[\x91\x90PV[\x80Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\n\xB0W`\0\x80\xFD[\x80Qa\xFF\xFF\x81\x16\x81\x14b\0\n\xB0W`\0\x80\xFD[\x80Qb\0\n\xB0\x81b\0\x07SV[`\0a\x01\xE0\x82\x84\x03\x12\x15b\0\n\xFFW`\0\x80\xFD[b\0\x0B\tb\0\x07\x82V[b\0\x0B\x15\x84\x84b\0\nSV[\x81Rb\0\x0B%` \x84\x01b\0\n\x98V[` \x82\x01Rb\0\x0B8`@\x84\x01b\0\n\x98V[`@\x82\x01Rb\0\x0BK``\x84\x01b\0\n\x98V[``\x82\x01Rb\0\x0B^`\x80\x84\x01b\0\n\x98V[`\x80\x82\x01Rb\0\x0Bq`\xA0\x84\x01b\0\n\x98V[`\xA0\x82\x01Rb\0\x0B\x84`\xC0\x84\x01b\0\n\xB5V[`\xC0\x82\x01Rb\0\x0B\x97`\xE0\x84\x01b\0\n\xCBV[`\xE0\x82\x01Ra\x01\0b\0\x0B\xAC\x81\x85\x01b\0\n\xDEV[\x90\x82\x01Ra\x01 b\0\x0B\xC0\x84\x82\x01b\0\n\xDEV[\x90\x82\x01Ra\x01@b\0\x0B\xD4\x84\x82\x01b\0\n\xDEV[\x90\x82\x01Ra\x01`b\0\x0B\xE8\x84\x82\x01b\0\n\xDEV[\x90\x82\x01Ra\x01\x80b\0\x0B\xFC\x84\x82\x01b\0\n\x98V[\x90\x82\x01Ra\x01\xA0b\0\x0C\x10\x84\x82\x01b\0\n\x98V[\x90\x82\x01Ra\x01\xC0b\0\x0C$\x84\x82\x01b\0\n\x98V[\x90\x82\x01R\x93\x92PPPV[`\0\x82Qb\0\x0CC\x81\x84` \x87\x01b\0\x07\xAEV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15b\0\x0C`W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\0\x0CzW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x0C\x8BW`\0\x80\xFD[\x93\x92PPPV[` \x81R`\0\x82Q\x80` \x84\x01Rb\0\x0C\xB3\x81`@\x85\x01` \x87\x01b\0\x07\xAEV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa-\x95b\0\rj`\09`\0\x81\x81a\x05\x11\x01R\x81\x81a\x17\x1A\x01Ra\"j\x01R`\0\x81\x81a\x05f\x01R\x81\x81a\x16>\x01R\x81\x81a!K\x01Ra\"\x0B\x01R`\0a\n\xA4\x01R`\0\x81\x81a\x04\x18\x01R\x81\x81a\t}\x01R\x81\x81a\x15p\x01R\x81\x81a\x1A\xDC\x01R\x81\x81a\x1B\x19\x01R\x81\x81a\x1B\xB4\x01R\x81\x81a\x1D}\x01R\x81\x81a\x1E\"\x01R\x81\x81a\x1F\x8C\x01R\x81\x81a!r\x01Ra\"2\x01Ra-\x95`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\xD6W`\x005`\xE0\x1C\x80c\x95\xD8\x9BA\x11a\x01\x82W\x80c\xBA\x08vR\x11a\0\xE9W\x80c\xD5Gt\x1F\x11a\0\xA2W\x80c\xDFi\xB2*\x11a\0|W\x80c\xDFi\xB2*\x14a\x06OW\x80c\xEF\x8B0\xF7\x14a\x05\xDDW\x80c\xFC\x0Et\xD1\x14a\x06bW\x80c\xFC{\x9C\x18\x14a\x06oW`\0\x80\xFD[\x80c\xD5Gt\x1F\x14a\x06\x16W\x80c\xD9\x05w~\x14a\x06)W\x80c\xDDb\xED>\x14a\x06<W`\0\x80\xFD[\x80c\xBA\x08vR\x14a\x05\xC1W\x80c\xC3S[R\x14a\x05\xD4W\x80c\xC6=u\xB6\x14a\x04cW\x80c\xC6\xE6\xF5\x92\x14a\x05\xDDW\x80c\xCE\x96\xCBw\x14a\x05\xF0W\x80c\xD4\xA2+\xDE\x14a\x06\x03W`\0\x80\xFD[\x80c\xA3\x18\xC1\xA4\x11a\x01;W\x80c\xA3\x18\xC1\xA4\x14a\x05;W\x80c\xA4W\xC2\xD7\x14a\x05NW\x80c\xA5\x9A\x99s\x14a\x05aW\x80c\xA9\x05\x9C\xBB\x14a\x05\x88W\x80c\xB3\xD7\xF6\xB9\x14a\x05\x9BW\x80c\xB4`\xAF\x94\x14a\x05\xAEW`\0\x80\xFD[\x80c\x95\xD8\x9BA\x14a\x04\xE0W\x80c\x99S\x0B\x06\x14a\x04\xE8W\x80c\x9A\xA7\xDF\x94\x14a\x04\xF0W\x80c\x9F@\xA7\xB3\x14a\x04\xF9W\x80c\xA0\xC1\xF1^\x14a\x05\x0CW\x80c\xA2\x17\xFD\xDF\x14a\x053W`\0\x80\xFD[\x80c//\xF1]\x11a\x02AW\x80cL\xDA\xD5\x06\x11a\x01\xFAW\x80cp\xA0\x821\x11a\x01\xD4W\x80cp\xA0\x821\x14a\x04\x94W\x80ct\x87G\xE6\x14a\x04\xA7W\x80c\x91\xD1HT\x14a\x04\xBAW\x80c\x94\xBF\x80M\x14a\x04\xCDW`\0\x80\xFD[\x80cL\xDA\xD5\x06\x14a\x03.W\x80cQA\xEE\xBB\x14a\x04xW\x80cnU?e\x14a\x04\x81W`\0\x80\xFD[\x80c//\xF1]\x14a\x03\xD4W\x80c1<\xE5g\x14a\x03\xE9W\x80c6V\x8A\xBE\x14a\x04\x03W\x80c8\xD5.\x0F\x14a\x04\x16W\x80c9P\x93Q\x14a\x04PW\x80c@-&}\x14a\x04cW`\0\x80\xFD[\x80c\n(\xA4w\x11a\x02\x93W\x80c\n(\xA4w\x14a\x03]W\x80c\x18\x16\r\xDD\x14a\x03pW\x80c#\xB8r\xDD\x14a\x03xW\x80c$\x8A\x9C\xA3\x14a\x03\x8BW\x80c&\x06\xA1\x0B\x14a\x03\xAEW\x80c-c&\x92\x14a\x03\xCBW`\0\x80\xFD[\x80c\x01\xE1\xD1\x14\x14a\x02\xDBW\x80c\x01\xFF\xC9\xA7\x14a\x02\xF6W\x80c\x06\xFD\xDE\x03\x14a\x03\x19W\x80c\x07\xA2\xD1:\x14a\x03.W\x80c\tR\x86N\x14a\x03AW\x80c\t^\xA7\xB3\x14a\x03JW[`\0\x80\xFD[a\x02\xE3a\x06xV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\ta\x03\x046`\x04a(\x18V[a\x06\x8FV[`@Q\x90\x15\x15\x81R` \x01a\x02\xEDV[a\x03!a\x06\xC6V[`@Qa\x02\xED\x91\x90a(fV[a\x02\xE3a\x03<6`\x04a(\x99V[a\x07XV[a\x02\xE3`\x0CT\x81V[a\x03\ta\x03X6`\x04a(\xC9V[a\x07eV[a\x02\xE3a\x03k6`\x04a(\x99V[a\x07}V[a\x02\xE3a\x07\x8AV[a\x03\ta\x03\x866`\x04a(\xF3V[a\x07\xA1V[a\x02\xE3a\x03\x996`\x04a(\x99V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[a\x03\xB6a\x07\xC7V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xEDV[a\x02\xE3`\nT\x81V[a\x03\xE7a\x03\xE26`\x04a)/V[a\nrV[\0[a\x03\xF1a\n\x9CV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\xEDV[a\x03\xE7a\x04\x116`\x04a)/V[a\n\xC8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xEDV[a\x03\ta\x04^6`\x04a(\xC9V[a\x0BFV[a\x02\xE3a\x04q6`\x04a)[V[P`\0\x19\x90V[a\x02\xE3`\tT\x81V[a\x02\xE3a\x04\x8F6`\x04a)/V[a\x0BhV[a\x02\xE3a\x04\xA26`\x04a)[V[a\x0B\x82V[a\x03\xE7a\x04\xB56`\x04a)[V[a\x0B\xDDV[a\x03\ta\x04\xC86`\x04a)/V[a\x0C>V[a\x02\xE3a\x04\xDB6`\x04a)/V[a\x0CgV[a\x03!a\x0C\x81V[a\x02\xE3a\x0C\x90V[a\x02\xE3`\x07T\x81V[a\x02\xE3a\x05\x076`\x04a)vV[a\x0C\xA8V[a\x048\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xE3`\0\x81V[a\x02\xE3a\x05I6`\x04a)vV[a\r)V[a\x03\ta\x05\\6`\x04a(\xC9V[a\r\x9EV[a\x048\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\ta\x05\x966`\x04a(\xC9V[a\x0E$V[a\x02\xE3a\x05\xA96`\x04a(\x99V[a\x0E2V[a\x02\xE3a\x05\xBC6`\x04a)\xBAV[a\x0E?V[a\x02\xE3a\x05\xCF6`\x04a)\xBAV[a\x0E\xBEV[a\x02\xE3`\x0BT\x81V[a\x02\xE3a\x05\xEB6`\x04a(\x99V[a\x0F5V[a\x02\xE3a\x05\xFE6`\x04a)[V[a\x0FBV[a\x03\xE7a\x06\x116`\x04a)[V[a\x0FWV[a\x03\xE7a\x06$6`\x04a)/V[a\x0F\xA3V[a\x02\xE3a\x0676`\x04a)[V[a\x0F\xC8V[a\x02\xE3a\x06J6`\x04a)\xF6V[a\x0F\xD3V[a\x03\xE7a\x06]6`\x04a(\x99V[a\x0F\xFEV[`\rTa\x03\t\x90`\xFF\x16\x81V[a\x02\xE3`\x08T\x81V[`\0`\x08T`\x07Ta\x06\x8A\x91\x90a*6V[\x90P\x90V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06\xC0WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[```\x05\x80Ta\x06\xD5\x90a*IV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\x01\x90a*IV[\x80\x15a\x07NW\x80`\x1F\x10a\x07#Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07NV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x071W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x06\xC0\x82`\0a\x10\xE8V[`\x003a\x07s\x81\x85\x85a\x11\x1CV[P`\x01\x93\x92PPPV[`\0a\x06\xC0\x82`\x01a\x12@V[`\0a\x07\x94a\x12\x8EV[`\x04Ta\x06\x8A\x91\x90a*\x83V[`\x003a\x07\xAF\x85\x82\x85a\x12\xEAV[a\x07\xBA\x85\x85\x85a\x13dV[`\x01\x91PP[\x93\x92PPPV[`\0\x80a\x07\xF4\x7Fq\xA9\x85\x9D}\xD2\x1B$PJo0`w\xFF\xC2\xD5\x10\xB4\xD4\xB6\x11(\xE91\xFE\x93tA\xAD\x1863a\x0C>V[a\x083W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj7\xB76<\x905\xB2\xB2\xB82\xB9`\xA9\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x08;a\x15\x0FV[`\0a\x08Ea\x06xV[\x90P`\0a\x08Qa\x15hV[\x90Pa\x08[a\x17\x9DV[`\0\x82\x82\x11\x15a\x08\x8BWa\x08o\x83\x83a*\x83V[\x94Pa\x08z\x85a\x0F5V[\x90Pa\x08\x860\x82a\x17\xD1V[a\x08\xCDV[a\x08\x95\x82\x84a*\x83V[\x93P\x83\x15a\x08\xCDW`\0a\x08\xB9a\x08\xAB\x86a\x0F5V[a\x08\xB40a\x0B\x82V[a\x18\x92V[\x90P\x80\x15a\x08\xCBWa\x08\xCB0\x82a\x18\xA8V[P[`\0a\x08\xD80a\x0B\x82V[\x90P\x80\x15a\tsW`\0B`\nT\x11\x15a\t\x11Wa\x08\xF6\x83\x83a*\x83V[B`\nTa\t\x04\x91\x90a*\x83V[a\t\x0E\x91\x90a*\x96V[\x90P[`\0\x82`\x0CT\x85a\t\"\x91\x90a*\x96V[a\t,\x90\x84a*6V[a\t6\x91\x90a*\xC3V[\x90P\x80a\tHd\xE8\xD4\xA5\x10\0\x85a*\x96V[a\tR\x91\x90a*\xC3V[`\tUa\t_\x81Ba*6V[`\x01`\x01`\x80\x1B\x03\x16`\nUPa\ty\x90PV[`\0`\tU[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x06\x91\x90a*\xE5V[`\x07\x81\x90U\x90Pa\n\x17\x81\x85a*\x83V[`\x08UB`\x01`\x01`\x80\x1B\x03\x16`\x0BU`@\x80Q\x88\x81R` \x81\x01\x88\x90R\x7Fo\xEA6\xBF\xB4\xB7\xC9\x12a7V\x95\x1F\xB5\xBF\x8D\xDA\xED\xB7M\xE1\xF2V\xD0\xBE\xF8\x153\x15\x8C\xDAT\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPa\nn`\x01\x80UV[\x90\x91V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\n\x8D\x81a\x19\xDCV[a\n\x97\x83\x83a\x19\xE6V[PPPV[`\0a\x06\x8A\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a*\xFEV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0B8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x08*V[a\x0BB\x82\x82a\x1AjV[PPV[`\x003a\x07s\x81\x85\x85a\x0BY\x83\x83a\x0F\xD3V[a\x0Bc\x91\x90a*6V[a\x11\x1CV[`\0\x80a\x0Bt\x84a\x0F5V[\x90Pa\x07\xC03\x84\x86\x84a\x1A\xCFV[`\x000`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\x0B\xBFWa\x0B\x9Ca\x12\x8EV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90 Ta\x06\xC0\x91\x90a*\x83V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02` R`@\x90 Ta\x06\xC0V[a\x0B\xF5`\0\x80Q` a-@\x839\x81Q\x91R3a\x0C>V[a\x0C\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08*\x90a+\x17V[a\x0C;\x7Fq\xA9\x85\x9D}\xD2\x1B$PJo0`w\xFF\xC2\xD5\x10\xB4\xD4\xB6\x11(\xE91\xFE\x93tA\xAD\x186\x82a\x1C\xE0V[PV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x80a\x0Cs\x84a\x0E2V[\x90Pa\x07\xC03\x84\x83\x87a\x1A\xCFV[```\x06\x80Ta\x06\xD5\x90a*IV[`\0a\x06\x8Aa\x0C\x9Da\n\x9CV[a\x03<\x90`\na,$V[`\0a\x0C\xB3\x83a\x0F\xC8V[\x85\x11\x15a\r\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC4626: redeem more than max\0\0\0`D\x82\x01R`d\x01a\x08*V[`\0a\r\r\x86a\x07XV[\x90Pa\r\x1D3\x86\x86\x84\x8A\x88a\x1C\xEAV[P\x90P[\x94\x93PPPPV[`\0a\r4\x83a\x0FBV[\x85\x11\x15a\r\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC4626: withdraw more than max\0`D\x82\x01R`d\x01a\x08*V[`\0a\r\x8E\x86a\x07}V[\x90Pa\r\x1D3\x86\x86\x89\x85\x88a\x1C\xEAV[`\x003\x81a\r\xAC\x82\x86a\x0F\xD3V[\x90P\x83\x81\x10\x15a\x0E\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x08*V[a\x0E\x19\x82\x86\x86\x84\x03a\x11\x1CV[P`\x01\x94\x93PPPPV[`\x003a\x07s\x81\x85\x85a\x13dV[`\0a\x06\xC0\x82`\x01a\x10\xE8V[`\0a\x0EJ\x82a\x0FBV[\x84\x11\x15a\x0E\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC4626: withdraw more than max\0`D\x82\x01R`d\x01a\x08*V[`\0a\x0E\xA4\x85a\x07}V[\x90Pa\x0E\xB53\x85\x85\x88\x85`\0a\x1C\xEAV[P\x94\x93PPPPV[`\0a\x0E\xC9\x82a\x0F\xC8V[\x84\x11\x15a\x0F\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC4626: redeem more than max\0\0\0`D\x82\x01R`d\x01a\x08*V[`\0a\x0F#\x85a\x07XV[\x90Pa\x0E\xB53\x85\x85\x84\x89a'\x10a\x1C\xEAV[`\0a\x06\xC0\x82`\0a\x12@V[`\0a\x06\xC0a\x0FP\x83a\x0B\x82V[`\0a\x10\xE8V[a\x0Fo`\0\x80Q` a-@\x839\x81Q\x91R3a\x0C>V[a\x0F\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08*\x90a+\x17V[a\x0C;`\0\x80Q` a-@\x839\x81Q\x91R\x82a\x1C\xE0V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0F\xBE\x81a\x19\xDCV[a\n\x97\x83\x83a\x1AjV[`\0a\x06\xC0\x82a\x0B\x82V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x10\x16`\0\x80Q` a-@\x839\x81Q\x91R3a\x0C>V[a\x102W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08*\x90a+\x17V[\x80`\0\x03a\x10nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`\xBA\x1B`D\x82\x01R`d\x01a\x08*V[c\x01\xE1\x85X\x81\x11\x15a\x10\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01Rgtoo long`\xC0\x1B`D\x82\x01R`d\x01a\x08*V[`\x0C\x81\x90U`@Q\x81\x81R\x7F\xF3a\xAE\xD4c\xDAo\xA2\x03X\xE4\\b\t\xF1\xD3\xE1mN\xCApnn\xAB\x0B\n\xEB3\x87)\xC7z\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80a\x10\xF3a\x07\x8AV[\x90P\x80\x15a\x11\x14Wa\x11\x0Fa\x11\x06a\x06xV[\x85\x90\x83\x86a \"V[a\r!V[P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x11~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x11\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0\x80a\x12Ka\x06xV[\x90P`\0a\x12Wa\x07\x8AV[\x90P\x81`\0\x03a\x12yW\x80\x15a\x12nW`\0a\x12pV[\x84[\x92PPPa\x06\xC0V[a\x12\x85\x85\x82\x84\x87a \"V[\x95\x94PPPPPV[`\0\x80B`\nT\x11\x15a\x12\xCBW`\0`\x0BTBa\x12\xAB\x91\x90a*\x83V[\x90Pd\xE8\xD4\xA5\x10\0`\tT\x82a\x12\xC1\x91\x90a*\x96V[a\x07\xC0\x91\x90a*\xC3V[`\nT\x15a\x12\xE5WP0`\0\x90\x81R`\x02` R`@\x90 T[\x91\x90PV[`\0a\x12\xF6\x84\x84a\x0F\xD3V[\x90P`\0\x19\x81\x14a\x13^W\x81\x81\x10\x15a\x13QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x08*V[a\x13^\x84\x84\x84\x84\x03a\x11\x1CV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x13\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a\x14\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x15\x02\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x13^V[`\x02`\x01T\x03a\x15aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x08*V[`\x02`\x01UV[`\rT`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\xFF\x16\x15a\x16\x9DW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x03\x91\x90a*\xE5V[\x90P\x80\x15a\x16\x9BW`@Qca{\xA07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R0`D\x83\x01R`\0`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ca{\xA07\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\x96W=`\0\x80>=`\0\xFD[PPPP[P[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x05\x91\x90a*\xE5V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x8D\x91\x90a*\xE5V[a\x17\x97\x91\x90a*6V[\x91PP\x90V[`\0a\x17\xA7a\x12\x8EV[\x90P\x80\x15a\x0C;WB`\nT\x11\x15a\x17\xC7W`\x01`\x01`\x80\x1B\x03B\x16`\x0BU[a\x0C;0\x82a\x18\xA8V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x18'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x08*V[\x80`\x04`\0\x82\x82Ta\x189\x91\x90a*6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\0\x81\x83\x10a\x18\xA1W\x81a\x07\xC0V[P\x90\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x19\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a\x19|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`\x04\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x0C;\x813a }V[a\x19\xF0\x82\x82a\x0C>V[a\x0BBW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1A&3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x1At\x82\x82a\x0C>V[\x15a\x0BBW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[a\x1A\xD7a\x15\x0FV[a\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x850\x85a \xD6V[`\0\x82`\x07Ta\x1B\x13\x91\x90a*6V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xA2\x91\x90a*\xE5V[\x90Pa\x1B\xAD\x82a!AV[`\0a\x1CM\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C=\x91\x90a*\xE5V[a\x1CG\x90\x84a*\x83V[\x84a\x18\x92V[\x90P\x80`\x08`\0\x82\x82Ta\x1Ca\x91\x90a*6V[\x90\x91UPa\x1Cq\x90P\x81\x84a*\x83V[`\x07Ua\x1C~\x86\x85a\x17\xD1V[\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x87\x87`@Qa\x1C\xCC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPa\x13^`\x01\x80UV[a\x0BB\x82\x82a\x19\xE6V[`\0a\x1C\xF4a\x15\x0FV[`\x01`\x01`\xA0\x1B\x03\x86\x16a\x1DJW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReceiver cannot be zero address\0`D\x82\x01R`d\x01a\x08*V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1DnWa\x1Dn\x85\x88\x85a\x12\xEAV[`\x07T\x84\x81\x10\x15a\x1FpW`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x06\x91\x90a*\xE5V[\x90Pa\x1E\x1Aa\x1E\x15\x83\x88a*\x83V[a\"\x01V[`\0a\x1E\xBD\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xAB\x91\x90a*\xE5V[a\x1E\xB5\x91\x90a*\x83V[`\x08Ta\x18\x92V[\x90Pa\x1E\xC9\x81\x84a*6V[\x92P`\0\x87\x84\x10\x15a\x1FLWa\x1E\xDF\x84\x89a*\x83V[\x90Pa'\x10\x86\x10\x15a\x1FHWa'\x10a\x1E\xF8\x87\x8Aa*\x96V[a\x1F\x02\x91\x90a*\xC3V[\x81\x11\x15a\x1FHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsLoss exceeds maxLoss``\x1B`D\x82\x01R`d\x01a\x08*V[\x83\x97P[a\x1FV\x81\x83a*6V[`\x08`\0\x82\x82Ta\x1Fg\x91\x90a*\x83V[\x90\x91UPPPPP[a\x1Fz\x85\x82a*\x83V[`\x07Ua\x1F\x87\x86\x85a\x18\xA8V[a\x1F\xB2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x87a#XV[\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x88\x88`@Qa \n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UP\x90\x94\x93PPPPV[`\0\x80a 0\x86\x86\x86a#\x88V[\x90P`\x01\x83`\x02\x81\x11\x15a FWa Fa,3V[\x14\x80\x15a cWP`\0\x84\x80a ^Wa ^a*\xADV[\x86\x88\t\x11[\x15a\x12\x85Wa s`\x01\x82a*6V[\x96\x95PPPPPPV[a \x87\x82\x82a\x0C>V[a\x0BBWa \x94\x81a$rV[a \x9F\x83` a$\x84V[`@Q` \x01a \xB0\x92\x91\x90a,IV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x08*\x91`\x04\x01a(fV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x13^\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra& V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16ca{\xA07\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R0`D\x82\x01R`\0`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xE6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\xFAW=`\0\x80>=`\0\xFD[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16ci2\x8D\xEC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\"\xE3\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xB9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xDD\x91\x90a*\xE5V[\x85a\x18\x92V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R0`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a#4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BB\x91\x90a*\xE5V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\n\x97\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a!\nV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a#\xC2W\x83\x82\x81a#\xB8Wa#\xB8a*\xADV[\x04\x92PPPa\x07\xC0V[\x80\x84\x11a$\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\x08*V[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[``a\x06\xC0`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a$\x93\x83`\x02a*\x96V[a$\x9E\x90`\x02a*6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xB6Wa$\xB6a,\xBEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a$\xE0W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a$\xFBWa$\xFBa,\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a%*Wa%*a,\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a%N\x84`\x02a*\x96V[a%Y\x90`\x01a*6V[\x90P[`\x01\x81\x11\x15a%\xD1Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a%\x8DWa%\x8Da,\xD4V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a%\xA3Wa%\xA3a,\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a%\xCA\x81a,\xEAV[\x90Pa%\\V[P\x83\x15a\x07\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x08*V[`\0a&u\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a&\xF5\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a&\x96WP\x80\x80` \x01\x90Q\x81\x01\x90a&\x96\x91\x90a-\x01V[a\n\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x08*V[``a\r!\x84\x84`\0\x85\x85`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa'\x1C\x91\x90a-#V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a'YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'^V[``\x91P[P\x91P\x91Pa'o\x87\x83\x83\x87a'zV[\x97\x96PPPPPPPV[``\x83\x15a'\xE9W\x82Q`\0\x03a'\xE2W`\x01`\x01`\xA0\x1B\x03\x85\x16;a'\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x08*V[P\x81a\r!V[a\r!\x83\x83\x81Q\x15a'\xFEW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08*\x91\x90a(fV[`\0` \x82\x84\x03\x12\x15a(*W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\xC0W`\0\x80\xFD[`\0[\x83\x81\x10\x15a(]W\x81\x81\x01Q\x83\x82\x01R` \x01a(EV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra(\x85\x81`@\x85\x01` \x87\x01a(BV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a(\xABW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12\xE5W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a(\xDCW`\0\x80\xFD[a(\xE5\x83a(\xB2V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a)\x08W`\0\x80\xFD[a)\x11\x84a(\xB2V[\x92Pa)\x1F` \x85\x01a(\xB2V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a)BW`\0\x80\xFD[\x825\x91Pa)R` \x84\x01a(\xB2V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a)mW`\0\x80\xFD[a\x07\xC0\x82a(\xB2V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a)\x8CW`\0\x80\xFD[\x845\x93Pa)\x9C` \x86\x01a(\xB2V[\x92Pa)\xAA`@\x86\x01a(\xB2V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a)\xCFW`\0\x80\xFD[\x835\x92Pa)\xDF` \x85\x01a(\xB2V[\x91Pa)\xED`@\x85\x01a(\xB2V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a*\tW`\0\x80\xFD[a*\x12\x83a(\xB2V[\x91Pa)R` \x84\x01a(\xB2V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xC0Wa\x06\xC0a* V[`\x01\x81\x81\x1C\x90\x82\x16\x80a*]W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a*}WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x06\xC0Wa\x06\xC0a* V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xC0Wa\x06\xC0a* V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a*\xE0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a*\xF7W`\0\x80\xFD[PQ\x91\x90PV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x06\xC0Wa\x06\xC0a* V[` \x80\x82R`\x0F\x90\x82\x01Rn\x1B\xDB\x9B\x1EH\x1BX[\x98Y\xD9[Y[\x9D`\x8A\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81[\x80\x85\x11\x15a+{W\x81`\0\x19\x04\x82\x11\x15a+aWa+aa* V[\x80\x85\x16\x15a+nW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a+EV[P\x92P\x92\x90PV[`\0\x82a+\x92WP`\x01a\x06\xC0V[\x81a+\x9FWP`\0a\x06\xC0V[\x81`\x01\x81\x14a+\xB5W`\x02\x81\x14a+\xBFWa+\xDBV[`\x01\x91PPa\x06\xC0V[`\xFF\x84\x11\x15a+\xD0Wa+\xD0a* V[PP`\x01\x82\x1Ba\x06\xC0V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a+\xFEWP\x81\x81\na\x06\xC0V[a,\x08\x83\x83a+@V[\x80`\0\x19\x04\x82\x11\x15a,\x1CWa,\x1Ca* V[\x02\x93\x92PPPV[`\0a\x07\xC0`\xFF\x84\x16\x83a+\x83V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa,\x81\x81`\x17\x85\x01` \x88\x01a(BV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa,\xB2\x81`(\x84\x01` \x88\x01a(BV[\x01`(\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a,\xF9Wa,\xF9a* V[P`\0\x19\x01\x90V[`\0` \x82\x84\x03\x12\x15a-\x13W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xC0W`\0\x80\xFD[`\0\x82Qa-5\x81\x84` \x87\x01a(BV[\x91\x90\x91\x01\x92\x91PPV\xFE\xB4\xD5\xC1a\x80\xAA\x1FB\xA7\x9CC\x01\xE7\xBDt\xB2J\xA5VV<f-\xB1\tW\xE0\xE1\x95\xB5\xBER\xA2dipfsX\"\x12 \\\xA6\xA3\xAF\x06\xE7O\xAAFz\xC3\xDE\0\x12\xFB\n\xF0\xCA\x93\x1Cn\xDF\xC9\x08\xA8Z-2\x85\x10r\xDDdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static AAVEV3STRATEGY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\xD6W`\x005`\xE0\x1C\x80c\x95\xD8\x9BA\x11a\x01\x82W\x80c\xBA\x08vR\x11a\0\xE9W\x80c\xD5Gt\x1F\x11a\0\xA2W\x80c\xDFi\xB2*\x11a\0|W\x80c\xDFi\xB2*\x14a\x06OW\x80c\xEF\x8B0\xF7\x14a\x05\xDDW\x80c\xFC\x0Et\xD1\x14a\x06bW\x80c\xFC{\x9C\x18\x14a\x06oW`\0\x80\xFD[\x80c\xD5Gt\x1F\x14a\x06\x16W\x80c\xD9\x05w~\x14a\x06)W\x80c\xDDb\xED>\x14a\x06<W`\0\x80\xFD[\x80c\xBA\x08vR\x14a\x05\xC1W\x80c\xC3S[R\x14a\x05\xD4W\x80c\xC6=u\xB6\x14a\x04cW\x80c\xC6\xE6\xF5\x92\x14a\x05\xDDW\x80c\xCE\x96\xCBw\x14a\x05\xF0W\x80c\xD4\xA2+\xDE\x14a\x06\x03W`\0\x80\xFD[\x80c\xA3\x18\xC1\xA4\x11a\x01;W\x80c\xA3\x18\xC1\xA4\x14a\x05;W\x80c\xA4W\xC2\xD7\x14a\x05NW\x80c\xA5\x9A\x99s\x14a\x05aW\x80c\xA9\x05\x9C\xBB\x14a\x05\x88W\x80c\xB3\xD7\xF6\xB9\x14a\x05\x9BW\x80c\xB4`\xAF\x94\x14a\x05\xAEW`\0\x80\xFD[\x80c\x95\xD8\x9BA\x14a\x04\xE0W\x80c\x99S\x0B\x06\x14a\x04\xE8W\x80c\x9A\xA7\xDF\x94\x14a\x04\xF0W\x80c\x9F@\xA7\xB3\x14a\x04\xF9W\x80c\xA0\xC1\xF1^\x14a\x05\x0CW\x80c\xA2\x17\xFD\xDF\x14a\x053W`\0\x80\xFD[\x80c//\xF1]\x11a\x02AW\x80cL\xDA\xD5\x06\x11a\x01\xFAW\x80cp\xA0\x821\x11a\x01\xD4W\x80cp\xA0\x821\x14a\x04\x94W\x80ct\x87G\xE6\x14a\x04\xA7W\x80c\x91\xD1HT\x14a\x04\xBAW\x80c\x94\xBF\x80M\x14a\x04\xCDW`\0\x80\xFD[\x80cL\xDA\xD5\x06\x14a\x03.W\x80cQA\xEE\xBB\x14a\x04xW\x80cnU?e\x14a\x04\x81W`\0\x80\xFD[\x80c//\xF1]\x14a\x03\xD4W\x80c1<\xE5g\x14a\x03\xE9W\x80c6V\x8A\xBE\x14a\x04\x03W\x80c8\xD5.\x0F\x14a\x04\x16W\x80c9P\x93Q\x14a\x04PW\x80c@-&}\x14a\x04cW`\0\x80\xFD[\x80c\n(\xA4w\x11a\x02\x93W\x80c\n(\xA4w\x14a\x03]W\x80c\x18\x16\r\xDD\x14a\x03pW\x80c#\xB8r\xDD\x14a\x03xW\x80c$\x8A\x9C\xA3\x14a\x03\x8BW\x80c&\x06\xA1\x0B\x14a\x03\xAEW\x80c-c&\x92\x14a\x03\xCBW`\0\x80\xFD[\x80c\x01\xE1\xD1\x14\x14a\x02\xDBW\x80c\x01\xFF\xC9\xA7\x14a\x02\xF6W\x80c\x06\xFD\xDE\x03\x14a\x03\x19W\x80c\x07\xA2\xD1:\x14a\x03.W\x80c\tR\x86N\x14a\x03AW\x80c\t^\xA7\xB3\x14a\x03JW[`\0\x80\xFD[a\x02\xE3a\x06xV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\ta\x03\x046`\x04a(\x18V[a\x06\x8FV[`@Q\x90\x15\x15\x81R` \x01a\x02\xEDV[a\x03!a\x06\xC6V[`@Qa\x02\xED\x91\x90a(fV[a\x02\xE3a\x03<6`\x04a(\x99V[a\x07XV[a\x02\xE3`\x0CT\x81V[a\x03\ta\x03X6`\x04a(\xC9V[a\x07eV[a\x02\xE3a\x03k6`\x04a(\x99V[a\x07}V[a\x02\xE3a\x07\x8AV[a\x03\ta\x03\x866`\x04a(\xF3V[a\x07\xA1V[a\x02\xE3a\x03\x996`\x04a(\x99V[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[a\x03\xB6a\x07\xC7V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02\xEDV[a\x02\xE3`\nT\x81V[a\x03\xE7a\x03\xE26`\x04a)/V[a\nrV[\0[a\x03\xF1a\n\x9CV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\xEDV[a\x03\xE7a\x04\x116`\x04a)/V[a\n\xC8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xEDV[a\x03\ta\x04^6`\x04a(\xC9V[a\x0BFV[a\x02\xE3a\x04q6`\x04a)[V[P`\0\x19\x90V[a\x02\xE3`\tT\x81V[a\x02\xE3a\x04\x8F6`\x04a)/V[a\x0BhV[a\x02\xE3a\x04\xA26`\x04a)[V[a\x0B\x82V[a\x03\xE7a\x04\xB56`\x04a)[V[a\x0B\xDDV[a\x03\ta\x04\xC86`\x04a)/V[a\x0C>V[a\x02\xE3a\x04\xDB6`\x04a)/V[a\x0CgV[a\x03!a\x0C\x81V[a\x02\xE3a\x0C\x90V[a\x02\xE3`\x07T\x81V[a\x02\xE3a\x05\x076`\x04a)vV[a\x0C\xA8V[a\x048\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xE3`\0\x81V[a\x02\xE3a\x05I6`\x04a)vV[a\r)V[a\x03\ta\x05\\6`\x04a(\xC9V[a\r\x9EV[a\x048\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\ta\x05\x966`\x04a(\xC9V[a\x0E$V[a\x02\xE3a\x05\xA96`\x04a(\x99V[a\x0E2V[a\x02\xE3a\x05\xBC6`\x04a)\xBAV[a\x0E?V[a\x02\xE3a\x05\xCF6`\x04a)\xBAV[a\x0E\xBEV[a\x02\xE3`\x0BT\x81V[a\x02\xE3a\x05\xEB6`\x04a(\x99V[a\x0F5V[a\x02\xE3a\x05\xFE6`\x04a)[V[a\x0FBV[a\x03\xE7a\x06\x116`\x04a)[V[a\x0FWV[a\x03\xE7a\x06$6`\x04a)/V[a\x0F\xA3V[a\x02\xE3a\x0676`\x04a)[V[a\x0F\xC8V[a\x02\xE3a\x06J6`\x04a)\xF6V[a\x0F\xD3V[a\x03\xE7a\x06]6`\x04a(\x99V[a\x0F\xFEV[`\rTa\x03\t\x90`\xFF\x16\x81V[a\x02\xE3`\x08T\x81V[`\0`\x08T`\x07Ta\x06\x8A\x91\x90a*6V[\x90P\x90V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06\xC0WPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[```\x05\x80Ta\x06\xD5\x90a*IV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\x01\x90a*IV[\x80\x15a\x07NW\x80`\x1F\x10a\x07#Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07NV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x071W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x06\xC0\x82`\0a\x10\xE8V[`\x003a\x07s\x81\x85\x85a\x11\x1CV[P`\x01\x93\x92PPPV[`\0a\x06\xC0\x82`\x01a\x12@V[`\0a\x07\x94a\x12\x8EV[`\x04Ta\x06\x8A\x91\x90a*\x83V[`\x003a\x07\xAF\x85\x82\x85a\x12\xEAV[a\x07\xBA\x85\x85\x85a\x13dV[`\x01\x91PP[\x93\x92PPPV[`\0\x80a\x07\xF4\x7Fq\xA9\x85\x9D}\xD2\x1B$PJo0`w\xFF\xC2\xD5\x10\xB4\xD4\xB6\x11(\xE91\xFE\x93tA\xAD\x1863a\x0C>V[a\x083W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj7\xB76<\x905\xB2\xB2\xB82\xB9`\xA9\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x08;a\x15\x0FV[`\0a\x08Ea\x06xV[\x90P`\0a\x08Qa\x15hV[\x90Pa\x08[a\x17\x9DV[`\0\x82\x82\x11\x15a\x08\x8BWa\x08o\x83\x83a*\x83V[\x94Pa\x08z\x85a\x0F5V[\x90Pa\x08\x860\x82a\x17\xD1V[a\x08\xCDV[a\x08\x95\x82\x84a*\x83V[\x93P\x83\x15a\x08\xCDW`\0a\x08\xB9a\x08\xAB\x86a\x0F5V[a\x08\xB40a\x0B\x82V[a\x18\x92V[\x90P\x80\x15a\x08\xCBWa\x08\xCB0\x82a\x18\xA8V[P[`\0a\x08\xD80a\x0B\x82V[\x90P\x80\x15a\tsW`\0B`\nT\x11\x15a\t\x11Wa\x08\xF6\x83\x83a*\x83V[B`\nTa\t\x04\x91\x90a*\x83V[a\t\x0E\x91\x90a*\x96V[\x90P[`\0\x82`\x0CT\x85a\t\"\x91\x90a*\x96V[a\t,\x90\x84a*6V[a\t6\x91\x90a*\xC3V[\x90P\x80a\tHd\xE8\xD4\xA5\x10\0\x85a*\x96V[a\tR\x91\x90a*\xC3V[`\tUa\t_\x81Ba*6V[`\x01`\x01`\x80\x1B\x03\x16`\nUPa\ty\x90PV[`\0`\tU[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x06\x91\x90a*\xE5V[`\x07\x81\x90U\x90Pa\n\x17\x81\x85a*\x83V[`\x08UB`\x01`\x01`\x80\x1B\x03\x16`\x0BU`@\x80Q\x88\x81R` \x81\x01\x88\x90R\x7Fo\xEA6\xBF\xB4\xB7\xC9\x12a7V\x95\x1F\xB5\xBF\x8D\xDA\xED\xB7M\xE1\xF2V\xD0\xBE\xF8\x153\x15\x8C\xDAT\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPa\nn`\x01\x80UV[\x90\x91V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\n\x8D\x81a\x19\xDCV[a\n\x97\x83\x83a\x19\xE6V[PPPV[`\0a\x06\x8A\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a*\xFEV[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0B8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x08*V[a\x0BB\x82\x82a\x1AjV[PPV[`\x003a\x07s\x81\x85\x85a\x0BY\x83\x83a\x0F\xD3V[a\x0Bc\x91\x90a*6V[a\x11\x1CV[`\0\x80a\x0Bt\x84a\x0F5V[\x90Pa\x07\xC03\x84\x86\x84a\x1A\xCFV[`\x000`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\x0B\xBFWa\x0B\x9Ca\x12\x8EV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90 Ta\x06\xC0\x91\x90a*\x83V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02` R`@\x90 Ta\x06\xC0V[a\x0B\xF5`\0\x80Q` a-@\x839\x81Q\x91R3a\x0C>V[a\x0C\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08*\x90a+\x17V[a\x0C;\x7Fq\xA9\x85\x9D}\xD2\x1B$PJo0`w\xFF\xC2\xD5\x10\xB4\xD4\xB6\x11(\xE91\xFE\x93tA\xAD\x186\x82a\x1C\xE0V[PV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x80a\x0Cs\x84a\x0E2V[\x90Pa\x07\xC03\x84\x83\x87a\x1A\xCFV[```\x06\x80Ta\x06\xD5\x90a*IV[`\0a\x06\x8Aa\x0C\x9Da\n\x9CV[a\x03<\x90`\na,$V[`\0a\x0C\xB3\x83a\x0F\xC8V[\x85\x11\x15a\r\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC4626: redeem more than max\0\0\0`D\x82\x01R`d\x01a\x08*V[`\0a\r\r\x86a\x07XV[\x90Pa\r\x1D3\x86\x86\x84\x8A\x88a\x1C\xEAV[P\x90P[\x94\x93PPPPV[`\0a\r4\x83a\x0FBV[\x85\x11\x15a\r\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC4626: withdraw more than max\0`D\x82\x01R`d\x01a\x08*V[`\0a\r\x8E\x86a\x07}V[\x90Pa\r\x1D3\x86\x86\x89\x85\x88a\x1C\xEAV[`\x003\x81a\r\xAC\x82\x86a\x0F\xD3V[\x90P\x83\x81\x10\x15a\x0E\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x08*V[a\x0E\x19\x82\x86\x86\x84\x03a\x11\x1CV[P`\x01\x94\x93PPPPV[`\x003a\x07s\x81\x85\x85a\x13dV[`\0a\x06\xC0\x82`\x01a\x10\xE8V[`\0a\x0EJ\x82a\x0FBV[\x84\x11\x15a\x0E\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC4626: withdraw more than max\0`D\x82\x01R`d\x01a\x08*V[`\0a\x0E\xA4\x85a\x07}V[\x90Pa\x0E\xB53\x85\x85\x88\x85`\0a\x1C\xEAV[P\x94\x93PPPPV[`\0a\x0E\xC9\x82a\x0F\xC8V[\x84\x11\x15a\x0F\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC4626: redeem more than max\0\0\0`D\x82\x01R`d\x01a\x08*V[`\0a\x0F#\x85a\x07XV[\x90Pa\x0E\xB53\x85\x85\x84\x89a'\x10a\x1C\xEAV[`\0a\x06\xC0\x82`\0a\x12@V[`\0a\x06\xC0a\x0FP\x83a\x0B\x82V[`\0a\x10\xE8V[a\x0Fo`\0\x80Q` a-@\x839\x81Q\x91R3a\x0C>V[a\x0F\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08*\x90a+\x17V[a\x0C;`\0\x80Q` a-@\x839\x81Q\x91R\x82a\x1C\xE0V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0F\xBE\x81a\x19\xDCV[a\n\x97\x83\x83a\x1AjV[`\0a\x06\xC0\x82a\x0B\x82V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x10\x16`\0\x80Q` a-@\x839\x81Q\x91R3a\x0C>V[a\x102W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08*\x90a+\x17V[\x80`\0\x03a\x10nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`\xBA\x1B`D\x82\x01R`d\x01a\x08*V[c\x01\xE1\x85X\x81\x11\x15a\x10\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01Rgtoo long`\xC0\x1B`D\x82\x01R`d\x01a\x08*V[`\x0C\x81\x90U`@Q\x81\x81R\x7F\xF3a\xAE\xD4c\xDAo\xA2\x03X\xE4\\b\t\xF1\xD3\xE1mN\xCApnn\xAB\x0B\n\xEB3\x87)\xC7z\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80a\x10\xF3a\x07\x8AV[\x90P\x80\x15a\x11\x14Wa\x11\x0Fa\x11\x06a\x06xV[\x85\x90\x83\x86a \"V[a\r!V[P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x11~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x11\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0\x80a\x12Ka\x06xV[\x90P`\0a\x12Wa\x07\x8AV[\x90P\x81`\0\x03a\x12yW\x80\x15a\x12nW`\0a\x12pV[\x84[\x92PPPa\x06\xC0V[a\x12\x85\x85\x82\x84\x87a \"V[\x95\x94PPPPPV[`\0\x80B`\nT\x11\x15a\x12\xCBW`\0`\x0BTBa\x12\xAB\x91\x90a*\x83V[\x90Pd\xE8\xD4\xA5\x10\0`\tT\x82a\x12\xC1\x91\x90a*\x96V[a\x07\xC0\x91\x90a*\xC3V[`\nT\x15a\x12\xE5WP0`\0\x90\x81R`\x02` R`@\x90 T[\x91\x90PV[`\0a\x12\xF6\x84\x84a\x0F\xD3V[\x90P`\0\x19\x81\x14a\x13^W\x81\x81\x10\x15a\x13QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x08*V[a\x13^\x84\x84\x84\x84\x03a\x11\x1CV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x13\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a\x14\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x15\x02\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x13^V[`\x02`\x01T\x03a\x15aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x08*V[`\x02`\x01UV[`\rT`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\xFF\x16\x15a\x16\x9DW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x03\x91\x90a*\xE5V[\x90P\x80\x15a\x16\x9BW`@Qca{\xA07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R0`D\x83\x01R`\0`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ca{\xA07\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\x96W=`\0\x80>=`\0\xFD[PPPP[P[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x05\x91\x90a*\xE5V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x8D\x91\x90a*\xE5V[a\x17\x97\x91\x90a*6V[\x91PP\x90V[`\0a\x17\xA7a\x12\x8EV[\x90P\x80\x15a\x0C;WB`\nT\x11\x15a\x17\xC7W`\x01`\x01`\x80\x1B\x03B\x16`\x0BU[a\x0C;0\x82a\x18\xA8V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x18'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x08*V[\x80`\x04`\0\x82\x82Ta\x189\x91\x90a*6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\0\x81\x83\x10a\x18\xA1W\x81a\x07\xC0V[P\x90\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x19\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a\x19|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x08*V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`\x04\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x0C;\x813a }V[a\x19\xF0\x82\x82a\x0C>V[a\x0BBW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1A&3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x1At\x82\x82a\x0C>V[\x15a\x0BBW`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[a\x1A\xD7a\x15\x0FV[a\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x850\x85a \xD6V[`\0\x82`\x07Ta\x1B\x13\x91\x90a*6V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xA2\x91\x90a*\xE5V[\x90Pa\x1B\xAD\x82a!AV[`\0a\x1CM\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C=\x91\x90a*\xE5V[a\x1CG\x90\x84a*\x83V[\x84a\x18\x92V[\x90P\x80`\x08`\0\x82\x82Ta\x1Ca\x91\x90a*6V[\x90\x91UPa\x1Cq\x90P\x81\x84a*\x83V[`\x07Ua\x1C~\x86\x85a\x17\xD1V[\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x87\x87`@Qa\x1C\xCC\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPa\x13^`\x01\x80UV[a\x0BB\x82\x82a\x19\xE6V[`\0a\x1C\xF4a\x15\x0FV[`\x01`\x01`\xA0\x1B\x03\x86\x16a\x1DJW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReceiver cannot be zero address\0`D\x82\x01R`d\x01a\x08*V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1DnWa\x1Dn\x85\x88\x85a\x12\xEAV[`\x07T\x84\x81\x10\x15a\x1FpW`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x06\x91\x90a*\xE5V[\x90Pa\x1E\x1Aa\x1E\x15\x83\x88a*\x83V[a\"\x01V[`\0a\x1E\xBD\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xAB\x91\x90a*\xE5V[a\x1E\xB5\x91\x90a*\x83V[`\x08Ta\x18\x92V[\x90Pa\x1E\xC9\x81\x84a*6V[\x92P`\0\x87\x84\x10\x15a\x1FLWa\x1E\xDF\x84\x89a*\x83V[\x90Pa'\x10\x86\x10\x15a\x1FHWa'\x10a\x1E\xF8\x87\x8Aa*\x96V[a\x1F\x02\x91\x90a*\xC3V[\x81\x11\x15a\x1FHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsLoss exceeds maxLoss``\x1B`D\x82\x01R`d\x01a\x08*V[\x83\x97P[a\x1FV\x81\x83a*6V[`\x08`\0\x82\x82Ta\x1Fg\x91\x90a*\x83V[\x90\x91UPPPPP[a\x1Fz\x85\x82a*\x83V[`\x07Ua\x1F\x87\x86\x85a\x18\xA8V[a\x1F\xB2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x87a#XV[\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x88\x88`@Qa \n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UP\x90\x94\x93PPPPV[`\0\x80a 0\x86\x86\x86a#\x88V[\x90P`\x01\x83`\x02\x81\x11\x15a FWa Fa,3V[\x14\x80\x15a cWP`\0\x84\x80a ^Wa ^a*\xADV[\x86\x88\t\x11[\x15a\x12\x85Wa s`\x01\x82a*6V[\x96\x95PPPPPPV[a \x87\x82\x82a\x0C>V[a\x0BBWa \x94\x81a$rV[a \x9F\x83` a$\x84V[`@Q` \x01a \xB0\x92\x91\x90a,IV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x08*\x91`\x04\x01a(fV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x13^\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra& V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16ca{\xA07\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x84\x90R0`D\x82\x01R`\0`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xE6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\xFAW=`\0\x80>=`\0\xFD[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16ci2\x8D\xEC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\"\xE3\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xB9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xDD\x91\x90a*\xE5V[\x85a\x18\x92V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R0`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a#4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BB\x91\x90a*\xE5V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\n\x97\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a!\nV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a#\xC2W\x83\x82\x81a#\xB8Wa#\xB8a*\xADV[\x04\x92PPPa\x07\xC0V[\x80\x84\x11a$\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\x08*V[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[``a\x06\xC0`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a$\x93\x83`\x02a*\x96V[a$\x9E\x90`\x02a*6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xB6Wa$\xB6a,\xBEV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a$\xE0W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a$\xFBWa$\xFBa,\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a%*Wa%*a,\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a%N\x84`\x02a*\x96V[a%Y\x90`\x01a*6V[\x90P[`\x01\x81\x11\x15a%\xD1Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a%\x8DWa%\x8Da,\xD4V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a%\xA3Wa%\xA3a,\xD4V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a%\xCA\x81a,\xEAV[\x90Pa%\\V[P\x83\x15a\x07\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x08*V[`\0a&u\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a&\xF5\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a&\x96WP\x80\x80` \x01\x90Q\x81\x01\x90a&\x96\x91\x90a-\x01V[a\n\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x08*V[``a\r!\x84\x84`\0\x85\x85`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa'\x1C\x91\x90a-#V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a'YW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a'^V[``\x91P[P\x91P\x91Pa'o\x87\x83\x83\x87a'zV[\x97\x96PPPPPPPV[``\x83\x15a'\xE9W\x82Q`\0\x03a'\xE2W`\x01`\x01`\xA0\x1B\x03\x85\x16;a'\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x08*V[P\x81a\r!V[a\r!\x83\x83\x81Q\x15a'\xFEW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08*\x91\x90a(fV[`\0` \x82\x84\x03\x12\x15a(*W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\xC0W`\0\x80\xFD[`\0[\x83\x81\x10\x15a(]W\x81\x81\x01Q\x83\x82\x01R` \x01a(EV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra(\x85\x81`@\x85\x01` \x87\x01a(BV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a(\xABW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12\xE5W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a(\xDCW`\0\x80\xFD[a(\xE5\x83a(\xB2V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a)\x08W`\0\x80\xFD[a)\x11\x84a(\xB2V[\x92Pa)\x1F` \x85\x01a(\xB2V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a)BW`\0\x80\xFD[\x825\x91Pa)R` \x84\x01a(\xB2V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a)mW`\0\x80\xFD[a\x07\xC0\x82a(\xB2V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a)\x8CW`\0\x80\xFD[\x845\x93Pa)\x9C` \x86\x01a(\xB2V[\x92Pa)\xAA`@\x86\x01a(\xB2V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a)\xCFW`\0\x80\xFD[\x835\x92Pa)\xDF` \x85\x01a(\xB2V[\x91Pa)\xED`@\x85\x01a(\xB2V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a*\tW`\0\x80\xFD[a*\x12\x83a(\xB2V[\x91Pa)R` \x84\x01a(\xB2V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xC0Wa\x06\xC0a* V[`\x01\x81\x81\x1C\x90\x82\x16\x80a*]W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a*}WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x06\xC0Wa\x06\xC0a* V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xC0Wa\x06\xC0a* V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a*\xE0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a*\xF7W`\0\x80\xFD[PQ\x91\x90PV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x06\xC0Wa\x06\xC0a* V[` \x80\x82R`\x0F\x90\x82\x01Rn\x1B\xDB\x9B\x1EH\x1BX[\x98Y\xD9[Y[\x9D`\x8A\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81[\x80\x85\x11\x15a+{W\x81`\0\x19\x04\x82\x11\x15a+aWa+aa* V[\x80\x85\x16\x15a+nW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a+EV[P\x92P\x92\x90PV[`\0\x82a+\x92WP`\x01a\x06\xC0V[\x81a+\x9FWP`\0a\x06\xC0V[\x81`\x01\x81\x14a+\xB5W`\x02\x81\x14a+\xBFWa+\xDBV[`\x01\x91PPa\x06\xC0V[`\xFF\x84\x11\x15a+\xD0Wa+\xD0a* V[PP`\x01\x82\x1Ba\x06\xC0V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a+\xFEWP\x81\x81\na\x06\xC0V[a,\x08\x83\x83a+@V[\x80`\0\x19\x04\x82\x11\x15a,\x1CWa,\x1Ca* V[\x02\x93\x92PPPV[`\0a\x07\xC0`\xFF\x84\x16\x83a+\x83V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa,\x81\x81`\x17\x85\x01` \x88\x01a(BV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa,\xB2\x81`(\x84\x01` \x88\x01a(BV[\x01`(\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a,\xF9Wa,\xF9a* V[P`\0\x19\x01\x90V[`\0` \x82\x84\x03\x12\x15a-\x13W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xC0W`\0\x80\xFD[`\0\x82Qa-5\x81\x84` \x87\x01a(BV[\x91\x90\x91\x01\x92\x91PPV\xFE\xB4\xD5\xC1a\x80\xAA\x1FB\xA7\x9CC\x01\xE7\xBDt\xB2J\xA5VV<f-\xB1\tW\xE0\xE1\x95\xB5\xBER\xA2dipfsX\"\x12 \\\xA6\xA3\xAF\x06\xE7O\xAAFz\xC3\xDE\0\x12\xFB\n\xF0\xCA\x93\x1Cn\xDF\xC9\x08\xA8Z-2\x85\x10r\xDDdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static AAVEV3STRATEGY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AaveV3Strategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AaveV3Strategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AaveV3Strategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AaveV3Strategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AaveV3Strategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AaveV3Strategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AaveV3Strategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AAVEV3STRATEGY_ABI.clone(),
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
                AAVEV3STRATEGY_ABI.clone(),
                AAVEV3STRATEGY_BYTECODE.clone().into(),
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
        ///Calls the contract's `aToken` (0xa0c1f15e) function
        pub fn a_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([160, 193, 241, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `asset` (0x38d52e0f) function
        pub fn asset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([56, 213, 46, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertToAssets` (0x07a2d13a) function
        pub fn convert_to_assets(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([7, 162, 209, 58], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertToShares` (0xc6e6f592) function
        pub fn convert_to_shares(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([198, 230, 245, 146], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowance` (0xa457c2d7) function
        pub fn decrease_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x6e553f65) function
        pub fn deposit(
            &self,
            assets: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([110, 85, 63, 101], (assets, receiver))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fullProfitUnlockDate` (0x2d632692) function
        pub fn full_profit_unlock_date(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([45, 99, 38, 146], ())
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
        ///Calls the contract's `increaseAllowance` (0x39509351) function
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastReport` (0xc3535b52) function
        pub fn last_report(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([195, 83, 91, 82], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lendingPool` (0xa59a9973) function
        pub fn lending_pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([165, 154, 153, 115], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxDeposit` (0x402d267d) function
        pub fn max_deposit(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 45, 38, 125], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxMint` (0xc63d75b6) function
        pub fn max_mint(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([198, 61, 117, 182], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxRedeem` (0xd905777e) function
        pub fn max_redeem(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([217, 5, 119, 126], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxWithdraw` (0xce96cb77) function
        pub fn max_withdraw(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([206, 150, 203, 119], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x94bf804d) function
        pub fn mint(
            &self,
            shares: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([148, 191, 128, 77], (shares, receiver))
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
        ///Calls the contract's `previewDeposit` (0xef8b30f7) function
        pub fn preview_deposit(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([239, 139, 48, 247], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewMint` (0xb3d7f6b9) function
        pub fn preview_mint(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 215, 246, 185], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewRedeem` (0x4cdad506) function
        pub fn preview_redeem(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([76, 218, 213, 6], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewWithdraw` (0x0a28a477) function
        pub fn preview_withdraw(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 40, 164, 119], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pricePerShare` (0x99530b06) function
        pub fn price_per_share(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([153, 83, 11, 6], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `profitMaxUnlockTime` (0x0952864e) function
        pub fn profit_max_unlock_time(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([9, 82, 134, 78], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `profitUnlockingRate` (0x5141eebb) function
        pub fn profit_unlocking_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([81, 65, 238, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeem` (0x9f40a7b3) function
        pub fn redeem_with_shares_and_receiver_and_max_loss(
            &self,
            shares: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
            max_loss: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([159, 64, 167, 179], (shares, receiver, owner, max_loss))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeem` (0xba087652) function
        pub fn redeem(
            &self,
            shares: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([186, 8, 118, 82], (shares, receiver, owner))
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
        ///Calls the contract's `report` (0x2606a10b) function
        pub fn report(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([38, 6, 161, 11], ())
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
        ///Calls the contract's `setKeeper` (0x748747e6) function
        pub fn set_keeper(
            &self,
            keeper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 135, 71, 230], keeper)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setManagement` (0xd4a22bde) function
        pub fn set_management(
            &self,
            management: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 162, 43, 222], management)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProfitMaxUnlockTime` (0xdf69b22a) function
        pub fn set_profit_max_unlock_time(
            &self,
            profit_max_unlock_time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 105, 178, 42], profit_max_unlock_time)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shutdown` (0xfc0e74d1) function
        pub fn shutdown(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([252, 14, 116, 209], ())
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
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalAssets` (0x01e1d114) function
        pub fn total_assets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([1, 225, 209, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalDebt` (0xfc7b9c18) function
        pub fn total_debt(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([252, 123, 156, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalIdle` (0x9aa7df94) function
        pub fn total_idle(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 167, 223, 148], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0xa318c1a4) function
        pub fn withdraw_with_assets_and_receiver_and_max_loss(
            &self,
            assets: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
            max_loss: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([163, 24, 193, 164], (assets, receiver, owner, max_loss))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0xb460af94) function
        pub fn withdraw(
            &self,
            assets: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([180, 96, 175, 148], (assets, receiver, owner))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `Reported` event
        pub fn reported_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReportedFilter,
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
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UpdateProfitMaxUnlockTime` event
        pub fn update_profit_max_unlock_time_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdateProfitMaxUnlockTimeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdraw` event
        pub fn withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AaveV3StrategyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AaveV3Strategy<M> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,uint256,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
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
    #[ethevent(name = "Reported", abi = "Reported(uint256,uint256)")]
    pub struct ReportedFilter {
        pub profit: ::ethers::core::types::U256,
        pub loss: ::ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
        name = "UpdateProfitMaxUnlockTime",
        abi = "UpdateProfitMaxUnlockTime(uint256)"
    )]
    pub struct UpdateProfitMaxUnlockTimeFilter {
        pub new_profit_max_unlock_time: ::ethers::core::types::U256,
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
        name = "Withdraw",
        abi = "Withdraw(address,address,address,uint256,uint256)"
    )]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
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
    pub enum AaveV3StrategyEvents {
        ApprovalFilter(ApprovalFilter),
        DepositFilter(DepositFilter),
        ReportedFilter(ReportedFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
        TransferFilter(TransferFilter),
        UpdateProfitMaxUnlockTimeFilter(UpdateProfitMaxUnlockTimeFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for AaveV3StrategyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(AaveV3StrategyEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(AaveV3StrategyEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = ReportedFilter::decode_log(log) {
                return Ok(AaveV3StrategyEvents::ReportedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(AaveV3StrategyEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(AaveV3StrategyEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(AaveV3StrategyEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(AaveV3StrategyEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UpdateProfitMaxUnlockTimeFilter::decode_log(log) {
                return Ok(
                    AaveV3StrategyEvents::UpdateProfitMaxUnlockTimeFilter(decoded),
                );
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(AaveV3StrategyEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AaveV3StrategyEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReportedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateProfitMaxUnlockTimeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for AaveV3StrategyEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for AaveV3StrategyEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<ReportedFilter> for AaveV3StrategyEvents {
        fn from(value: ReportedFilter) -> Self {
            Self::ReportedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for AaveV3StrategyEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for AaveV3StrategyEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for AaveV3StrategyEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for AaveV3StrategyEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<UpdateProfitMaxUnlockTimeFilter>
    for AaveV3StrategyEvents {
        fn from(value: UpdateProfitMaxUnlockTimeFilter) -> Self {
            Self::UpdateProfitMaxUnlockTimeFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for AaveV3StrategyEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
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
    ///Container type for all input parameters for the `aToken` function with signature `aToken()` and selector `0xa0c1f15e`
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
    #[ethcall(name = "aToken", abi = "aToken()")]
    pub struct AtokenCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    #[ethcall(name = "asset", abi = "asset()")]
    pub struct AssetCall;
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `convertToAssets` function with signature `convertToAssets(uint256)` and selector `0x07a2d13a`
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
    #[ethcall(name = "convertToAssets", abi = "convertToAssets(uint256)")]
    pub struct ConvertToAssetsCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `convertToShares` function with signature `convertToShares(uint256)` and selector `0xc6e6f592`
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
    #[ethcall(name = "convertToShares", abi = "convertToShares(uint256)")]
    pub struct ConvertToSharesCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256,address)` and selector `0x6e553f65`
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
    #[ethcall(name = "deposit", abi = "deposit(uint256,address)")]
    pub struct DepositCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `fullProfitUnlockDate` function with signature `fullProfitUnlockDate()` and selector `0x2d632692`
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
    #[ethcall(name = "fullProfitUnlockDate", abi = "fullProfitUnlockDate()")]
    pub struct FullProfitUnlockDateCall;
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
    ///Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `lastReport` function with signature `lastReport()` and selector `0xc3535b52`
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
    #[ethcall(name = "lastReport", abi = "lastReport()")]
    pub struct LastReportCall;
    ///Container type for all input parameters for the `lendingPool` function with signature `lendingPool()` and selector `0xa59a9973`
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
    #[ethcall(name = "lendingPool", abi = "lendingPool()")]
    pub struct LendingPoolCall;
    ///Container type for all input parameters for the `maxDeposit` function with signature `maxDeposit(address)` and selector `0x402d267d`
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
    #[ethcall(name = "maxDeposit", abi = "maxDeposit(address)")]
    pub struct MaxDepositCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `maxMint` function with signature `maxMint(address)` and selector `0xc63d75b6`
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
    #[ethcall(name = "maxMint", abi = "maxMint(address)")]
    pub struct MaxMintCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `maxRedeem` function with signature `maxRedeem(address)` and selector `0xd905777e`
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
    #[ethcall(name = "maxRedeem", abi = "maxRedeem(address)")]
    pub struct MaxRedeemCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `maxWithdraw` function with signature `maxWithdraw(address)` and selector `0xce96cb77`
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
    #[ethcall(name = "maxWithdraw", abi = "maxWithdraw(address)")]
    pub struct MaxWithdrawCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(uint256,address)` and selector `0x94bf804d`
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
    #[ethcall(name = "mint", abi = "mint(uint256,address)")]
    pub struct MintCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `previewDeposit` function with signature `previewDeposit(uint256)` and selector `0xef8b30f7`
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
    #[ethcall(name = "previewDeposit", abi = "previewDeposit(uint256)")]
    pub struct PreviewDepositCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewMint` function with signature `previewMint(uint256)` and selector `0xb3d7f6b9`
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
    #[ethcall(name = "previewMint", abi = "previewMint(uint256)")]
    pub struct PreviewMintCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewRedeem` function with signature `previewRedeem(uint256)` and selector `0x4cdad506`
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
    #[ethcall(name = "previewRedeem", abi = "previewRedeem(uint256)")]
    pub struct PreviewRedeemCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewWithdraw` function with signature `previewWithdraw(uint256)` and selector `0x0a28a477`
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
    #[ethcall(name = "previewWithdraw", abi = "previewWithdraw(uint256)")]
    pub struct PreviewWithdrawCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pricePerShare` function with signature `pricePerShare()` and selector `0x99530b06`
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
    #[ethcall(name = "pricePerShare", abi = "pricePerShare()")]
    pub struct PricePerShareCall;
    ///Container type for all input parameters for the `profitMaxUnlockTime` function with signature `profitMaxUnlockTime()` and selector `0x0952864e`
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
    #[ethcall(name = "profitMaxUnlockTime", abi = "profitMaxUnlockTime()")]
    pub struct ProfitMaxUnlockTimeCall;
    ///Container type for all input parameters for the `profitUnlockingRate` function with signature `profitUnlockingRate()` and selector `0x5141eebb`
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
    #[ethcall(name = "profitUnlockingRate", abi = "profitUnlockingRate()")]
    pub struct ProfitUnlockingRateCall;
    ///Container type for all input parameters for the `redeem` function with signature `redeem(uint256,address,address,uint256)` and selector `0x9f40a7b3`
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
    #[ethcall(name = "redeem", abi = "redeem(uint256,address,address,uint256)")]
    pub struct RedeemWithSharesAndReceiverAndMaxLossCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
        pub max_loss: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `redeem` function with signature `redeem(uint256,address,address)` and selector `0xba087652`
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
    #[ethcall(name = "redeem", abi = "redeem(uint256,address,address)")]
    pub struct RedeemCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `report` function with signature `report()` and selector `0x2606a10b`
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
    #[ethcall(name = "report", abi = "report()")]
    pub struct ReportCall;
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
    ///Container type for all input parameters for the `setKeeper` function with signature `setKeeper(address)` and selector `0x748747e6`
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
    #[ethcall(name = "setKeeper", abi = "setKeeper(address)")]
    pub struct SetKeeperCall {
        pub keeper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setManagement` function with signature `setManagement(address)` and selector `0xd4a22bde`
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
    #[ethcall(name = "setManagement", abi = "setManagement(address)")]
    pub struct SetManagementCall {
        pub management: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setProfitMaxUnlockTime` function with signature `setProfitMaxUnlockTime(uint256)` and selector `0xdf69b22a`
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
    #[ethcall(name = "setProfitMaxUnlockTime", abi = "setProfitMaxUnlockTime(uint256)")]
    pub struct SetProfitMaxUnlockTimeCall {
        pub profit_max_unlock_time: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalAssets` function with signature `totalAssets()` and selector `0x01e1d114`
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
    #[ethcall(name = "totalAssets", abi = "totalAssets()")]
    pub struct TotalAssetsCall;
    ///Container type for all input parameters for the `totalDebt` function with signature `totalDebt()` and selector `0xfc7b9c18`
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
    #[ethcall(name = "totalDebt", abi = "totalDebt()")]
    pub struct TotalDebtCall;
    ///Container type for all input parameters for the `totalIdle` function with signature `totalIdle()` and selector `0x9aa7df94`
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
    #[ethcall(name = "totalIdle", abi = "totalIdle()")]
    pub struct TotalIdleCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,address,address,uint256)` and selector `0xa318c1a4`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address,address,uint256)")]
    pub struct WithdrawWithAssetsAndReceiverAndMaxLossCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
        pub max_loss: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,address,address)` and selector `0xb460af94`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address,address)")]
    pub struct WithdrawCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
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
    pub enum AaveV3StrategyCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        Atoken(AtokenCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        BalanceOf(BalanceOfCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToShares(ConvertToSharesCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        Deposit(DepositCall),
        FullProfitUnlockDate(FullProfitUnlockDateCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        LastReport(LastReportCall),
        LendingPool(LendingPoolCall),
        MaxDeposit(MaxDepositCall),
        MaxMint(MaxMintCall),
        MaxRedeem(MaxRedeemCall),
        MaxWithdraw(MaxWithdrawCall),
        Mint(MintCall),
        Name(NameCall),
        PreviewDeposit(PreviewDepositCall),
        PreviewMint(PreviewMintCall),
        PreviewRedeem(PreviewRedeemCall),
        PreviewWithdraw(PreviewWithdrawCall),
        PricePerShare(PricePerShareCall),
        ProfitMaxUnlockTime(ProfitMaxUnlockTimeCall),
        ProfitUnlockingRate(ProfitUnlockingRateCall),
        RedeemWithSharesAndReceiverAndMaxLoss(RedeemWithSharesAndReceiverAndMaxLossCall),
        Redeem(RedeemCall),
        RenounceRole(RenounceRoleCall),
        Report(ReportCall),
        RevokeRole(RevokeRoleCall),
        SetKeeper(SetKeeperCall),
        SetManagement(SetManagementCall),
        SetProfitMaxUnlockTime(SetProfitMaxUnlockTimeCall),
        Shutdown(ShutdownCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TotalAssets(TotalAssetsCall),
        TotalDebt(TotalDebtCall),
        TotalIdle(TotalIdleCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        WithdrawWithAssetsAndReceiverAndMaxLoss(
            WithdrawWithAssetsAndReceiverAndMaxLossCall,
        ),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for AaveV3StrategyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) = <AtokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Atoken(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <AssetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Asset(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <ConvertToAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConvertToAssets(decoded));
            }
            if let Ok(decoded) = <ConvertToSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConvertToShares(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <FullProfitUnlockDateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FullProfitUnlockDate(decoded));
            }
            if let Ok(decoded) = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRoleAdmin(decoded));
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
            if let Ok(decoded) = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = <LastReportCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastReport(decoded));
            }
            if let Ok(decoded) = <LendingPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LendingPool(decoded));
            }
            if let Ok(decoded) = <MaxDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxDeposit(decoded));
            }
            if let Ok(decoded) = <MaxMintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxMint(decoded));
            }
            if let Ok(decoded) = <MaxRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxRedeem(decoded));
            }
            if let Ok(decoded) = <MaxWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxWithdraw(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <PreviewDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewDeposit(decoded));
            }
            if let Ok(decoded) = <PreviewMintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewMint(decoded));
            }
            if let Ok(decoded) = <PreviewRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewRedeem(decoded));
            }
            if let Ok(decoded) = <PreviewWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewWithdraw(decoded));
            }
            if let Ok(decoded) = <PricePerShareCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PricePerShare(decoded));
            }
            if let Ok(decoded) = <ProfitMaxUnlockTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProfitMaxUnlockTime(decoded));
            }
            if let Ok(decoded) = <ProfitUnlockingRateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProfitUnlockingRate(decoded));
            }
            if let Ok(decoded) = <RedeemWithSharesAndReceiverAndMaxLossCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RedeemWithSharesAndReceiverAndMaxLoss(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Redeem(decoded));
            }
            if let Ok(decoded) = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded) = <ReportCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Report(decoded));
            }
            if let Ok(decoded) = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded) = <SetKeeperCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetKeeper(decoded));
            }
            if let Ok(decoded) = <SetManagementCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetManagement(decoded));
            }
            if let Ok(decoded) = <SetProfitMaxUnlockTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetProfitMaxUnlockTime(decoded));
            }
            if let Ok(decoded) = <ShutdownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Shutdown(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalAssets(decoded));
            }
            if let Ok(decoded) = <TotalDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalDebt(decoded));
            }
            if let Ok(decoded) = <TotalIdleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalIdle(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <WithdrawWithAssetsAndReceiverAndMaxLossCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawWithAssetsAndReceiverAndMaxLoss(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AaveV3StrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Atoken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Asset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertToAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertToShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FullProfitUnlockDate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastReport(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LendingPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxMint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PreviewDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviewMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviewRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviewWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PricePerShare(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProfitMaxUnlockTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProfitUnlockingRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemWithSharesAndReceiverAndMaxLoss(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Redeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Report(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetKeeper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetManagement(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProfitMaxUnlockTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Shutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalIdle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawWithAssetsAndReceiverAndMaxLoss(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AaveV3StrategyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Atoken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Asset(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::FullProfitUnlockDate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastReport(element) => ::core::fmt::Display::fmt(element, f),
                Self::LendingPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::PricePerShare(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProfitMaxUnlockTime(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProfitUnlockingRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RedeemWithSharesAndReceiverAndMaxLoss(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Redeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Report(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetKeeper(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetManagement(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProfitMaxUnlockTime(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Shutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalIdle(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawWithAssetsAndReceiverAndMaxLoss(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for AaveV3StrategyCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<AtokenCall> for AaveV3StrategyCalls {
        fn from(value: AtokenCall) -> Self {
            Self::Atoken(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for AaveV3StrategyCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for AaveV3StrategyCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AssetCall> for AaveV3StrategyCalls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for AaveV3StrategyCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ConvertToAssetsCall> for AaveV3StrategyCalls {
        fn from(value: ConvertToAssetsCall) -> Self {
            Self::ConvertToAssets(value)
        }
    }
    impl ::core::convert::From<ConvertToSharesCall> for AaveV3StrategyCalls {
        fn from(value: ConvertToSharesCall) -> Self {
            Self::ConvertToShares(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for AaveV3StrategyCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for AaveV3StrategyCalls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<DepositCall> for AaveV3StrategyCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<FullProfitUnlockDateCall> for AaveV3StrategyCalls {
        fn from(value: FullProfitUnlockDateCall) -> Self {
            Self::FullProfitUnlockDate(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for AaveV3StrategyCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for AaveV3StrategyCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for AaveV3StrategyCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for AaveV3StrategyCalls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<LastReportCall> for AaveV3StrategyCalls {
        fn from(value: LastReportCall) -> Self {
            Self::LastReport(value)
        }
    }
    impl ::core::convert::From<LendingPoolCall> for AaveV3StrategyCalls {
        fn from(value: LendingPoolCall) -> Self {
            Self::LendingPool(value)
        }
    }
    impl ::core::convert::From<MaxDepositCall> for AaveV3StrategyCalls {
        fn from(value: MaxDepositCall) -> Self {
            Self::MaxDeposit(value)
        }
    }
    impl ::core::convert::From<MaxMintCall> for AaveV3StrategyCalls {
        fn from(value: MaxMintCall) -> Self {
            Self::MaxMint(value)
        }
    }
    impl ::core::convert::From<MaxRedeemCall> for AaveV3StrategyCalls {
        fn from(value: MaxRedeemCall) -> Self {
            Self::MaxRedeem(value)
        }
    }
    impl ::core::convert::From<MaxWithdrawCall> for AaveV3StrategyCalls {
        fn from(value: MaxWithdrawCall) -> Self {
            Self::MaxWithdraw(value)
        }
    }
    impl ::core::convert::From<MintCall> for AaveV3StrategyCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for AaveV3StrategyCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<PreviewDepositCall> for AaveV3StrategyCalls {
        fn from(value: PreviewDepositCall) -> Self {
            Self::PreviewDeposit(value)
        }
    }
    impl ::core::convert::From<PreviewMintCall> for AaveV3StrategyCalls {
        fn from(value: PreviewMintCall) -> Self {
            Self::PreviewMint(value)
        }
    }
    impl ::core::convert::From<PreviewRedeemCall> for AaveV3StrategyCalls {
        fn from(value: PreviewRedeemCall) -> Self {
            Self::PreviewRedeem(value)
        }
    }
    impl ::core::convert::From<PreviewWithdrawCall> for AaveV3StrategyCalls {
        fn from(value: PreviewWithdrawCall) -> Self {
            Self::PreviewWithdraw(value)
        }
    }
    impl ::core::convert::From<PricePerShareCall> for AaveV3StrategyCalls {
        fn from(value: PricePerShareCall) -> Self {
            Self::PricePerShare(value)
        }
    }
    impl ::core::convert::From<ProfitMaxUnlockTimeCall> for AaveV3StrategyCalls {
        fn from(value: ProfitMaxUnlockTimeCall) -> Self {
            Self::ProfitMaxUnlockTime(value)
        }
    }
    impl ::core::convert::From<ProfitUnlockingRateCall> for AaveV3StrategyCalls {
        fn from(value: ProfitUnlockingRateCall) -> Self {
            Self::ProfitUnlockingRate(value)
        }
    }
    impl ::core::convert::From<RedeemWithSharesAndReceiverAndMaxLossCall>
    for AaveV3StrategyCalls {
        fn from(value: RedeemWithSharesAndReceiverAndMaxLossCall) -> Self {
            Self::RedeemWithSharesAndReceiverAndMaxLoss(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for AaveV3StrategyCalls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for AaveV3StrategyCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<ReportCall> for AaveV3StrategyCalls {
        fn from(value: ReportCall) -> Self {
            Self::Report(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for AaveV3StrategyCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetKeeperCall> for AaveV3StrategyCalls {
        fn from(value: SetKeeperCall) -> Self {
            Self::SetKeeper(value)
        }
    }
    impl ::core::convert::From<SetManagementCall> for AaveV3StrategyCalls {
        fn from(value: SetManagementCall) -> Self {
            Self::SetManagement(value)
        }
    }
    impl ::core::convert::From<SetProfitMaxUnlockTimeCall> for AaveV3StrategyCalls {
        fn from(value: SetProfitMaxUnlockTimeCall) -> Self {
            Self::SetProfitMaxUnlockTime(value)
        }
    }
    impl ::core::convert::From<ShutdownCall> for AaveV3StrategyCalls {
        fn from(value: ShutdownCall) -> Self {
            Self::Shutdown(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for AaveV3StrategyCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for AaveV3StrategyCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalAssetsCall> for AaveV3StrategyCalls {
        fn from(value: TotalAssetsCall) -> Self {
            Self::TotalAssets(value)
        }
    }
    impl ::core::convert::From<TotalDebtCall> for AaveV3StrategyCalls {
        fn from(value: TotalDebtCall) -> Self {
            Self::TotalDebt(value)
        }
    }
    impl ::core::convert::From<TotalIdleCall> for AaveV3StrategyCalls {
        fn from(value: TotalIdleCall) -> Self {
            Self::TotalIdle(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for AaveV3StrategyCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for AaveV3StrategyCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for AaveV3StrategyCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<WithdrawWithAssetsAndReceiverAndMaxLossCall>
    for AaveV3StrategyCalls {
        fn from(value: WithdrawWithAssetsAndReceiverAndMaxLossCall) -> Self {
            Self::WithdrawWithAssetsAndReceiverAndMaxLoss(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for AaveV3StrategyCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
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
    ///Container type for all return fields from the `aToken` function with signature `aToken()` and selector `0xa0c1f15e`
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
    pub struct AtokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    pub struct AssetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `convertToAssets` function with signature `convertToAssets(uint256)` and selector `0x07a2d13a`
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
    pub struct ConvertToAssetsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `convertToShares` function with signature `convertToShares(uint256)` and selector `0xc6e6f592`
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
    pub struct ConvertToSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
    pub struct DecreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `deposit` function with signature `deposit(uint256,address)` and selector `0x6e553f65`
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
    pub struct DepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `fullProfitUnlockDate` function with signature `fullProfitUnlockDate()` and selector `0x2d632692`
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
    pub struct FullProfitUnlockDateReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
    pub struct IncreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `lastReport` function with signature `lastReport()` and selector `0xc3535b52`
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
    pub struct LastReportReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lendingPool` function with signature `lendingPool()` and selector `0xa59a9973`
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
    pub struct LendingPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `maxDeposit` function with signature `maxDeposit(address)` and selector `0x402d267d`
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
    pub struct MaxDepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxMint` function with signature `maxMint(address)` and selector `0xc63d75b6`
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
    pub struct MaxMintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxRedeem` function with signature `maxRedeem(address)` and selector `0xd905777e`
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
    pub struct MaxRedeemReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxWithdraw` function with signature `maxWithdraw(address)` and selector `0xce96cb77`
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
    pub struct MaxWithdrawReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mint` function with signature `mint(uint256,address)` and selector `0x94bf804d`
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
    pub struct MintReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `previewDeposit` function with signature `previewDeposit(uint256)` and selector `0xef8b30f7`
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
    pub struct PreviewDepositReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `previewMint` function with signature `previewMint(uint256)` and selector `0xb3d7f6b9`
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
    pub struct PreviewMintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `previewRedeem` function with signature `previewRedeem(uint256)` and selector `0x4cdad506`
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
    pub struct PreviewRedeemReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `previewWithdraw` function with signature `previewWithdraw(uint256)` and selector `0x0a28a477`
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
    pub struct PreviewWithdrawReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pricePerShare` function with signature `pricePerShare()` and selector `0x99530b06`
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
    pub struct PricePerShareReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `profitMaxUnlockTime` function with signature `profitMaxUnlockTime()` and selector `0x0952864e`
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
    pub struct ProfitMaxUnlockTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `profitUnlockingRate` function with signature `profitUnlockingRate()` and selector `0x5141eebb`
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
    pub struct ProfitUnlockingRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `redeem` function with signature `redeem(uint256,address,address,uint256)` and selector `0x9f40a7b3`
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
    pub struct RedeemWithSharesAndReceiverAndMaxLossReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `redeem` function with signature `redeem(uint256,address,address)` and selector `0xba087652`
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
    pub struct RedeemReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `report` function with signature `report()` and selector `0x2606a10b`
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
    pub struct ReportReturn {
        pub profit: ::ethers::core::types::U256,
        pub loss: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalAssets` function with signature `totalAssets()` and selector `0x01e1d114`
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
    pub struct TotalAssetsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalDebt` function with signature `totalDebt()` and selector `0xfc7b9c18`
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
    pub struct TotalDebtReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalIdle` function with signature `totalIdle()` and selector `0x9aa7df94`
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
    pub struct TotalIdleReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
    ///Container type for all return fields from the `withdraw` function with signature `withdraw(uint256,address,address,uint256)` and selector `0xa318c1a4`
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
    pub struct WithdrawWithAssetsAndReceiverAndMaxLossReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `withdraw` function with signature `withdraw(uint256,address,address)` and selector `0xb460af94`
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
    pub struct WithdrawReturn(pub ::ethers::core::types::U256);
}
