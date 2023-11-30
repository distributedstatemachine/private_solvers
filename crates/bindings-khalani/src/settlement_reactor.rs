pub use settlement_reactor::*;
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
pub mod settlement_reactor {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_verifierRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract VerifierRegistry"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_rewarder"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IRewarder"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_intentsMempool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IntentsMempool"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("settle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("settle"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapIntent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentsLibrary.SwapIntent",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("filler"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fillTimeStamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fillAmount"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SETTLEMENTREACTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\t\x838\x03\x80a\t\x83\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\x89V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x93\x85\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\x02\x80T\x91\x90\x93\x16\x91\x16\x17\x90Ua\0\xD6V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x86W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\0\x9EW`\0\x80\xFD[\x83Qa\0\xA9\x81a\0qV[` \x85\x01Q\x90\x93Pa\0\xBA\x81a\0qV[`@\x85\x01Q\x90\x92Pa\0\xCB\x81a\0qV[\x80\x91PP\x92P\x92P\x92V[a\x08\x9E\x80a\0\xE5`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xF4\x0ElY\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x05*V[a\0EV[\0[`\0a\0Xa\0S\x86a\x06pV[a\x04\x9CV[`\0T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16c\xB6n\x93_a\0|``\x88\x01`@\x89\x01a\x07TV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xE2\x91\x90a\x07vV[`\x01`\x01`\xA0\x1B\x03\x16c\xB8l\x94\xB2a\x01\x12\x83`@\x80Q` \x80\x82\x01\x83R`\0\x90\x91R\x81Q\x90\x81\x01\x90\x91R\x90\x81R\x90V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x011\x91Q\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01t\x91\x90a\x07\x93V[a\x01\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Ftoken not locked at source\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`\xA0\x1B\x03\x16c\xB6n\x93_a\x01\xE6`\x80\x88\x01``\x89\x01a\x07TV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02L\x91\x90a\x07vV[`\x01`\x01`\xA0\x1B\x03\x16c\xEB\x8Eq-a\x02\xAF\x83\x87\x87\x87`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RP`@\x80Q`\x80\x81\x01\x82R\x94\x85R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x85\x01R\x91\x83\x01R``\x82\x01R\x90V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R\x82Q`\x04\x82\x01R` \x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x01Q`D\x82\x01R``\x90\x91\x01Q`d\x82\x01R`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x038\x91\x90a\x07\x93V[a\x03\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fswap not fulfilled at destinatio`D\x82\x01R`7`\xF9\x1B`d\x82\x01R`\x84\x01a\x01\xBCV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16c\x18\x06\xC3Ca\x03\xAF``\x88\x01`@\x89\x01a\x07TV[a\x03\xBF`\xA0\x89\x01`\x80\x8A\x01a\x07\xB5V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x04\x83\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`$\x83\x01R`\xC0\x89\x015`D\x83\x01R\x87\x16`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x1FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x043W=`\0\x80>=`\0\xFD[PP`\x02T`@Qc\x0F\x7F\x17q`\xE3\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc{\xF8\xBB\x88\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04}W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x91W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x04\xE5\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x07\xD2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x17W`\0\x80\xFD[PV[\x805a\x05%\x81a\x05\x02V[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05@W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05WW`\0\x80\xFD[\x85\x01a\x01@\x81\x88\x03\x12\x15a\x05jW`\0\x80\xFD[\x93P` \x85\x015a\x05z\x81a\x05\x02V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\xC9Wa\x05\xC9a\x05\x8FV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x05\xE0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xFBWa\x05\xFBa\x05\x8FV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x06#Wa\x06#a\x05\x8FV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x06<W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05%W`\0\x80\xFD[`\0a\x01@\x826\x03\x12\x15a\x06\x83W`\0\x80\xFD[a\x06\x8Ba\x05\xA5V[a\x06\x94\x83a\x05\x1AV[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xB1W`\0\x80\xFD[a\x06\xBD6\x83\x87\x01a\x05\xCFV[` \x84\x01Ra\x06\xCE`@\x86\x01a\x06\\V[`@\x84\x01Ra\x06\xDF``\x86\x01a\x06\\V[``\x84\x01Ra\x06\xF0`\x80\x86\x01a\x05\x1AV[`\x80\x84\x01Ra\x07\x01`\xA0\x86\x01a\x05\x1AV[`\xA0\x84\x01R`\xC0\x85\x015`\xC0\x84\x01R`\xE0\x85\x015\x91P\x80\x82\x11\x15a\x07$W`\0\x80\xFD[Pa\x0716\x82\x86\x01a\x05\xCFV[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0` \x82\x84\x03\x12\x15a\x07fW`\0\x80\xFD[a\x07o\x82a\x06\\V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x07\x88W`\0\x80\xFD[\x81Qa\x07o\x81a\x05\x02V[`\0` \x82\x84\x03\x12\x15a\x07\xA5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07oW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x07\xC7W`\0\x80\xFD[\x815a\x07o\x81a\x05\x02V[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Q`\0[\x81\x81\x10\x15a\x08HW` \x81\x88\x01\x81\x01Q`d\x86\x84\x01\x01R\x01a\x08+V[P\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \x05\x06\xE1w\xE8\xAD\xEA\xE7@\xB2\x85.\x99\xB2\x93+\xEC\x8E\xB4\xFC\x7F(`\xEE\xBB\x06\xFD\x96\x99P\xA12dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static SETTLEMENTREACTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xF4\x0ElY\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x05*V[a\0EV[\0[`\0a\0Xa\0S\x86a\x06pV[a\x04\x9CV[`\0T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16c\xB6n\x93_a\0|``\x88\x01`@\x89\x01a\x07TV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\0\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xE2\x91\x90a\x07vV[`\x01`\x01`\xA0\x1B\x03\x16c\xB8l\x94\xB2a\x01\x12\x83`@\x80Q` \x80\x82\x01\x83R`\0\x90\x91R\x81Q\x90\x81\x01\x90\x91R\x90\x81R\x90V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x011\x91Q\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01t\x91\x90a\x07\x93V[a\x01\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7Ftoken not locked at source\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0T`\x01`\x01`\xA0\x1B\x03\x16c\xB6n\x93_a\x01\xE6`\x80\x88\x01``\x89\x01a\x07TV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02L\x91\x90a\x07vV[`\x01`\x01`\xA0\x1B\x03\x16c\xEB\x8Eq-a\x02\xAF\x83\x87\x87\x87`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RP`@\x80Q`\x80\x81\x01\x82R\x94\x85R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x85\x01R\x91\x83\x01R``\x82\x01R\x90V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R\x82Q`\x04\x82\x01R` \x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R\x90\x82\x01Q`D\x82\x01R``\x90\x91\x01Q`d\x82\x01R`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x038\x91\x90a\x07\x93V[a\x03\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7Fswap not fulfilled at destinatio`D\x82\x01R`7`\xF9\x1B`d\x82\x01R`\x84\x01a\x01\xBCV[`\x01T`\x01`\x01`\xA0\x1B\x03\x16c\x18\x06\xC3Ca\x03\xAF``\x88\x01`@\x89\x01a\x07TV[a\x03\xBF`\xA0\x89\x01`\x80\x8A\x01a\x07\xB5V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x04\x83\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`$\x83\x01R`\xC0\x89\x015`D\x83\x01R\x87\x16`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x1FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x043W=`\0\x80>=`\0\xFD[PP`\x02T`@Qc\x0F\x7F\x17q`\xE3\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pc{\xF8\xBB\x88\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04}W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\x91W=`\0\x80>=`\0\xFD[PPPPPPPPPV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q\x88a\x01\0\x01Q\x89a\x01 \x01Q`@Q` \x01a\x04\xE5\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x07\xD2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x17W`\0\x80\xFD[PV[\x805a\x05%\x81a\x05\x02V[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05@W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05WW`\0\x80\xFD[\x85\x01a\x01@\x81\x88\x03\x12\x15a\x05jW`\0\x80\xFD[\x93P` \x85\x015a\x05z\x81a\x05\x02V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x05\xC9Wa\x05\xC9a\x05\x8FV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x05\xE0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x05\xFBWa\x05\xFBa\x05\x8FV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x06#Wa\x06#a\x05\x8FV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x06<W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05%W`\0\x80\xFD[`\0a\x01@\x826\x03\x12\x15a\x06\x83W`\0\x80\xFD[a\x06\x8Ba\x05\xA5V[a\x06\x94\x83a\x05\x1AV[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x06\xB1W`\0\x80\xFD[a\x06\xBD6\x83\x87\x01a\x05\xCFV[` \x84\x01Ra\x06\xCE`@\x86\x01a\x06\\V[`@\x84\x01Ra\x06\xDF``\x86\x01a\x06\\V[``\x84\x01Ra\x06\xF0`\x80\x86\x01a\x05\x1AV[`\x80\x84\x01Ra\x07\x01`\xA0\x86\x01a\x05\x1AV[`\xA0\x84\x01R`\xC0\x85\x015`\xC0\x84\x01R`\xE0\x85\x015\x91P\x80\x82\x11\x15a\x07$W`\0\x80\xFD[Pa\x0716\x82\x86\x01a\x05\xCFV[`\xE0\x83\x01RPa\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0` \x82\x84\x03\x12\x15a\x07fW`\0\x80\xFD[a\x07o\x82a\x06\\V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x07\x88W`\0\x80\xFD[\x81Qa\x07o\x81a\x05\x02V[`\0` \x82\x84\x03\x12\x15a\x07\xA5W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x07oW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x07\xC7W`\0\x80\xFD[\x815a\x07o\x81a\x05\x02V[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8C``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8C`\xE0\x1B\x16`\x14\x85\x01R\x80\x8B`\xE0\x1B\x16`\x18\x85\x01RP\x80\x89``\x1B\x16`\x1C\x84\x01R\x80\x88``\x1B\x16`0\x84\x01RP\x85`D\x83\x01R\x84Q`\0[\x81\x81\x10\x15a\x08HW` \x81\x88\x01\x81\x01Q`d\x86\x84\x01\x01R\x01a\x08+V[P\x90\x91\x01`d\x81\x01\x93\x90\x93RP`\x84\x82\x01R`\xA4\x01\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \x05\x06\xE1w\xE8\xAD\xEA\xE7@\xB2\x85.\x99\xB2\x93+\xEC\x8E\xB4\xFC\x7F(`\xEE\xBB\x06\xFD\x96\x99P\xA12dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static SETTLEMENTREACTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SettlementReactor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SettlementReactor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SettlementReactor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SettlementReactor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SettlementReactor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SettlementReactor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SettlementReactor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SETTLEMENTREACTOR_ABI.clone(),
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
                SETTLEMENTREACTOR_ABI.clone(),
                SETTLEMENTREACTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `settle` (0xf40e6c59) function
        pub fn settle(
            &self,
            swap_intent: SwapIntent,
            filler: ::ethers::core::types::Address,
            fill_time_stamp: ::ethers::core::types::U256,
            fill_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [244, 14, 108, 89],
                    (swap_intent, filler, fill_time_stamp, fill_amount),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SettlementReactor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `settle` function with signature `settle((address,bytes,uint32,uint32,address,address,uint256,bytes,uint256,uint256),address,uint256,uint256)` and selector `0xf40e6c59`
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
        name = "settle",
        abi = "settle((address,bytes,uint32,uint32,address,address,uint256,bytes,uint256,uint256),address,uint256,uint256)"
    )]
    pub struct SettleCall {
        pub swap_intent: SwapIntent,
        pub filler: ::ethers::core::types::Address,
        pub fill_time_stamp: ::ethers::core::types::U256,
        pub fill_amount: ::ethers::core::types::U256,
    }
}
