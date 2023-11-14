pub use hyperlane_adapter::*;
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
pub mod hyperlane_adapter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_mailbox"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_ism"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_nexus"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_igp"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("handle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("handle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_origin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
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
                (
                    ::std::borrow::ToOwned::to_owned("interchainGasPaymaster"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "interchainGasPaymaster",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IGasPayMaster"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("interchainSecurityModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "interchainSecurityModule",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IInterchainSecurityModule",
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
                    ::std::borrow::ToOwned::to_owned("mailbox"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mailbox"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IMailbox"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nexus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nexus"),
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
                    ::std::borrow::ToOwned::to_owned("payRelayer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("payRelayer"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_numTokens"),
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
                    ::std::borrow::ToOwned::to_owned("quoteSend"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quoteSend"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_destinationDomain",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_numTokens"),
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
                    ::std::borrow::ToOwned::to_owned("relayMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("relayMessage"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("messageId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInbox"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidInbox"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidNexus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidNexus"),
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
    pub static HYPERLANEADAPTER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@Qa\x08\xA78\x03\x80a\x08\xA7\x839\x81\x01`@\x81\x90Ra\x000\x91a\0sV[`\x01`\0U`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x80R\x91\x83\x16`\xA0R\x82\x16`\xE0R\x16`\xC0Ra\0\xC7V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0nW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\0\x89W`\0\x80\xFD[a\0\x92\x85a\0WV[\x93Pa\0\xA0` \x86\x01a\0WV[\x92Pa\0\xAE`@\x86\x01a\0WV[\x91Pa\0\xBC``\x86\x01a\0WV[\x90P\x92\x95\x91\x94P\x92PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x07~a\x01)`\09`\0\x81\x81a\x01F\x01R\x81\x81a\x02\x05\x01Ra\x03\xE9\x01R`\0\x81\x81`\xC5\x01R\x81\x81a\x03%\x01Ra\x04\x87\x01R`\0a\x01\xAE\x01R`\0\x81\x81a\x01z\x01R\x81\x81a\x02Z\x01Ra\x03\x94\x01Ra\x07~`\0\xF3\xFE`\x80`@R`\x046\x10a\0{W`\x005`\xE0\x1C\x80c\xA3\xF5\xC1\xD2\x11a\0NW\x80c\xA3\xF5\xC1\xD2\x14a\x014W\x80c\xD5C\x8E\xAE\x14a\x01hW\x80c\xDER<\xF3\x14a\x01\x9CW\x80c\xF2G_\xCE\x14a\x01\xD0W`\0\x80\xFD[\x80c\x01B\x8A\x07\x14a\0\x80W\x80c9\xBBJ\xD9\x14a\0\xB3W\x80cL\xDD\x14\x9B\x14a\0\xFFW\x80cV\xD5\xD4u\x14a\x01\x14W[`\0\x80\xFD[4\x80\x15a\0\x8CW`\0\x80\xFD[Pa\0\xA0a\0\x9B6`\x04a\x05\xA7V[a\x01\xF0V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xBFW`\0\x80\xFD[Pa\0\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xAAV[a\x01\x12a\x01\r6`\x04a\x05\xFAV[a\x02\xECV[\0[4\x80\x15a\x01 W`\0\x80\xFD[Pa\x01\x12a\x01/6`\x04a\x06GV[a\x03\x89V[4\x80\x15a\x01@W`\0\x80\xFD[Pa\0\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01tW`\0\x80\xFD[Pa\0\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01\xA8W`\0\x80\xFD[Pa\0\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01\xDCW`\0\x80\xFD[Pa\0\xA0a\x01\xEB6`\x04a\x06\x94V[a\x04bV[`\0a\x01\xFAa\x05\x01V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02CW`@Qc\x05\xED\x9F\x1D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xFA1\xDE\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFA1\xDE\x01\x90a\x02\x95\x90\x88\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x06\xDFV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xD8\x91\x90a\x07\x0FV[\x90Pa\x02\xE4`\x01`\0UV[\x94\x93PPPPV[`@Qc\x027\xE5\x83`\xE3\x1B\x81R`\x04\x81\x01\x85\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x11\xBF,\x18\x904\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x03jW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03~W=`\0\x80>=`\0\xFD[PPPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\xD2W`@Qcp\xFC\xDD\x1D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xC2\xA1\xCB\x9D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC2\xA1\xCB\x9D\x90a\x04*\x90c\xFF\xFF\xFF\xFF\x88\x16\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x07(V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04XW=`\0\x80>=`\0\xFD[PPPPPPPPV[`@Qc?`\x0B+`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\x80,\xAC\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xFA\x91\x90a\x07\x0FV[\x93\x92PPPV[`\x02`\0T\x03a\x05WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0\x80\x83`\x1F\x84\x01\x12a\x05pW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x88W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x05\xA0W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x05\xBDW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xE2W`\0\x80\xFD[a\x05\xEE\x87\x82\x88\x01a\x05^V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x06\x10W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06<W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x06]W`\0\x80\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06qW`\0\x80\xFD[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xE2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xA7W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x07\x05``\x83\x01\x84\x86a\x06\xB6V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x07!W`\0\x80\xFD[PQ\x91\x90PV[\x84\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x07\x05``\x83\x01\x84\x86a\x06\xB6V\xFE\xA2dipfsX\"\x12 `\x12\xD3\xD9\x1D\xE2\xF5\xBB\x16\xA7\xB2EB\x96)\x1Em\xA7\x06y\xE7\\\xF8\x8C\x02\x95G\xCDM\x9A\xD7\xB8dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static HYPERLANEADAPTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0{W`\x005`\xE0\x1C\x80c\xA3\xF5\xC1\xD2\x11a\0NW\x80c\xA3\xF5\xC1\xD2\x14a\x014W\x80c\xD5C\x8E\xAE\x14a\x01hW\x80c\xDER<\xF3\x14a\x01\x9CW\x80c\xF2G_\xCE\x14a\x01\xD0W`\0\x80\xFD[\x80c\x01B\x8A\x07\x14a\0\x80W\x80c9\xBBJ\xD9\x14a\0\xB3W\x80cL\xDD\x14\x9B\x14a\0\xFFW\x80cV\xD5\xD4u\x14a\x01\x14W[`\0\x80\xFD[4\x80\x15a\0\x8CW`\0\x80\xFD[Pa\0\xA0a\0\x9B6`\x04a\x05\xA7V[a\x01\xF0V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xBFW`\0\x80\xFD[Pa\0\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xAAV[a\x01\x12a\x01\r6`\x04a\x05\xFAV[a\x02\xECV[\0[4\x80\x15a\x01 W`\0\x80\xFD[Pa\x01\x12a\x01/6`\x04a\x06GV[a\x03\x89V[4\x80\x15a\x01@W`\0\x80\xFD[Pa\0\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01tW`\0\x80\xFD[Pa\0\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01\xA8W`\0\x80\xFD[Pa\0\xE7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x01\xDCW`\0\x80\xFD[Pa\0\xA0a\x01\xEB6`\x04a\x06\x94V[a\x04bV[`\0a\x01\xFAa\x05\x01V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02CW`@Qc\x05\xED\x9F\x1D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xFA1\xDE\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFA1\xDE\x01\x90a\x02\x95\x90\x88\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x06\xDFV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xD8\x91\x90a\x07\x0FV[\x90Pa\x02\xE4`\x01`\0UV[\x94\x93PPPPV[`@Qc\x027\xE5\x83`\xE3\x1B\x81R`\x04\x81\x01\x85\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x11\xBF,\x18\x904\x90`\x84\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x03jW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03~W=`\0\x80>=`\0\xFD[PPPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\xD2W`@Qcp\xFC\xDD\x1D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xC2\xA1\xCB\x9D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC2\xA1\xCB\x9D\x90a\x04*\x90c\xFF\xFF\xFF\xFF\x88\x16\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x07(V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04XW=`\0\x80>=`\0\xFD[PPPPPPPPV[`@Qc?`\x0B+`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xFD\x80,\xAC\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xFA\x91\x90a\x07\x0FV[\x93\x92PPPV[`\x02`\0T\x03a\x05WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0UV[`\0\x80\x83`\x1F\x84\x01\x12a\x05pW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x88W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x05\xA0W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x05\xBDW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xE2W`\0\x80\xFD[a\x05\xEE\x87\x82\x88\x01a\x05^V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x06\x10W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06<W`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\x06]W`\0\x80\xFD[\x845c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06qW`\0\x80\xFD[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xE2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x06\xA7W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x07\x05``\x83\x01\x84\x86a\x06\xB6V[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x07!W`\0\x80\xFD[PQ\x91\x90PV[\x84\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x07\x05``\x83\x01\x84\x86a\x06\xB6V\xFE\xA2dipfsX\"\x12 `\x12\xD3\xD9\x1D\xE2\xF5\xBB\x16\xA7\xB2EB\x96)\x1Em\xA7\x06y\xE7\\\xF8\x8C\x02\x95G\xCDM\x9A\xD7\xB8dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static HYPERLANEADAPTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct HyperlaneAdapter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for HyperlaneAdapter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HyperlaneAdapter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HyperlaneAdapter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HyperlaneAdapter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(HyperlaneAdapter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> HyperlaneAdapter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    HYPERLANEADAPTER_ABI.clone(),
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
                HYPERLANEADAPTER_ABI.clone(),
                HYPERLANEADAPTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `handle` (0x56d5d475) function
        pub fn handle(
            &self,
            origin: u32,
            sender: [u8; 32],
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 213, 212, 117], (origin, sender, message))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `interchainGasPaymaster` (0x39bb4ad9) function
        pub fn interchain_gas_paymaster(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([57, 187, 74, 217], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `interchainSecurityModule` (0xde523cf3) function
        pub fn interchain_security_module(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([222, 82, 60, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mailbox` (0xd5438eae) function
        pub fn mailbox(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([213, 67, 142, 174], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nexus` (0xa3f5c1d2) function
        pub fn nexus(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([163, 245, 193, 210], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payRelayer` (0x4cdd149b) function
        pub fn pay_relayer(
            &self,
            message_id: [u8; 32],
            destination_domain: ::ethers::core::types::U256,
            num_tokens: ::ethers::core::types::U256,
            refund_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [76, 221, 20, 155],
                    (message_id, destination_domain, num_tokens, refund_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quoteSend` (0xf2475fce) function
        pub fn quote_send(
            &self,
            destination_domain: ::ethers::core::types::U256,
            num_tokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([242, 71, 95, 206], (destination_domain, num_tokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayMessage` (0x01428a07) function
        pub fn relay_message(
            &self,
            chain: ::ethers::core::types::U256,
            receiver: [u8; 32],
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([1, 66, 138, 7], (chain, receiver, payload))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for HyperlaneAdapter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidInbox` with signature `InvalidInbox()` and selector `0x70fcdd1d`
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
    #[etherror(name = "InvalidInbox", abi = "InvalidInbox()")]
    pub struct InvalidInbox;
    ///Custom Error type `InvalidNexus` with signature `InvalidNexus()` and selector `0x2f6cf8e8`
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
    #[etherror(name = "InvalidNexus", abi = "InvalidNexus()")]
    pub struct InvalidNexus;
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
    pub enum HyperlaneAdapterErrors {
        InvalidInbox(InvalidInbox),
        InvalidNexus(InvalidNexus),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for HyperlaneAdapterErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidInbox as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidInbox(decoded));
            }
            if let Ok(decoded) = <InvalidNexus as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidNexus(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HyperlaneAdapterErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidInbox(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidNexus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for HyperlaneAdapterErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidInbox as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidNexus as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for HyperlaneAdapterErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidInbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidNexus(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for HyperlaneAdapterErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidInbox> for HyperlaneAdapterErrors {
        fn from(value: InvalidInbox) -> Self {
            Self::InvalidInbox(value)
        }
    }
    impl ::core::convert::From<InvalidNexus> for HyperlaneAdapterErrors {
        fn from(value: InvalidNexus) -> Self {
            Self::InvalidNexus(value)
        }
    }
    ///Container type for all input parameters for the `handle` function with signature `handle(uint32,bytes32,bytes)` and selector `0x56d5d475`
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
    #[ethcall(name = "handle", abi = "handle(uint32,bytes32,bytes)")]
    pub struct HandleCall {
        pub origin: u32,
        pub sender: [u8; 32],
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `interchainGasPaymaster` function with signature `interchainGasPaymaster()` and selector `0x39bb4ad9`
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
    #[ethcall(name = "interchainGasPaymaster", abi = "interchainGasPaymaster()")]
    pub struct InterchainGasPaymasterCall;
    ///Container type for all input parameters for the `interchainSecurityModule` function with signature `interchainSecurityModule()` and selector `0xde523cf3`
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
    #[ethcall(name = "interchainSecurityModule", abi = "interchainSecurityModule()")]
    pub struct InterchainSecurityModuleCall;
    ///Container type for all input parameters for the `mailbox` function with signature `mailbox()` and selector `0xd5438eae`
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
    #[ethcall(name = "mailbox", abi = "mailbox()")]
    pub struct MailboxCall;
    ///Container type for all input parameters for the `nexus` function with signature `nexus()` and selector `0xa3f5c1d2`
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
    #[ethcall(name = "nexus", abi = "nexus()")]
    pub struct NexusCall;
    ///Container type for all input parameters for the `payRelayer` function with signature `payRelayer(bytes32,uint256,uint256,address)` and selector `0x4cdd149b`
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
    #[ethcall(name = "payRelayer", abi = "payRelayer(bytes32,uint256,uint256,address)")]
    pub struct PayRelayerCall {
        pub message_id: [u8; 32],
        pub destination_domain: ::ethers::core::types::U256,
        pub num_tokens: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `quoteSend` function with signature `quoteSend(uint256,uint256)` and selector `0xf2475fce`
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
    #[ethcall(name = "quoteSend", abi = "quoteSend(uint256,uint256)")]
    pub struct QuoteSendCall {
        pub destination_domain: ::ethers::core::types::U256,
        pub num_tokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `relayMessage` function with signature `relayMessage(uint256,bytes32,bytes)` and selector `0x01428a07`
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
    #[ethcall(name = "relayMessage", abi = "relayMessage(uint256,bytes32,bytes)")]
    pub struct RelayMessageCall {
        pub chain: ::ethers::core::types::U256,
        pub receiver: [u8; 32],
        pub payload: ::ethers::core::types::Bytes,
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
    pub enum HyperlaneAdapterCalls {
        Handle(HandleCall),
        InterchainGasPaymaster(InterchainGasPaymasterCall),
        InterchainSecurityModule(InterchainSecurityModuleCall),
        Mailbox(MailboxCall),
        Nexus(NexusCall),
        PayRelayer(PayRelayerCall),
        QuoteSend(QuoteSendCall),
        RelayMessage(RelayMessageCall),
    }
    impl ::ethers::core::abi::AbiDecode for HyperlaneAdapterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <HandleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Handle(decoded));
            }
            if let Ok(decoded) = <InterchainGasPaymasterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InterchainGasPaymaster(decoded));
            }
            if let Ok(decoded) = <InterchainSecurityModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InterchainSecurityModule(decoded));
            }
            if let Ok(decoded) = <MailboxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mailbox(decoded));
            }
            if let Ok(decoded) = <NexusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nexus(decoded));
            }
            if let Ok(decoded) = <PayRelayerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PayRelayer(decoded));
            }
            if let Ok(decoded) = <QuoteSendCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuoteSend(decoded));
            }
            if let Ok(decoded) = <RelayMessageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayMessage(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for HyperlaneAdapterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Handle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InterchainGasPaymaster(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InterchainSecurityModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mailbox(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nexus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PayRelayer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuoteSend(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for HyperlaneAdapterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Handle(element) => ::core::fmt::Display::fmt(element, f),
                Self::InterchainGasPaymaster(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InterchainSecurityModule(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Mailbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nexus(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayRelayer(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuoteSend(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayMessage(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<HandleCall> for HyperlaneAdapterCalls {
        fn from(value: HandleCall) -> Self {
            Self::Handle(value)
        }
    }
    impl ::core::convert::From<InterchainGasPaymasterCall> for HyperlaneAdapterCalls {
        fn from(value: InterchainGasPaymasterCall) -> Self {
            Self::InterchainGasPaymaster(value)
        }
    }
    impl ::core::convert::From<InterchainSecurityModuleCall> for HyperlaneAdapterCalls {
        fn from(value: InterchainSecurityModuleCall) -> Self {
            Self::InterchainSecurityModule(value)
        }
    }
    impl ::core::convert::From<MailboxCall> for HyperlaneAdapterCalls {
        fn from(value: MailboxCall) -> Self {
            Self::Mailbox(value)
        }
    }
    impl ::core::convert::From<NexusCall> for HyperlaneAdapterCalls {
        fn from(value: NexusCall) -> Self {
            Self::Nexus(value)
        }
    }
    impl ::core::convert::From<PayRelayerCall> for HyperlaneAdapterCalls {
        fn from(value: PayRelayerCall) -> Self {
            Self::PayRelayer(value)
        }
    }
    impl ::core::convert::From<QuoteSendCall> for HyperlaneAdapterCalls {
        fn from(value: QuoteSendCall) -> Self {
            Self::QuoteSend(value)
        }
    }
    impl ::core::convert::From<RelayMessageCall> for HyperlaneAdapterCalls {
        fn from(value: RelayMessageCall) -> Self {
            Self::RelayMessage(value)
        }
    }
    ///Container type for all return fields from the `interchainGasPaymaster` function with signature `interchainGasPaymaster()` and selector `0x39bb4ad9`
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
    pub struct InterchainGasPaymasterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `interchainSecurityModule` function with signature `interchainSecurityModule()` and selector `0xde523cf3`
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
    pub struct InterchainSecurityModuleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `mailbox` function with signature `mailbox()` and selector `0xd5438eae`
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
    pub struct MailboxReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `nexus` function with signature `nexus()` and selector `0xa3f5c1d2`
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
    pub struct NexusReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `quoteSend` function with signature `quoteSend(uint256,uint256)` and selector `0xf2475fce`
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
    pub struct QuoteSendReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `relayMessage` function with signature `relayMessage(uint256,bytes32,bytes)` and selector `0x01428a07`
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
    pub struct RelayMessageReturn {
        pub message_id: [u8; 32],
    }
}
