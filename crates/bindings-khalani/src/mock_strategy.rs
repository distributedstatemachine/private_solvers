pub use mock_strategy::*;
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
pub mod mock_strategy {
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
                        name: ::std::borrow::ToOwned::to_owned("_yieldSource"),
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
                    ::std::borrow::ToOwned::to_owned("kept"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("kept"),
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
                    ::std::borrow::ToOwned::to_owned("managed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("managed"),
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
                    ::std::borrow::ToOwned::to_owned("trigger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trigger"),
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
                (
                    ::std::borrow::ToOwned::to_owned("yieldSource"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("yieldSource"),
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
    pub static MOCKSTRATEGY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\x002\x8B8\x03\x80b\x002\x8B\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x03\xC3V[`@\x80Q\x80\x82\x01\x82R`\r\x81RlTest Strategy`\x98\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x06\x83Re\x15\x14\xD5\x14\x90U`\xD2\x1B\x90\x83\x01R`\x01\x80U\x83\x91\x82\x82\x82`\x05b\0\0\x8C\x83\x82b\0\x04\xA7V[P`\x06b\0\0\x9B\x82\x82b\0\x04\xA7V[PPP`\0\x80b\0\0\xB2\x83b\0\x02\x15` \x1B` \x1CV[\x91P\x91P\x81b\0\0\xC4W`\x12b\0\0\xC6V[\x80[`\xFF\x16`\xA0RPP`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\0\xE7`\x003b\0\x02\xFAV[b\0\x01\x13\x7F\xB4\xD5\xC1a\x80\xAA\x1FB\xA7\x9CC\x01\xE7\xBDt\xB2J\xA5VV<f-\xB1\tW\xE0\xE1\x95\xB5\xBER3b\0\x02\xFAV[b\0\x01?\x7Fq\xA9\x85\x9D}\xD2\x1B$PJo0`w\xFF\xC2\xD5\x10\xB4\xD4\xB6\x11(\xE91\xFE\x93tA\xAD\x1863b\0\x02\xFAV[PPb\t:\x80`\x0CUPB`\x0BU`\rTa\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15b\0\x01iW`\0\x80\xFD[`\r\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x84\x16\x02\x17\x90Ub\0\x01\x93`\x80Q\x90V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x19`$\x83\x01R\x91\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15b\0\x01\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02\x0C\x91\x90b\0\x05sV[PPPb\0\x05\xE9V[`@\x80Q`\x04\x81R`$\x81\x01\x82R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1<\xE5g`\xE0\x1B\x17\x90R\x90Q`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x87\x16\x91b\0\x02^\x91b\0\x05\x9EV[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14b\0\x02\x9BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x02\xA0V[``\x91P[P\x91P\x91P\x81\x80\x15b\0\x02\xB5WP` \x81Q\x10\x15[\x15b\0\x02\xEDW`\0\x81\x80` \x01\x90Q\x81\x01\x90b\0\x02\xD3\x91\x90b\0\x05\xCFV[\x90P`\xFF\x81\x11b\0\x02\xEBW`\x01\x96\x90\x95P\x93PPPPV[P[P`\0\x94\x85\x94P\x92PPPV[b\0\x03\x06\x82\x82b\0\x03\nV[PPV[`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16b\0\x03\x06W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x03f3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x03\xC0W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x03\xD7W`\0\x80\xFD[\x82Qb\0\x03\xE4\x81b\0\x03\xAAV[` \x84\x01Q\x90\x92Pb\0\x03\xF7\x81b\0\x03\xAAV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x04-W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x04NWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x04\xA2W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x04}WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x04\x9EW\x82\x81U`\x01\x01b\0\x04\x89V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x04\xC3Wb\0\x04\xC3b\0\x04\x02V[b\0\x04\xDB\x81b\0\x04\xD4\x84Tb\0\x04\x18V[\x84b\0\x04TV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x05\x13W`\0\x84\x15b\0\x04\xFAWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x04\x9EV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x05DW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x05#V[P\x85\x82\x10\x15b\0\x05cW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15b\0\x05\x86W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x05\x97W`\0\x80\xFD[\x93\x92PPPV[`\0\x82Q`\0[\x81\x81\x10\x15b\0\x05\xC1W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01b\0\x05\xA5V[P`\0\x92\x01\x91\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\0\x05\xE2W`\0\x80\xFD[PQ\x91\x90PV[`\x80Q`\xA0Qa,=b\0\x06N`\09`\0a\n\xE0\x01R`\0\x81\x81a\x04b\x01R\x81\x81a\t\xB9\x01R\x81\x81a\x15\xA9\x01R\x81\x81a\x16\xB1\x01R\x81\x81a\x1B\0\x01R\x81\x81a\x1B=\x01R\x81\x81a\x1B\xD8\x01R\x81\x81a\x1D\xA1\x01R\x81\x81a\x1EF\x01Ra\x1F\xB0\x01Ra,=`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\x0CW`\x005`\xE0\x1C\x80c\x94\xBF\x80M\x11a\x01\x9DW\x80c\xBA\x08vR\x11a\0\xE9W\x80c\xD5Gt\x1F\x11a\0\xA2W\x80c\xDFi\xB2*\x11a\0|W\x80c\xDFi\xB2*\x14a\x06\x8BW\x80c\xEF\x8B0\xF7\x14a\x06\x19W\x80c\xFC\x0Et\xD1\x14a\x06\x9EW\x80c\xFC{\x9C\x18\x14a\x06\xABW`\0\x80\xFD[\x80c\xD5Gt\x1F\x14a\x06RW\x80c\xD9\x05w~\x14a\x06eW\x80c\xDDb\xED>\x14a\x06xW`\0\x80\xFD[\x80c\xBA\x08vR\x14a\x05\xFDW\x80c\xC3S[R\x14a\x06\x10W\x80c\xC6=u\xB6\x14a\x04\xADW\x80c\xC6\xE6\xF5\x92\x14a\x06\x19W\x80c\xCE\x96\xCBw\x14a\x06,W\x80c\xD4\xA2+\xDE\x14a\x06?W`\0\x80\xFD[\x80c\xA3\x18\xC1\xA4\x11a\x01VW\x80c\xAC\xC3\xE8O\x11a\x010W\x80c\xAC\xC3\xE8O\x14a\x05\xABW\x80c\xB2G\x0E\\\x14a\x05\xBFW\x80c\xB3\xD7\xF6\xB9\x14a\x05\xD7W\x80c\xB4`\xAF\x94\x14a\x05\xEAW`\0\x80\xFD[\x80c\xA3\x18\xC1\xA4\x14a\x05rW\x80c\xA4W\xC2\xD7\x14a\x05\x85W\x80c\xA9\x05\x9C\xBB\x14a\x05\x98W`\0\x80\xFD[\x80c\x94\xBF\x80M\x14a\x05+W\x80c\x95\xD8\x9BA\x14a\x05>W\x80c\x99S\x0B\x06\x14a\x05FW\x80c\x9A\xA7\xDF\x94\x14a\x05NW\x80c\x9F@\xA7\xB3\x14a\x05WW\x80c\xA2\x17\xFD\xDF\x14a\x05jW`\0\x80\xFD[\x80c/\xC6\\6\x11a\x02\\W\x80cL\xDA\xD5\x06\x11a\x02\x15W\x80cp\xA0\x821\x11a\x01\xEFW\x80cp\xA0\x821\x14a\x04\xDEW\x80ct\x87G\xE6\x14a\x04\xF1W\x80c\x7F\xEC\x8D8\x14a\x05\x04W\x80c\x91\xD1HT\x14a\x05\x18W`\0\x80\xFD[\x80cL\xDA\xD5\x06\x14a\x03dW\x80cQA\xEE\xBB\x14a\x04\xC2W\x80cnU?e\x14a\x04\xCBW`\0\x80\xFD[\x80c/\xC6\\6\x14a\x04\x1FW\x80c1<\xE5g\x14a\x043W\x80c6V\x8A\xBE\x14a\x04MW\x80c8\xD5.\x0F\x14a\x04`W\x80c9P\x93Q\x14a\x04\x9AW\x80c@-&}\x14a\x04\xADW`\0\x80\xFD[\x80c\n(\xA4w\x11a\x02\xC9W\x80c$\x8A\x9C\xA3\x11a\x02\xA3W\x80c$\x8A\x9C\xA3\x14a\x03\xC1W\x80c&\x06\xA1\x0B\x14a\x03\xE4W\x80c-c&\x92\x14a\x04\x01W\x80c//\xF1]\x14a\x04\nW`\0\x80\xFD[\x80c\n(\xA4w\x14a\x03\x93W\x80c\x18\x16\r\xDD\x14a\x03\xA6W\x80c#\xB8r\xDD\x14a\x03\xAEW`\0\x80\xFD[\x80c\x01\xE1\xD1\x14\x14a\x03\x11W\x80c\x01\xFF\xC9\xA7\x14a\x03,W\x80c\x06\xFD\xDE\x03\x14a\x03OW\x80c\x07\xA2\xD1:\x14a\x03dW\x80c\tR\x86N\x14a\x03wW\x80c\t^\xA7\xB3\x14a\x03\x80W[`\0\x80\xFD[a\x03\x19a\x06\xB4V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03?a\x03:6`\x04a&\xC0V[a\x06\xCBV[`@Q\x90\x15\x15\x81R` \x01a\x03#V[a\x03Wa\x07\x02V[`@Qa\x03#\x91\x90a'\x0EV[a\x03\x19a\x03r6`\x04a'AV[a\x07\x94V[a\x03\x19`\x0CT\x81V[a\x03?a\x03\x8E6`\x04a'qV[a\x07\xA1V[a\x03\x19a\x03\xA16`\x04a'AV[a\x07\xB9V[a\x03\x19a\x07\xC6V[a\x03?a\x03\xBC6`\x04a'\x9BV[a\x07\xDDV[a\x03\x19a\x03\xCF6`\x04a'AV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[a\x03\xECa\x08\x03V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03#V[a\x03\x19`\nT\x81V[a\x04\x1Da\x04\x186`\x04a'\xD7V[a\n\xAEV[\0[`\rTa\x03?\x90`\x01`\xB0\x1B\x90\x04`\xFF\x16\x81V[a\x04;a\n\xD8V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03#V[a\x04\x1Da\x04[6`\x04a'\xD7V[a\x0B\x04V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03#V[a\x03?a\x04\xA86`\x04a'qV[a\x0B\x82V[a\x03\x19a\x04\xBB6`\x04a(\x03V[P`\0\x19\x90V[a\x03\x19`\tT\x81V[a\x03\x19a\x04\xD96`\x04a'\xD7V[a\x0B\xA4V[a\x03\x19a\x04\xEC6`\x04a(\x03V[a\x0B\xBEV[a\x04\x1Da\x04\xFF6`\x04a(\x03V[a\x0C\x19V[`\rTa\x03?\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[a\x03?a\x05&6`\x04a'\xD7V[a\x0CzV[a\x03\x19a\x0596`\x04a'\xD7V[a\x0C\xA3V[a\x03Wa\x0C\xBDV[a\x03\x19a\x0C\xCCV[a\x03\x19`\x07T\x81V[a\x03\x19a\x05e6`\x04a(\x1EV[a\x0C\xE4V[a\x03\x19`\0\x81V[a\x03\x19a\x05\x806`\x04a(\x1EV[a\reV[a\x03?a\x05\x936`\x04a'qV[a\r\xDAV[a\x03?a\x05\xA66`\x04a'qV[a\x0E`V[`\rTa\x03?\x90`\x01`\xB8\x1B\x90\x04`\xFF\x16\x81V[`\rTa\x04\x82\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x19a\x05\xE56`\x04a'AV[a\x0EnV[a\x03\x19a\x05\xF86`\x04a(bV[a\x0E{V[a\x03\x19a\x06\x0B6`\x04a(bV[a\x0E\xFAV[a\x03\x19`\x0BT\x81V[a\x03\x19a\x06'6`\x04a'AV[a\x0FqV[a\x03\x19a\x06:6`\x04a(\x03V[a\x0F~V[a\x04\x1Da\x06M6`\x04a(\x03V[a\x0F\x93V[a\x04\x1Da\x06`6`\x04a'\xD7V[a\x0F\xDFV[a\x03\x19a\x06s6`\x04a(\x03V[a\x10\x04V[a\x03\x19a\x06\x866`\x04a(\x9EV[a\x10\x0FV[a\x04\x1Da\x06\x996`\x04a'AV[a\x10:V[`\rTa\x03?\x90`\xFF\x16\x81V[a\x03\x19`\x08T\x81V[`\0`\x08T`\x07Ta\x06\xC6\x91\x90a(\xDEV[\x90P\x90V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06\xFCWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[```\x05\x80Ta\x07\x11\x90a(\xF1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07=\x90a(\xF1V[\x80\x15a\x07\x8AW\x80`\x1F\x10a\x07_Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x8AV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07mW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x06\xFC\x82`\0a\x11$V[`\x003a\x07\xAF\x81\x85\x85a\x11XV[P`\x01\x93\x92PPPV[`\0a\x06\xFC\x82`\x01a\x12|V[`\0a\x07\xD0a\x12\xCAV[`\x04Ta\x06\xC6\x91\x90a)+V[`\x003a\x07\xEB\x85\x82\x85a\x13&V[a\x07\xF6\x85\x85\x85a\x13\xA0V[`\x01\x91PP[\x93\x92PPPV[`\0\x80a\x080\x7Fq\xA9\x85\x9D}\xD2\x1B$PJo0`w\xFF\xC2\xD5\x10\xB4\xD4\xB6\x11(\xE91\xFE\x93tA\xAD\x1863a\x0CzV[a\x08oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj7\xB76<\x905\xB2\xB2\xB82\xB9`\xA9\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x08wa\x15KV[`\0a\x08\x81a\x06\xB4V[\x90P`\0a\x08\x8Da\x15\xA4V[\x90Pa\x08\x97a\x17\xC1V[`\0\x82\x82\x11\x15a\x08\xC7Wa\x08\xAB\x83\x83a)+V[\x94Pa\x08\xB6\x85a\x0FqV[\x90Pa\x08\xC20\x82a\x17\xF5V[a\t\tV[a\x08\xD1\x82\x84a)+V[\x93P\x83\x15a\t\tW`\0a\x08\xF5a\x08\xE7\x86a\x0FqV[a\x08\xF00a\x0B\xBEV[a\x18\xB6V[\x90P\x80\x15a\t\x07Wa\t\x070\x82a\x18\xCCV[P[`\0a\t\x140a\x0B\xBEV[\x90P\x80\x15a\t\xAFW`\0B`\nT\x11\x15a\tMWa\t2\x83\x83a)+V[B`\nTa\t@\x91\x90a)+V[a\tJ\x91\x90a)>V[\x90P[`\0\x82`\x0CT\x85a\t^\x91\x90a)>V[a\th\x90\x84a(\xDEV[a\tr\x91\x90a)kV[\x90P\x80a\t\x84d\xE8\xD4\xA5\x10\0\x85a)>V[a\t\x8E\x91\x90a)kV[`\tUa\t\x9B\x81Ba(\xDEV[`\x01`\x01`\x80\x1B\x03\x16`\nUPa\t\xB5\x90PV[`\0`\tU[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nB\x91\x90a)\x8DV[`\x07\x81\x90U\x90Pa\nS\x81\x85a)+V[`\x08UB`\x01`\x01`\x80\x1B\x03\x16`\x0BU`@\x80Q\x88\x81R` \x81\x01\x88\x90R\x7Fo\xEA6\xBF\xB4\xB7\xC9\x12a7V\x95\x1F\xB5\xBF\x8D\xDA\xED\xB7M\xE1\xF2V\xD0\xBE\xF8\x153\x15\x8C\xDAT\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPa\n\xAA`\x01\x80UV[\x90\x91V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\n\xC9\x81a\x1A\0V[a\n\xD3\x83\x83a\x1A\nV[PPPV[`\0a\x06\xC6\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a)\xA6V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0BtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x08fV[a\x0B~\x82\x82a\x1A\x8EV[PPV[`\x003a\x07\xAF\x81\x85\x85a\x0B\x95\x83\x83a\x10\x0FV[a\x0B\x9F\x91\x90a(\xDEV[a\x11XV[`\0\x80a\x0B\xB0\x84a\x0FqV[\x90Pa\x07\xFC3\x84\x86\x84a\x1A\xF3V[`\x000`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\x0B\xFBWa\x0B\xD8a\x12\xCAV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90 Ta\x06\xFC\x91\x90a)+V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02` R`@\x90 Ta\x06\xFCV[a\x0C1`\0\x80Q` a+\xE8\x839\x81Q\x91R3a\x0CzV[a\x0CMW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08f\x90a)\xBFV[a\x0Cw\x7Fq\xA9\x85\x9D}\xD2\x1B$PJo0`w\xFF\xC2\xD5\x10\xB4\xD4\xB6\x11(\xE91\xFE\x93tA\xAD\x186\x82a\x1D\x04V[PV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x80a\x0C\xAF\x84a\x0EnV[\x90Pa\x07\xFC3\x84\x83\x87a\x1A\xF3V[```\x06\x80Ta\x07\x11\x90a(\xF1V[`\0a\x06\xC6a\x0C\xD9a\n\xD8V[a\x03r\x90`\na*\xCCV[`\0a\x0C\xEF\x83a\x10\x04V[\x85\x11\x15a\r>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC4626: redeem more than max\0\0\0`D\x82\x01R`d\x01a\x08fV[`\0a\rI\x86a\x07\x94V[\x90Pa\rY3\x86\x86\x84\x8A\x88a\x1D\x0EV[P\x90P[\x94\x93PPPPV[`\0a\rp\x83a\x0F~V[\x85\x11\x15a\r\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC4626: withdraw more than max\0`D\x82\x01R`d\x01a\x08fV[`\0a\r\xCA\x86a\x07\xB9V[\x90Pa\rY3\x86\x86\x89\x85\x88a\x1D\x0EV[`\x003\x81a\r\xE8\x82\x86a\x10\x0FV[\x90P\x83\x81\x10\x15a\x0EHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x08fV[a\x0EU\x82\x86\x86\x84\x03a\x11XV[P`\x01\x94\x93PPPPV[`\x003a\x07\xAF\x81\x85\x85a\x13\xA0V[`\0a\x06\xFC\x82`\x01a\x11$V[`\0a\x0E\x86\x82a\x0F~V[\x84\x11\x15a\x0E\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC4626: withdraw more than max\0`D\x82\x01R`d\x01a\x08fV[`\0a\x0E\xE0\x85a\x07\xB9V[\x90Pa\x0E\xF13\x85\x85\x88\x85`\0a\x1D\x0EV[P\x94\x93PPPPV[`\0a\x0F\x05\x82a\x10\x04V[\x84\x11\x15a\x0FTW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC4626: redeem more than max\0\0\0`D\x82\x01R`d\x01a\x08fV[`\0a\x0F_\x85a\x07\x94V[\x90Pa\x0E\xF13\x85\x85\x84\x89a'\x10a\x1D\x0EV[`\0a\x06\xFC\x82`\0a\x12|V[`\0a\x06\xFCa\x0F\x8C\x83a\x0B\xBEV[`\0a\x11$V[a\x0F\xAB`\0\x80Q` a+\xE8\x839\x81Q\x91R3a\x0CzV[a\x0F\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08f\x90a)\xBFV[a\x0Cw`\0\x80Q` a+\xE8\x839\x81Q\x91R\x82a\x1D\x04V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0F\xFA\x81a\x1A\0V[a\n\xD3\x83\x83a\x1A\x8EV[`\0a\x06\xFC\x82a\x0B\xBEV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x10R`\0\x80Q` a+\xE8\x839\x81Q\x91R3a\x0CzV[a\x10nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08f\x90a)\xBFV[\x80`\0\x03a\x10\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`\xBA\x1B`D\x82\x01R`d\x01a\x08fV[c\x01\xE1\x85X\x81\x11\x15a\x10\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01Rgtoo long`\xC0\x1B`D\x82\x01R`d\x01a\x08fV[`\x0C\x81\x90U`@Q\x81\x81R\x7F\xF3a\xAE\xD4c\xDAo\xA2\x03X\xE4\\b\t\xF1\xD3\xE1mN\xCApnn\xAB\x0B\n\xEB3\x87)\xC7z\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80a\x11/a\x07\xC6V[\x90P\x80\x15a\x11PWa\x11Ka\x11Ba\x06\xB4V[\x85\x90\x83\x86a FV[a\r]V[P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x11\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x12\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0\x80a\x12\x87a\x06\xB4V[\x90P`\0a\x12\x93a\x07\xC6V[\x90P\x81`\0\x03a\x12\xB5W\x80\x15a\x12\xAAW`\0a\x12\xACV[\x84[\x92PPPa\x06\xFCV[a\x12\xC1\x85\x82\x84\x87a FV[\x95\x94PPPPPV[`\0\x80B`\nT\x11\x15a\x13\x07W`\0`\x0BTBa\x12\xE7\x91\x90a)+V[\x90Pd\xE8\xD4\xA5\x10\0`\tT\x82a\x12\xFD\x91\x90a)>V[a\x07\xFC\x91\x90a)kV[`\nT\x15a\x13!WP0`\0\x90\x81R`\x02` R`@\x90 T[\x91\x90PV[`\0a\x132\x84\x84a\x10\x0FV[\x90P`\0\x19\x81\x14a\x13\x9AW\x81\x81\x10\x15a\x13\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x08fV[a\x13\x9A\x84\x84\x84\x84\x03a\x11XV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x14\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a\x14\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x15>\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x13\x9AV[`\x02`\x01T\x03a\x15\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x08fV[`\x02`\x01UV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x162\x91\x90a)\x8DV[\x90P`\0\x81\x11\x80\x15a\x16GWP`\rT`\xFF\x16\x15[\x15a\x16\xAFW`\rT`@Qc\xB6\xB5_%`\xE0\x1B\x81R`\x04\x81\x01\x83\x90Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB6\xB5_%\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x96W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xAAW=`\0\x80>=`\0\xFD[PPPP[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x16W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17:\x91\x90a)\x8DV[`\r`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xB6\x9E\xF8\xA8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xB1\x91\x90a)\x8DV[a\x17\xBB\x91\x90a(\xDEV[\x91PP\x90V[`\0a\x17\xCBa\x12\xCAV[\x90P\x80\x15a\x0CwWB`\nT\x11\x15a\x17\xEBW`\x01`\x01`\x80\x1B\x03B\x16`\x0BU[a\x0Cw0\x82a\x18\xCCV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x18KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x08fV[\x80`\x04`\0\x82\x82Ta\x18]\x91\x90a(\xDEV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\0\x81\x83\x10a\x18\xC5W\x81a\x07\xFCV[P\x90\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x19,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a\x19\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`\x04\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x0Cw\x813a \xA1V[a\x1A\x14\x82\x82a\x0CzV[a\x0B~W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1AJ3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x1A\x98\x82\x82a\x0CzV[\x15a\x0B~W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[a\x1A\xFBa\x15KV[a\x1B'\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x850\x85a \xFAV[`\0\x82`\x07Ta\x1B7\x91\x90a(\xDEV[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xC6\x91\x90a)\x8DV[\x90Pa\x1B\xD1\x82a!eV[`\0a\x1Cq\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ca\x91\x90a)\x8DV[a\x1Ck\x90\x84a)+V[\x84a\x18\xB6V[\x90P\x80`\x08`\0\x82\x82Ta\x1C\x85\x91\x90a(\xDEV[\x90\x91UPa\x1C\x95\x90P\x81\x84a)+V[`\x07Ua\x1C\xA2\x86\x85a\x17\xF5V[\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x87\x87`@Qa\x1C\xF0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPa\x13\x9A`\x01\x80UV[a\x0B~\x82\x82a\x1A\nV[`\0a\x1D\x18a\x15KV[`\x01`\x01`\xA0\x1B\x03\x86\x16a\x1DnW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReceiver cannot be zero address\0`D\x82\x01R`d\x01a\x08fV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\x92Wa\x1D\x92\x85\x88\x85a\x13&V[`\x07T\x84\x81\x10\x15a\x1F\x94W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E*\x91\x90a)\x8DV[\x90Pa\x1E>a\x1E9\x83\x88a)+V[a!\xCBV[`\0a\x1E\xE1\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xCF\x91\x90a)\x8DV[a\x1E\xD9\x91\x90a)+V[`\x08Ta\x18\xB6V[\x90Pa\x1E\xED\x81\x84a(\xDEV[\x92P`\0\x87\x84\x10\x15a\x1FpWa\x1F\x03\x84\x89a)+V[\x90Pa'\x10\x86\x10\x15a\x1FlWa'\x10a\x1F\x1C\x87\x8Aa)>V[a\x1F&\x91\x90a)kV[\x81\x11\x15a\x1FlW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsLoss exceeds maxLoss``\x1B`D\x82\x01R`d\x01a\x08fV[\x83\x97P[a\x1Fz\x81\x83a(\xDEV[`\x08`\0\x82\x82Ta\x1F\x8B\x91\x90a)+V[\x90\x91UPPPPP[a\x1F\x9E\x85\x82a)+V[`\x07Ua\x1F\xAB\x86\x85a\x18\xCCV[a\x1F\xD6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x87a\"\0V[\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x88\x88`@Qa .\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UP\x90\x94\x93PPPPV[`\0\x80a T\x86\x86\x86a\"0V[\x90P`\x01\x83`\x02\x81\x11\x15a jWa ja*\xDBV[\x14\x80\x15a \x87WP`\0\x84\x80a \x82Wa \x82a)UV[\x86\x88\t\x11[\x15a\x12\xC1Wa \x97`\x01\x82a(\xDEV[\x96\x95PPPPPPV[a \xAB\x82\x82a\x0CzV[a\x0B~Wa \xB8\x81a#\x1AV[a \xC3\x83` a#,V[`@Q` \x01a \xD4\x92\x91\x90a*\xF1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x08f\x91`\x04\x01a'\x0EV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x13\x9A\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra$\xC8V[`\rT`@Qc\xB6\xB5_%`\xE0\x1B\x81R`\x04\x81\x01\x83\x90Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB6\xB5_%\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xB0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\xC4W=`\0\x80>=`\0\xFD[PPPPPV[`\rT`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x83\x90Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c.\x1A}M\x90`$\x01a!\x96V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\n\xD3\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a!.V[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\"jW\x83\x82\x81a\"`Wa\"`a)UV[\x04\x92PPPa\x07\xFCV[\x80\x84\x11a\"\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\x08fV[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[``a\x06\xFC`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a#;\x83`\x02a)>V[a#F\x90`\x02a(\xDEV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#^Wa#^a+fV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#\x88W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a#\xA3Wa#\xA3a+|V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a#\xD2Wa#\xD2a+|V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a#\xF6\x84`\x02a)>V[a$\x01\x90`\x01a(\xDEV[\x90P[`\x01\x81\x11\x15a$yWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a$5Wa$5a+|V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a$KWa$Ka+|V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a$r\x81a+\x92V[\x90Pa$\x04V[P\x83\x15a\x07\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x08fV[`\0a%\x1D\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a%\x9D\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a%>WP\x80\x80` \x01\x90Q\x81\x01\x90a%>\x91\x90a+\xA9V[a\n\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x08fV[``a\r]\x84\x84`\0\x85\x85`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa%\xC4\x91\x90a+\xCBV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a&\x01W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a&\x06V[``\x91P[P\x91P\x91Pa&\x17\x87\x83\x83\x87a&\"V[\x97\x96PPPPPPPV[``\x83\x15a&\x91W\x82Q`\0\x03a&\x8AW`\x01`\x01`\xA0\x1B\x03\x85\x16;a&\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x08fV[P\x81a\r]V[a\r]\x83\x83\x81Q\x15a&\xA6W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08f\x91\x90a'\x0EV[`\0` \x82\x84\x03\x12\x15a&\xD2W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\xFCW`\0\x80\xFD[`\0[\x83\x81\x10\x15a'\x05W\x81\x81\x01Q\x83\x82\x01R` \x01a&\xEDV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra'-\x81`@\x85\x01` \x87\x01a&\xEAV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'SW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13!W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a'\x84W`\0\x80\xFD[a'\x8D\x83a'ZV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\xB0W`\0\x80\xFD[a'\xB9\x84a'ZV[\x92Pa'\xC7` \x85\x01a'ZV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a'\xEAW`\0\x80\xFD[\x825\x91Pa'\xFA` \x84\x01a'ZV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a(\x15W`\0\x80\xFD[a\x07\xFC\x82a'ZV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a(4W`\0\x80\xFD[\x845\x93Pa(D` \x86\x01a'ZV[\x92Pa(R`@\x86\x01a'ZV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a(wW`\0\x80\xFD[\x835\x92Pa(\x87` \x85\x01a'ZV[\x91Pa(\x95`@\x85\x01a'ZV[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a(\xB1W`\0\x80\xFD[a(\xBA\x83a'ZV[\x91Pa'\xFA` \x84\x01a'ZV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xFCWa\x06\xFCa(\xC8V[`\x01\x81\x81\x1C\x90\x82\x16\x80a)\x05W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a)%WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x06\xFCWa\x06\xFCa(\xC8V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xFCWa\x06\xFCa(\xC8V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)\x88WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a)\x9FW`\0\x80\xFD[PQ\x91\x90PV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x06\xFCWa\x06\xFCa(\xC8V[` \x80\x82R`\x0F\x90\x82\x01Rn\x1B\xDB\x9B\x1EH\x1BX[\x98Y\xD9[Y[\x9D`\x8A\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81[\x80\x85\x11\x15a*#W\x81`\0\x19\x04\x82\x11\x15a*\tWa*\ta(\xC8V[\x80\x85\x16\x15a*\x16W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a)\xEDV[P\x92P\x92\x90PV[`\0\x82a*:WP`\x01a\x06\xFCV[\x81a*GWP`\0a\x06\xFCV[\x81`\x01\x81\x14a*]W`\x02\x81\x14a*gWa*\x83V[`\x01\x91PPa\x06\xFCV[`\xFF\x84\x11\x15a*xWa*xa(\xC8V[PP`\x01\x82\x1Ba\x06\xFCV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a*\xA6WP\x81\x81\na\x06\xFCV[a*\xB0\x83\x83a)\xE8V[\x80`\0\x19\x04\x82\x11\x15a*\xC4Wa*\xC4a(\xC8V[\x02\x93\x92PPPV[`\0a\x07\xFC`\xFF\x84\x16\x83a*+V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa+)\x81`\x17\x85\x01` \x88\x01a&\xEAV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa+Z\x81`(\x84\x01` \x88\x01a&\xEAV[\x01`(\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a+\xA1Wa+\xA1a(\xC8V[P`\0\x19\x01\x90V[`\0` \x82\x84\x03\x12\x15a+\xBBW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xFCW`\0\x80\xFD[`\0\x82Qa+\xDD\x81\x84` \x87\x01a&\xEAV[\x91\x90\x91\x01\x92\x91PPV\xFE\xB4\xD5\xC1a\x80\xAA\x1FB\xA7\x9CC\x01\xE7\xBDt\xB2J\xA5VV<f-\xB1\tW\xE0\xE1\x95\xB5\xBER\xA2dipfsX\"\x12 X\x12\xECR\xB0\n\xCCM\x80\x9A\xF9j\x89S\xAD\x86\x1D\xE8\x85:\xFB\x83R {\xD71\x1F\xEF\xCC\xBB\x80dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKSTRATEGY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\x0CW`\x005`\xE0\x1C\x80c\x94\xBF\x80M\x11a\x01\x9DW\x80c\xBA\x08vR\x11a\0\xE9W\x80c\xD5Gt\x1F\x11a\0\xA2W\x80c\xDFi\xB2*\x11a\0|W\x80c\xDFi\xB2*\x14a\x06\x8BW\x80c\xEF\x8B0\xF7\x14a\x06\x19W\x80c\xFC\x0Et\xD1\x14a\x06\x9EW\x80c\xFC{\x9C\x18\x14a\x06\xABW`\0\x80\xFD[\x80c\xD5Gt\x1F\x14a\x06RW\x80c\xD9\x05w~\x14a\x06eW\x80c\xDDb\xED>\x14a\x06xW`\0\x80\xFD[\x80c\xBA\x08vR\x14a\x05\xFDW\x80c\xC3S[R\x14a\x06\x10W\x80c\xC6=u\xB6\x14a\x04\xADW\x80c\xC6\xE6\xF5\x92\x14a\x06\x19W\x80c\xCE\x96\xCBw\x14a\x06,W\x80c\xD4\xA2+\xDE\x14a\x06?W`\0\x80\xFD[\x80c\xA3\x18\xC1\xA4\x11a\x01VW\x80c\xAC\xC3\xE8O\x11a\x010W\x80c\xAC\xC3\xE8O\x14a\x05\xABW\x80c\xB2G\x0E\\\x14a\x05\xBFW\x80c\xB3\xD7\xF6\xB9\x14a\x05\xD7W\x80c\xB4`\xAF\x94\x14a\x05\xEAW`\0\x80\xFD[\x80c\xA3\x18\xC1\xA4\x14a\x05rW\x80c\xA4W\xC2\xD7\x14a\x05\x85W\x80c\xA9\x05\x9C\xBB\x14a\x05\x98W`\0\x80\xFD[\x80c\x94\xBF\x80M\x14a\x05+W\x80c\x95\xD8\x9BA\x14a\x05>W\x80c\x99S\x0B\x06\x14a\x05FW\x80c\x9A\xA7\xDF\x94\x14a\x05NW\x80c\x9F@\xA7\xB3\x14a\x05WW\x80c\xA2\x17\xFD\xDF\x14a\x05jW`\0\x80\xFD[\x80c/\xC6\\6\x11a\x02\\W\x80cL\xDA\xD5\x06\x11a\x02\x15W\x80cp\xA0\x821\x11a\x01\xEFW\x80cp\xA0\x821\x14a\x04\xDEW\x80ct\x87G\xE6\x14a\x04\xF1W\x80c\x7F\xEC\x8D8\x14a\x05\x04W\x80c\x91\xD1HT\x14a\x05\x18W`\0\x80\xFD[\x80cL\xDA\xD5\x06\x14a\x03dW\x80cQA\xEE\xBB\x14a\x04\xC2W\x80cnU?e\x14a\x04\xCBW`\0\x80\xFD[\x80c/\xC6\\6\x14a\x04\x1FW\x80c1<\xE5g\x14a\x043W\x80c6V\x8A\xBE\x14a\x04MW\x80c8\xD5.\x0F\x14a\x04`W\x80c9P\x93Q\x14a\x04\x9AW\x80c@-&}\x14a\x04\xADW`\0\x80\xFD[\x80c\n(\xA4w\x11a\x02\xC9W\x80c$\x8A\x9C\xA3\x11a\x02\xA3W\x80c$\x8A\x9C\xA3\x14a\x03\xC1W\x80c&\x06\xA1\x0B\x14a\x03\xE4W\x80c-c&\x92\x14a\x04\x01W\x80c//\xF1]\x14a\x04\nW`\0\x80\xFD[\x80c\n(\xA4w\x14a\x03\x93W\x80c\x18\x16\r\xDD\x14a\x03\xA6W\x80c#\xB8r\xDD\x14a\x03\xAEW`\0\x80\xFD[\x80c\x01\xE1\xD1\x14\x14a\x03\x11W\x80c\x01\xFF\xC9\xA7\x14a\x03,W\x80c\x06\xFD\xDE\x03\x14a\x03OW\x80c\x07\xA2\xD1:\x14a\x03dW\x80c\tR\x86N\x14a\x03wW\x80c\t^\xA7\xB3\x14a\x03\x80W[`\0\x80\xFD[a\x03\x19a\x06\xB4V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03?a\x03:6`\x04a&\xC0V[a\x06\xCBV[`@Q\x90\x15\x15\x81R` \x01a\x03#V[a\x03Wa\x07\x02V[`@Qa\x03#\x91\x90a'\x0EV[a\x03\x19a\x03r6`\x04a'AV[a\x07\x94V[a\x03\x19`\x0CT\x81V[a\x03?a\x03\x8E6`\x04a'qV[a\x07\xA1V[a\x03\x19a\x03\xA16`\x04a'AV[a\x07\xB9V[a\x03\x19a\x07\xC6V[a\x03?a\x03\xBC6`\x04a'\x9BV[a\x07\xDDV[a\x03\x19a\x03\xCF6`\x04a'AV[`\0\x90\x81R` \x81\x90R`@\x90 `\x01\x01T\x90V[a\x03\xECa\x08\x03V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x03#V[a\x03\x19`\nT\x81V[a\x04\x1Da\x04\x186`\x04a'\xD7V[a\n\xAEV[\0[`\rTa\x03?\x90`\x01`\xB0\x1B\x90\x04`\xFF\x16\x81V[a\x04;a\n\xD8V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03#V[a\x04\x1Da\x04[6`\x04a'\xD7V[a\x0B\x04V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03#V[a\x03?a\x04\xA86`\x04a'qV[a\x0B\x82V[a\x03\x19a\x04\xBB6`\x04a(\x03V[P`\0\x19\x90V[a\x03\x19`\tT\x81V[a\x03\x19a\x04\xD96`\x04a'\xD7V[a\x0B\xA4V[a\x03\x19a\x04\xEC6`\x04a(\x03V[a\x0B\xBEV[a\x04\x1Da\x04\xFF6`\x04a(\x03V[a\x0C\x19V[`\rTa\x03?\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[a\x03?a\x05&6`\x04a'\xD7V[a\x0CzV[a\x03\x19a\x0596`\x04a'\xD7V[a\x0C\xA3V[a\x03Wa\x0C\xBDV[a\x03\x19a\x0C\xCCV[a\x03\x19`\x07T\x81V[a\x03\x19a\x05e6`\x04a(\x1EV[a\x0C\xE4V[a\x03\x19`\0\x81V[a\x03\x19a\x05\x806`\x04a(\x1EV[a\reV[a\x03?a\x05\x936`\x04a'qV[a\r\xDAV[a\x03?a\x05\xA66`\x04a'qV[a\x0E`V[`\rTa\x03?\x90`\x01`\xB8\x1B\x90\x04`\xFF\x16\x81V[`\rTa\x04\x82\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x19a\x05\xE56`\x04a'AV[a\x0EnV[a\x03\x19a\x05\xF86`\x04a(bV[a\x0E{V[a\x03\x19a\x06\x0B6`\x04a(bV[a\x0E\xFAV[a\x03\x19`\x0BT\x81V[a\x03\x19a\x06'6`\x04a'AV[a\x0FqV[a\x03\x19a\x06:6`\x04a(\x03V[a\x0F~V[a\x04\x1Da\x06M6`\x04a(\x03V[a\x0F\x93V[a\x04\x1Da\x06`6`\x04a'\xD7V[a\x0F\xDFV[a\x03\x19a\x06s6`\x04a(\x03V[a\x10\x04V[a\x03\x19a\x06\x866`\x04a(\x9EV[a\x10\x0FV[a\x04\x1Da\x06\x996`\x04a'AV[a\x10:V[`\rTa\x03?\x90`\xFF\x16\x81V[a\x03\x19`\x08T\x81V[`\0`\x08T`\x07Ta\x06\xC6\x91\x90a(\xDEV[\x90P\x90V[`\0`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x06\xFCWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14[\x92\x91PPV[```\x05\x80Ta\x07\x11\x90a(\xF1V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07=\x90a(\xF1V[\x80\x15a\x07\x8AW\x80`\x1F\x10a\x07_Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x8AV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07mW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x06\xFC\x82`\0a\x11$V[`\x003a\x07\xAF\x81\x85\x85a\x11XV[P`\x01\x93\x92PPPV[`\0a\x06\xFC\x82`\x01a\x12|V[`\0a\x07\xD0a\x12\xCAV[`\x04Ta\x06\xC6\x91\x90a)+V[`\x003a\x07\xEB\x85\x82\x85a\x13&V[a\x07\xF6\x85\x85\x85a\x13\xA0V[`\x01\x91PP[\x93\x92PPPV[`\0\x80a\x080\x7Fq\xA9\x85\x9D}\xD2\x1B$PJo0`w\xFF\xC2\xD5\x10\xB4\xD4\xB6\x11(\xE91\xFE\x93tA\xAD\x1863a\x0CzV[a\x08oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj7\xB76<\x905\xB2\xB2\xB82\xB9`\xA9\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x08wa\x15KV[`\0a\x08\x81a\x06\xB4V[\x90P`\0a\x08\x8Da\x15\xA4V[\x90Pa\x08\x97a\x17\xC1V[`\0\x82\x82\x11\x15a\x08\xC7Wa\x08\xAB\x83\x83a)+V[\x94Pa\x08\xB6\x85a\x0FqV[\x90Pa\x08\xC20\x82a\x17\xF5V[a\t\tV[a\x08\xD1\x82\x84a)+V[\x93P\x83\x15a\t\tW`\0a\x08\xF5a\x08\xE7\x86a\x0FqV[a\x08\xF00a\x0B\xBEV[a\x18\xB6V[\x90P\x80\x15a\t\x07Wa\t\x070\x82a\x18\xCCV[P[`\0a\t\x140a\x0B\xBEV[\x90P\x80\x15a\t\xAFW`\0B`\nT\x11\x15a\tMWa\t2\x83\x83a)+V[B`\nTa\t@\x91\x90a)+V[a\tJ\x91\x90a)>V[\x90P[`\0\x82`\x0CT\x85a\t^\x91\x90a)>V[a\th\x90\x84a(\xDEV[a\tr\x91\x90a)kV[\x90P\x80a\t\x84d\xE8\xD4\xA5\x10\0\x85a)>V[a\t\x8E\x91\x90a)kV[`\tUa\t\x9B\x81Ba(\xDEV[`\x01`\x01`\x80\x1B\x03\x16`\nUPa\t\xB5\x90PV[`\0`\tU[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nB\x91\x90a)\x8DV[`\x07\x81\x90U\x90Pa\nS\x81\x85a)+V[`\x08UB`\x01`\x01`\x80\x1B\x03\x16`\x0BU`@\x80Q\x88\x81R` \x81\x01\x88\x90R\x7Fo\xEA6\xBF\xB4\xB7\xC9\x12a7V\x95\x1F\xB5\xBF\x8D\xDA\xED\xB7M\xE1\xF2V\xD0\xBE\xF8\x153\x15\x8C\xDAT\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPPa\n\xAA`\x01\x80UV[\x90\x91V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\n\xC9\x81a\x1A\0V[a\n\xD3\x83\x83a\x1A\nV[PPPV[`\0a\x06\xC6\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a)\xA6V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0BtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FAccessControl: can only renounce`D\x82\x01Rn\x1097\xB62\xB9\x9037\xB9\x109\xB2\xB63`\x89\x1B`d\x82\x01R`\x84\x01a\x08fV[a\x0B~\x82\x82a\x1A\x8EV[PPV[`\x003a\x07\xAF\x81\x85\x85a\x0B\x95\x83\x83a\x10\x0FV[a\x0B\x9F\x91\x90a(\xDEV[a\x11XV[`\0\x80a\x0B\xB0\x84a\x0FqV[\x90Pa\x07\xFC3\x84\x86\x84a\x1A\xF3V[`\x000`\x01`\x01`\xA0\x1B\x03\x83\x16\x03a\x0B\xFBWa\x0B\xD8a\x12\xCAV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90 Ta\x06\xFC\x91\x90a)+V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02` R`@\x90 Ta\x06\xFCV[a\x0C1`\0\x80Q` a+\xE8\x839\x81Q\x91R3a\x0CzV[a\x0CMW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08f\x90a)\xBFV[a\x0Cw\x7Fq\xA9\x85\x9D}\xD2\x1B$PJo0`w\xFF\xC2\xD5\x10\xB4\xD4\xB6\x11(\xE91\xFE\x93tA\xAD\x186\x82a\x1D\x04V[PV[`\0\x91\x82R` \x82\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[`\0\x80a\x0C\xAF\x84a\x0EnV[\x90Pa\x07\xFC3\x84\x83\x87a\x1A\xF3V[```\x06\x80Ta\x07\x11\x90a(\xF1V[`\0a\x06\xC6a\x0C\xD9a\n\xD8V[a\x03r\x90`\na*\xCCV[`\0a\x0C\xEF\x83a\x10\x04V[\x85\x11\x15a\r>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC4626: redeem more than max\0\0\0`D\x82\x01R`d\x01a\x08fV[`\0a\rI\x86a\x07\x94V[\x90Pa\rY3\x86\x86\x84\x8A\x88a\x1D\x0EV[P\x90P[\x94\x93PPPPV[`\0a\rp\x83a\x0F~V[\x85\x11\x15a\r\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC4626: withdraw more than max\0`D\x82\x01R`d\x01a\x08fV[`\0a\r\xCA\x86a\x07\xB9V[\x90Pa\rY3\x86\x86\x89\x85\x88a\x1D\x0EV[`\x003\x81a\r\xE8\x82\x86a\x10\x0FV[\x90P\x83\x81\x10\x15a\x0EHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x08fV[a\x0EU\x82\x86\x86\x84\x03a\x11XV[P`\x01\x94\x93PPPPV[`\x003a\x07\xAF\x81\x85\x85a\x13\xA0V[`\0a\x06\xFC\x82`\x01a\x11$V[`\0a\x0E\x86\x82a\x0F~V[\x84\x11\x15a\x0E\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC4626: withdraw more than max\0`D\x82\x01R`d\x01a\x08fV[`\0a\x0E\xE0\x85a\x07\xB9V[\x90Pa\x0E\xF13\x85\x85\x88\x85`\0a\x1D\x0EV[P\x94\x93PPPPV[`\0a\x0F\x05\x82a\x10\x04V[\x84\x11\x15a\x0FTW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC4626: redeem more than max\0\0\0`D\x82\x01R`d\x01a\x08fV[`\0a\x0F_\x85a\x07\x94V[\x90Pa\x0E\xF13\x85\x85\x84\x89a'\x10a\x1D\x0EV[`\0a\x06\xFC\x82`\0a\x12|V[`\0a\x06\xFCa\x0F\x8C\x83a\x0B\xBEV[`\0a\x11$V[a\x0F\xAB`\0\x80Q` a+\xE8\x839\x81Q\x91R3a\x0CzV[a\x0F\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08f\x90a)\xBFV[a\x0Cw`\0\x80Q` a+\xE8\x839\x81Q\x91R\x82a\x1D\x04V[`\0\x82\x81R` \x81\x90R`@\x90 `\x01\x01Ta\x0F\xFA\x81a\x1A\0V[a\n\xD3\x83\x83a\x1A\x8EV[`\0a\x06\xFC\x82a\x0B\xBEV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[a\x10R`\0\x80Q` a+\xE8\x839\x81Q\x91R3a\x0CzV[a\x10nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08f\x90a)\xBFV[\x80`\0\x03a\x10\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x1D\x1B\xDB\xC8\x1C\xDA\x1B\xDC\x9D`\xBA\x1B`D\x82\x01R`d\x01a\x08fV[c\x01\xE1\x85X\x81\x11\x15a\x10\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01Rgtoo long`\xC0\x1B`D\x82\x01R`d\x01a\x08fV[`\x0C\x81\x90U`@Q\x81\x81R\x7F\xF3a\xAE\xD4c\xDAo\xA2\x03X\xE4\\b\t\xF1\xD3\xE1mN\xCApnn\xAB\x0B\n\xEB3\x87)\xC7z\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0\x80a\x11/a\x07\xC6V[\x90P\x80\x15a\x11PWa\x11Ka\x11Ba\x06\xB4V[\x85\x90\x83\x86a FV[a\r]V[P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x11\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x12\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0\x80a\x12\x87a\x06\xB4V[\x90P`\0a\x12\x93a\x07\xC6V[\x90P\x81`\0\x03a\x12\xB5W\x80\x15a\x12\xAAW`\0a\x12\xACV[\x84[\x92PPPa\x06\xFCV[a\x12\xC1\x85\x82\x84\x87a FV[\x95\x94PPPPPV[`\0\x80B`\nT\x11\x15a\x13\x07W`\0`\x0BTBa\x12\xE7\x91\x90a)+V[\x90Pd\xE8\xD4\xA5\x10\0`\tT\x82a\x12\xFD\x91\x90a)>V[a\x07\xFC\x91\x90a)kV[`\nT\x15a\x13!WP0`\0\x90\x81R`\x02` R`@\x90 T[\x91\x90PV[`\0a\x132\x84\x84a\x10\x0FV[\x90P`\0\x19\x81\x14a\x13\x9AW\x81\x81\x10\x15a\x13\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x08fV[a\x13\x9A\x84\x84\x84\x84\x03a\x11XV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x14\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a\x14\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x02` R`@\x80\x82 \x86\x86\x03\x90U\x92\x86\x16\x80\x82R\x90\x83\x90 \x80T\x86\x01\x90U\x91Q\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90a\x15>\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a\x13\x9AV[`\x02`\x01T\x03a\x15\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x08fV[`\x02`\x01UV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x162\x91\x90a)\x8DV[\x90P`\0\x81\x11\x80\x15a\x16GWP`\rT`\xFF\x16\x15[\x15a\x16\xAFW`\rT`@Qc\xB6\xB5_%`\xE0\x1B\x81R`\x04\x81\x01\x83\x90Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB6\xB5_%\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x96W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16\xAAW=`\0\x80>=`\0\xFD[PPPP[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x16W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17:\x91\x90a)\x8DV[`\r`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xB6\x9E\xF8\xA8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xB1\x91\x90a)\x8DV[a\x17\xBB\x91\x90a(\xDEV[\x91PP\x90V[`\0a\x17\xCBa\x12\xCAV[\x90P\x80\x15a\x0CwWB`\nT\x11\x15a\x17\xEBW`\x01`\x01`\x80\x1B\x03B\x16`\x0BU[a\x0Cw0\x82a\x18\xCCV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x18KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x08fV[\x80`\x04`\0\x82\x82Ta\x18]\x91\x90a(\xDEV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[`\0\x81\x83\x10a\x18\xC5W\x81a\x07\xFCV[P\x90\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x19,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x02` R`@\x90 T\x81\x81\x10\x15a\x19\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x08fV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 \x86\x86\x03\x90U`\x04\x80T\x87\x90\x03\x90UQ\x85\x81R\x91\x92\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[a\x0Cw\x813a \xA1V[a\x1A\x14\x82\x82a\x0CzV[a\x0B~W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x1AJ3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x83\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4PPV[a\x1A\x98\x82\x82a\x0CzV[\x15a\x0B~W`\0\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x85\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4PPV[a\x1A\xFBa\x15KV[a\x1B'\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x850\x85a \xFAV[`\0\x82`\x07Ta\x1B7\x91\x90a(\xDEV[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xC6\x91\x90a)\x8DV[\x90Pa\x1B\xD1\x82a!eV[`\0a\x1Cq\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ca\x91\x90a)\x8DV[a\x1Ck\x90\x84a)+V[\x84a\x18\xB6V[\x90P\x80`\x08`\0\x82\x82Ta\x1C\x85\x91\x90a(\xDEV[\x90\x91UPa\x1C\x95\x90P\x81\x84a)+V[`\x07Ua\x1C\xA2\x86\x85a\x17\xF5V[\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x87\x87`@Qa\x1C\xF0\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPa\x13\x9A`\x01\x80UV[a\x0B~\x82\x82a\x1A\nV[`\0a\x1D\x18a\x15KV[`\x01`\x01`\xA0\x1B\x03\x86\x16a\x1DnW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReceiver cannot be zero address\0`D\x82\x01R`d\x01a\x08fV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1D\x92Wa\x1D\x92\x85\x88\x85a\x13&V[`\x07T\x84\x81\x10\x15a\x1F\x94W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E*\x91\x90a)\x8DV[\x90Pa\x1E>a\x1E9\x83\x88a)+V[a!\xCBV[`\0a\x1E\xE1\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xCF\x91\x90a)\x8DV[a\x1E\xD9\x91\x90a)+V[`\x08Ta\x18\xB6V[\x90Pa\x1E\xED\x81\x84a(\xDEV[\x92P`\0\x87\x84\x10\x15a\x1FpWa\x1F\x03\x84\x89a)+V[\x90Pa'\x10\x86\x10\x15a\x1FlWa'\x10a\x1F\x1C\x87\x8Aa)>V[a\x1F&\x91\x90a)kV[\x81\x11\x15a\x1FlW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsLoss exceeds maxLoss``\x1B`D\x82\x01R`d\x01a\x08fV[\x83\x97P[a\x1Fz\x81\x83a(\xDEV[`\x08`\0\x82\x82Ta\x1F\x8B\x91\x90a)+V[\x90\x91UPPPPP[a\x1F\x9E\x85\x82a)+V[`\x07Ua\x1F\xAB\x86\x85a\x18\xCCV[a\x1F\xD6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x87a\"\0V[\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x88\x88`@Qa .\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PP`\x01\x80UP\x90\x94\x93PPPPV[`\0\x80a T\x86\x86\x86a\"0V[\x90P`\x01\x83`\x02\x81\x11\x15a jWa ja*\xDBV[\x14\x80\x15a \x87WP`\0\x84\x80a \x82Wa \x82a)UV[\x86\x88\t\x11[\x15a\x12\xC1Wa \x97`\x01\x82a(\xDEV[\x96\x95PPPPPPV[a \xAB\x82\x82a\x0CzV[a\x0B~Wa \xB8\x81a#\x1AV[a \xC3\x83` a#,V[`@Q` \x01a \xD4\x92\x91\x90a*\xF1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RbF\x1B\xCD`\xE5\x1B\x82Ra\x08f\x91`\x04\x01a'\x0EV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x13\x9A\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra$\xC8V[`\rT`@Qc\xB6\xB5_%`\xE0\x1B\x81R`\x04\x81\x01\x83\x90Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB6\xB5_%\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!\xB0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\xC4W=`\0\x80>=`\0\xFD[PPPPPV[`\rT`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x83\x90Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c.\x1A}M\x90`$\x01a!\x96V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\n\xD3\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a!.V[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\"jW\x83\x82\x81a\"`Wa\"`a)UV[\x04\x92PPPa\x07\xFCV[\x80\x84\x11a\"\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\x08fV[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[``a\x06\xFC`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14[```\0a#;\x83`\x02a)>V[a#F\x90`\x02a(\xDEV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#^Wa#^a+fV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#\x88W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81`\0\x81Q\x81\x10a#\xA3Wa#\xA3a+|V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a#\xD2Wa#\xD2a+|V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\0a#\xF6\x84`\x02a)>V[a$\x01\x90`\x01a(\xDEV[\x90P[`\x01\x81\x11\x15a$yWo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x85`\x0F\x16`\x10\x81\x10a$5Wa$5a+|V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a$KWa$Ka+|V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`\x04\x94\x90\x94\x1C\x93a$r\x81a+\x92V[\x90Pa$\x04V[P\x83\x15a\x07\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FStrings: hex length insufficient`D\x82\x01R`d\x01a\x08fV[`\0a%\x1D\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a%\x9D\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a%>WP\x80\x80` \x01\x90Q\x81\x01\x90a%>\x91\x90a+\xA9V[a\n\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x08fV[``a\r]\x84\x84`\0\x85\x85`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa%\xC4\x91\x90a+\xCBV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a&\x01W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a&\x06V[``\x91P[P\x91P\x91Pa&\x17\x87\x83\x83\x87a&\"V[\x97\x96PPPPPPPV[``\x83\x15a&\x91W\x82Q`\0\x03a&\x8AW`\x01`\x01`\xA0\x1B\x03\x85\x16;a&\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x08fV[P\x81a\r]V[a\r]\x83\x83\x81Q\x15a&\xA6W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08f\x91\x90a'\x0EV[`\0` \x82\x84\x03\x12\x15a&\xD2W`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\xFCW`\0\x80\xFD[`\0[\x83\x81\x10\x15a'\x05W\x81\x81\x01Q\x83\x82\x01R` \x01a&\xEDV[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01Ra'-\x81`@\x85\x01` \x87\x01a&\xEAV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'SW`\0\x80\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13!W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a'\x84W`\0\x80\xFD[a'\x8D\x83a'ZV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\xB0W`\0\x80\xFD[a'\xB9\x84a'ZV[\x92Pa'\xC7` \x85\x01a'ZV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a'\xEAW`\0\x80\xFD[\x825\x91Pa'\xFA` \x84\x01a'ZV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a(\x15W`\0\x80\xFD[a\x07\xFC\x82a'ZV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a(4W`\0\x80\xFD[\x845\x93Pa(D` \x86\x01a'ZV[\x92Pa(R`@\x86\x01a'ZV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a(wW`\0\x80\xFD[\x835\x92Pa(\x87` \x85\x01a'ZV[\x91Pa(\x95`@\x85\x01a'ZV[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a(\xB1W`\0\x80\xFD[a(\xBA\x83a'ZV[\x91Pa'\xFA` \x84\x01a'ZV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x06\xFCWa\x06\xFCa(\xC8V[`\x01\x81\x81\x1C\x90\x82\x16\x80a)\x05W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a)%WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x06\xFCWa\x06\xFCa(\xC8V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\xFCWa\x06\xFCa(\xC8V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)\x88WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a)\x9FW`\0\x80\xFD[PQ\x91\x90PV[`\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x06\xFCWa\x06\xFCa(\xC8V[` \x80\x82R`\x0F\x90\x82\x01Rn\x1B\xDB\x9B\x1EH\x1BX[\x98Y\xD9[Y[\x9D`\x8A\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81[\x80\x85\x11\x15a*#W\x81`\0\x19\x04\x82\x11\x15a*\tWa*\ta(\xC8V[\x80\x85\x16\x15a*\x16W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a)\xEDV[P\x92P\x92\x90PV[`\0\x82a*:WP`\x01a\x06\xFCV[\x81a*GWP`\0a\x06\xFCV[\x81`\x01\x81\x14a*]W`\x02\x81\x14a*gWa*\x83V[`\x01\x91PPa\x06\xFCV[`\xFF\x84\x11\x15a*xWa*xa(\xC8V[PP`\x01\x82\x1Ba\x06\xFCV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a*\xA6WP\x81\x81\na\x06\xFCV[a*\xB0\x83\x83a)\xE8V[\x80`\0\x19\x04\x82\x11\x15a*\xC4Wa*\xC4a(\xC8V[\x02\x93\x92PPPV[`\0a\x07\xFC`\xFF\x84\x16\x83a*+V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x7FAccessControl: account \0\0\0\0\0\0\0\0\0\x81R`\0\x83Qa+)\x81`\x17\x85\x01` \x88\x01a&\xEAV[p\x01\x03K\x99\x03kK\x9B\x9BKs9\x03\x93{c)`}\x1B`\x17\x91\x84\x01\x91\x82\x01R\x83Qa+Z\x81`(\x84\x01` \x88\x01a&\xEAV[\x01`(\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81a+\xA1Wa+\xA1a(\xC8V[P`\0\x19\x01\x90V[`\0` \x82\x84\x03\x12\x15a+\xBBW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xFCW`\0\x80\xFD[`\0\x82Qa+\xDD\x81\x84` \x87\x01a&\xEAV[\x91\x90\x91\x01\x92\x91PPV\xFE\xB4\xD5\xC1a\x80\xAA\x1FB\xA7\x9CC\x01\xE7\xBDt\xB2J\xA5VV<f-\xB1\tW\xE0\xE1\x95\xB5\xBER\xA2dipfsX\"\x12 X\x12\xECR\xB0\n\xCCM\x80\x9A\xF9j\x89S\xAD\x86\x1D\xE8\x85:\xFB\x83R {\xD71\x1F\xEF\xCC\xBB\x80dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKSTRATEGY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockStrategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockStrategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockStrategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockStrategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockStrategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockStrategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKSTRATEGY_ABI.clone(),
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
                MOCKSTRATEGY_ABI.clone(),
                MOCKSTRATEGY_BYTECODE.clone().into(),
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
        ///Calls the contract's `kept` (0xacc3e84f) function
        pub fn kept(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([172, 195, 232, 79], ())
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
        ///Calls the contract's `managed` (0x2fc65c36) function
        pub fn managed(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 198, 92, 54], ())
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
        ///Calls the contract's `trigger` (0x7fec8d38) function
        pub fn trigger(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([127, 236, 141, 56], ())
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
        ///Calls the contract's `yieldSource` (0xb2470e5c) function
        pub fn yield_source(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([178, 71, 14, 92], ())
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
            MockStrategyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockStrategy<M> {
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
    pub enum MockStrategyEvents {
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
    impl ::ethers::contract::EthLogDecode for MockStrategyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockStrategyEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(MockStrategyEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = ReportedFilter::decode_log(log) {
                return Ok(MockStrategyEvents::ReportedFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(MockStrategyEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(MockStrategyEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(MockStrategyEvents::RoleRevokedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockStrategyEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UpdateProfitMaxUnlockTimeFilter::decode_log(log) {
                return Ok(MockStrategyEvents::UpdateProfitMaxUnlockTimeFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(MockStrategyEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockStrategyEvents {
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
    impl ::core::convert::From<ApprovalFilter> for MockStrategyEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for MockStrategyEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<ReportedFilter> for MockStrategyEvents {
        fn from(value: ReportedFilter) -> Self {
            Self::ReportedFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter> for MockStrategyEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter> for MockStrategyEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter> for MockStrategyEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockStrategyEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<UpdateProfitMaxUnlockTimeFilter> for MockStrategyEvents {
        fn from(value: UpdateProfitMaxUnlockTimeFilter) -> Self {
            Self::UpdateProfitMaxUnlockTimeFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for MockStrategyEvents {
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
    ///Container type for all input parameters for the `kept` function with signature `kept()` and selector `0xacc3e84f`
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
    #[ethcall(name = "kept", abi = "kept()")]
    pub struct KeptCall;
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
    ///Container type for all input parameters for the `managed` function with signature `managed()` and selector `0x2fc65c36`
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
    #[ethcall(name = "managed", abi = "managed()")]
    pub struct ManagedCall;
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
    ///Container type for all input parameters for the `trigger` function with signature `trigger()` and selector `0x7fec8d38`
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
    #[ethcall(name = "trigger", abi = "trigger()")]
    pub struct TriggerCall;
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
    ///Container type for all input parameters for the `yieldSource` function with signature `yieldSource()` and selector `0xb2470e5c`
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
    #[ethcall(name = "yieldSource", abi = "yieldSource()")]
    pub struct YieldSourceCall;
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
    pub enum MockStrategyCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
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
        Kept(KeptCall),
        LastReport(LastReportCall),
        Managed(ManagedCall),
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
        Trigger(TriggerCall),
        WithdrawWithAssetsAndReceiverAndMaxLoss(
            WithdrawWithAssetsAndReceiverAndMaxLossCall,
        ),
        Withdraw(WithdrawCall),
        YieldSource(YieldSourceCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockStrategyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DefaultAdminRole(decoded));
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
            if let Ok(decoded) = <KeptCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Kept(decoded));
            }
            if let Ok(decoded) = <LastReportCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastReport(decoded));
            }
            if let Ok(decoded) = <ManagedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Managed(decoded));
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
            if let Ok(decoded) = <TriggerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Trigger(decoded));
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
            if let Ok(decoded) = <YieldSourceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::YieldSource(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::Kept(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastReport(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Managed(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Trigger(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawWithAssetsAndReceiverAndMaxLoss(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::YieldSource(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockStrategyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Kept(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastReport(element) => ::core::fmt::Display::fmt(element, f),
                Self::Managed(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Trigger(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawWithAssetsAndReceiverAndMaxLoss(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::YieldSource(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall> for MockStrategyCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for MockStrategyCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for MockStrategyCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AssetCall> for MockStrategyCalls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockStrategyCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ConvertToAssetsCall> for MockStrategyCalls {
        fn from(value: ConvertToAssetsCall) -> Self {
            Self::ConvertToAssets(value)
        }
    }
    impl ::core::convert::From<ConvertToSharesCall> for MockStrategyCalls {
        fn from(value: ConvertToSharesCall) -> Self {
            Self::ConvertToShares(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for MockStrategyCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for MockStrategyCalls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MockStrategyCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<FullProfitUnlockDateCall> for MockStrategyCalls {
        fn from(value: FullProfitUnlockDateCall) -> Self {
            Self::FullProfitUnlockDate(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall> for MockStrategyCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for MockStrategyCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for MockStrategyCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for MockStrategyCalls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<KeptCall> for MockStrategyCalls {
        fn from(value: KeptCall) -> Self {
            Self::Kept(value)
        }
    }
    impl ::core::convert::From<LastReportCall> for MockStrategyCalls {
        fn from(value: LastReportCall) -> Self {
            Self::LastReport(value)
        }
    }
    impl ::core::convert::From<ManagedCall> for MockStrategyCalls {
        fn from(value: ManagedCall) -> Self {
            Self::Managed(value)
        }
    }
    impl ::core::convert::From<MaxDepositCall> for MockStrategyCalls {
        fn from(value: MaxDepositCall) -> Self {
            Self::MaxDeposit(value)
        }
    }
    impl ::core::convert::From<MaxMintCall> for MockStrategyCalls {
        fn from(value: MaxMintCall) -> Self {
            Self::MaxMint(value)
        }
    }
    impl ::core::convert::From<MaxRedeemCall> for MockStrategyCalls {
        fn from(value: MaxRedeemCall) -> Self {
            Self::MaxRedeem(value)
        }
    }
    impl ::core::convert::From<MaxWithdrawCall> for MockStrategyCalls {
        fn from(value: MaxWithdrawCall) -> Self {
            Self::MaxWithdraw(value)
        }
    }
    impl ::core::convert::From<MintCall> for MockStrategyCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockStrategyCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<PreviewDepositCall> for MockStrategyCalls {
        fn from(value: PreviewDepositCall) -> Self {
            Self::PreviewDeposit(value)
        }
    }
    impl ::core::convert::From<PreviewMintCall> for MockStrategyCalls {
        fn from(value: PreviewMintCall) -> Self {
            Self::PreviewMint(value)
        }
    }
    impl ::core::convert::From<PreviewRedeemCall> for MockStrategyCalls {
        fn from(value: PreviewRedeemCall) -> Self {
            Self::PreviewRedeem(value)
        }
    }
    impl ::core::convert::From<PreviewWithdrawCall> for MockStrategyCalls {
        fn from(value: PreviewWithdrawCall) -> Self {
            Self::PreviewWithdraw(value)
        }
    }
    impl ::core::convert::From<PricePerShareCall> for MockStrategyCalls {
        fn from(value: PricePerShareCall) -> Self {
            Self::PricePerShare(value)
        }
    }
    impl ::core::convert::From<ProfitMaxUnlockTimeCall> for MockStrategyCalls {
        fn from(value: ProfitMaxUnlockTimeCall) -> Self {
            Self::ProfitMaxUnlockTime(value)
        }
    }
    impl ::core::convert::From<ProfitUnlockingRateCall> for MockStrategyCalls {
        fn from(value: ProfitUnlockingRateCall) -> Self {
            Self::ProfitUnlockingRate(value)
        }
    }
    impl ::core::convert::From<RedeemWithSharesAndReceiverAndMaxLossCall>
    for MockStrategyCalls {
        fn from(value: RedeemWithSharesAndReceiverAndMaxLossCall) -> Self {
            Self::RedeemWithSharesAndReceiverAndMaxLoss(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for MockStrategyCalls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall> for MockStrategyCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<ReportCall> for MockStrategyCalls {
        fn from(value: ReportCall) -> Self {
            Self::Report(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for MockStrategyCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<SetKeeperCall> for MockStrategyCalls {
        fn from(value: SetKeeperCall) -> Self {
            Self::SetKeeper(value)
        }
    }
    impl ::core::convert::From<SetManagementCall> for MockStrategyCalls {
        fn from(value: SetManagementCall) -> Self {
            Self::SetManagement(value)
        }
    }
    impl ::core::convert::From<SetProfitMaxUnlockTimeCall> for MockStrategyCalls {
        fn from(value: SetProfitMaxUnlockTimeCall) -> Self {
            Self::SetProfitMaxUnlockTime(value)
        }
    }
    impl ::core::convert::From<ShutdownCall> for MockStrategyCalls {
        fn from(value: ShutdownCall) -> Self {
            Self::Shutdown(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for MockStrategyCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockStrategyCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalAssetsCall> for MockStrategyCalls {
        fn from(value: TotalAssetsCall) -> Self {
            Self::TotalAssets(value)
        }
    }
    impl ::core::convert::From<TotalDebtCall> for MockStrategyCalls {
        fn from(value: TotalDebtCall) -> Self {
            Self::TotalDebt(value)
        }
    }
    impl ::core::convert::From<TotalIdleCall> for MockStrategyCalls {
        fn from(value: TotalIdleCall) -> Self {
            Self::TotalIdle(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for MockStrategyCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for MockStrategyCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockStrategyCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TriggerCall> for MockStrategyCalls {
        fn from(value: TriggerCall) -> Self {
            Self::Trigger(value)
        }
    }
    impl ::core::convert::From<WithdrawWithAssetsAndReceiverAndMaxLossCall>
    for MockStrategyCalls {
        fn from(value: WithdrawWithAssetsAndReceiverAndMaxLossCall) -> Self {
            Self::WithdrawWithAssetsAndReceiverAndMaxLoss(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for MockStrategyCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<YieldSourceCall> for MockStrategyCalls {
        fn from(value: YieldSourceCall) -> Self {
            Self::YieldSource(value)
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
    ///Container type for all return fields from the `kept` function with signature `kept()` and selector `0xacc3e84f`
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
    pub struct KeptReturn(pub bool);
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
    ///Container type for all return fields from the `managed` function with signature `managed()` and selector `0x2fc65c36`
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
    pub struct ManagedReturn(pub bool);
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
    ///Container type for all return fields from the `trigger` function with signature `trigger()` and selector `0x7fec8d38`
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
    pub struct TriggerReturn(pub bool);
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
    ///Container type for all return fields from the `yieldSource` function with signature `yieldSource()` and selector `0xb2470e5c`
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
    pub struct YieldSourceReturn(pub ::ethers::core::types::Address);
}
