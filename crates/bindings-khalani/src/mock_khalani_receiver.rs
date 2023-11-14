pub use mock_khalani_receiver::*;
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
pub mod mock_khalani_receiver {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("onMessageReceive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onMessageReceive"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokens"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
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
    pub static MOCKKHALANIRECEIVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\x17\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xCB\x05\x07\n\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01\\V[a\0EV[\0[\x7F\x84\x89$\xC2\xDCy\xF9\xAA\xE6\xE3\x8C\x9C\x8BP8\x1AJKw;\x05\xF0|fu\xD6\xEF\x83$W\x0B(\x85\x85\x85`@Qa\0x\x93\x92\x91\x90a\x02uV[`@Q\x80\x91\x03\x90\xA1PPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x9EW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\0\xDCWa\0\xDCa\0\xA3V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\x0BWa\x01\x0Ba\0\xA3V[`@R\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x01%W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01=W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x01UW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x01tW`\0\x80\xFD[\x855\x94P` a\x01\x85\x81\x88\x01a\0\x87V[\x94P`@\x80\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xA3W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x01\xB7W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\xC9Wa\x01\xC9a\0\xA3V[a\x01\xD7\x85\x82`\x05\x1B\x01a\0\xE2V[\x81\x81R`\x06\x91\x90\x91\x1B\x83\x01\x85\x01\x90\x85\x81\x01\x90\x8D\x83\x11\x15a\x01\xF6W`\0\x80\xFD[\x93\x86\x01\x93[\x82\x85\x10\x15a\x02=W\x85\x85\x8F\x03\x12\x15a\x02\x13W`\0\x80\x81\xFD[a\x02\x1Ba\0\xB9V[a\x02$\x86a\0\x87V[\x81R\x85\x88\x015\x88\x82\x01R\x82R\x93\x85\x01\x93\x90\x86\x01\x90a\x01\xFBV[\x98PPP``\x8A\x015\x93P\x80\x84\x11\x15a\x02UW`\0\x80\xFD[PPPa\x02d\x88\x82\x89\x01a\x01\x13V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0``\x82\x01\x85\x83R` `\x01\x80`\xA0\x1B\x03\x80\x87\x16\x82\x86\x01R`@``\x81\x87\x01R\x83\x87Q\x80\x86R`\x80\x88\x01\x91P\x84\x89\x01\x95P`\0[\x81\x81\x10\x15a\x02\xD1W\x86Q\x80Q\x86\x16\x84R\x86\x01Q\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01a\x02\xAAV[P\x90\x9A\x99PPPPPPPPPPV\xFE\xA2dipfsX\"\x12 \xFB\x82\xA6_\x95\x19\xD0\x19\x14\x88M\xD6I\x9Fb$nT\xD2B\xB0\xBA)\xBB\xE6G\xC2\xED\xDE\x95\x1B\xDBdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKKHALANIRECEIVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xCB\x05\x07\n\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01\\V[a\0EV[\0[\x7F\x84\x89$\xC2\xDCy\xF9\xAA\xE6\xE3\x8C\x9C\x8BP8\x1AJKw;\x05\xF0|fu\xD6\xEF\x83$W\x0B(\x85\x85\x85`@Qa\0x\x93\x92\x91\x90a\x02uV[`@Q\x80\x91\x03\x90\xA1PPPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x9EW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\0\xDCWa\0\xDCa\0\xA3V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x01\x0BWa\x01\x0Ba\0\xA3V[`@R\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x01%W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01=W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x01UW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x01tW`\0\x80\xFD[\x855\x94P` a\x01\x85\x81\x88\x01a\0\x87V[\x94P`@\x80\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01\xA3W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a\x01\xB7W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\xC9Wa\x01\xC9a\0\xA3V[a\x01\xD7\x85\x82`\x05\x1B\x01a\0\xE2V[\x81\x81R`\x06\x91\x90\x91\x1B\x83\x01\x85\x01\x90\x85\x81\x01\x90\x8D\x83\x11\x15a\x01\xF6W`\0\x80\xFD[\x93\x86\x01\x93[\x82\x85\x10\x15a\x02=W\x85\x85\x8F\x03\x12\x15a\x02\x13W`\0\x80\x81\xFD[a\x02\x1Ba\0\xB9V[a\x02$\x86a\0\x87V[\x81R\x85\x88\x015\x88\x82\x01R\x82R\x93\x85\x01\x93\x90\x86\x01\x90a\x01\xFBV[\x98PPP``\x8A\x015\x93P\x80\x84\x11\x15a\x02UW`\0\x80\xFD[PPPa\x02d\x88\x82\x89\x01a\x01\x13V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0``\x82\x01\x85\x83R` `\x01\x80`\xA0\x1B\x03\x80\x87\x16\x82\x86\x01R`@``\x81\x87\x01R\x83\x87Q\x80\x86R`\x80\x88\x01\x91P\x84\x89\x01\x95P`\0[\x81\x81\x10\x15a\x02\xD1W\x86Q\x80Q\x86\x16\x84R\x86\x01Q\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01a\x02\xAAV[P\x90\x9A\x99PPPPPPPPPPV\xFE\xA2dipfsX\"\x12 \xFB\x82\xA6_\x95\x19\xD0\x19\x14\x88M\xD6I\x9Fb$nT\xD2B\xB0\xBA)\xBB\xE6G\xC2\xED\xDE\x95\x1B\xDBdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKKHALANIRECEIVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockKhalaniReceiver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockKhalaniReceiver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockKhalaniReceiver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockKhalaniReceiver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockKhalaniReceiver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockKhalaniReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockKhalaniReceiver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKKHALANIRECEIVER_ABI.clone(),
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
                MOCKKHALANIRECEIVER_ABI.clone(),
                MOCKKHALANIRECEIVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `onMessageReceive` (0xcb05070a) function
        pub fn on_message_receive(
            &self,
            chain_id: ::ethers::core::types::U256,
            sender: ::ethers::core::types::Address,
            tokens: ::std::vec::Vec<Token>,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([203, 5, 7, 10], (chain_id, sender, tokens, message))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Received` event
        pub fn received_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReceivedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReceivedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockKhalaniReceiver<M> {
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
    #[ethevent(name = "Received", abi = "Received(uint256,address,(address,uint256)[])")]
    pub struct ReceivedFilter {
        pub chain_id: ::ethers::core::types::U256,
        pub sender: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<Token>,
    }
    ///Container type for all input parameters for the `onMessageReceive` function with signature `onMessageReceive(uint256,address,(address,uint256)[],bytes)` and selector `0xcb05070a`
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
        name = "onMessageReceive",
        abi = "onMessageReceive(uint256,address,(address,uint256)[],bytes)"
    )]
    pub struct OnMessageReceiveCall {
        pub chain_id: ::ethers::core::types::U256,
        pub sender: ::ethers::core::types::Address,
        pub tokens: ::std::vec::Vec<Token>,
        pub message: ::ethers::core::types::Bytes,
    }
}
