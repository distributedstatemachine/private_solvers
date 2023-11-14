pub use mock_liquidity_projector::*;
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
pub mod mock_liquidity_projector {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("lockOrBurn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lockOrBurn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationChainId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Token[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Token[]"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MOCKLIQUIDITYPROJECTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\xF6\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xB0bQ\xCA\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x02\x13V[a\0YV[`@Qa\0P\x91\x90a\x03\x02V[`@Q\x80\x91\x03\x90\xF3[```\0[\x82Q\x81\x10\x15a\x01@W\x82\x81\x81Q\x81\x10a\0yWa\0ya\x03ZV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD\x850\x86\x85\x81Q\x81\x10a\0\xA7Wa\0\xA7a\x03ZV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01-\x91\x90a\x03pV[P\x80a\x018\x81a\x03\x99V[\x91PPa\0^V[P`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R\x90a\x01~V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01WW\x90P[P\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x9EW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\xDCWa\x01\xDCa\x01\xA3V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\x0BWa\x02\x0Ba\x01\xA3V[`@R\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x02(W`\0\x80\xFD[\x835\x92P` a\x029\x81\x86\x01a\x01\x87V[\x92P`@\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02WW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x02kW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02}Wa\x02}a\x01\xA3V[a\x02\x8B\x85\x82`\x05\x1B\x01a\x01\xE2V[\x81\x81R\x85\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x85\x01\x90\x8A\x82\x11\x15a\x02\xABW`\0\x80\xFD[\x92\x85\x01\x92[\x81\x84\x10\x15a\x02\xF2W\x84\x84\x8C\x03\x12\x15a\x02\xC8W`\0\x80\x81\xFD[a\x02\xD0a\x01\xB9V[a\x02\xD9\x85a\x01\x87V[\x81R\x84\x87\x015\x87\x82\x01R\x83R\x92\x84\x01\x92\x91\x85\x01\x91a\x02\xB0V[\x80\x96PPPPPPP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x03MW\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x03\x1FV[P\x91\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x03\x82W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\x92W`\0\x80\xFD[\x93\x92PPPV[`\0`\x01\x82\x01a\x03\xB9WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \x1D\xB3\xC1\xD8R\xB5\x9C\xA1Y\xA67:\x89\x1F\x86YVE\xEF\xDF\x87/!\xA38\x06\x9A|YnhadsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKLIQUIDITYPROJECTOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xB0bQ\xCA\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x02\x13V[a\0YV[`@Qa\0P\x91\x90a\x03\x02V[`@Q\x80\x91\x03\x90\xF3[```\0[\x82Q\x81\x10\x15a\x01@W\x82\x81\x81Q\x81\x10a\0yWa\0ya\x03ZV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD\x850\x86\x85\x81Q\x81\x10a\0\xA7Wa\0\xA7a\x03ZV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01-\x91\x90a\x03pV[P\x80a\x018\x81a\x03\x99V[\x91PPa\0^V[P`@\x80Q`\0\x80\x82R` \x82\x01\x90\x92R\x90a\x01~V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01WW\x90P[P\x94\x93PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x9EW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\xDCWa\x01\xDCa\x01\xA3V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02\x0BWa\x02\x0Ba\x01\xA3V[`@R\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x02(W`\0\x80\xFD[\x835\x92P` a\x029\x81\x86\x01a\x01\x87V[\x92P`@\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02WW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x02kW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02}Wa\x02}a\x01\xA3V[a\x02\x8B\x85\x82`\x05\x1B\x01a\x01\xE2V[\x81\x81R\x85\x81\x01\x92P`\x06\x91\x90\x91\x1B\x83\x01\x85\x01\x90\x8A\x82\x11\x15a\x02\xABW`\0\x80\xFD[\x92\x85\x01\x92[\x81\x84\x10\x15a\x02\xF2W\x84\x84\x8C\x03\x12\x15a\x02\xC8W`\0\x80\x81\xFD[a\x02\xD0a\x01\xB9V[a\x02\xD9\x85a\x01\x87V[\x81R\x84\x87\x015\x87\x82\x01R\x83R\x92\x84\x01\x92\x91\x85\x01\x91a\x02\xB0V[\x80\x96PPPPPPP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x03MW\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x01Q\x86\x85\x01R\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x03\x1FV[P\x91\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x03\x82W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\x92W`\0\x80\xFD[\x93\x92PPPV[`\0`\x01\x82\x01a\x03\xB9WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V\xFE\xA2dipfsX\"\x12 \x1D\xB3\xC1\xD8R\xB5\x9C\xA1Y\xA67:\x89\x1F\x86YVE\xEF\xDF\x87/!\xA38\x06\x9A|YnhadsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKLIQUIDITYPROJECTOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockLiquidityProjector<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockLiquidityProjector<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockLiquidityProjector<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockLiquidityProjector<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockLiquidityProjector<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockLiquidityProjector))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockLiquidityProjector<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKLIQUIDITYPROJECTOR_ABI.clone(),
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
                MOCKLIQUIDITYPROJECTOR_ABI.clone(),
                MOCKLIQUIDITYPROJECTOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `lockOrBurn` (0xb06251ca) function
        pub fn lock_or_burn(
            &self,
            destination_chain_id: ::ethers::core::types::U256,
            sender: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<Token>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Token>> {
            self.0
                .method_hash([176, 98, 81, 202], (destination_chain_id, sender, tokens))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockLiquidityProjector<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `lockOrBurn` function with signature `lockOrBurn(uint256,address,(address,uint256)[])` and selector `0xb06251ca`
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
        name = "lockOrBurn",
        abi = "lockOrBurn(uint256,address,(address,uint256)[])"
    )]
    pub struct LockOrBurnCall {
        pub destination_chain_id: ::ethers::core::types::U256,
        pub sender: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<Token>,
    }
    ///Container type for all return fields from the `lockOrBurn` function with signature `lockOrBurn(uint256,address,(address,uint256)[])` and selector `0xb06251ca`
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
    pub struct LockOrBurnReturn(pub ::std::vec::Vec<Token>);
}
