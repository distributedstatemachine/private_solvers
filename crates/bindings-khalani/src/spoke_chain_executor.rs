pub use spoke_chain_executor::*;
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
pub mod spoke_chain_executor {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_intentEventProver"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IntentEventProver",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("callSpoke"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("callSpoke"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "spokeChainCallIntentId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("contractToCall"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("callData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("intentEventProver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("intentEventProver"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IntentEventProver",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("SpokeCalled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SpokeCalled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("contractToCall"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
    pub static SPOKECHAINEXECUTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\t18\x03\x80a\t1\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x08\x9E\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c)\xA4\xD7U\x14a\0;W\x80c\x86\\\xB8\xC2\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x06\x92V[a\0\x7FV[\0[`\0Ta\0c\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\0\xA5Wa\0\x9A\x8230\x84a\x02\x86V[a\0\xA5\x82\x86\x83a\x02\xF7V[`\0\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x85`@Qa\0\xC1\x92\x91\x90a\x078V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\0\xFEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\x03V[``\x91P[PP\x90P\x80a\x01YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCall to contract failed\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xDAH5\xC6`@Q\x80`\xA0\x01`@R\x80\x8A\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R`@\x90\x81\x01\x86\x90RQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Ra\x02\n\x91\x90`\x04\x01a\x07\x98V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02$W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x028W=`\0\x80>=`\0\xFD[PPPP\x81\x83`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1FG\xD7\xE1\x13 \x1Ef\xC1\xA1\xB8)\xD5\x010\xAD\x83@e2\x12\xF2\x19\xD7M\xA5\xD4/\x89\xF7\xDB\xD8`@Q`@Q\x80\x91\x03\x90\xA4PPPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x02\xF1\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x04\x11V[PPPPV[\x80\x15\x80a\x03qWP`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03o\x91\x90a\x07\xF7V[\x15[a\x03\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FSafeERC20: approve from non-zero`D\x82\x01Ru to non-zero allowance`P\x1B`d\x82\x01R`\x84\x01a\x01PV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x04\x0C\x90\x84\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01a\x02\xBAV[PPPV[`\0a\x04f\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x04\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x04\x87WP\x80\x80` \x01\x90Q\x81\x01\x90a\x04\x87\x91\x90a\x08\x10V[a\x04\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x01PV[``a\x04\xF5\x84\x84`\0\x85a\x04\xFDV[\x94\x93PPPPV[``\x82G\x10\x15a\x05^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x01PV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x05z\x91\x90a\x089V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x05\xB7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\xBCV[``\x91P[P\x91P\x91Pa\x05\xCD\x87\x83\x83\x87a\x05\xD8V[\x97\x96PPPPPPPV[``\x83\x15a\x06GW\x82Q`\0\x03a\x06@W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x06@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01PV[P\x81a\x04\xF5V[a\x04\xF5\x83\x83\x81Q\x15a\x06\\W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01P\x91\x90a\x08UV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x8DW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x06\xABW`\0\x80\xFD[\x865\x95Pa\x06\xBB` \x88\x01a\x06vV[\x94P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xD8W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x06\xECW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06\xFBW`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x07\rW`\0\x80\xFD[` \x83\x01\x96P\x80\x95PPPPa\x07%``\x88\x01a\x06vV[\x91P`\x80\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0[\x83\x81\x10\x15a\x07cW\x81\x81\x01Q\x83\x82\x01R` \x01a\x07KV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x07\x84\x81` \x86\x01` \x86\x01a\x07HV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01R\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x91\x90\x91R\x83\x01Q`\xA0``\x84\x01R`\0\x91a\x07\xD4`\xC0\x85\x01\x83a\x07lV[\x91P\x80``\x86\x01Q\x16`\x80\x85\x01RP`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x08\tW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x08\"W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x082W`\0\x80\xFD[\x93\x92PPPV[`\0\x82Qa\x08K\x81\x84` \x87\x01a\x07HV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x082` \x83\x01\x84a\x07lV\xFE\xA2dipfsX\"\x12 \x12\x18!4b\\\x133!\xBEY\t3\xF87im}\xC0:\x83\xF0\rc\xE4\xEEs\xB3\xA8\x87\xEA\x16dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SPOKECHAINEXECUTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c)\xA4\xD7U\x14a\0;W\x80c\x86\\\xB8\xC2\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x06\x92V[a\0\x7FV[\0[`\0Ta\0c\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\0\xA5Wa\0\x9A\x8230\x84a\x02\x86V[a\0\xA5\x82\x86\x83a\x02\xF7V[`\0\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x85`@Qa\0\xC1\x92\x91\x90a\x078V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\0\xFEW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\x03V[``\x91P[PP\x90P\x80a\x01YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCall to contract failed\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xDAH5\xC6`@Q\x80`\xA0\x01`@R\x80\x8A\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R`@\x90\x81\x01\x86\x90RQ`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Ra\x02\n\x91\x90`\x04\x01a\x07\x98V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02$W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x028W=`\0\x80>=`\0\xFD[PPPP\x81\x83`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1FG\xD7\xE1\x13 \x1Ef\xC1\xA1\xB8)\xD5\x010\xAD\x83@e2\x12\xF2\x19\xD7M\xA5\xD4/\x89\xF7\xDB\xD8`@Q`@Q\x80\x91\x03\x90\xA4PPPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra\x02\xF1\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra\x04\x11V[PPPPV[\x80\x15\x80a\x03qWP`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03o\x91\x90a\x07\xF7V[\x15[a\x03\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FSafeERC20: approve from non-zero`D\x82\x01Ru to non-zero allowance`P\x1B`d\x82\x01R`\x84\x01a\x01PV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x04\x0C\x90\x84\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01a\x02\xBAV[PPPV[`\0a\x04f\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a\x04\xE6\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a\x04\x87WP\x80\x80` \x01\x90Q\x81\x01\x90a\x04\x87\x91\x90a\x08\x10V[a\x04\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x01PV[``a\x04\xF5\x84\x84`\0\x85a\x04\xFDV[\x94\x93PPPPV[``\x82G\x10\x15a\x05^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x01PV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x05z\x91\x90a\x089V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x05\xB7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x05\xBCV[``\x91P[P\x91P\x91Pa\x05\xCD\x87\x83\x83\x87a\x05\xD8V[\x97\x96PPPPPPPV[``\x83\x15a\x06GW\x82Q`\0\x03a\x06@W`\x01`\x01`\xA0\x1B\x03\x85\x16;a\x06@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01PV[P\x81a\x04\xF5V[a\x04\xF5\x83\x83\x81Q\x15a\x06\\W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01P\x91\x90a\x08UV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x8DW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x06\xABW`\0\x80\xFD[\x865\x95Pa\x06\xBB` \x88\x01a\x06vV[\x94P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xD8W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12a\x06\xECW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x06\xFBW`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15a\x07\rW`\0\x80\xFD[` \x83\x01\x96P\x80\x95PPPPa\x07%``\x88\x01a\x06vV[\x91P`\x80\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0[\x83\x81\x10\x15a\x07cW\x81\x81\x01Q\x83\x82\x01R` \x01a\x07KV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x07\x84\x81` \x86\x01` \x86\x01a\x07HV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01R\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x84\x01\x91\x90\x91R\x83\x01Q`\xA0``\x84\x01R`\0\x91a\x07\xD4`\xC0\x85\x01\x83a\x07lV[\x91P\x80``\x86\x01Q\x16`\x80\x85\x01RP`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x08\tW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x08\"W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x082W`\0\x80\xFD[\x93\x92PPPV[`\0\x82Qa\x08K\x81\x84` \x87\x01a\x07HV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x082` \x83\x01\x84a\x07lV\xFE\xA2dipfsX\"\x12 \x12\x18!4b\\\x133!\xBEY\t3\xF87im}\xC0:\x83\xF0\rc\xE4\xEEs\xB3\xA8\x87\xEA\x16dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static SPOKECHAINEXECUTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SpokeChainExecutor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SpokeChainExecutor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SpokeChainExecutor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SpokeChainExecutor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SpokeChainExecutor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SpokeChainExecutor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SpokeChainExecutor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SPOKECHAINEXECUTOR_ABI.clone(),
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
                SPOKECHAINEXECUTOR_ABI.clone(),
                SPOKECHAINEXECUTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `callSpoke` (0x29a4d755) function
        pub fn call_spoke(
            &self,
            spoke_chain_call_intent_id: [u8; 32],
            contract_to_call: ::ethers::core::types::Address,
            call_data: ::ethers::core::types::Bytes,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [41, 164, 215, 85],
                    (
                        spoke_chain_call_intent_id,
                        contract_to_call,
                        call_data,
                        token,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `intentEventProver` (0x865cb8c2) function
        pub fn intent_event_prover(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([134, 92, 184, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SpokeCalled` event
        pub fn spoke_called_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SpokeCalledFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SpokeCalledFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SpokeChainExecutor<M> {
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
    #[ethevent(name = "SpokeCalled", abi = "SpokeCalled(address,address,uint256)")]
    pub struct SpokeCalledFilter {
        #[ethevent(indexed)]
        pub contract_to_call: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `callSpoke` function with signature `callSpoke(bytes32,address,bytes,address,uint256)` and selector `0x29a4d755`
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
        name = "callSpoke",
        abi = "callSpoke(bytes32,address,bytes,address,uint256)"
    )]
    pub struct CallSpokeCall {
        pub spoke_chain_call_intent_id: [u8; 32],
        pub contract_to_call: ::ethers::core::types::Address,
        pub call_data: ::ethers::core::types::Bytes,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `intentEventProver` function with signature `intentEventProver()` and selector `0x865cb8c2`
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
    #[ethcall(name = "intentEventProver", abi = "intentEventProver()")]
    pub struct IntentEventProverCall;
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
    pub enum SpokeChainExecutorCalls {
        CallSpoke(CallSpokeCall),
        IntentEventProver(IntentEventProverCall),
    }
    impl ::ethers::core::abi::AbiDecode for SpokeChainExecutorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CallSpokeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CallSpoke(decoded));
            }
            if let Ok(decoded) = <IntentEventProverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IntentEventProver(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SpokeChainExecutorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CallSpoke(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntentEventProver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SpokeChainExecutorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallSpoke(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntentEventProver(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CallSpokeCall> for SpokeChainExecutorCalls {
        fn from(value: CallSpokeCall) -> Self {
            Self::CallSpoke(value)
        }
    }
    impl ::core::convert::From<IntentEventProverCall> for SpokeChainExecutorCalls {
        fn from(value: IntentEventProverCall) -> Self {
            Self::IntentEventProver(value)
        }
    }
    ///Container type for all return fields from the `intentEventProver` function with signature `intentEventProver()` and selector `0x865cb8c2`
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
    pub struct IntentEventProverReturn(pub ::ethers::core::types::Address);
}
