pub use interchain_gas_paymaster::*;
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
pub mod interchain_gas_paymaster {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("beneficiary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("beneficiary"),
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
                    ::std::borrow::ToOwned::to_owned("claim"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claim"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("gasOracles"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gasOracles"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IGasOracle"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_beneficiary"),
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
                    ::std::borrow::ToOwned::to_owned("payForGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("payForGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_destinationDomain",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_refundAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quoteGasPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quoteGasPayment"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gasAmount"),
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
                    ::std::borrow::ToOwned::to_owned("setBeneficiary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBeneficiary"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_beneficiary"),
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
                    ::std::borrow::ToOwned::to_owned("setGasOracles"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGasOracles"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_configs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct InterchainGasPaymaster.GasOracleConfig[]",
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
                    ::std::borrow::ToOwned::to_owned("BeneficiarySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BeneficiarySet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("beneficiary"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GasOracleSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GasOracleSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("remoteDomain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gasOracle"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GasPayment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GasPayment"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gasAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payment"),
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
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
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
    pub static INTERCHAINGASPAYMASTER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0C\x85\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xA7W`\x005`\xE0\x1C\x80cNq\xD9-\x11a\0dW\x80cNq\xD9-\x14a\x01\x94W\x80c`\xFC\xEF|\x14a\x01\xA9W\x80cqP\x18\xA6\x14a\x01\xE9W\x80c\x8D\xA5\xCB[\x14a\x01\xFEW\x80c\xA6\x92\x97\x93\x14a\x02\x1CW\x80c\xF2\xFD\xE3\x8B\x14a\x02JW`\0\x80\xFD[\x80c\x11\xBF,\x18\x14a\0\xACW\x80c\x1C1\xF7\x10\x14a\0\xC1W\x80c\x1D\x16\xC8\xC8\x14a\0\xE1W\x80c/ &P\x14a\x014W\x80c8\xAF>\xED\x14a\x01TW\x80cH\\\xC9U\x14a\x01tW[`\0\x80\xFD[a\0\xBFa\0\xBA6`\x04a\t\xDDV[a\x02jV[\0[4\x80\x15a\0\xCDW`\0\x80\xFD[Pa\0\xBFa\0\xDC6`\x04a\n#V[a\x03\xE1V[4\x80\x15a\0\xEDW`\0\x80\xFD[Pa\x01\x17a\0\xFC6`\x04a\nEV[`e` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01@W`\0\x80\xFD[Pa\0\xBFa\x01O6`\x04a\n`V[a\x03\xF5V[4\x80\x15a\x01`W`\0\x80\xFD[P`fTa\x01\x17\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01\x80W`\0\x80\xFD[Pa\0\xBFa\x01\x8F6`\x04a\n\xD5V[a\x04{V[4\x80\x15a\x01\xA0W`\0\x80\xFD[Pa\0\xBFa\x05\xA0V[4\x80\x15a\x01\xB5W`\0\x80\xFD[Pa\x01\xC9a\x01\xC46`\x04a\nEV[a\x06/V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x01+V[4\x80\x15a\x01\xF5W`\0\x80\xFD[Pa\0\xBFa\x07\x02V[4\x80\x15a\x02\nW`\0\x80\xFD[P`3T`\x01`\x01`\xA0\x1B\x03\x16a\x01\x17V[4\x80\x15a\x02(W`\0\x80\xFD[Pa\x02<a\x0276`\x04a\x0B\x08V[a\x07\x16V[`@Q\x90\x81R` \x01a\x01+V[4\x80\x15a\x02VW`\0\x80\xFD[Pa\0\xBFa\x02e6`\x04a\n#V[a\x07oV[`\0a\x02v\x84\x84a\x07\x16V[\x90P\x804\x10\x15a\x02\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finsufficient interchain gas paym`D\x82\x01Rb\x19[\x9D`\xEA\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x02\xE5\x824a\x0BHV[\x90P\x80\x15a\x03\x9EW`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x03:W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03?V[``\x91P[PP\x90P\x80a\x03\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FInterchain gas payment refund fa`D\x82\x01Rc\x1A[\x19Y`\xE2\x1B`d\x82\x01R`\x84\x01a\x02\xD0V[P[`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x87\x91\x7F\xF7\x15\xE6m,\xD2\xA0\x17\x90i\xDC\xDA*A\xA4\xDA6\xA3\x0E\x1A\xAD\x12\x18\x7F\xBB|]\x10\n\xFDb\x1C\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPV[a\x03\xE9a\x07\xE5V[a\x03\xF2\x81a\x08?V[PV[a\x03\xFDa\x07\xE5V[\x80`\0[\x81\x81\x10\x15a\x04uWa\x04c\x84\x84\x83\x81\x81\x10a\x04\x1EWa\x04\x1Ea\x0B[V[a\x044\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\nEV[\x85\x85\x84\x81\x81\x10a\x04FWa\x04Fa\x0B[V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x04^\x91\x90a\n#V[a\x08\x93V[\x80a\x04m\x81a\x0BqV[\x91PPa\x04\x01V[PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x04\x9BWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x04\xB5WP0;\x15\x80\x15a\x04\xB5WP`\0T`\xFF\x16`\x01\x14[a\x05\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x02\xD0V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x05;W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x05Ca\x08\xFCV[a\x05L\x83a\t+V[a\x05U\x82a\x08?V[\x80\x15a\x05\x9BW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`fT`@Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90G\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x05\xEDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\xF2V[``\x91P[PP\x90P\x80a\x03\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x10\xBA90\xB79\xB32\xB9`\xB9\x1B`D\x82\x01R`d\x01a\x02\xD0V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`e` R`@\x81 T\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x80a\x06\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj!gas oracle`\xA8\x1B`D\x82\x01R`d\x01a\x02\xD0V[`@Qc\x18?;\xDF`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c`\xFC\xEF|\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xF8\x91\x90a\x0B\xA1V[\x92P\x92PP\x91P\x91V[a\x07\na\x07\xE5V[a\x07\x14`\0a\t+V[V[`\0\x80`\0a\x07$\x85a\x06/V[\x90\x92P\x90P`\0a\x07>`\x01`\x01`\x80\x1B\x03\x83\x16\x86a\x0B\xCBV[\x90Pd\x02T\x0B\xE4\0a\x07Y`\x01`\x01`\x80\x1B\x03\x85\x16\x83a\x0B\xCBV[a\x07c\x91\x90a\x0B\xE2V[\x93PPPP[\x92\x91PPV[a\x07wa\x07\xE5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x02\xD0V[a\x03\xF2\x81a\t+V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xD0V[`f\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x04\xD5Z\x8B\xE1\x81\xFB\x8Du\xB7o-H\xAA\x0B.\xE4\x0FG\xE5=nav>\xEE\xECF\xFE\xEA\x8A$\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x81\x81R`e` \x90\x81R`@\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F8\xB8B\xE1\x16\xFF2\x0Fjy\xBAL\xF44\xEE\xE9'\xFA\x15k\"le\xC9\rb\x98\x9D\xE5\x89\xB4\xE7\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\t#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xD0\x90a\x0C\x04V[a\x07\x14a\t}V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\t\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xD0\x90a\x0C\x04V[a\x07\x143a\t+V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\xC1W`\0\x80\xFD[\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xC1W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\t\xF3W`\0\x80\xFD[\x845\x93Pa\n\x03` \x86\x01a\t\xADV[\x92P`@\x85\x015\x91Pa\n\x18``\x86\x01a\t\xC6V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a\n5W`\0\x80\xFD[a\n>\x82a\t\xC6V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\nWW`\0\x80\xFD[a\n>\x82a\t\xADV[`\0\x80` \x83\x85\x03\x12\x15a\nsW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\x8BW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\n\x9FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\n\xAEW`\0\x80\xFD[\x86` \x82`\x06\x1B\x85\x01\x01\x11\x15a\n\xC3W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\n\xE8W`\0\x80\xFD[a\n\xF1\x83a\t\xC6V[\x91Pa\n\xFF` \x84\x01a\t\xC6V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\x1BW`\0\x80\xFD[a\x0B$\x83a\t\xADV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07iWa\x07ia\x0B2V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x0B\x83Wa\x0B\x83a\x0B2V[P`\x01\x01\x90V[\x80Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\t\xC1W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xB4W`\0\x80\xFD[a\x0B\xBD\x83a\x0B\x8AV[\x91Pa\n\xFF` \x84\x01a\x0B\x8AV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07iWa\x07ia\x0B2V[`\0\x82a\x0B\xFFWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V\xFE\xA2dipfsX\"\x12 \x1D+\x16\xCF\x98\xE6u\x0C\xDA\x1E\xDC\xD5\x94A\xAF\xAB\x8C\x9A\xE63|\xD9p\xFF\xF0\xEB\xB4+\xA5\x81\0\0dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static INTERCHAINGASPAYMASTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xA7W`\x005`\xE0\x1C\x80cNq\xD9-\x11a\0dW\x80cNq\xD9-\x14a\x01\x94W\x80c`\xFC\xEF|\x14a\x01\xA9W\x80cqP\x18\xA6\x14a\x01\xE9W\x80c\x8D\xA5\xCB[\x14a\x01\xFEW\x80c\xA6\x92\x97\x93\x14a\x02\x1CW\x80c\xF2\xFD\xE3\x8B\x14a\x02JW`\0\x80\xFD[\x80c\x11\xBF,\x18\x14a\0\xACW\x80c\x1C1\xF7\x10\x14a\0\xC1W\x80c\x1D\x16\xC8\xC8\x14a\0\xE1W\x80c/ &P\x14a\x014W\x80c8\xAF>\xED\x14a\x01TW\x80cH\\\xC9U\x14a\x01tW[`\0\x80\xFD[a\0\xBFa\0\xBA6`\x04a\t\xDDV[a\x02jV[\0[4\x80\x15a\0\xCDW`\0\x80\xFD[Pa\0\xBFa\0\xDC6`\x04a\n#V[a\x03\xE1V[4\x80\x15a\0\xEDW`\0\x80\xFD[Pa\x01\x17a\0\xFC6`\x04a\nEV[`e` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01@W`\0\x80\xFD[Pa\0\xBFa\x01O6`\x04a\n`V[a\x03\xF5V[4\x80\x15a\x01`W`\0\x80\xFD[P`fTa\x01\x17\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01\x80W`\0\x80\xFD[Pa\0\xBFa\x01\x8F6`\x04a\n\xD5V[a\x04{V[4\x80\x15a\x01\xA0W`\0\x80\xFD[Pa\0\xBFa\x05\xA0V[4\x80\x15a\x01\xB5W`\0\x80\xFD[Pa\x01\xC9a\x01\xC46`\x04a\nEV[a\x06/V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x01+V[4\x80\x15a\x01\xF5W`\0\x80\xFD[Pa\0\xBFa\x07\x02V[4\x80\x15a\x02\nW`\0\x80\xFD[P`3T`\x01`\x01`\xA0\x1B\x03\x16a\x01\x17V[4\x80\x15a\x02(W`\0\x80\xFD[Pa\x02<a\x0276`\x04a\x0B\x08V[a\x07\x16V[`@Q\x90\x81R` \x01a\x01+V[4\x80\x15a\x02VW`\0\x80\xFD[Pa\0\xBFa\x02e6`\x04a\n#V[a\x07oV[`\0a\x02v\x84\x84a\x07\x16V[\x90P\x804\x10\x15a\x02\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Finsufficient interchain gas paym`D\x82\x01Rb\x19[\x9D`\xEA\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x02\xE5\x824a\x0BHV[\x90P\x80\x15a\x03\x9EW`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x03:W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x03?V[``\x91P[PP\x90P\x80a\x03\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FInterchain gas payment refund fa`D\x82\x01Rc\x1A[\x19Y`\xE2\x1B`d\x82\x01R`\x84\x01a\x02\xD0V[P[`@\x80Q\x85\x81R` \x81\x01\x84\x90R\x87\x91\x7F\xF7\x15\xE6m,\xD2\xA0\x17\x90i\xDC\xDA*A\xA4\xDA6\xA3\x0E\x1A\xAD\x12\x18\x7F\xBB|]\x10\n\xFDb\x1C\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPV[a\x03\xE9a\x07\xE5V[a\x03\xF2\x81a\x08?V[PV[a\x03\xFDa\x07\xE5V[\x80`\0[\x81\x81\x10\x15a\x04uWa\x04c\x84\x84\x83\x81\x81\x10a\x04\x1EWa\x04\x1Ea\x0B[V[a\x044\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa\nEV[\x85\x85\x84\x81\x81\x10a\x04FWa\x04Fa\x0B[V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x04^\x91\x90a\n#V[a\x08\x93V[\x80a\x04m\x81a\x0BqV[\x91PPa\x04\x01V[PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x04\x9BWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x04\xB5WP0;\x15\x80\x15a\x04\xB5WP`\0T`\xFF\x16`\x01\x14[a\x05\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x02\xD0V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x05;W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x05Ca\x08\xFCV[a\x05L\x83a\t+V[a\x05U\x82a\x08?V[\x80\x15a\x05\x9BW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`fT`@Q`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90G\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x05\xEDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\xF2V[``\x91P[PP\x90P\x80a\x03\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x10\xBA90\xB79\xB32\xB9`\xB9\x1B`D\x82\x01R`d\x01a\x02\xD0V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`e` R`@\x81 T\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x80a\x06\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj!gas oracle`\xA8\x1B`D\x82\x01R`d\x01a\x02\xD0V[`@Qc\x18?;\xDF`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c`\xFC\xEF|\x90`$\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xF8\x91\x90a\x0B\xA1V[\x92P\x92PP\x91P\x91V[a\x07\na\x07\xE5V[a\x07\x14`\0a\t+V[V[`\0\x80`\0a\x07$\x85a\x06/V[\x90\x92P\x90P`\0a\x07>`\x01`\x01`\x80\x1B\x03\x83\x16\x86a\x0B\xCBV[\x90Pd\x02T\x0B\xE4\0a\x07Y`\x01`\x01`\x80\x1B\x03\x85\x16\x83a\x0B\xCBV[a\x07c\x91\x90a\x0B\xE2V[\x93PPPP[\x92\x91PPV[a\x07wa\x07\xE5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x02\xD0V[a\x03\xF2\x81a\t+V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xD0V[`f\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x04\xD5Z\x8B\xE1\x81\xFB\x8Du\xB7o-H\xAA\x0B.\xE4\x0FG\xE5=nav>\xEE\xECF\xFE\xEA\x8A$\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x81\x81R`e` \x90\x81R`@\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F8\xB8B\xE1\x16\xFF2\x0Fjy\xBAL\xF44\xEE\xE9'\xFA\x15k\"le\xC9\rb\x98\x9D\xE5\x89\xB4\xE7\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\t#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xD0\x90a\x0C\x04V[a\x07\x14a\t}V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\t\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xD0\x90a\x0C\x04V[a\x07\x143a\t+V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\xC1W`\0\x80\xFD[\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xC1W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\t\xF3W`\0\x80\xFD[\x845\x93Pa\n\x03` \x86\x01a\t\xADV[\x92P`@\x85\x015\x91Pa\n\x18``\x86\x01a\t\xC6V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a\n5W`\0\x80\xFD[a\n>\x82a\t\xC6V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\nWW`\0\x80\xFD[a\n>\x82a\t\xADV[`\0\x80` \x83\x85\x03\x12\x15a\nsW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\x8BW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\n\x9FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\n\xAEW`\0\x80\xFD[\x86` \x82`\x06\x1B\x85\x01\x01\x11\x15a\n\xC3W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\n\xE8W`\0\x80\xFD[a\n\xF1\x83a\t\xC6V[\x91Pa\n\xFF` \x84\x01a\t\xC6V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\x1BW`\0\x80\xFD[a\x0B$\x83a\t\xADV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07iWa\x07ia\x0B2V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x0B\x83Wa\x0B\x83a\x0B2V[P`\x01\x01\x90V[\x80Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14a\t\xC1W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xB4W`\0\x80\xFD[a\x0B\xBD\x83a\x0B\x8AV[\x91Pa\n\xFF` \x84\x01a\x0B\x8AV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07iWa\x07ia\x0B2V[`\0\x82a\x0B\xFFWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V\xFE\xA2dipfsX\"\x12 \x1D+\x16\xCF\x98\xE6u\x0C\xDA\x1E\xDC\xD5\x94A\xAF\xAB\x8C\x9A\xE63|\xD9p\xFF\xF0\xEB\xB4+\xA5\x81\0\0dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static INTERCHAINGASPAYMASTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct InterchainGasPaymaster<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for InterchainGasPaymaster<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for InterchainGasPaymaster<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for InterchainGasPaymaster<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for InterchainGasPaymaster<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(InterchainGasPaymaster))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> InterchainGasPaymaster<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    INTERCHAINGASPAYMASTER_ABI.clone(),
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
                INTERCHAINGASPAYMASTER_ABI.clone(),
                INTERCHAINGASPAYMASTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `beneficiary` (0x38af3eed) function
        pub fn beneficiary(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([56, 175, 62, 237], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claim` (0x4e71d92d) function
        pub fn claim(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 113, 217, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gasOracles` (0x1d16c8c8) function
        pub fn gas_oracles(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([29, 22, 200, 200], p0)
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            owner: ::ethers::core::types::Address,
            beneficiary: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (owner, beneficiary))
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
        ///Calls the contract's `payForGas` (0x11bf2c18) function
        pub fn pay_for_gas(
            &self,
            message_id: [u8; 32],
            destination_domain: u32,
            gas_amount: ::ethers::core::types::U256,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [17, 191, 44, 24],
                    (message_id, destination_domain, gas_amount, refund_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteGasPayment` (0xa6929793) function
        pub fn quote_gas_payment(
            &self,
            destination_domain: u32,
            gas_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([166, 146, 151, 147], (destination_domain, gas_amount))
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
        ///Calls the contract's `setBeneficiary` (0x1c31f710) function
        pub fn set_beneficiary(
            &self,
            beneficiary: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 49, 247, 16], beneficiary)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGasOracles` (0x2f202650) function
        pub fn set_gas_oracles(
            &self,
            configs: ::std::vec::Vec<GasOracleConfig>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 32, 38, 80], configs)
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
        ///Gets the contract's `BeneficiarySet` event
        pub fn beneficiary_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BeneficiarySetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GasOracleSet` event
        pub fn gas_oracle_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GasOracleSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GasPayment` event
        pub fn gas_payment_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GasPaymentFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
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
            InterchainGasPaymasterEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for InterchainGasPaymaster<M> {
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
    #[ethevent(name = "BeneficiarySet", abi = "BeneficiarySet(address)")]
    pub struct BeneficiarySetFilter {
        pub beneficiary: ::ethers::core::types::Address,
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
    #[ethevent(name = "GasOracleSet", abi = "GasOracleSet(uint32,address)")]
    pub struct GasOracleSetFilter {
        #[ethevent(indexed)]
        pub remote_domain: u32,
        pub gas_oracle: ::ethers::core::types::Address,
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
    #[ethevent(name = "GasPayment", abi = "GasPayment(bytes32,uint256,uint256)")]
    pub struct GasPaymentFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
        pub gas_amount: ::ethers::core::types::U256,
        pub payment: ::ethers::core::types::U256,
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
    pub enum InterchainGasPaymasterEvents {
        BeneficiarySetFilter(BeneficiarySetFilter),
        GasOracleSetFilter(GasOracleSetFilter),
        GasPaymentFilter(GasPaymentFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for InterchainGasPaymasterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BeneficiarySetFilter::decode_log(log) {
                return Ok(InterchainGasPaymasterEvents::BeneficiarySetFilter(decoded));
            }
            if let Ok(decoded) = GasOracleSetFilter::decode_log(log) {
                return Ok(InterchainGasPaymasterEvents::GasOracleSetFilter(decoded));
            }
            if let Ok(decoded) = GasPaymentFilter::decode_log(log) {
                return Ok(InterchainGasPaymasterEvents::GasPaymentFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(InterchainGasPaymasterEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    InterchainGasPaymasterEvents::OwnershipTransferredFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for InterchainGasPaymasterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BeneficiarySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GasOracleSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GasPaymentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BeneficiarySetFilter> for InterchainGasPaymasterEvents {
        fn from(value: BeneficiarySetFilter) -> Self {
            Self::BeneficiarySetFilter(value)
        }
    }
    impl ::core::convert::From<GasOracleSetFilter> for InterchainGasPaymasterEvents {
        fn from(value: GasOracleSetFilter) -> Self {
            Self::GasOracleSetFilter(value)
        }
    }
    impl ::core::convert::From<GasPaymentFilter> for InterchainGasPaymasterEvents {
        fn from(value: GasPaymentFilter) -> Self {
            Self::GasPaymentFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for InterchainGasPaymasterEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for InterchainGasPaymasterEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `beneficiary` function with signature `beneficiary()` and selector `0x38af3eed`
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
    #[ethcall(name = "beneficiary", abi = "beneficiary()")]
    pub struct BeneficiaryCall;
    ///Container type for all input parameters for the `claim` function with signature `claim()` and selector `0x4e71d92d`
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
    #[ethcall(name = "claim", abi = "claim()")]
    pub struct ClaimCall;
    ///Container type for all input parameters for the `gasOracles` function with signature `gasOracles(uint32)` and selector `0x1d16c8c8`
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
    #[ethcall(name = "gasOracles", abi = "gasOracles(uint32)")]
    pub struct GasOraclesCall(pub u32);
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub owner: ::ethers::core::types::Address,
        pub beneficiary: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `payForGas` function with signature `payForGas(bytes32,uint32,uint256,address)` and selector `0x11bf2c18`
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
    #[ethcall(name = "payForGas", abi = "payForGas(bytes32,uint32,uint256,address)")]
    pub struct PayForGasCall {
        pub message_id: [u8; 32],
        pub destination_domain: u32,
        pub gas_amount: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `quoteGasPayment` function with signature `quoteGasPayment(uint32,uint256)` and selector `0xa6929793`
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
    #[ethcall(name = "quoteGasPayment", abi = "quoteGasPayment(uint32,uint256)")]
    pub struct QuoteGasPaymentCall {
        pub destination_domain: u32,
        pub gas_amount: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `setBeneficiary` function with signature `setBeneficiary(address)` and selector `0x1c31f710`
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
    #[ethcall(name = "setBeneficiary", abi = "setBeneficiary(address)")]
    pub struct SetBeneficiaryCall {
        pub beneficiary: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setGasOracles` function with signature `setGasOracles((uint32,address)[])` and selector `0x2f202650`
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
    #[ethcall(name = "setGasOracles", abi = "setGasOracles((uint32,address)[])")]
    pub struct SetGasOraclesCall {
        pub configs: ::std::vec::Vec<GasOracleConfig>,
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
    pub enum InterchainGasPaymasterCalls {
        Beneficiary(BeneficiaryCall),
        Claim(ClaimCall),
        GasOracles(GasOraclesCall),
        GetExchangeRateAndGasPrice(GetExchangeRateAndGasPriceCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        PayForGas(PayForGasCall),
        QuoteGasPayment(QuoteGasPaymentCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetBeneficiary(SetBeneficiaryCall),
        SetGasOracles(SetGasOraclesCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for InterchainGasPaymasterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BeneficiaryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Beneficiary(decoded));
            }
            if let Ok(decoded) = <ClaimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Claim(decoded));
            }
            if let Ok(decoded) = <GasOraclesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GasOracles(decoded));
            }
            if let Ok(decoded) = <GetExchangeRateAndGasPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetExchangeRateAndGasPrice(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PayForGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PayForGas(decoded));
            }
            if let Ok(decoded) = <QuoteGasPaymentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteGasPayment(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetBeneficiaryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBeneficiary(decoded));
            }
            if let Ok(decoded) = <SetGasOraclesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetGasOracles(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for InterchainGasPaymasterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Beneficiary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Claim(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GasOracles(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetExchangeRateAndGasPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayForGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteGasPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBeneficiary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGasOracles(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for InterchainGasPaymasterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Beneficiary(element) => ::core::fmt::Display::fmt(element, f),
                Self::Claim(element) => ::core::fmt::Display::fmt(element, f),
                Self::GasOracles(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetExchangeRateAndGasPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayForGas(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteGasPayment(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBeneficiary(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGasOracles(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BeneficiaryCall> for InterchainGasPaymasterCalls {
        fn from(value: BeneficiaryCall) -> Self {
            Self::Beneficiary(value)
        }
    }
    impl ::core::convert::From<ClaimCall> for InterchainGasPaymasterCalls {
        fn from(value: ClaimCall) -> Self {
            Self::Claim(value)
        }
    }
    impl ::core::convert::From<GasOraclesCall> for InterchainGasPaymasterCalls {
        fn from(value: GasOraclesCall) -> Self {
            Self::GasOracles(value)
        }
    }
    impl ::core::convert::From<GetExchangeRateAndGasPriceCall>
    for InterchainGasPaymasterCalls {
        fn from(value: GetExchangeRateAndGasPriceCall) -> Self {
            Self::GetExchangeRateAndGasPrice(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for InterchainGasPaymasterCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for InterchainGasPaymasterCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PayForGasCall> for InterchainGasPaymasterCalls {
        fn from(value: PayForGasCall) -> Self {
            Self::PayForGas(value)
        }
    }
    impl ::core::convert::From<QuoteGasPaymentCall> for InterchainGasPaymasterCalls {
        fn from(value: QuoteGasPaymentCall) -> Self {
            Self::QuoteGasPayment(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for InterchainGasPaymasterCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetBeneficiaryCall> for InterchainGasPaymasterCalls {
        fn from(value: SetBeneficiaryCall) -> Self {
            Self::SetBeneficiary(value)
        }
    }
    impl ::core::convert::From<SetGasOraclesCall> for InterchainGasPaymasterCalls {
        fn from(value: SetGasOraclesCall) -> Self {
            Self::SetGasOracles(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for InterchainGasPaymasterCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `beneficiary` function with signature `beneficiary()` and selector `0x38af3eed`
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
    pub struct BeneficiaryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `gasOracles` function with signature `gasOracles(uint32)` and selector `0x1d16c8c8`
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
    pub struct GasOraclesReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `quoteGasPayment` function with signature `quoteGasPayment(uint32,uint256)` and selector `0xa6929793`
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
    pub struct QuoteGasPaymentReturn(pub ::ethers::core::types::U256);
    ///`GasOracleConfig(uint32,address)`
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
    pub struct GasOracleConfig {
        pub remote_domain: u32,
        pub gas_oracle: ::ethers::core::types::Address,
    }
}
