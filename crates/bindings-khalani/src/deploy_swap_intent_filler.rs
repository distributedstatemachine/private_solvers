pub use deploy_swap_intent_filler::*;
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
pub mod deploy_swap_intent_filler {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
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
                    ::std::borrow::ToOwned::to_owned("KHALANI_CHAIN_ID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("KHALANI_CHAIN_ID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("KHALANI_CHAIN_URL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("KHALANI_CHAIN_URL"),
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
                    ::std::borrow::ToOwned::to_owned("KHALANI_MAILBOX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("KHALANI_MAILBOX"),
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
                    ::std::borrow::ToOwned::to_owned("SPOKE_CHAIN_EVENT_PROVER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SPOKE_CHAIN_EVENT_PROVER",
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
                (
                    ::std::borrow::ToOwned::to_owned("SPOKE_CHAIN_ID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("SPOKE_CHAIN_ID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SPOKE_CHAIN_MAILBOX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SPOKE_CHAIN_MAILBOX",
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
                (
                    ::std::borrow::ToOwned::to_owned("SPOKE_CHAIN_URL"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("SPOKE_CHAIN_URL"),
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
                    ::std::borrow::ToOwned::to_owned("run"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("run"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
    pub static DEPLOYSWAPINTENTFILLER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x04\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0C\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[Pa\x0E\xD1\x80a\0=`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x93W`\x005`\xE0\x1C\x80c\x8A}\x0BF\x11a\0fW\x80c\x8A}\x0BF\x14a\x01OW\x80c\x8F\xBB\x93i\x14a\x01mW\x80c\xAB\x04\x94\x89\x14a\x01uW\x80c\xC0@b&\x14a\x01~W\x80c\xF8\xCC\xBFG\x14a\x01\x88W`\0\x80\xFD[\x80c\x024.\xF0\x14a\0\x98W\x80cJ\xAD\xDB_\x14a\0\xD0W\x80cP\x97R\xF9\x14a\x01\x19W\x80c\x80\x05\x9E\xF4\x14a\x014W[`\0\x80\xFD[a\0\xB3sN\x8A\xF6'\x9A\xED\x9D\xAA\xA1\xBD&\x12\x12\x02$\\\xC4\x8C\xA2\xC6\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x0C`@Q\x80`@\x01`@R\x80`\x1F\x81R` \x01\x7Fhttps://testnet.khalani.network\0\x81RP\x81V[`@Qa\0\xC7\x91\x90a\x04\xE5V[a\0\xB3s\xCF#\x97\x18\xB2.M\xF0+\xEA\x0E\x13\x8E\xFDx\x86\x95\x1A\x91\x8B\x81V[a\0\xB3s\xCCsz\x94\xFE\xCA\xEC\x16Z\xBC\xF1-\xED\t[\xB1?\x03v\x85\x81V[a\x01Xa'\x1C\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xC7V[a\x01\x0Ca\x01\xA5V[a\x01Xa\xA8i\x81V[a\x01\x86a\x01\xC1V[\0[`\x0CTa\x01\x95\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xC7V[`@Q\x80``\x01`@R\x80`*\x81R` \x01a\x0Er`*\x919\x81V[`@\x80Q``\x81\x01\x90\x91R`*\x80\x82Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\x98h\x004\x91a\x0Er` \x83\x019`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x11\x91\x90a\x04\xE5V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x020W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02T\x91\x90a\x04\xFFV[P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xB5)\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xB3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xC7W=`\0\x80>=`\0\xFD[PPPP`\0sN\x8A\xF6'\x9A\xED\x9D\xAA\xA1\xBD&\x12\x12\x02$\\\xC4\x8C\xA2\xC6`@Qa\x02\xEE\x90a\x04\x92V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x03\x1AW=`\0\x80>=`\0\xFD[P\x90Pa\x03?`@Q\x80``\x01`@R\x80`!\x81R` \x01a\x0EQ`!\x919\x82a\x04(V[`@Qco\xF3\xFE\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01RsN\x8A\xF6'\x9A\xED\x9D\xAA\xA1\xBD&\x12\x12\x02$\\\xC4\x8C\xA2\xC6\x90\x81\x90co\xF3\xFE\x95\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x96W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xAAW=`\0\x80>=`\0\xFD[PPPP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cv\xEA\xDD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x0CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04 W=`\0\x80>=`\0\xFD[PPPPPPV[a\x04m\x82\x82`@Q`$\x01a\x04>\x92\x91\x90a\x05\x18V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra\x04qV[PPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[a\t\x0E\x80a\x05C\x839\x01\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x04\xC5W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x04\xA9V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x04\xF8` \x83\x01\x84a\x04\x9FV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05\x11W`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\0a\x05+`@\x83\x01\x85a\x04\x9FV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\t\x0E8\x03\x80a\t\x0E\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x08{\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x86\\\xB8\xC2\x14a\0;W\x80c\xA7\x11\xFFA\x14a\0jW[`\0\x80\xFD[`\0Ta\0N\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0}a\0x6`\x04a\x06\0V[a\0\x7FV[\0[a\0\x88\x83a\x01\xA5V[a\0\x9C\x83`\xA0\x01Q3\x85`\0\x01Q\x84a\x01\xCEV[`\0a\0\xA7\x84a\x02.V[`@\x80Q`\x80\x81\x01\x82R\x82\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x83\x01\x90\x81RB\x83\x85\x01\x81\x81R``\x85\x01\x89\x81R`\0T\x96Qc~\xB6 \xA7`\xE0\x1B\x81R\x86Q`\x04\x82\x01R\x93Q\x85\x16`$\x85\x01R\x90Q`D\x84\x01RQ`d\x83\x01R\x94\x95P\x91\x92\x16\x90c~\xB6 \xA7\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01*W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01>W=`\0\x80>=`\0\xFD[PPPP\x85`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F\xEC\x0C\x95D\x9E\xD1W\x15\xF5\xCFa\xFF;.\xC4r\xAF\x1C^L\x983pvg\x94\xCE\xF7\xC8D\xF5a\x85\x88`@Qa\x01\x95\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPPV[\x80a\x01 \x01QB\x11\x15a\x01\xCBW`@Qc\x1A\xB7\xDAk`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x02(\x90\x85\x90a\x02\x94V[PPPPV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x02w\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x07GV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0a\x02\xE9\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x03s\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x03\nWP\x80\x80` \x01\x90Q\x81\x01\x90a\x03\n\x91\x90a\x07\xCDV[a\x03nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\x03\x82\x84\x84`\0\x85a\x03\x8AV[\x94\x93PPPPV[``\x82G\x10\x15a\x03\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x03eV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x04\x07\x91\x90a\x07\xF6V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04IV[``\x91P[P\x91P\x91Pa\x04Z\x87\x83\x83\x87a\x04eV[\x97\x96PPPPPPPV[``\x83\x15a\x04\xD4W\x82Q`\0\x03a\x04\xCDW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x04\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x03eV[P\x81a\x03\x82V[a\x03\x82\x83\x83\x81Q\x15a\x04\xE9W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03e\x91\x90a\x08\x12V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05=Wa\x05=a\x05\x03V[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05ZW`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x05pW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\x8BWa\x05\x8Ba\x05\x03V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x05\xB3Wa\x05\xB3a\x05\x03V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x05\xCCW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05ZW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06\x15W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06-W`\0\x80\xFD[\x90\x85\x01\x90a\x01@\x82\x88\x03\x12\x15a\x06BW`\0\x80\xFD[a\x06Ja\x05\x19V[a\x06S\x83a\x05CV[\x81R` \x83\x015\x82\x81\x11\x15a\x06gW`\0\x80\xFD[a\x06s\x89\x82\x86\x01a\x05_V[` \x83\x01RPa\x06\x85`@\x84\x01a\x05\xECV[`@\x82\x01Ra\x06\x96``\x84\x01a\x05\xECV[``\x82\x01Ra\x06\xA7`\x80\x84\x01a\x05CV[`\x80\x82\x01Ra\x06\xB8`\xA0\x84\x01a\x05CV[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x06\xD9W`\0\x80\xFD[a\x06\xE5\x89\x82\x86\x01a\x05_V[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x92Pa\x07\x13` \x85\x01a\x05CV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x07>W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07&V[PP`\0\x91\x01RV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Qa\x07\xAE\x81`d\x85\x01` \x89\x01a\x07#V[\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x07\xDFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xEFW`\0\x80\xFD[\x93\x92PPPV[`\0\x82Qa\x08\x08\x81\x84` \x87\x01a\x07#V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x081\x81`@\x85\x01` \x87\x01a\x07#V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 Dt\xE6@\xB9L\xFE\xE5\xFD\x04^\xA2\xD0\xA0&\x8AM\t\x99\xB2G\xCDM\xBB\xEBx\x9Av<\xED<gdsolcC\0\x08\x13\x003Deployed Swap Intent Filler at %shttps://api.avax-test.network/ext/bc/C/rpc\xA2dipfsX\"\x12 \xCE+'\x9B\x10j\x85\xB5*\xB2\x1D\xEC9c\xF3\x8A,\xFDI\x9E\x8B\xF5\x8E}h2W\x0B\xB51Z\xE6dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static DEPLOYSWAPINTENTFILLER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x93W`\x005`\xE0\x1C\x80c\x8A}\x0BF\x11a\0fW\x80c\x8A}\x0BF\x14a\x01OW\x80c\x8F\xBB\x93i\x14a\x01mW\x80c\xAB\x04\x94\x89\x14a\x01uW\x80c\xC0@b&\x14a\x01~W\x80c\xF8\xCC\xBFG\x14a\x01\x88W`\0\x80\xFD[\x80c\x024.\xF0\x14a\0\x98W\x80cJ\xAD\xDB_\x14a\0\xD0W\x80cP\x97R\xF9\x14a\x01\x19W\x80c\x80\x05\x9E\xF4\x14a\x014W[`\0\x80\xFD[a\0\xB3sN\x8A\xF6'\x9A\xED\x9D\xAA\xA1\xBD&\x12\x12\x02$\\\xC4\x8C\xA2\xC6\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x0C`@Q\x80`@\x01`@R\x80`\x1F\x81R` \x01\x7Fhttps://testnet.khalani.network\0\x81RP\x81V[`@Qa\0\xC7\x91\x90a\x04\xE5V[a\0\xB3s\xCF#\x97\x18\xB2.M\xF0+\xEA\x0E\x13\x8E\xFDx\x86\x95\x1A\x91\x8B\x81V[a\0\xB3s\xCCsz\x94\xFE\xCA\xEC\x16Z\xBC\xF1-\xED\t[\xB1?\x03v\x85\x81V[a\x01Xa'\x1C\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xC7V[a\x01\x0Ca\x01\xA5V[a\x01Xa\xA8i\x81V[a\x01\x86a\x01\xC1V[\0[`\x0CTa\x01\x95\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xC7V[`@Q\x80``\x01`@R\x80`*\x81R` \x01a\x0Er`*\x919\x81V[`@\x80Q``\x81\x01\x90\x91R`*\x80\x82Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c\x98h\x004\x91a\x0Er` \x83\x019`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x11\x91\x90a\x04\xE5V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x020W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02T\x91\x90a\x04\xFFV[P\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16c\x7F\xB5)\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xB3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xC7W=`\0\x80>=`\0\xFD[PPPP`\0sN\x8A\xF6'\x9A\xED\x9D\xAA\xA1\xBD&\x12\x12\x02$\\\xC4\x8C\xA2\xC6`@Qa\x02\xEE\x90a\x04\x92V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x03\x1AW=`\0\x80>=`\0\xFD[P\x90Pa\x03?`@Q\x80``\x01`@R\x80`!\x81R` \x01a\x0EQ`!\x919\x82a\x04(V[`@Qco\xF3\xFE\x95`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01RsN\x8A\xF6'\x9A\xED\x9D\xAA\xA1\xBD&\x12\x12\x02$\\\xC4\x8C\xA2\xC6\x90\x81\x90co\xF3\xFE\x95\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x96W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xAAW=`\0\x80>=`\0\xFD[PPPP\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\0\x1C`\x01`\x01`\xA0\x1B\x03\x16cv\xEA\xDD6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x0CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04 W=`\0\x80>=`\0\xFD[PPPPPPV[a\x04m\x82\x82`@Q`$\x01a\x04>\x92\x91\x90a\x05\x18V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra\x04qV[PPV[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[a\t\x0E\x80a\x05C\x839\x01\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x04\xC5W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x04\xA9V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x04\xF8` \x83\x01\x84a\x04\x9FV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x05\x11W`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\0a\x05+`@\x83\x01\x85a\x04\x9FV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\t\x0E8\x03\x80a\t\x0E\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x08{\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x86\\\xB8\xC2\x14a\0;W\x80c\xA7\x11\xFFA\x14a\0jW[`\0\x80\xFD[`\0Ta\0N\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0}a\0x6`\x04a\x06\0V[a\0\x7FV[\0[a\0\x88\x83a\x01\xA5V[a\0\x9C\x83`\xA0\x01Q3\x85`\0\x01Q\x84a\x01\xCEV[`\0a\0\xA7\x84a\x02.V[`@\x80Q`\x80\x81\x01\x82R\x82\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x83\x01\x90\x81RB\x83\x85\x01\x81\x81R``\x85\x01\x89\x81R`\0T\x96Qc~\xB6 \xA7`\xE0\x1B\x81R\x86Q`\x04\x82\x01R\x93Q\x85\x16`$\x85\x01R\x90Q`D\x84\x01RQ`d\x83\x01R\x94\x95P\x91\x92\x16\x90c~\xB6 \xA7\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01*W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01>W=`\0\x80>=`\0\xFD[PPPP\x85`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F\xEC\x0C\x95D\x9E\xD1W\x15\xF5\xCFa\xFF;.\xC4r\xAF\x1C^L\x983pvg\x94\xCE\xF7\xC8D\xF5a\x85\x88`@Qa\x01\x95\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPPPPV[\x80a\x01 \x01QB\x11\x15a\x01\xCBW`@Qc\x1A\xB7\xDAk`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x02(\x90\x85\x90a\x02\x94V[PPPPV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x02w\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x07GV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0a\x02\xE9\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x03s\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x03\nWP\x80\x80` \x01\x90Q\x81\x01\x90a\x03\n\x91\x90a\x07\xCDV[a\x03nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[PPPV[``a\x03\x82\x84\x84`\0\x85a\x03\x8AV[\x94\x93PPPPV[``\x82G\x10\x15a\x03\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x03eV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x04\x07\x91\x90a\x07\xF6V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x04DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x04IV[``\x91P[P\x91P\x91Pa\x04Z\x87\x83\x83\x87a\x04eV[\x97\x96PPPPPPPV[``\x83\x15a\x04\xD4W\x82Q`\0\x03a\x04\xCDW`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x04\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x03eV[P\x81a\x03\x82V[a\x03\x82\x83\x83\x81Q\x15a\x04\xE9W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03e\x91\x90a\x08\x12V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05=Wa\x05=a\x05\x03V[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05ZW`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x05pW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\x8BWa\x05\x8Ba\x05\x03V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x05\xB3Wa\x05\xB3a\x05\x03V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x05\xCCW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05ZW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06\x15W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06-W`\0\x80\xFD[\x90\x85\x01\x90a\x01@\x82\x88\x03\x12\x15a\x06BW`\0\x80\xFD[a\x06Ja\x05\x19V[a\x06S\x83a\x05CV[\x81R` \x83\x015\x82\x81\x11\x15a\x06gW`\0\x80\xFD[a\x06s\x89\x82\x86\x01a\x05_V[` \x83\x01RPa\x06\x85`@\x84\x01a\x05\xECV[`@\x82\x01Ra\x06\x96``\x84\x01a\x05\xECV[``\x82\x01Ra\x06\xA7`\x80\x84\x01a\x05CV[`\x80\x82\x01Ra\x06\xB8`\xA0\x84\x01a\x05CV[`\xA0\x82\x01R`\xC0\x83\x015`\xC0\x82\x01R`\xE0\x83\x015\x82\x81\x11\x15a\x06\xD9W`\0\x80\xFD[a\x06\xE5\x89\x82\x86\x01a\x05_V[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x92Pa\x07\x13` \x85\x01a\x05CV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x07>W\x81\x81\x01Q\x83\x82\x01R` \x01a\x07&V[PP`\0\x91\x01RV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Qa\x07\xAE\x81`d\x85\x01` \x89\x01a\x07#V[\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x07\xDFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xEFW`\0\x80\xFD[\x93\x92PPPV[`\0\x82Qa\x08\x08\x81\x84` \x87\x01a\x07#V[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x081\x81`@\x85\x01` \x87\x01a\x07#V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 Dt\xE6@\xB9L\xFE\xE5\xFD\x04^\xA2\xD0\xA0&\x8AM\t\x99\xB2G\xCDM\xBB\xEBx\x9Av<\xED<gdsolcC\0\x08\x13\x003Deployed Swap Intent Filler at %shttps://api.avax-test.network/ext/bc/C/rpc\xA2dipfsX\"\x12 \xCE+'\x9B\x10j\x85\xB5*\xB2\x1D\xEC9c\xF3\x8A,\xFDI\x9E\x8B\xF5\x8E}h2W\x0B\xB51Z\xE6dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static DEPLOYSWAPINTENTFILLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DeploySwapIntentFiller<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DeploySwapIntentFiller<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DeploySwapIntentFiller<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DeploySwapIntentFiller<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DeploySwapIntentFiller<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DeploySwapIntentFiller))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DeploySwapIntentFiller<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEPLOYSWAPINTENTFILLER_ABI.clone(),
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
                DEPLOYSWAPINTENTFILLER_ABI.clone(),
                DEPLOYSWAPINTENTFILLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function
        pub fn is_script(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `KHALANI_CHAIN_ID` (0x8a7d0b46) function
        pub fn khalani_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([138, 125, 11, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `KHALANI_CHAIN_URL` (0x4aaddb5f) function
        pub fn khalani_chain_url(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([74, 173, 219, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `KHALANI_MAILBOX` (0x509752f9) function
        pub fn khalani_mailbox(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([80, 151, 82, 249], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SPOKE_CHAIN_EVENT_PROVER` (0x02342ef0) function
        pub fn spoke_chain_event_prover(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([2, 52, 46, 240], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SPOKE_CHAIN_ID` (0xab049489) function
        pub fn spoke_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([171, 4, 148, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SPOKE_CHAIN_MAILBOX` (0x80059ef4) function
        pub fn spoke_chain_mailbox(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([128, 5, 158, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SPOKE_CHAIN_URL` (0x8fbb9369) function
        pub fn spoke_chain_url(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([143, 187, 147, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `run` (0xc0406226) function
        pub fn run(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 64, 98, 38], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DeploySwapIntentFiller<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    ///Container type for all input parameters for the `KHALANI_CHAIN_ID` function with signature `KHALANI_CHAIN_ID()` and selector `0x8a7d0b46`
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
    #[ethcall(name = "KHALANI_CHAIN_ID", abi = "KHALANI_CHAIN_ID()")]
    pub struct KhalaniChainIdCall;
    ///Container type for all input parameters for the `KHALANI_CHAIN_URL` function with signature `KHALANI_CHAIN_URL()` and selector `0x4aaddb5f`
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
    #[ethcall(name = "KHALANI_CHAIN_URL", abi = "KHALANI_CHAIN_URL()")]
    pub struct KhalaniChainUrlCall;
    ///Container type for all input parameters for the `KHALANI_MAILBOX` function with signature `KHALANI_MAILBOX()` and selector `0x509752f9`
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
    #[ethcall(name = "KHALANI_MAILBOX", abi = "KHALANI_MAILBOX()")]
    pub struct KhalaniMailboxCall;
    ///Container type for all input parameters for the `SPOKE_CHAIN_EVENT_PROVER` function with signature `SPOKE_CHAIN_EVENT_PROVER()` and selector `0x02342ef0`
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
    #[ethcall(name = "SPOKE_CHAIN_EVENT_PROVER", abi = "SPOKE_CHAIN_EVENT_PROVER()")]
    pub struct SpokeChainEventProverCall;
    ///Container type for all input parameters for the `SPOKE_CHAIN_ID` function with signature `SPOKE_CHAIN_ID()` and selector `0xab049489`
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
    #[ethcall(name = "SPOKE_CHAIN_ID", abi = "SPOKE_CHAIN_ID()")]
    pub struct SpokeChainIdCall;
    ///Container type for all input parameters for the `SPOKE_CHAIN_MAILBOX` function with signature `SPOKE_CHAIN_MAILBOX()` and selector `0x80059ef4`
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
    #[ethcall(name = "SPOKE_CHAIN_MAILBOX", abi = "SPOKE_CHAIN_MAILBOX()")]
    pub struct SpokeChainMailboxCall;
    ///Container type for all input parameters for the `SPOKE_CHAIN_URL` function with signature `SPOKE_CHAIN_URL()` and selector `0x8fbb9369`
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
    #[ethcall(name = "SPOKE_CHAIN_URL", abi = "SPOKE_CHAIN_URL()")]
    pub struct SpokeChainUrlCall;
    ///Container type for all input parameters for the `run` function with signature `run()` and selector `0xc0406226`
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
    #[ethcall(name = "run", abi = "run()")]
    pub struct RunCall;
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
    pub enum DeploySwapIntentFillerCalls {
        IsScript(IsScriptCall),
        KhalaniChainId(KhalaniChainIdCall),
        KhalaniChainUrl(KhalaniChainUrlCall),
        KhalaniMailbox(KhalaniMailboxCall),
        SpokeChainEventProver(SpokeChainEventProverCall),
        SpokeChainId(SpokeChainIdCall),
        SpokeChainMailbox(SpokeChainMailboxCall),
        SpokeChainUrl(SpokeChainUrlCall),
        Run(RunCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeploySwapIntentFillerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsScript(decoded));
            }
            if let Ok(decoded) = <KhalaniChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KhalaniChainId(decoded));
            }
            if let Ok(decoded) = <KhalaniChainUrlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KhalaniChainUrl(decoded));
            }
            if let Ok(decoded) = <KhalaniMailboxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::KhalaniMailbox(decoded));
            }
            if let Ok(decoded) = <SpokeChainEventProverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SpokeChainEventProver(decoded));
            }
            if let Ok(decoded) = <SpokeChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SpokeChainId(decoded));
            }
            if let Ok(decoded) = <SpokeChainMailboxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SpokeChainMailbox(decoded));
            }
            if let Ok(decoded) = <SpokeChainUrlCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SpokeChainUrl(decoded));
            }
            if let Ok(decoded) = <RunCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Run(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeploySwapIntentFillerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsScript(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KhalaniChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KhalaniChainUrl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::KhalaniMailbox(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpokeChainEventProver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpokeChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpokeChainMailbox(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpokeChainUrl(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Run(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DeploySwapIntentFillerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsScript(element) => ::core::fmt::Display::fmt(element, f),
                Self::KhalaniChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::KhalaniChainUrl(element) => ::core::fmt::Display::fmt(element, f),
                Self::KhalaniMailbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpokeChainEventProver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SpokeChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpokeChainMailbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpokeChainUrl(element) => ::core::fmt::Display::fmt(element, f),
                Self::Run(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsScriptCall> for DeploySwapIntentFillerCalls {
        fn from(value: IsScriptCall) -> Self {
            Self::IsScript(value)
        }
    }
    impl ::core::convert::From<KhalaniChainIdCall> for DeploySwapIntentFillerCalls {
        fn from(value: KhalaniChainIdCall) -> Self {
            Self::KhalaniChainId(value)
        }
    }
    impl ::core::convert::From<KhalaniChainUrlCall> for DeploySwapIntentFillerCalls {
        fn from(value: KhalaniChainUrlCall) -> Self {
            Self::KhalaniChainUrl(value)
        }
    }
    impl ::core::convert::From<KhalaniMailboxCall> for DeploySwapIntentFillerCalls {
        fn from(value: KhalaniMailboxCall) -> Self {
            Self::KhalaniMailbox(value)
        }
    }
    impl ::core::convert::From<SpokeChainEventProverCall>
    for DeploySwapIntentFillerCalls {
        fn from(value: SpokeChainEventProverCall) -> Self {
            Self::SpokeChainEventProver(value)
        }
    }
    impl ::core::convert::From<SpokeChainIdCall> for DeploySwapIntentFillerCalls {
        fn from(value: SpokeChainIdCall) -> Self {
            Self::SpokeChainId(value)
        }
    }
    impl ::core::convert::From<SpokeChainMailboxCall> for DeploySwapIntentFillerCalls {
        fn from(value: SpokeChainMailboxCall) -> Self {
            Self::SpokeChainMailbox(value)
        }
    }
    impl ::core::convert::From<SpokeChainUrlCall> for DeploySwapIntentFillerCalls {
        fn from(value: SpokeChainUrlCall) -> Self {
            Self::SpokeChainUrl(value)
        }
    }
    impl ::core::convert::From<RunCall> for DeploySwapIntentFillerCalls {
        fn from(value: RunCall) -> Self {
            Self::Run(value)
        }
    }
    ///Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    pub struct IsScriptReturn(pub bool);
    ///Container type for all return fields from the `KHALANI_CHAIN_ID` function with signature `KHALANI_CHAIN_ID()` and selector `0x8a7d0b46`
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
    pub struct KhalaniChainIdReturn(pub u32);
    ///Container type for all return fields from the `KHALANI_CHAIN_URL` function with signature `KHALANI_CHAIN_URL()` and selector `0x4aaddb5f`
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
    pub struct KhalaniChainUrlReturn(pub ::std::string::String);
    ///Container type for all return fields from the `KHALANI_MAILBOX` function with signature `KHALANI_MAILBOX()` and selector `0x509752f9`
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
    pub struct KhalaniMailboxReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `SPOKE_CHAIN_EVENT_PROVER` function with signature `SPOKE_CHAIN_EVENT_PROVER()` and selector `0x02342ef0`
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
    pub struct SpokeChainEventProverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `SPOKE_CHAIN_ID` function with signature `SPOKE_CHAIN_ID()` and selector `0xab049489`
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
    pub struct SpokeChainIdReturn(pub u32);
    ///Container type for all return fields from the `SPOKE_CHAIN_MAILBOX` function with signature `SPOKE_CHAIN_MAILBOX()` and selector `0x80059ef4`
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
    pub struct SpokeChainMailboxReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `SPOKE_CHAIN_URL` function with signature `SPOKE_CHAIN_URL()` and selector `0x8fbb9369`
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
    pub struct SpokeChainUrlReturn(pub ::std::string::String);
}
