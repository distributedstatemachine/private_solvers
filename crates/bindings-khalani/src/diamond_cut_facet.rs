pub use diamond_cut_facet::*;
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
pub mod diamond_cut_facet {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("diamondCut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("diamondCut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_diamondCut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDiamond.FacetCut[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_init"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_calldata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("DiamondCut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DiamondCut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_diamondCut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_init"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_calldata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
                        "CannotAddFunctionToDiamondThatAlreadyExists",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotAddFunctionToDiamondThatAlreadyExists",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotAddSelectorsToZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotAddSelectorsToZeroAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selectors"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4[]"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotRemoveFunctionThatDoesNotExist",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotRemoveFunctionThatDoesNotExist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotRemoveImmutableFunction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotRemoveImmutableFunction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotReplaceFunctionThatDoesNotExists",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceFunctionThatDoesNotExists",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "CannotReplaceFunctionsFromFacetWithZeroAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceFunctionsFromFacetWithZeroAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selectors"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4[]"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CannotReplaceImmutableFunction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CannotReplaceImmutableFunction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_selector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncorrectFacetCutAction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "IncorrectFacetCutAction",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_action"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InitializationFunctionReverted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InitializationFunctionReverted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_initializationContractAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_calldata"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoBytecodeAtAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NoBytecodeAtAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_contractAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NoSelectorsProvidedForFacetForCut",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NoSelectorsProvidedForFacetForCut",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_facetAddress"),
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
                    ::std::borrow::ToOwned::to_owned("NotContractOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotContractOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_contractOwner"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "RemoveFacetAddressMustBeZeroAddress",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RemoveFacetAddressMustBeZeroAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_facetAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DIAMONDCUTFACET_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0F\xC2\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x1F\x93\x1C\x1C\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\t\xCAV[a\0EV[\0[a\0Ma\0\x9EV[a\0\x97a\0Z\x85\x87a\x0B\x10V[\x84\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x01\x17\x92PPPV[PPPPPV[`\0\x80Q` a\x0E\xF9\x839\x81Q\x91R`\x03\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x15W\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1FT`@Q`\x01b\xBE\xD85`\xE0\x1B\x03\x19\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[V[`\0[\x83Q\x81\x10\x15a\x02|W`\0\x84\x82\x81Q\x81\x10a\x017Wa\x017a\x0CTV[` \x02` \x01\x01Q`@\x01Q\x90P`\0\x85\x83\x81Q\x81\x10a\x01YWa\x01Ya\x0CTV[` \x02` \x01\x01Q`\0\x01Q\x90P\x81Q`\0\x03a\x01\x94W`@Qc\xE7g\xF9\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x01\x0CV[`\0\x86\x84\x81Q\x81\x10a\x01\xA8Wa\x01\xA8a\x0CTV[` \x02` \x01\x01Q` \x01Q\x90P`\0`\x02\x81\x11\x15a\x01\xC9Wa\x01\xC9a\x0CjV[\x81`\x02\x81\x11\x15a\x01\xDBWa\x01\xDBa\x0CjV[\x03a\x01\xEFWa\x01\xEA\x82\x84a\x02\xC7V[a\x02fV[`\x01\x81`\x02\x81\x11\x15a\x02\x03Wa\x02\x03a\x0CjV[\x03a\x02\x12Wa\x01\xEA\x82\x84a\x04{V[`\x02\x81`\x02\x81\x11\x15a\x02&Wa\x02&a\x0CjV[\x03a\x025Wa\x01\xEA\x82\x84a\x05\xF0V[\x80`\x02\x81\x11\x15a\x02GWa\x02Ga\x0CjV[`@Qc?\xF4\xD2\x0F`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x01\x0CV[PPP\x80\x80a\x02t\x90a\x0C\x96V[\x91PPa\x01\x1AV[P\x7F\x8F\xAAp\x87\x86q\xCC\xD2\x12\xD2\x07q\xB7\x95\xC5\n\xF8\xFD?\xF6\xCF'\xF4\xBD\xE5~]M\xE0\xAE\xB6s\x83\x83\x83`@Qa\x02\xB0\x93\x92\x91\x90a\x0C\xFFV[`@Q\x80\x91\x03\x90\xA1a\x02\xC2\x82\x82a\x08wV[PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02\xF0W\x80`@Qc\x02\xB8\xDA\x07`\xE2\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\r\xFFV[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1DT`@\x80Q``\x81\x01\x90\x91R`$\x80\x82R`\0\x80Q` a\x0E\xF9\x839\x81Q\x91R\x92\x91a\x03G\x91\x86\x91\x90a\x0F\x19` \x83\x019a\t=V[`\0[\x83Q\x81\x10\x15a\0\x97W`\0\x84\x82\x81Q\x81\x10a\x03gWa\x03ga\x0CTV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\0\x90\x81R\x91\x86\x90R`@\x90\x91 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x03\xBAW\x81`@Qc\xEB\xBF]\x07`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16\x82Ra\xFF\xFF\x80\x88\x16` \x80\x85\x01\x91\x82R`\x01`\x01`\xE0\x1B\x03\x19\x88\x16`\0\x90\x81R\x8B\x82R\x95\x86 \x94Q\x85T\x92Q\x90\x93\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x92\x90\x93\x16\x91\x90\x91\x17\x17\x90\x91U`\x01\x80\x88\x01\x80T\x91\x82\x01\x81U\x83R\x91 `\x08\x82\x04\x01\x80T`\xE0\x85\x90\x1C`\x04`\x07\x90\x94\x16\x93\x90\x93\x02a\x01\0\n\x92\x83\x02c\xFF\xFF\xFF\xFF\x90\x93\x02\x19\x16\x91\x90\x91\x17\x90U\x83a\x04c\x81a\x0EbV[\x94PPPP\x80\x80a\x04s\x90a\x0C\x96V[\x91PPa\x03JV[`\0\x80Q` a\x0E\xF9\x839\x81Q\x91R`\x01`\x01`\xA0\x1B\x03\x83\x16a\x04\xB3W\x81`@Qc\xCD\x98\xA9o`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\r\xFFV[a\x04\xD5\x83`@Q\x80``\x01`@R\x80`(\x81R` \x01a\x0Fe`(\x919a\t=V[`\0[\x82Q\x81\x10\x15a\x05\xEAW`\0\x83\x82\x81Q\x81\x10a\x04\xF5Wa\x04\xF5a\x0CTV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\0\x90\x81R\x91\x85\x90R`@\x90\x91 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x160\x81\x03a\x05IW\x81`@Qc)\x01\x80m`\xE1\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[\x85`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05}W\x81`@Qc\x1A\xC6\xCE\x8D`\xE1\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\xA6W\x81`@Qcty\xF99`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[P`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R` \x83\x90R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x90U\x80a\x05\xE2\x81a\x0C\x96V[\x91PPa\x04\xD8V[PPPPV[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1DT`\0\x80Q` a\x0E\xF9\x839\x81Q\x91R\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x06UW`@Qc\xD0\x91\xBC\x81`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x01\x0CV[`\0[\x83Q\x81\x10\x15a\0\x97W`\0\x84\x82\x81Q\x81\x10a\x06uWa\x06ua\x0CTV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\0\x90\x81R\x86\x83R`@\x90\x81\x90 \x81Q\x80\x83\x01\x90\x92RT`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x83R`\x01`\xA0\x1B\x90\x91\x04a\xFF\xFF\x16\x93\x82\x01\x93\x90\x93R\x90\x92P\x90a\x06\xE6W\x81`@Qcz\x08\xA2-`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[\x80Q0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x07\x14W\x81`@Qc\r\xF5\xFDa`\xE3\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[\x83a\x07\x1E\x81a\x0E\x83V[\x94PP\x83\x81` \x01Qa\xFF\xFF\x16\x14a\x07\xFCW`\0\x85`\x01\x01\x85\x81T\x81\x10a\x07GWa\x07Ga\x0CTV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B\x90P\x80\x86`\x01\x01\x83` \x01Qa\xFF\xFF\x16\x81T\x81\x10a\x07\x8AWa\x07\x8Aa\x0CTV[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01\x80Tc\xFF\xFF\xFF\xFF`\x07\x90\x94\x16`\x04\x02a\x01\0\n\x93\x84\x02\x19\x16`\xE0\x95\x90\x95\x1C\x92\x90\x92\x02\x93\x90\x93\x17\x90U\x83\x82\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x93\x90\x93\x16\x81R\x90\x87\x90R`@\x90 \x80Ta\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1Ba\xFF\xFF\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90U[\x84`\x01\x01\x80T\x80a\x08\x0FWa\x08\x0Fa\x0E\x9AV[`\0\x82\x81R` \x80\x82 `\x08`\0\x19\x90\x94\x01\x93\x84\x04\x01\x80Tc\xFF\xFF\xFF\xFF`\x04`\x07\x87\x16\x02a\x01\0\n\x02\x19\x16\x90U\x91\x90\x92U`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x81R\x91\x85\x90RP`@\x90 \x80T`\x01`\x01`\xB0\x1B\x03\x19\x16\x90U\x80a\x08o\x81a\x0C\x96V[\x91PPa\x06XV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x08\x89WPPV[a\x08\xAB\x82`@Q\x80``\x01`@R\x80`(\x81R` \x01a\x0F=`(\x919a\t=V[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x08\xC6\x91\x90a\x0E\xB0V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\t\x01W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\x06V[``\x91P[P\x91P\x91P\x81a\x05\xEAW\x80Q\x15a\t W\x80Q\x80\x82` \x01\xFD[\x83\x83`@Qc\x19!\x05\xD7`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x92\x91\x90a\x0E\xCCV[\x81;`\0\x81\x90\x03a\x02\xC2W\x82\x82`@Qc\x91\x984\xB9`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x92\x91\x90a\x0E\xCCV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t|W`\0\x80\xFD[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\t\x93W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xABW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\t\xC3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\t\xE2W`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\xFAW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\n\x0EW`\0\x80\xFD[\x815\x81\x81\x11\x15a\n\x1DW`\0\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\n2W`\0\x80\xFD[` \x83\x01\x97P\x80\x96PPa\nH` \x89\x01a\teV[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\n^W`\0\x80\xFD[Pa\nk\x88\x82\x89\x01a\t\x81V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xB5Wa\n\xB5a\n|V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xE4Wa\n\xE4a\n|V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\x06Wa\x0B\x06a\n|V[P`\x05\x1B` \x01\x90V[`\0a\x0B#a\x0B\x1E\x84a\n\xECV[a\n\xBBV[\x83\x81R` \x80\x82\x01\x91\x90`\x05\x86\x81\x1B\x86\x016\x81\x11\x15a\x0BAW`\0\x80\xFD[\x86[\x81\x81\x10\x15a\x0CGW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BdW`\0\x80\x81\xFD[\x81\x8A\x01\x91P``\x826\x03\x12\x15a\x0BzW`\0\x80\x81\xFD[a\x0B\x82a\n\x92V[a\x0B\x8B\x83a\teV[\x81R\x86\x83\x015`\x03\x81\x10a\x0B\x9FW`\0\x80\x81\xFD[\x81\x88\x01R`@\x83\x81\x015\x83\x81\x11\x15a\x0B\xB7W`\0\x80\x81\xFD[\x93\x90\x93\x01\x926`\x1F\x85\x01\x12a\x0B\xCEW`\0\x92P\x82\x83\xFD[\x835\x92Pa\x0B\xDEa\x0B\x1E\x84a\n\xECV[\x83\x81R\x92\x87\x1B\x84\x01\x88\x01\x92\x88\x81\x01\x906\x85\x11\x15a\x0B\xFBW`\0\x80\x81\xFD[\x94\x89\x01\x94[\x84\x86\x10\x15a\x0C0W\x855`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0C!W`\0\x80\x81\xFD[\x82R\x94\x89\x01\x94\x90\x89\x01\x90a\x0C\0V[\x91\x83\x01\x91\x90\x91RP\x88RPP\x94\x83\x01\x94\x83\x01a\x0BCV[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x0C\xA8Wa\x0C\xA8a\x0C\x80V[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15a\x0C\xCAW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0C\xB2V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0C\xEB\x81` \x86\x01` \x86\x01a\x0C\xAFV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0``\x80\x83\x01\x81\x84R\x80\x87Q\x80\x83R`\x80\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01` \x80\x8B\x01`\0[\x84\x81\x10\x15a\r\xCFW\x89\x84\x03`\x7F\x19\x01\x86R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x83\x81\x01Q\x89\x86\x01\x90`\x03\x81\x10a\rnWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x86\x86\x01R`@\x91\x82\x01Q\x91\x86\x01\x8A\x90R\x81Q\x90\x81\x90R\x90\x84\x01\x90`\0\x90\x89\x87\x01\x90[\x80\x83\x10\x15a\r\xBAW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x86\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x86\x01\x90a\r\x90V[P\x97\x85\x01\x97\x95PPP\x90\x82\x01\x90`\x01\x01a\r(V[PP`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90\x88\x01R\x86\x81\x03`@\x88\x01Ra\r\xF1\x81\x89a\x0C\xD3V[\x9A\x99PPPPPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0EAW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E\x1BV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xE0\x1B\x03\x19\x91\x90\x91\x16\x81R` \x01\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a\x0EyWa\x0Eya\x0C\x80V[`\x01\x01\x93\x92PPPV[`\0\x81a\x0E\x92Wa\x0E\x92a\x0C\x80V[P`\0\x19\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82Qa\x0E\xC2\x81\x84` \x87\x01a\x0C\xAFV[\x91\x90\x91\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x0E\xF0\x90\x83\x01\x84a\x0C\xD3V[\x94\x93PPPPV\xFE\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1CLibDiamondCut: Add facet has no codeLibDiamondCut: _init address has no codeLibDiamondCut: Replace facet has no code\xA2dipfsX\"\x12 l\xE5\xABe\x9B\x93E\x86\xC34\xF1\xE5\x11\xB3\xECP\x91^\xE6\x05\x05\xF7\x8A\xDF\x0C\x89B\x80\x81\xAB\xD5\xB5dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static DIAMONDCUTFACET_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x1F\x93\x1C\x1C\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\t\xCAV[a\0EV[\0[a\0Ma\0\x9EV[a\0\x97a\0Z\x85\x87a\x0B\x10V[\x84\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x01\x17\x92PPPV[PPPPPV[`\0\x80Q` a\x0E\xF9\x839\x81Q\x91R`\x03\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x15W\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1FT`@Q`\x01b\xBE\xD85`\xE0\x1B\x03\x19\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[V[`\0[\x83Q\x81\x10\x15a\x02|W`\0\x84\x82\x81Q\x81\x10a\x017Wa\x017a\x0CTV[` \x02` \x01\x01Q`@\x01Q\x90P`\0\x85\x83\x81Q\x81\x10a\x01YWa\x01Ya\x0CTV[` \x02` \x01\x01Q`\0\x01Q\x90P\x81Q`\0\x03a\x01\x94W`@Qc\xE7g\xF9\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x01\x0CV[`\0\x86\x84\x81Q\x81\x10a\x01\xA8Wa\x01\xA8a\x0CTV[` \x02` \x01\x01Q` \x01Q\x90P`\0`\x02\x81\x11\x15a\x01\xC9Wa\x01\xC9a\x0CjV[\x81`\x02\x81\x11\x15a\x01\xDBWa\x01\xDBa\x0CjV[\x03a\x01\xEFWa\x01\xEA\x82\x84a\x02\xC7V[a\x02fV[`\x01\x81`\x02\x81\x11\x15a\x02\x03Wa\x02\x03a\x0CjV[\x03a\x02\x12Wa\x01\xEA\x82\x84a\x04{V[`\x02\x81`\x02\x81\x11\x15a\x02&Wa\x02&a\x0CjV[\x03a\x025Wa\x01\xEA\x82\x84a\x05\xF0V[\x80`\x02\x81\x11\x15a\x02GWa\x02Ga\x0CjV[`@Qc?\xF4\xD2\x0F`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R`$\x01a\x01\x0CV[PPP\x80\x80a\x02t\x90a\x0C\x96V[\x91PPa\x01\x1AV[P\x7F\x8F\xAAp\x87\x86q\xCC\xD2\x12\xD2\x07q\xB7\x95\xC5\n\xF8\xFD?\xF6\xCF'\xF4\xBD\xE5~]M\xE0\xAE\xB6s\x83\x83\x83`@Qa\x02\xB0\x93\x92\x91\x90a\x0C\xFFV[`@Q\x80\x91\x03\x90\xA1a\x02\xC2\x82\x82a\x08wV[PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x02\xF0W\x80`@Qc\x02\xB8\xDA\x07`\xE2\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\r\xFFV[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1DT`@\x80Q``\x81\x01\x90\x91R`$\x80\x82R`\0\x80Q` a\x0E\xF9\x839\x81Q\x91R\x92\x91a\x03G\x91\x86\x91\x90a\x0F\x19` \x83\x019a\t=V[`\0[\x83Q\x81\x10\x15a\0\x97W`\0\x84\x82\x81Q\x81\x10a\x03gWa\x03ga\x0CTV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\0\x90\x81R\x91\x86\x90R`@\x90\x91 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x03\xBAW\x81`@Qc\xEB\xBF]\x07`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16\x82Ra\xFF\xFF\x80\x88\x16` \x80\x85\x01\x91\x82R`\x01`\x01`\xE0\x1B\x03\x19\x88\x16`\0\x90\x81R\x8B\x82R\x95\x86 \x94Q\x85T\x92Q\x90\x93\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x92\x90\x93\x16\x91\x90\x91\x17\x17\x90\x91U`\x01\x80\x88\x01\x80T\x91\x82\x01\x81U\x83R\x91 `\x08\x82\x04\x01\x80T`\xE0\x85\x90\x1C`\x04`\x07\x90\x94\x16\x93\x90\x93\x02a\x01\0\n\x92\x83\x02c\xFF\xFF\xFF\xFF\x90\x93\x02\x19\x16\x91\x90\x91\x17\x90U\x83a\x04c\x81a\x0EbV[\x94PPPP\x80\x80a\x04s\x90a\x0C\x96V[\x91PPa\x03JV[`\0\x80Q` a\x0E\xF9\x839\x81Q\x91R`\x01`\x01`\xA0\x1B\x03\x83\x16a\x04\xB3W\x81`@Qc\xCD\x98\xA9o`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\r\xFFV[a\x04\xD5\x83`@Q\x80``\x01`@R\x80`(\x81R` \x01a\x0Fe`(\x919a\t=V[`\0[\x82Q\x81\x10\x15a\x05\xEAW`\0\x83\x82\x81Q\x81\x10a\x04\xF5Wa\x04\xF5a\x0CTV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\0\x90\x81R\x91\x85\x90R`@\x90\x91 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x160\x81\x03a\x05IW\x81`@Qc)\x01\x80m`\xE1\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[\x85`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05}W\x81`@Qc\x1A\xC6\xCE\x8D`\xE1\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\xA6W\x81`@Qcty\xF99`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[P`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R` \x83\x90R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x90U\x80a\x05\xE2\x81a\x0C\x96V[\x91PPa\x04\xD8V[PPPPV[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1DT`\0\x80Q` a\x0E\xF9\x839\x81Q\x91R\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x06UW`@Qc\xD0\x91\xBC\x81`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x01\x0CV[`\0[\x83Q\x81\x10\x15a\0\x97W`\0\x84\x82\x81Q\x81\x10a\x06uWa\x06ua\x0CTV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16`\0\x90\x81R\x86\x83R`@\x90\x81\x90 \x81Q\x80\x83\x01\x90\x92RT`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x83R`\x01`\xA0\x1B\x90\x91\x04a\xFF\xFF\x16\x93\x82\x01\x93\x90\x93R\x90\x92P\x90a\x06\xE6W\x81`@Qcz\x08\xA2-`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[\x80Q0`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x07\x14W\x81`@Qc\r\xF5\xFDa`\xE3\x1B\x81R`\x04\x01a\x01\x0C\x91\x90a\x0EMV[\x83a\x07\x1E\x81a\x0E\x83V[\x94PP\x83\x81` \x01Qa\xFF\xFF\x16\x14a\x07\xFCW`\0\x85`\x01\x01\x85\x81T\x81\x10a\x07GWa\x07Ga\x0CTV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B\x90P\x80\x86`\x01\x01\x83` \x01Qa\xFF\xFF\x16\x81T\x81\x10a\x07\x8AWa\x07\x8Aa\x0CTV[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01\x80Tc\xFF\xFF\xFF\xFF`\x07\x90\x94\x16`\x04\x02a\x01\0\n\x93\x84\x02\x19\x16`\xE0\x95\x90\x95\x1C\x92\x90\x92\x02\x93\x90\x93\x17\x90U\x83\x82\x01Q`\x01`\x01`\xE0\x1B\x03\x19\x93\x90\x93\x16\x81R\x90\x87\x90R`@\x90 \x80Ta\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1Ba\xFF\xFF\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90U[\x84`\x01\x01\x80T\x80a\x08\x0FWa\x08\x0Fa\x0E\x9AV[`\0\x82\x81R` \x80\x82 `\x08`\0\x19\x90\x94\x01\x93\x84\x04\x01\x80Tc\xFF\xFF\xFF\xFF`\x04`\x07\x87\x16\x02a\x01\0\n\x02\x19\x16\x90U\x91\x90\x92U`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x81R\x91\x85\x90RP`@\x90 \x80T`\x01`\x01`\xB0\x1B\x03\x19\x16\x90U\x80a\x08o\x81a\x0C\x96V[\x91PPa\x06XV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x08\x89WPPV[a\x08\xAB\x82`@Q\x80``\x01`@R\x80`(\x81R` \x01a\x0F=`(\x919a\t=V[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa\x08\xC6\x91\x90a\x0E\xB0V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\t\x01W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\x06V[``\x91P[P\x91P\x91P\x81a\x05\xEAW\x80Q\x15a\t W\x80Q\x80\x82` \x01\xFD[\x83\x83`@Qc\x19!\x05\xD7`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x92\x91\x90a\x0E\xCCV[\x81;`\0\x81\x90\x03a\x02\xC2W\x82\x82`@Qc\x91\x984\xB9`\xE0\x1B\x81R`\x04\x01a\x01\x0C\x92\x91\x90a\x0E\xCCV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t|W`\0\x80\xFD[\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\t\x93W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xABW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\t\xC3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a\t\xE2W`\0\x80\xFD[\x855g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\xFAW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\n\x0EW`\0\x80\xFD[\x815\x81\x81\x11\x15a\n\x1DW`\0\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\n2W`\0\x80\xFD[` \x83\x01\x97P\x80\x96PPa\nH` \x89\x01a\teV[\x94P`@\x88\x015\x91P\x80\x82\x11\x15a\n^W`\0\x80\xFD[Pa\nk\x88\x82\x89\x01a\t\x81V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xB5Wa\n\xB5a\n|V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xE4Wa\n\xE4a\n|V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\x06Wa\x0B\x06a\n|V[P`\x05\x1B` \x01\x90V[`\0a\x0B#a\x0B\x1E\x84a\n\xECV[a\n\xBBV[\x83\x81R` \x80\x82\x01\x91\x90`\x05\x86\x81\x1B\x86\x016\x81\x11\x15a\x0BAW`\0\x80\xFD[\x86[\x81\x81\x10\x15a\x0CGW\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BdW`\0\x80\x81\xFD[\x81\x8A\x01\x91P``\x826\x03\x12\x15a\x0BzW`\0\x80\x81\xFD[a\x0B\x82a\n\x92V[a\x0B\x8B\x83a\teV[\x81R\x86\x83\x015`\x03\x81\x10a\x0B\x9FW`\0\x80\x81\xFD[\x81\x88\x01R`@\x83\x81\x015\x83\x81\x11\x15a\x0B\xB7W`\0\x80\x81\xFD[\x93\x90\x93\x01\x926`\x1F\x85\x01\x12a\x0B\xCEW`\0\x92P\x82\x83\xFD[\x835\x92Pa\x0B\xDEa\x0B\x1E\x84a\n\xECV[\x83\x81R\x92\x87\x1B\x84\x01\x88\x01\x92\x88\x81\x01\x906\x85\x11\x15a\x0B\xFBW`\0\x80\x81\xFD[\x94\x89\x01\x94[\x84\x86\x10\x15a\x0C0W\x855`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0C!W`\0\x80\x81\xFD[\x82R\x94\x89\x01\x94\x90\x89\x01\x90a\x0C\0V[\x91\x83\x01\x91\x90\x91RP\x88RPP\x94\x83\x01\x94\x83\x01a\x0BCV[P\x92\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x0C\xA8Wa\x0C\xA8a\x0C\x80V[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15a\x0C\xCAW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0C\xB2V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0C\xEB\x81` \x86\x01` \x86\x01a\x0C\xAFV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0``\x80\x83\x01\x81\x84R\x80\x87Q\x80\x83R`\x80\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01` \x80\x8B\x01`\0[\x84\x81\x10\x15a\r\xCFW\x89\x84\x03`\x7F\x19\x01\x86R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x83\x81\x01Q\x89\x86\x01\x90`\x03\x81\x10a\rnWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x86\x86\x01R`@\x91\x82\x01Q\x91\x86\x01\x8A\x90R\x81Q\x90\x81\x90R\x90\x84\x01\x90`\0\x90\x89\x87\x01\x90[\x80\x83\x10\x15a\r\xBAW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x86\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x86\x01\x90a\r\x90V[P\x97\x85\x01\x97\x95PPP\x90\x82\x01\x90`\x01\x01a\r(V[PP`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90\x88\x01R\x86\x81\x03`@\x88\x01Ra\r\xF1\x81\x89a\x0C\xD3V[\x9A\x99PPPPPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0EAW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E\x1BV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xE0\x1B\x03\x19\x91\x90\x91\x16\x81R` \x01\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a\x0EyWa\x0Eya\x0C\x80V[`\x01\x01\x93\x92PPPV[`\0\x81a\x0E\x92Wa\x0E\x92a\x0C\x80V[P`\0\x19\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82Qa\x0E\xC2\x81\x84` \x87\x01a\x0C\xAFV[\x91\x90\x91\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x0E\xF0\x90\x83\x01\x84a\x0C\xD3V[\x94\x93PPPPV\xFE\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1CLibDiamondCut: Add facet has no codeLibDiamondCut: _init address has no codeLibDiamondCut: Replace facet has no code\xA2dipfsX\"\x12 l\xE5\xABe\x9B\x93E\x86\xC34\xF1\xE5\x11\xB3\xECP\x91^\xE6\x05\x05\xF7\x8A\xDF\x0C\x89B\x80\x81\xAB\xD5\xB5dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static DIAMONDCUTFACET_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DiamondCutFacet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DiamondCutFacet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DiamondCutFacet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DiamondCutFacet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DiamondCutFacet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DiamondCutFacet))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DiamondCutFacet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DIAMONDCUTFACET_ABI.clone(),
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
                DIAMONDCUTFACET_ABI.clone(),
                DIAMONDCUTFACET_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `diamondCut` (0x1f931c1c) function
        pub fn diamond_cut(
            &self,
            diamond_cut: ::std::vec::Vec<FacetCut>,
            init: ::ethers::core::types::Address,
            calldata: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 147, 28, 28], (diamond_cut, init, calldata))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DiamondCut` event
        pub fn diamond_cut_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DiamondCutFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DiamondCutFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DiamondCutFacet<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CannotAddFunctionToDiamondThatAlreadyExists` with signature `CannotAddFunctionToDiamondThatAlreadyExists(bytes4)` and selector `0xebbf5d07`
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
        name = "CannotAddFunctionToDiamondThatAlreadyExists",
        abi = "CannotAddFunctionToDiamondThatAlreadyExists(bytes4)"
    )]
    pub struct CannotAddFunctionToDiamondThatAlreadyExists {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotAddSelectorsToZeroAddress` with signature `CannotAddSelectorsToZeroAddress(bytes4[])` and selector `0x0ae3681c`
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
        name = "CannotAddSelectorsToZeroAddress",
        abi = "CannotAddSelectorsToZeroAddress(bytes4[])"
    )]
    pub struct CannotAddSelectorsToZeroAddress {
        pub selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///Custom Error type `CannotRemoveFunctionThatDoesNotExist` with signature `CannotRemoveFunctionThatDoesNotExist(bytes4)` and selector `0x7a08a22d`
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
        name = "CannotRemoveFunctionThatDoesNotExist",
        abi = "CannotRemoveFunctionThatDoesNotExist(bytes4)"
    )]
    pub struct CannotRemoveFunctionThatDoesNotExist {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotRemoveImmutableFunction` with signature `CannotRemoveImmutableFunction(bytes4)` and selector `0x6fafeb08`
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
        name = "CannotRemoveImmutableFunction",
        abi = "CannotRemoveImmutableFunction(bytes4)"
    )]
    pub struct CannotRemoveImmutableFunction {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotReplaceFunctionThatDoesNotExists` with signature `CannotReplaceFunctionThatDoesNotExists(bytes4)` and selector `0x7479f939`
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
        name = "CannotReplaceFunctionThatDoesNotExists",
        abi = "CannotReplaceFunctionThatDoesNotExists(bytes4)"
    )]
    pub struct CannotReplaceFunctionThatDoesNotExists {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet` with signature `CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(bytes4)` and selector `0x358d9d1a`
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
        name = "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet",
        abi = "CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(bytes4)"
    )]
    pub struct CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet {
        pub selector: [u8; 4],
    }
    ///Custom Error type `CannotReplaceFunctionsFromFacetWithZeroAddress` with signature `CannotReplaceFunctionsFromFacetWithZeroAddress(bytes4[])` and selector `0xcd98a96f`
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
        name = "CannotReplaceFunctionsFromFacetWithZeroAddress",
        abi = "CannotReplaceFunctionsFromFacetWithZeroAddress(bytes4[])"
    )]
    pub struct CannotReplaceFunctionsFromFacetWithZeroAddress {
        pub selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///Custom Error type `CannotReplaceImmutableFunction` with signature `CannotReplaceImmutableFunction(bytes4)` and selector `0x520300da`
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
        name = "CannotReplaceImmutableFunction",
        abi = "CannotReplaceImmutableFunction(bytes4)"
    )]
    pub struct CannotReplaceImmutableFunction {
        pub selector: [u8; 4],
    }
    ///Custom Error type `IncorrectFacetCutAction` with signature `IncorrectFacetCutAction(uint8)` and selector `0x7fe9a41e`
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
    #[etherror(name = "IncorrectFacetCutAction", abi = "IncorrectFacetCutAction(uint8)")]
    pub struct IncorrectFacetCutAction {
        pub action: u8,
    }
    ///Custom Error type `InitializationFunctionReverted` with signature `InitializationFunctionReverted(address,bytes)` and selector `0x192105d7`
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
        name = "InitializationFunctionReverted",
        abi = "InitializationFunctionReverted(address,bytes)"
    )]
    pub struct InitializationFunctionReverted {
        pub initialization_contract_address: ::ethers::core::types::Address,
        pub calldata: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `NoBytecodeAtAddress` with signature `NoBytecodeAtAddress(address,string)` and selector `0x919834b9`
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
        name = "NoBytecodeAtAddress",
        abi = "NoBytecodeAtAddress(address,string)"
    )]
    pub struct NoBytecodeAtAddress {
        pub contract_address: ::ethers::core::types::Address,
        pub message: ::std::string::String,
    }
    ///Custom Error type `NoSelectorsProvidedForFacetForCut` with signature `NoSelectorsProvidedForFacetForCut(address)` and selector `0xe767f91f`
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
        name = "NoSelectorsProvidedForFacetForCut",
        abi = "NoSelectorsProvidedForFacetForCut(address)"
    )]
    pub struct NoSelectorsProvidedForFacetForCut {
        pub facet_address: ::ethers::core::types::Address,
    }
    ///Custom Error type `NotContractOwner` with signature `NotContractOwner(address,address)` and selector `0xff4127cb`
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
    #[etherror(name = "NotContractOwner", abi = "NotContractOwner(address,address)")]
    pub struct NotContractOwner {
        pub user: ::ethers::core::types::Address,
        pub contract_owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `RemoveFacetAddressMustBeZeroAddress` with signature `RemoveFacetAddressMustBeZeroAddress(address)` and selector `0xd091bc81`
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
        name = "RemoveFacetAddressMustBeZeroAddress",
        abi = "RemoveFacetAddressMustBeZeroAddress(address)"
    )]
    pub struct RemoveFacetAddressMustBeZeroAddress {
        pub facet_address: ::ethers::core::types::Address,
    }
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
    pub enum DiamondCutFacetErrors {
        CannotAddFunctionToDiamondThatAlreadyExists(
            CannotAddFunctionToDiamondThatAlreadyExists,
        ),
        CannotAddSelectorsToZeroAddress(CannotAddSelectorsToZeroAddress),
        CannotRemoveFunctionThatDoesNotExist(CannotRemoveFunctionThatDoesNotExist),
        CannotRemoveImmutableFunction(CannotRemoveImmutableFunction),
        CannotReplaceFunctionThatDoesNotExists(CannotReplaceFunctionThatDoesNotExists),
        CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
            CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet,
        ),
        CannotReplaceFunctionsFromFacetWithZeroAddress(
            CannotReplaceFunctionsFromFacetWithZeroAddress,
        ),
        CannotReplaceImmutableFunction(CannotReplaceImmutableFunction),
        IncorrectFacetCutAction(IncorrectFacetCutAction),
        InitializationFunctionReverted(InitializationFunctionReverted),
        NoBytecodeAtAddress(NoBytecodeAtAddress),
        NoSelectorsProvidedForFacetForCut(NoSelectorsProvidedForFacetForCut),
        NotContractOwner(NotContractOwner),
        RemoveFacetAddressMustBeZeroAddress(RemoveFacetAddressMustBeZeroAddress),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DiamondCutFacetErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <CannotAddFunctionToDiamondThatAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotAddFunctionToDiamondThatAlreadyExists(decoded));
            }
            if let Ok(decoded) = <CannotAddSelectorsToZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotAddSelectorsToZeroAddress(decoded));
            }
            if let Ok(decoded) = <CannotRemoveFunctionThatDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotRemoveFunctionThatDoesNotExist(decoded));
            }
            if let Ok(decoded) = <CannotRemoveImmutableFunction as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotRemoveImmutableFunction(decoded));
            }
            if let Ok(decoded) = <CannotReplaceFunctionThatDoesNotExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotReplaceFunctionThatDoesNotExists(decoded));
            }
            if let Ok(decoded) = <CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = <CannotReplaceFunctionsFromFacetWithZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotReplaceFunctionsFromFacetWithZeroAddress(decoded));
            }
            if let Ok(decoded) = <CannotReplaceImmutableFunction as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CannotReplaceImmutableFunction(decoded));
            }
            if let Ok(decoded) = <IncorrectFacetCutAction as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncorrectFacetCutAction(decoded));
            }
            if let Ok(decoded) = <InitializationFunctionReverted as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitializationFunctionReverted(decoded));
            }
            if let Ok(decoded) = <NoBytecodeAtAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NoBytecodeAtAddress(decoded));
            }
            if let Ok(decoded) = <NoSelectorsProvidedForFacetForCut as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NoSelectorsProvidedForFacetForCut(decoded));
            }
            if let Ok(decoded) = <NotContractOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotContractOwner(decoded));
            }
            if let Ok(decoded) = <RemoveFacetAddressMustBeZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveFacetAddressMustBeZeroAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DiamondCutFacetErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CannotAddFunctionToDiamondThatAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotAddSelectorsToZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotRemoveFunctionThatDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotRemoveImmutableFunction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotReplaceFunctionThatDoesNotExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CannotReplaceFunctionsFromFacetWithZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CannotReplaceImmutableFunction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectFacetCutAction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializationFunctionReverted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoBytecodeAtAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoSelectorsProvidedForFacetForCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotContractOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveFacetAddressMustBeZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DiamondCutFacetErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CannotAddFunctionToDiamondThatAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotAddSelectorsToZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotRemoveFunctionThatDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotRemoveImmutableFunction as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceFunctionThatDoesNotExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceFunctionsFromFacetWithZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CannotReplaceImmutableFunction as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <IncorrectFacetCutAction as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InitializationFunctionReverted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoBytecodeAtAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoSelectorsProvidedForFacetForCut as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotContractOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RemoveFacetAddressMustBeZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DiamondCutFacetErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CannotAddFunctionToDiamondThatAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotAddSelectorsToZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotRemoveFunctionThatDoesNotExist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotRemoveImmutableFunction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotReplaceFunctionThatDoesNotExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::CannotReplaceFunctionsFromFacetWithZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CannotReplaceImmutableFunction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncorrectFacetCutAction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializationFunctionReverted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoBytecodeAtAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoSelectorsProvidedForFacetForCut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotContractOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveFacetAddressMustBeZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DiamondCutFacetErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CannotAddFunctionToDiamondThatAlreadyExists>
    for DiamondCutFacetErrors {
        fn from(value: CannotAddFunctionToDiamondThatAlreadyExists) -> Self {
            Self::CannotAddFunctionToDiamondThatAlreadyExists(value)
        }
    }
    impl ::core::convert::From<CannotAddSelectorsToZeroAddress>
    for DiamondCutFacetErrors {
        fn from(value: CannotAddSelectorsToZeroAddress) -> Self {
            Self::CannotAddSelectorsToZeroAddress(value)
        }
    }
    impl ::core::convert::From<CannotRemoveFunctionThatDoesNotExist>
    for DiamondCutFacetErrors {
        fn from(value: CannotRemoveFunctionThatDoesNotExist) -> Self {
            Self::CannotRemoveFunctionThatDoesNotExist(value)
        }
    }
    impl ::core::convert::From<CannotRemoveImmutableFunction> for DiamondCutFacetErrors {
        fn from(value: CannotRemoveImmutableFunction) -> Self {
            Self::CannotRemoveImmutableFunction(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionThatDoesNotExists>
    for DiamondCutFacetErrors {
        fn from(value: CannotReplaceFunctionThatDoesNotExists) -> Self {
            Self::CannotReplaceFunctionThatDoesNotExists(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet>
    for DiamondCutFacetErrors {
        fn from(
            value: CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet,
        ) -> Self {
            Self::CannotReplaceFunctionWithTheSameFunctionFromTheSameFacet(value)
        }
    }
    impl ::core::convert::From<CannotReplaceFunctionsFromFacetWithZeroAddress>
    for DiamondCutFacetErrors {
        fn from(value: CannotReplaceFunctionsFromFacetWithZeroAddress) -> Self {
            Self::CannotReplaceFunctionsFromFacetWithZeroAddress(value)
        }
    }
    impl ::core::convert::From<CannotReplaceImmutableFunction>
    for DiamondCutFacetErrors {
        fn from(value: CannotReplaceImmutableFunction) -> Self {
            Self::CannotReplaceImmutableFunction(value)
        }
    }
    impl ::core::convert::From<IncorrectFacetCutAction> for DiamondCutFacetErrors {
        fn from(value: IncorrectFacetCutAction) -> Self {
            Self::IncorrectFacetCutAction(value)
        }
    }
    impl ::core::convert::From<InitializationFunctionReverted>
    for DiamondCutFacetErrors {
        fn from(value: InitializationFunctionReverted) -> Self {
            Self::InitializationFunctionReverted(value)
        }
    }
    impl ::core::convert::From<NoBytecodeAtAddress> for DiamondCutFacetErrors {
        fn from(value: NoBytecodeAtAddress) -> Self {
            Self::NoBytecodeAtAddress(value)
        }
    }
    impl ::core::convert::From<NoSelectorsProvidedForFacetForCut>
    for DiamondCutFacetErrors {
        fn from(value: NoSelectorsProvidedForFacetForCut) -> Self {
            Self::NoSelectorsProvidedForFacetForCut(value)
        }
    }
    impl ::core::convert::From<NotContractOwner> for DiamondCutFacetErrors {
        fn from(value: NotContractOwner) -> Self {
            Self::NotContractOwner(value)
        }
    }
    impl ::core::convert::From<RemoveFacetAddressMustBeZeroAddress>
    for DiamondCutFacetErrors {
        fn from(value: RemoveFacetAddressMustBeZeroAddress) -> Self {
            Self::RemoveFacetAddressMustBeZeroAddress(value)
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
        name = "DiamondCut",
        abi = "DiamondCut((address,uint8,bytes4[])[],address,bytes)"
    )]
    pub struct DiamondCutFilter {
        pub diamond_cut: ::std::vec::Vec<FacetCut>,
        pub init: ::ethers::core::types::Address,
        pub calldata: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `diamondCut` function with signature `diamondCut((address,uint8,bytes4[])[],address,bytes)` and selector `0x1f931c1c`
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
        name = "diamondCut",
        abi = "diamondCut((address,uint8,bytes4[])[],address,bytes)"
    )]
    pub struct DiamondCutCall {
        pub diamond_cut: ::std::vec::Vec<FacetCut>,
        pub init: ::ethers::core::types::Address,
        pub calldata: ::ethers::core::types::Bytes,
    }
}
