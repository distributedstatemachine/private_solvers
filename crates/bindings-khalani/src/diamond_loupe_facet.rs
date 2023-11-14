pub use diamond_loupe_facet::*;
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
pub mod diamond_loupe_facet {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("facetAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facetAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_functionSelector"),
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
                                    name: ::std::borrow::ToOwned::to_owned("facetAddress_"),
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
                    ::std::borrow::ToOwned::to_owned("facetAddresses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facetAddresses"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("facetAddresses_"),
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
                    ::std::borrow::ToOwned::to_owned("facetFunctionSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "facetFunctionSelectors",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_facet"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_facetFunctionSelectors",
                                    ),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("facets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("facets"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("facets_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
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
                                            "struct IDiamondLoupe.Facet[]",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("_interfaceId"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DIAMONDLOUPEFACET_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\n\x84\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\0\\W\x80cR\xEFk,\x14a\0\xBDW\x80cz\x0E\xD6'\x14a\0\xD2W\x80c\xAD\xFC\xA1^\x14a\0\xE7W\x80c\xCD\xFF\xAC\xC6\x14a\x01\x07W[`\0\x80\xFD[a\0\xA8a\0j6`\x04a\x086V[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1E` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC5a\x01_V[`@Qa\0\xB4\x91\x90a\x08gV[a\0\xDAa\x03\0V[`@Qa\0\xB4\x91\x90a\x08\xF9V[a\0\xFAa\0\xF56`\x04a\tvV[a\x06\xEDV[`@Qa\0\xB4\x91\x90a\t\x9FV[a\x01Ga\x01\x156`\x04a\x086V[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R`\0\x80Q` a\n/\x839\x81Q\x91R` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB4V[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1DT``\x90`\0\x80Q` a\n/\x839\x81Q\x91R\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xADWa\x01\xADa\t\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xD6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0\x80[\x82\x81\x10\x15a\x02\xF6W`\0\x84`\x01\x01\x82\x81T\x81\x10a\x01\xFCWa\x01\xFCa\t\xC8V[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x83R\x90\x87\x90R`@\x82 T\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90\x80[\x85\x81\x10\x15a\x02\x98W\x88\x81\x81Q\x81\x10a\x02]Wa\x02]a\t\xC8V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x02\x86W`\x01\x91Pa\x02\x98V[\x80a\x02\x90\x81a\t\xF4V[\x91PPa\x02CV[P\x80\x15a\x02\xA8WPa\x02\xE4\x91PPV[\x81\x88\x86\x81Q\x81\x10a\x02\xBBWa\x02\xBBa\t\xC8V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\x02\xDD\x81a\t\xF4V[\x95PPPPP[\x80a\x02\xEE\x81a\t\xF4V[\x91PPa\x01\xDDV[P\x80\x84RPPP\x90V[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1DT``\x90`\0\x80Q` a\n/\x839\x81Q\x91R\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03NWa\x03Na\t\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\x94W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03lW\x90P[P\x92P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xB2Wa\x03\xB2a\t\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xDBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x83\x81\x10\x15a\x06zW`\0\x85`\x01\x01\x82\x81T\x81\x10a\x04\x01Wa\x04\x01a\t\xC8V[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x83R\x90\x88\x90R`@\x82 T\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90\x80[\x85\x81\x10\x15a\x05>W\x82`\x01`\x01`\xA0\x1B\x03\x16\x8A\x82\x81Q\x81\x10a\x04lWa\x04la\t\xC8V[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05,W\x83\x8A\x82\x81Q\x81\x10a\x04\x99Wa\x04\x99a\t\xC8V[` \x02` \x01\x01Q` \x01Q\x88\x83\x81Q\x81\x10a\x04\xB7Wa\x04\xB7a\t\xC8V[` \x02` \x01\x01Qa\xFF\xFF\x16\x81Q\x81\x10a\x04\xD3Wa\x04\xD3a\t\xC8V[` \x02` \x01\x01\x90`\x01`\x01`\xE0\x1B\x03\x19\x16\x90\x81`\x01`\x01`\xE0\x1B\x03\x19\x16\x81RPP\x86\x81\x81Q\x81\x10a\x05\x07Wa\x05\x07a\t\xC8V[` \x02` \x01\x01\x80Q\x80\x91\x90a\x05\x1C\x90a\n\rV[a\xFF\xFF\x16\x90RP`\x01\x91Pa\x05>V[\x80a\x056\x81a\t\xF4V[\x91PPa\x04HV[P\x80\x15a\x05NWPa\x06h\x91PPV[\x81\x89\x86\x81Q\x81\x10a\x05aWa\x05aa\t\xC8V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90R\x86g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x92Wa\x05\x92a\t\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xBBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x89\x86\x81Q\x81\x10a\x05\xCEWa\x05\xCEa\t\xC8V[` \x02` \x01\x01Q` \x01\x81\x90RP\x82\x89\x86\x81Q\x81\x10a\x05\xF0Wa\x05\xF0a\t\xC8V[` \x02` \x01\x01Q` \x01Q`\0\x81Q\x81\x10a\x06\x0EWa\x06\x0Ea\t\xC8V[` \x02` \x01\x01\x90`\x01`\x01`\xE0\x1B\x03\x19\x16\x90\x81`\x01`\x01`\xE0\x1B\x03\x19\x16\x81RPP`\x01\x86\x86\x81Q\x81\x10a\x06DWa\x06Da\t\xC8V[a\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\x06a\x81a\t\xF4V[\x95PPPPP[\x80a\x06r\x81a\t\xF4V[\x91PPa\x03\xE2V[P`\0[\x81\x81\x10\x15a\x06\xE2W`\0\x83\x82\x81Q\x81\x10a\x06\x9AWa\x06\x9Aa\t\xC8V[` \x02` \x01\x01Qa\xFF\xFF\x16\x90P`\0\x87\x83\x81Q\x81\x10a\x06\xBCWa\x06\xBCa\t\xC8V[` \x02` \x01\x01Q` \x01Q\x90P\x81\x81RPP\x80\x80a\x06\xDA\x90a\t\xF4V[\x91PPa\x06~V[P\x80\x85RPPPP\x90V[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1DT``\x90`\0\x80Q` a\n/\x839\x81Q\x91R\x90`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07=Wa\x07=a\t\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07fW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x93P`\0[\x82\x81\x10\x15a\x08+W`\0\x84`\x01\x01\x82\x81T\x81\x10a\x07\x8BWa\x07\x8Ba\t\xC8V[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x83R\x90\x87\x90R`@\x90\x91 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x88\x16\x81\x90\x03a\x08\x16W\x81\x87\x85\x81Q\x81\x10a\x07\xEFWa\x07\xEFa\t\xC8V[`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x83a\x08\x12\x81a\t\xF4V[\x94PP[PP\x80\x80a\x08#\x90a\t\xF4V[\x91PPa\x07lV[P\x83RP\x90\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x08HW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x08`W`\0\x80\xFD[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x08\xA8W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x08\x83V[P\x90\x96\x95PPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x08\xEEW\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x08\xC8V[P\x94\x95\x94PPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a\thW\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x87\x01Q\x87\x84\x01\x87\x90Ra\tU\x87\x85\x01\x82a\x08\xB4V[\x95\x88\x01\x95\x93PP\x90\x86\x01\x90`\x01\x01a\t V[P\x90\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\t\x88W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08`W`\0\x80\xFD[` \x81R`\0a\x08`` \x83\x01\x84a\x08\xB4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\n\x06Wa\n\x06a\t\xDEV[P`\x01\x01\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a\n$Wa\n$a\t\xDEV[`\x01\x01\x93\x92PPPV\xFE\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1C\xA2dipfsX\"\x12 \xDC\":ok9b\x99\x8A\x0C\x94\x9C\xE6i1\xA9U\xB3\x81\xE1\xE4\xDA\x16\xDFq\xBE\x1ELjI~\xFDdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static DIAMONDLOUPEFACET_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x01\xFF\xC9\xA7\x14a\0\\W\x80cR\xEFk,\x14a\0\xBDW\x80cz\x0E\xD6'\x14a\0\xD2W\x80c\xAD\xFC\xA1^\x14a\0\xE7W\x80c\xCD\xFF\xAC\xC6\x14a\x01\x07W[`\0\x80\xFD[a\0\xA8a\0j6`\x04a\x086V[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1E` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC5a\x01_V[`@Qa\0\xB4\x91\x90a\x08gV[a\0\xDAa\x03\0V[`@Qa\0\xB4\x91\x90a\x08\xF9V[a\0\xFAa\0\xF56`\x04a\tvV[a\x06\xEDV[`@Qa\0\xB4\x91\x90a\t\x9FV[a\x01Ga\x01\x156`\x04a\x086V[`\x01`\x01`\xE0\x1B\x03\x19\x16`\0\x90\x81R`\0\x80Q` a\n/\x839\x81Q\x91R` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB4V[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1DT``\x90`\0\x80Q` a\n/\x839\x81Q\x91R\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xADWa\x01\xADa\t\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xD6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0\x80[\x82\x81\x10\x15a\x02\xF6W`\0\x84`\x01\x01\x82\x81T\x81\x10a\x01\xFCWa\x01\xFCa\t\xC8V[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x83R\x90\x87\x90R`@\x82 T\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90\x80[\x85\x81\x10\x15a\x02\x98W\x88\x81\x81Q\x81\x10a\x02]Wa\x02]a\t\xC8V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x02\x86W`\x01\x91Pa\x02\x98V[\x80a\x02\x90\x81a\t\xF4V[\x91PPa\x02CV[P\x80\x15a\x02\xA8WPa\x02\xE4\x91PPV[\x81\x88\x86\x81Q\x81\x10a\x02\xBBWa\x02\xBBa\t\xC8V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\x02\xDD\x81a\t\xF4V[\x95PPPPP[\x80a\x02\xEE\x81a\t\xF4V[\x91PPa\x01\xDDV[P\x80\x84RPPP\x90V[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1DT``\x90`\0\x80Q` a\n/\x839\x81Q\x91R\x90\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03NWa\x03Na\t\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\x94W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x03lW\x90P[P\x92P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\xB2Wa\x03\xB2a\t\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\xDBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x83\x81\x10\x15a\x06zW`\0\x85`\x01\x01\x82\x81T\x81\x10a\x04\x01Wa\x04\x01a\t\xC8V[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x83R\x90\x88\x90R`@\x82 T\x90\x92P`\x01`\x01`\xA0\x1B\x03\x16\x90\x80[\x85\x81\x10\x15a\x05>W\x82`\x01`\x01`\xA0\x1B\x03\x16\x8A\x82\x81Q\x81\x10a\x04lWa\x04la\t\xC8V[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05,W\x83\x8A\x82\x81Q\x81\x10a\x04\x99Wa\x04\x99a\t\xC8V[` \x02` \x01\x01Q` \x01Q\x88\x83\x81Q\x81\x10a\x04\xB7Wa\x04\xB7a\t\xC8V[` \x02` \x01\x01Qa\xFF\xFF\x16\x81Q\x81\x10a\x04\xD3Wa\x04\xD3a\t\xC8V[` \x02` \x01\x01\x90`\x01`\x01`\xE0\x1B\x03\x19\x16\x90\x81`\x01`\x01`\xE0\x1B\x03\x19\x16\x81RPP\x86\x81\x81Q\x81\x10a\x05\x07Wa\x05\x07a\t\xC8V[` \x02` \x01\x01\x80Q\x80\x91\x90a\x05\x1C\x90a\n\rV[a\xFF\xFF\x16\x90RP`\x01\x91Pa\x05>V[\x80a\x056\x81a\t\xF4V[\x91PPa\x04HV[P\x80\x15a\x05NWPa\x06h\x91PPV[\x81\x89\x86\x81Q\x81\x10a\x05aWa\x05aa\t\xC8V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90R\x86g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x92Wa\x05\x92a\t\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xBBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x89\x86\x81Q\x81\x10a\x05\xCEWa\x05\xCEa\t\xC8V[` \x02` \x01\x01Q` \x01\x81\x90RP\x82\x89\x86\x81Q\x81\x10a\x05\xF0Wa\x05\xF0a\t\xC8V[` \x02` \x01\x01Q` \x01Q`\0\x81Q\x81\x10a\x06\x0EWa\x06\x0Ea\t\xC8V[` \x02` \x01\x01\x90`\x01`\x01`\xE0\x1B\x03\x19\x16\x90\x81`\x01`\x01`\xE0\x1B\x03\x19\x16\x81RPP`\x01\x86\x86\x81Q\x81\x10a\x06DWa\x06Da\t\xC8V[a\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\x06a\x81a\t\xF4V[\x95PPPPP[\x80a\x06r\x81a\t\xF4V[\x91PPa\x03\xE2V[P`\0[\x81\x81\x10\x15a\x06\xE2W`\0\x83\x82\x81Q\x81\x10a\x06\x9AWa\x06\x9Aa\t\xC8V[` \x02` \x01\x01Qa\xFF\xFF\x16\x90P`\0\x87\x83\x81Q\x81\x10a\x06\xBCWa\x06\xBCa\t\xC8V[` \x02` \x01\x01Q` \x01Q\x90P\x81\x81RPP\x80\x80a\x06\xDA\x90a\t\xF4V[\x91PPa\x06~V[P\x80\x85RPPPP\x90V[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1DT``\x90`\0\x80Q` a\n/\x839\x81Q\x91R\x90`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07=Wa\x07=a\t\xB2V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07fW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x93P`\0[\x82\x81\x10\x15a\x08+W`\0\x84`\x01\x01\x82\x81T\x81\x10a\x07\x8BWa\x07\x8Ba\t\xC8V[`\0\x91\x82R` \x80\x83 `\x08\x83\x04\x01T`\x07\x90\x92\x16`\x04\x02a\x01\0\n\x90\x91\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x83R\x90\x87\x90R`@\x90\x91 T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x88\x16\x81\x90\x03a\x08\x16W\x81\x87\x85\x81Q\x81\x10a\x07\xEFWa\x07\xEFa\t\xC8V[`\x01`\x01`\xE0\x1B\x03\x19\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x83a\x08\x12\x81a\t\xF4V[\x94PP[PP\x80\x80a\x08#\x90a\t\xF4V[\x91PPa\x07lV[P\x83RP\x90\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x08HW`\0\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x08`W`\0\x80\xFD[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x08\xA8W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x08\x83V[P\x90\x96\x95PPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x08\xEEW\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x08\xC8V[P\x94\x95\x94PPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a\thW\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x87\x01Q\x87\x84\x01\x87\x90Ra\tU\x87\x85\x01\x82a\x08\xB4V[\x95\x88\x01\x95\x93PP\x90\x86\x01\x90`\x01\x01a\t V[P\x90\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\t\x88W`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08`W`\0\x80\xFD[` \x81R`\0a\x08`` \x83\x01\x84a\x08\xB4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\n\x06Wa\n\x06a\t\xDEV[P`\x01\x01\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a\n$Wa\n$a\t\xDEV[`\x01\x01\x93\x92PPPV\xFE\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1C\xA2dipfsX\"\x12 \xDC\":ok9b\x99\x8A\x0C\x94\x9C\xE6i1\xA9U\xB3\x81\xE1\xE4\xDA\x16\xDFq\xBE\x1ELjI~\xFDdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static DIAMONDLOUPEFACET_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DiamondLoupeFacet<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DiamondLoupeFacet<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DiamondLoupeFacet<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DiamondLoupeFacet<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DiamondLoupeFacet<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DiamondLoupeFacet))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DiamondLoupeFacet<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DIAMONDLOUPEFACET_ABI.clone(),
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
                DIAMONDLOUPEFACET_ABI.clone(),
                DIAMONDLOUPEFACET_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `facetAddress` (0xcdffacc6) function
        pub fn facet_address(
            &self,
            function_selector: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([205, 255, 172, 198], function_selector)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facetAddresses` (0x52ef6b2c) function
        pub fn facet_addresses(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([82, 239, 107, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facetFunctionSelectors` (0xadfca15e) function
        pub fn facet_function_selectors(
            &self,
            facet: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 4]>> {
            self.0
                .method_hash([173, 252, 161, 94], facet)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `facets` (0x7a0ed627) function
        pub fn facets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Facet>> {
            self.0
                .method_hash([122, 14, 214, 39], ())
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
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DiamondLoupeFacet<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `facetAddress` function with signature `facetAddress(bytes4)` and selector `0xcdffacc6`
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
    #[ethcall(name = "facetAddress", abi = "facetAddress(bytes4)")]
    pub struct FacetAddressCall {
        pub function_selector: [u8; 4],
    }
    ///Container type for all input parameters for the `facetAddresses` function with signature `facetAddresses()` and selector `0x52ef6b2c`
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
    #[ethcall(name = "facetAddresses", abi = "facetAddresses()")]
    pub struct FacetAddressesCall;
    ///Container type for all input parameters for the `facetFunctionSelectors` function with signature `facetFunctionSelectors(address)` and selector `0xadfca15e`
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
    #[ethcall(name = "facetFunctionSelectors", abi = "facetFunctionSelectors(address)")]
    pub struct FacetFunctionSelectorsCall {
        pub facet: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `facets` function with signature `facets()` and selector `0x7a0ed627`
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
    #[ethcall(name = "facets", abi = "facets()")]
    pub struct FacetsCall;
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
    pub enum DiamondLoupeFacetCalls {
        FacetAddress(FacetAddressCall),
        FacetAddresses(FacetAddressesCall),
        FacetFunctionSelectors(FacetFunctionSelectorsCall),
        Facets(FacetsCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for DiamondLoupeFacetCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FacetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FacetAddress(decoded));
            }
            if let Ok(decoded) = <FacetAddressesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FacetAddresses(decoded));
            }
            if let Ok(decoded) = <FacetFunctionSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FacetFunctionSelectors(decoded));
            }
            if let Ok(decoded) = <FacetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Facets(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DiamondLoupeFacetCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::FacetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FacetAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FacetFunctionSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Facets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DiamondLoupeFacetCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FacetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::FacetFunctionSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Facets(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FacetAddressCall> for DiamondLoupeFacetCalls {
        fn from(value: FacetAddressCall) -> Self {
            Self::FacetAddress(value)
        }
    }
    impl ::core::convert::From<FacetAddressesCall> for DiamondLoupeFacetCalls {
        fn from(value: FacetAddressesCall) -> Self {
            Self::FacetAddresses(value)
        }
    }
    impl ::core::convert::From<FacetFunctionSelectorsCall> for DiamondLoupeFacetCalls {
        fn from(value: FacetFunctionSelectorsCall) -> Self {
            Self::FacetFunctionSelectors(value)
        }
    }
    impl ::core::convert::From<FacetsCall> for DiamondLoupeFacetCalls {
        fn from(value: FacetsCall) -> Self {
            Self::Facets(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for DiamondLoupeFacetCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    ///Container type for all return fields from the `facetAddress` function with signature `facetAddress(bytes4)` and selector `0xcdffacc6`
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
    pub struct FacetAddressReturn {
        pub facet_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `facetAddresses` function with signature `facetAddresses()` and selector `0x52ef6b2c`
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
    pub struct FacetAddressesReturn {
        pub facet_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `facetFunctionSelectors` function with signature `facetFunctionSelectors(address)` and selector `0xadfca15e`
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
    pub struct FacetFunctionSelectorsReturn {
        pub facet_function_selectors: ::std::vec::Vec<[u8; 4]>,
    }
    ///Container type for all return fields from the `facets` function with signature `facets()` and selector `0x7a0ed627`
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
    pub struct FacetsReturn {
        pub facets: ::std::vec::Vec<Facet>,
    }
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
}
