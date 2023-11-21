pub use escrow::*;
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
pub mod escrow {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("lockTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lockTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("intent"),
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SwapIntentsLibrary.SwapIntent",
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("TokensLocked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokensLocked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("intentId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
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
    pub static ESCROW_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\xBE\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0*W`\x005`\xE0\x1C\x80b\xE8:\xFD\x14a\0/W[`\0\x80\xFD[a\0Ba\0=6`\x04a\0\xEEV[a\0DV[\0[`\0a\0Wa\0R\x83a\x02.V[a\0\x96V[\x90P\x7F\xB9\xEED\xDA\x07N\x91\xFD\xE0Z\xF9Q]$\xBF\n\x9C\x9A\x83E\x196\x8F\0\xEB\xFE is\x0E\xE1\x0F\x81`@Qa\0\x8A\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PPV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q`@Q` \x01a\0\xD1\x97\x96\x95\x94\x93\x92\x91\x90a\x02\xFBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x17W`\0\x80\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a\x01*W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01kWa\x01ka\x011V[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x88W`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x01\x9EW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xB9Wa\x01\xB9a\x011V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x01\xE1Wa\x01\xE1a\x011V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x01\xFAW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\x88W`\0\x80\xFD[`\0a\x01\0\x826\x03\x12\x15a\x02AW`\0\x80\xFD[a\x02Ia\x01GV[a\x02R\x83a\x01qV[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02oW`\0\x80\xFD[a\x02{6\x83\x87\x01a\x01\x8DV[` \x84\x01Ra\x02\x8C`@\x86\x01a\x02\x1AV[`@\x84\x01Ra\x02\x9D``\x86\x01a\x02\x1AV[``\x84\x01Ra\x02\xAE`\x80\x86\x01a\x01qV[`\x80\x84\x01Ra\x02\xBF`\xA0\x86\x01a\x01qV[`\xA0\x84\x01R`\xC0\x85\x015`\xC0\x84\x01R`\xE0\x85\x015\x91P\x80\x82\x11\x15a\x02\xE2W`\0\x80\xFD[Pa\x02\xEF6\x82\x86\x01a\x01\x8DV[`\xE0\x83\x01RP\x92\x91PPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8A``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8A`\xE0\x1B\x16`\x14\x85\x01R\x80\x89`\xE0\x1B\x16`\x18\x85\x01RP\x80\x87``\x1B\x16`\x1C\x84\x01R\x80\x86``\x1B\x16`0\x84\x01RP\x83`D\x83\x01R\x82Q`\0[\x81\x81\x10\x15a\x03qW` \x81\x86\x01\x81\x01Q`d\x86\x84\x01\x01R\x01a\x03TV[P`\0\x92\x01`d\x01\x91\x82RP\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \x19;W3\xE1\xC1f(7by\xFB_\xD2bhru3\xA3\xDE\x90\xF4J\xB0g\x9C\xD7\x13\x16V\x14dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static ESCROW_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0*W`\x005`\xE0\x1C\x80b\xE8:\xFD\x14a\0/W[`\0\x80\xFD[a\0Ba\0=6`\x04a\0\xEEV[a\0DV[\0[`\0a\0Wa\0R\x83a\x02.V[a\0\x96V[\x90P\x7F\xB9\xEED\xDA\x07N\x91\xFD\xE0Z\xF9Q]$\xBF\n\x9C\x9A\x83E\x196\x8F\0\xEB\xFE is\x0E\xE1\x0F\x81`@Qa\0\x8A\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PPV[`\0\x81`\0\x01Q\x82`@\x01Q\x83``\x01Q\x84`\x80\x01Q\x85`\xA0\x01Q\x86`\xC0\x01Q\x87`\xE0\x01Q`@Q` \x01a\0\xD1\x97\x96\x95\x94\x93\x92\x91\x90a\x02\xFBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x17W`\0\x80\xFD[\x82\x01a\x01\0\x81\x85\x03\x12\x15a\x01*W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01\0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01kWa\x01ka\x011V[`@R\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x88W`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x01\x9EW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xB9Wa\x01\xB9a\x011V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x01\xE1Wa\x01\xE1a\x011V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x01\xFAW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\x88W`\0\x80\xFD[`\0a\x01\0\x826\x03\x12\x15a\x02AW`\0\x80\xFD[a\x02Ia\x01GV[a\x02R\x83a\x01qV[\x81R` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02oW`\0\x80\xFD[a\x02{6\x83\x87\x01a\x01\x8DV[` \x84\x01Ra\x02\x8C`@\x86\x01a\x02\x1AV[`@\x84\x01Ra\x02\x9D``\x86\x01a\x02\x1AV[``\x84\x01Ra\x02\xAE`\x80\x86\x01a\x01qV[`\x80\x84\x01Ra\x02\xBF`\xA0\x86\x01a\x01qV[`\xA0\x84\x01R`\xC0\x85\x015`\xC0\x84\x01R`\xE0\x85\x015\x91P\x80\x82\x11\x15a\x02\xE2W`\0\x80\xFD[Pa\x02\xEF6\x82\x86\x01a\x01\x8DV[`\xE0\x83\x01RP\x92\x91PPV[`\0k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x8A``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xE0\x1B\x80\x8A`\xE0\x1B\x16`\x14\x85\x01R\x80\x89`\xE0\x1B\x16`\x18\x85\x01RP\x80\x87``\x1B\x16`\x1C\x84\x01R\x80\x86``\x1B\x16`0\x84\x01RP\x83`D\x83\x01R\x82Q`\0[\x81\x81\x10\x15a\x03qW` \x81\x86\x01\x81\x01Q`d\x86\x84\x01\x01R\x01a\x03TV[P`\0\x92\x01`d\x01\x91\x82RP\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \x19;W3\xE1\xC1f(7by\xFB_\xD2bhru3\xA3\xDE\x90\xF4J\xB0g\x9C\xD7\x13\x16V\x14dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static ESCROW_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Escrow<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Escrow<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Escrow<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Escrow<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Escrow<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Escrow)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Escrow<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ESCROW_ABI.clone(),
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
                ESCROW_ABI.clone(),
                ESCROW_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `lockTokens` (0x00e83afd) function
        pub fn lock_tokens(
            &self,
            intent: SwapIntent,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 232, 58, 253], (intent,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `TokensLocked` event
        pub fn tokens_locked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokensLockedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokensLockedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Escrow<M> {
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
    #[ethevent(name = "TokensLocked", abi = "TokensLocked(bytes32)")]
    pub struct TokensLockedFilter {
        pub intent_id: [u8; 32],
    }
    ///Container type for all input parameters for the `lockTokens` function with signature `lockTokens((address,bytes,uint32,uint32,address,address,uint256,bytes))` and selector `0x00e83afd`
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
        name = "lockTokens",
        abi = "lockTokens((address,bytes,uint32,uint32,address,address,uint256,bytes))"
    )]
    pub struct LockTokensCall {
        pub intent: SwapIntent,
    }
}
