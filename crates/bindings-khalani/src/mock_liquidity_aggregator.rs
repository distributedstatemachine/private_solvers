pub use mock_liquidity_aggregator::*;
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
pub mod mock_liquidity_aggregator {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_klnToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Token"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Token"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
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
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
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
    pub static MOCKLIQUIDITYAGGREGATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x03&8\x03\x80a\x03&\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x02\x93\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x9B}\xA9\x04\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01\xB1V[a\0kV[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xF3[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x82Q` \x84\x01Q`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xFF\x91\x90a\x024V[P`\0T` \x84\x01Q`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R\x91\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01SW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01gW=`\0\x80>=`\0\xFD[PP`@\x80Q\x80\x82\x01\x90\x91R`\0T`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x86\x81\x01Q\x90\x82\x01R\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xACW`\0\x80\xFD[\x91\x90PV[`\0\x80\x82\x84\x03``\x81\x12\x15a\x01\xC5W`\0\x80\xFD[`@\x81\x12\x15a\x01\xD3W`\0\x80\xFD[P`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x02\x05WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Ra\x02\x11\x84a\x01\x95V[\x81R` \x84\x81\x015\x90\x82\x01R\x91Pa\x02+`@\x84\x01a\x01\x95V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x02FW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02VW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 ,\xDCz\x9E\xF4\xAC\x06\xF8\xF0\xFB\x8B\xA6IE\x10\x845o\xB1Pu\xAFZ\xC4\x05\x0F\x11\xC62/\xFA\xF6dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKLIQUIDITYAGGREGATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\x9B}\xA9\x04\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01\xB1V[a\0kV[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xF3[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x82Q` \x84\x01Q`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\0\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\0\xFF\x91\x90a\x024V[P`\0T` \x84\x01Q`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R\x91\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01SW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01gW=`\0\x80>=`\0\xFD[PP`@\x80Q\x80\x82\x01\x90\x91R`\0T`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x86\x81\x01Q\x90\x82\x01R\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xACW`\0\x80\xFD[\x91\x90PV[`\0\x80\x82\x84\x03``\x81\x12\x15a\x01\xC5W`\0\x80\xFD[`@\x81\x12\x15a\x01\xD3W`\0\x80\xFD[P`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x02\x05WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Ra\x02\x11\x84a\x01\x95V[\x81R` \x84\x81\x015\x90\x82\x01R\x91Pa\x02+`@\x84\x01a\x01\x95V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x02FW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02VW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 ,\xDCz\x9E\xF4\xAC\x06\xF8\xF0\xFB\x8B\xA6IE\x10\x845o\xB1Pu\xAFZ\xC4\x05\x0F\x11\xC62/\xFA\xF6dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKLIQUIDITYAGGREGATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockLiquidityAggregator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockLiquidityAggregator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockLiquidityAggregator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockLiquidityAggregator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockLiquidityAggregator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockLiquidityAggregator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockLiquidityAggregator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKLIQUIDITYAGGREGATOR_ABI.clone(),
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
                MOCKLIQUIDITYAGGREGATOR_ABI.clone(),
                MOCKLIQUIDITYAGGREGATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `deposit` (0x9b7da904) function
        pub fn deposit(
            &self,
            token: Token,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Token> {
            self.0
                .method_hash([155, 125, 169, 4], (token, receiver))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockLiquidityAggregator<M> {
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,(address,uint256))")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub token: Token,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit((address,uint256),address)` and selector `0x9b7da904`
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
    #[ethcall(name = "deposit", abi = "deposit((address,uint256),address)")]
    pub struct DepositCall {
        pub token: Token,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deposit` function with signature `deposit((address,uint256),address)` and selector `0x9b7da904`
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
    pub struct DepositReturn(pub Token);
}
