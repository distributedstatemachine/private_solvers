pub use mock_gmp_intent_event_verifier::*;
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
pub mod mock_gmp_intent_event_verifier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("flag"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flag"),
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
                    ::std::borrow::ToOwned::to_owned("setFlag"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFlag"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_flag"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("verifySwapIntentFilledEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verifySwapIntentFilledEvent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("event_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentEventLibrary.SwapIntentFilled",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("verifySwapIntentTokenLockEvent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verifySwapIntentTokenLockEvent",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("event_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentEventLibrary.SwapIntentTokenLock",
                                        ),
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
    pub static MOCKGMPINTENTEVENTVERIFIER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\x1A\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c9'\xF6\xAF\x14a\0QW\x80cQ\x83'o\x14a\0tW\x80c\x89\x0E\xBAh\x14a\0\x9BW\x80c\xB8l\x94\xB2\x14a\0\xA8W[`\0\x80\xFD[a\0ra\0_6`\x04a\x01\xC5V[`\0\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[a\0\x87a\0\x826`\x04a\x01\xEEV[a\0\xBBV[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\x87\x90`\xFF\x16\x81V[a\0\x87a\0\xB66`\x04a\x02\x06V[a\0\xF9V[`\0\x80a\0\xD5a\0\xD06\x85\x90\x03\x85\x01\x85a\x02\x18V[a\x01\x13V[`\0T\x90\x91P`\xFF\x16\x15\x15`\x01\x03a\0\xF0WP`\x01\x92\x91PPV[P`\0\x92\x91PPV[`\0\x80a\0\xD5a\x01\x0E6\x85\x90\x03\x85\x01\x85a\x02\x94V[a\x01\x91V[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q`@Q` \x01a\x01t\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x93\x90\x93R``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x83\x01R`D\x82\x01R`d\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x01tV[`\0` \x82\x84\x03\x12\x15a\x01\xD7W`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x01\xE7W`\0\x80\xFD[\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a\x02\0W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\0W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a\x02*W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x02[WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x825\x81R` \x83\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02|W`\0\x80\xFD[` \x82\x01R`@\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\xA6W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x02\xD7WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x915\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 \x89\xE9\x10\xB7L\x9BP\xCF\xE8\xA5\xB966\xFE!\xCD2\xAB2\xFD_gE\xD8\xB3[x\xE4\x96\x01\xA6\x87dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKGMPINTENTEVENTVERIFIER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c9'\xF6\xAF\x14a\0QW\x80cQ\x83'o\x14a\0tW\x80c\x89\x0E\xBAh\x14a\0\x9BW\x80c\xB8l\x94\xB2\x14a\0\xA8W[`\0\x80\xFD[a\0ra\0_6`\x04a\x01\xC5V[`\0\x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[\0[a\0\x87a\0\x826`\x04a\x01\xEEV[a\0\xBBV[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\x87\x90`\xFF\x16\x81V[a\0\x87a\0\xB66`\x04a\x02\x06V[a\0\xF9V[`\0\x80a\0\xD5a\0\xD06\x85\x90\x03\x85\x01\x85a\x02\x18V[a\x01\x13V[`\0T\x90\x91P`\xFF\x16\x15\x15`\x01\x03a\0\xF0WP`\x01\x92\x91PPV[P`\0\x92\x91PPV[`\0\x80a\0\xD5a\x01\x0E6\x85\x90\x03\x85\x01\x85a\x02\x94V[a\x01\x91V[`\0\x81`\0\x01Q\x82` \x01Q\x83`@\x01Q`@Q` \x01a\x01t\x93\x92\x91\x90o\x14\xDD\xD8\\\x12[\x9D\x19[\x9D\x11\x9A[\x1B\x19Y`\x82\x1B\x81R`\x10\x81\x01\x93\x90\x93R``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`0\x83\x01R`D\x82\x01R`d\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x80Q`@QrSwapIntentTokenLock`h\x1B` \x82\x01R`3\x81\x01\x91\x90\x91R`\0\x90`S\x01a\x01tV[`\0` \x82\x84\x03\x12\x15a\x01\xD7W`\0\x80\xFD[\x815\x80\x15\x15\x81\x14a\x01\xE7W`\0\x80\xFD[\x93\x92PPPV[`\0``\x82\x84\x03\x12\x15a\x02\0W`\0\x80\xFD[P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\0W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a\x02*W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x02[WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x825\x81R` \x83\x015`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02|W`\0\x80\xFD[` \x82\x01R`@\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x02\xA6W`\0\x80\xFD[`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x02\xD7WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x915\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 \x89\xE9\x10\xB7L\x9BP\xCF\xE8\xA5\xB966\xFE!\xCD2\xAB2\xFD_gE\xD8\xB3[x\xE4\x96\x01\xA6\x87dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKGMPINTENTEVENTVERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockGMPIntentEventVerifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockGMPIntentEventVerifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockGMPIntentEventVerifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockGMPIntentEventVerifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockGMPIntentEventVerifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockGMPIntentEventVerifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockGMPIntentEventVerifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKGMPINTENTEVENTVERIFIER_ABI.clone(),
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
                MOCKGMPINTENTEVENTVERIFIER_ABI.clone(),
                MOCKGMPINTENTEVENTVERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `flag` (0x890eba68) function
        pub fn flag(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([137, 14, 186, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFlag` (0x3927f6af) function
        pub fn set_flag(
            &self,
            flag: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 39, 246, 175], flag)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySwapIntentFilledEvent` (0x5183276f) function
        pub fn verify_swap_intent_filled_event(
            &self,
            event: SwapIntentFilled,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([81, 131, 39, 111], (event,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifySwapIntentTokenLockEvent` (0xb86c94b2) function
        pub fn verify_swap_intent_token_lock_event(
            &self,
            event: SwapIntentTokenLock,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([184, 108, 148, 178], (event,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockGMPIntentEventVerifier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `flag` function with signature `flag()` and selector `0x890eba68`
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
    #[ethcall(name = "flag", abi = "flag()")]
    pub struct FlagCall;
    ///Container type for all input parameters for the `setFlag` function with signature `setFlag(bool)` and selector `0x3927f6af`
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
    #[ethcall(name = "setFlag", abi = "setFlag(bool)")]
    pub struct SetFlagCall {
        pub flag: bool,
    }
    ///Container type for all input parameters for the `verifySwapIntentFilledEvent` function with signature `verifySwapIntentFilledEvent((bytes32,address,uint256))` and selector `0x5183276f`
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
        name = "verifySwapIntentFilledEvent",
        abi = "verifySwapIntentFilledEvent((bytes32,address,uint256))"
    )]
    pub struct VerifySwapIntentFilledEventCall {
        pub event: SwapIntentFilled,
    }
    ///Container type for all input parameters for the `verifySwapIntentTokenLockEvent` function with signature `verifySwapIntentTokenLockEvent((bytes32))` and selector `0xb86c94b2`
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
        name = "verifySwapIntentTokenLockEvent",
        abi = "verifySwapIntentTokenLockEvent((bytes32))"
    )]
    pub struct VerifySwapIntentTokenLockEventCall {
        pub event: SwapIntentTokenLock,
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
    pub enum MockGMPIntentEventVerifierCalls {
        Flag(FlagCall),
        SetFlag(SetFlagCall),
        VerifySwapIntentFilledEvent(VerifySwapIntentFilledEventCall),
        VerifySwapIntentTokenLockEvent(VerifySwapIntentTokenLockEventCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockGMPIntentEventVerifierCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FlagCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Flag(decoded));
            }
            if let Ok(decoded) = <SetFlagCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFlag(decoded));
            }
            if let Ok(decoded) = <VerifySwapIntentFilledEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifySwapIntentFilledEvent(decoded));
            }
            if let Ok(decoded) = <VerifySwapIntentTokenLockEventCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifySwapIntentTokenLockEvent(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockGMPIntentEventVerifierCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Flag(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFlag(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifySwapIntentFilledEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifySwapIntentTokenLockEvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MockGMPIntentEventVerifierCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Flag(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFlag(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifySwapIntentFilledEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VerifySwapIntentTokenLockEvent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FlagCall> for MockGMPIntentEventVerifierCalls {
        fn from(value: FlagCall) -> Self {
            Self::Flag(value)
        }
    }
    impl ::core::convert::From<SetFlagCall> for MockGMPIntentEventVerifierCalls {
        fn from(value: SetFlagCall) -> Self {
            Self::SetFlag(value)
        }
    }
    impl ::core::convert::From<VerifySwapIntentFilledEventCall>
    for MockGMPIntentEventVerifierCalls {
        fn from(value: VerifySwapIntentFilledEventCall) -> Self {
            Self::VerifySwapIntentFilledEvent(value)
        }
    }
    impl ::core::convert::From<VerifySwapIntentTokenLockEventCall>
    for MockGMPIntentEventVerifierCalls {
        fn from(value: VerifySwapIntentTokenLockEventCall) -> Self {
            Self::VerifySwapIntentTokenLockEvent(value)
        }
    }
    ///Container type for all return fields from the `flag` function with signature `flag()` and selector `0x890eba68`
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
    pub struct FlagReturn(pub bool);
    ///Container type for all return fields from the `verifySwapIntentFilledEvent` function with signature `verifySwapIntentFilledEvent((bytes32,address,uint256))` and selector `0x5183276f`
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
    pub struct VerifySwapIntentFilledEventReturn(pub bool);
    ///Container type for all return fields from the `verifySwapIntentTokenLockEvent` function with signature `verifySwapIntentTokenLockEvent((bytes32))` and selector `0xb86c94b2`
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
    pub struct VerifySwapIntentTokenLockEventReturn(pub bool);
    ///`SwapIntentFilled(bytes32,address,uint256)`
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
    pub struct SwapIntentFilled {
        pub intent_id: [u8; 32],
        pub filler: ::ethers::core::types::Address,
        pub fill_amount: ::ethers::core::types::U256,
    }
    ///`SwapIntentTokenLock(bytes32)`
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
    pub struct SwapIntentTokenLock {
        pub intent_id: [u8; 32],
    }
}
