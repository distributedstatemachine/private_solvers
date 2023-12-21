pub use khalani_setter::*;
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
pub mod khalani_setter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("initializeRemoteRequestProcessor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "initializeRemoteRequestProcessor",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hyperlaneAdapter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidityProjector",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "interchainLiquidityHub",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidityAggregator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("registerRemoteAdapter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerRemoteAdapter",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adapter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("RemoteAdapterRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RemoteAdapterRegistered",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("originChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("adapter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "RemoteRequestProcessorInitialized",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RemoteRequestProcessorInitialized",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("hyperlaneAdapter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidityProjector",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "interchainLiquidityHub",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "liquidityAggregator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NotContractOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotContractOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_contractOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static KHALANISETTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02\xE1\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x87pA\xB0\x14a\0;W\x80c\xA2q\x9D\x9C\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x02+V[a\0cV[\0[a\0Na\0^6`\x04a\x02\x7FV[a\x01\x0EV[a\0ka\x01\x85V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x81\x16` \x83\x01R\x84\x81\x16\x82\x84\x01R\x83\x16``\x82\x01R\x90Q\x7F\x91\x1DQ\x95FX\xCB\x80Vk\xB0\xB7\xD5$\xA97\xEDKDn\xC1\xFA\xF7\xC9\xC8\x83\xC0\xD6\x1B\x1E{\x9D\x91\x81\x90\x03`\x80\x01\x90\xA1`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x03\x80T\x94\x86\x16\x94\x82\x16\x94\x90\x94\x17\x90\x93U`\x04\x80T\x92\x85\x16\x92\x84\x16\x92\x90\x92\x17\x90\x91U`\x05\x80T\x91\x90\x93\x16\x91\x16\x17\x90UV[a\x01\x16a\x01\x85V[`@\x80Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R\x7FD9\xC2+\xF4\x8E\x80\x16{T\xC6F4\xBE\xBC\xEF$\x89\xC1\xB8\xEF\x94'\xAE\xE1\x11\xCE\xB8-\x85`9\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x91\x82R`\x06` R`@\x90\x91 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1C`\x03\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\rW\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1FT`@\x80Q`\x01b\xBE\xD85`\xE0\x1B\x03\x19\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01RQ\x90\x81\x90\x03`D\x01\x90\xFD[V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02&W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x02AW`\0\x80\xFD[a\x02J\x85a\x02\x0FV[\x93Pa\x02X` \x86\x01a\x02\x0FV[\x92Pa\x02f`@\x86\x01a\x02\x0FV[\x91Pa\x02t``\x86\x01a\x02\x0FV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\x92W`\0\x80\xFD[\x825\x91Pa\x02\xA2` \x84\x01a\x02\x0FV[\x90P\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 \xC8d\xF9?\x025\xF6.p]a\x06X\xC4\xAE,\xCB\xA5\xCB-\x96\xE1\xEFv;&U\xC4\xACj\xC1\xB5dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static KHALANISETTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c\x87pA\xB0\x14a\0;W\x80c\xA2q\x9D\x9C\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x02+V[a\0cV[\0[a\0Na\0^6`\x04a\x02\x7FV[a\x01\x0EV[a\0ka\x01\x85V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x81\x16` \x83\x01R\x84\x81\x16\x82\x84\x01R\x83\x16``\x82\x01R\x90Q\x7F\x91\x1DQ\x95FX\xCB\x80Vk\xB0\xB7\xD5$\xA97\xEDKDn\xC1\xFA\xF7\xC9\xC8\x83\xC0\xD6\x1B\x1E{\x9D\x91\x81\x90\x03`\x80\x01\x90\xA1`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x03\x80T\x94\x86\x16\x94\x82\x16\x94\x90\x94\x17\x90\x93U`\x04\x80T\x92\x85\x16\x92\x84\x16\x92\x90\x92\x17\x90\x91U`\x05\x80T\x91\x90\x93\x16\x91\x16\x17\x90UV[a\x01\x16a\x01\x85V[`@\x80Q\x83\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R\x7FD9\xC2+\xF4\x8E\x80\x16{T\xC6F4\xBE\xBC\xEF$\x89\xC1\xB8\xEF\x94'\xAE\xE1\x11\xCE\xB8-\x85`9\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x91\x82R`\x06` R`@\x90\x91 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1C`\x03\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02\rW\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1FT`@\x80Q`\x01b\xBE\xD85`\xE0\x1B\x03\x19\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01RQ\x90\x81\x90\x03`D\x01\x90\xFD[V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02&W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x02AW`\0\x80\xFD[a\x02J\x85a\x02\x0FV[\x93Pa\x02X` \x86\x01a\x02\x0FV[\x92Pa\x02f`@\x86\x01a\x02\x0FV[\x91Pa\x02t``\x86\x01a\x02\x0FV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\x92W`\0\x80\xFD[\x825\x91Pa\x02\xA2` \x84\x01a\x02\x0FV[\x90P\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 \xC8d\xF9?\x025\xF6.p]a\x06X\xC4\xAE,\xCB\xA5\xCB-\x96\xE1\xEFv;&U\xC4\xACj\xC1\xB5dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static KHALANISETTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct KhalaniSetter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for KhalaniSetter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for KhalaniSetter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for KhalaniSetter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for KhalaniSetter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(KhalaniSetter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> KhalaniSetter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    KHALANISETTER_ABI.clone(),
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
                KHALANISETTER_ABI.clone(),
                KHALANISETTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `initializeRemoteRequestProcessor` (0x877041b0) function
        pub fn initialize_remote_request_processor(
            &self,
            hyperlane_adapter: ::ethers::core::types::Address,
            liquidity_projector: ::ethers::core::types::Address,
            interchain_liquidity_hub: ::ethers::core::types::Address,
            liquidity_aggregator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [135, 112, 65, 176],
                    (
                        hyperlane_adapter,
                        liquidity_projector,
                        interchain_liquidity_hub,
                        liquidity_aggregator,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerRemoteAdapter` (0xa2719d9c) function
        pub fn register_remote_adapter(
            &self,
            chain_id: ::ethers::core::types::U256,
            adapter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 113, 157, 156], (chain_id, adapter))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `RemoteAdapterRegistered` event
        pub fn remote_adapter_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoteAdapterRegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RemoteRequestProcessorInitialized` event
        pub fn remote_request_processor_initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RemoteRequestProcessorInitializedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            KhalaniSetterEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for KhalaniSetter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotContractOwner` with signature `NotContractOwner(address,address)` and selector `0xff4127cb`
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
    #[etherror(name = "NotContractOwner", abi = "NotContractOwner(address,address)")]
    pub struct NotContractOwner {
        pub user: ::ethers::core::types::Address,
        pub contract_owner: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "RemoteAdapterRegistered",
        abi = "RemoteAdapterRegistered(uint256,address)"
    )]
    pub struct RemoteAdapterRegisteredFilter {
        pub origin_chain_id: ::ethers::core::types::U256,
        pub adapter: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "RemoteRequestProcessorInitialized",
        abi = "RemoteRequestProcessorInitialized(address,address,address,address)"
    )]
    pub struct RemoteRequestProcessorInitializedFilter {
        pub hyperlane_adapter: ::ethers::core::types::Address,
        pub liquidity_projector: ::ethers::core::types::Address,
        pub interchain_liquidity_hub: ::ethers::core::types::Address,
        pub liquidity_aggregator: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
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
    pub enum KhalaniSetterEvents {
        RemoteAdapterRegisteredFilter(RemoteAdapterRegisteredFilter),
        RemoteRequestProcessorInitializedFilter(RemoteRequestProcessorInitializedFilter),
    }
    impl ::ethers::contract::EthLogDecode for KhalaniSetterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = RemoteAdapterRegisteredFilter::decode_log(log) {
                return Ok(KhalaniSetterEvents::RemoteAdapterRegisteredFilter(decoded));
            }
            if let Ok(decoded) = RemoteRequestProcessorInitializedFilter::decode_log(
                log,
            ) {
                return Ok(
                    KhalaniSetterEvents::RemoteRequestProcessorInitializedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for KhalaniSetterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RemoteAdapterRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoteRequestProcessorInitializedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<RemoteAdapterRegisteredFilter> for KhalaniSetterEvents {
        fn from(value: RemoteAdapterRegisteredFilter) -> Self {
            Self::RemoteAdapterRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<RemoteRequestProcessorInitializedFilter>
    for KhalaniSetterEvents {
        fn from(value: RemoteRequestProcessorInitializedFilter) -> Self {
            Self::RemoteRequestProcessorInitializedFilter(value)
        }
    }
    ///Container type for all input parameters for the `initializeRemoteRequestProcessor` function with signature `initializeRemoteRequestProcessor(address,address,address,address)` and selector `0x877041b0`
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
        name = "initializeRemoteRequestProcessor",
        abi = "initializeRemoteRequestProcessor(address,address,address,address)"
    )]
    pub struct InitializeRemoteRequestProcessorCall {
        pub hyperlane_adapter: ::ethers::core::types::Address,
        pub liquidity_projector: ::ethers::core::types::Address,
        pub interchain_liquidity_hub: ::ethers::core::types::Address,
        pub liquidity_aggregator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `registerRemoteAdapter` function with signature `registerRemoteAdapter(uint256,address)` and selector `0xa2719d9c`
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
        name = "registerRemoteAdapter",
        abi = "registerRemoteAdapter(uint256,address)"
    )]
    pub struct RegisterRemoteAdapterCall {
        pub chain_id: ::ethers::core::types::U256,
        pub adapter: ::ethers::core::types::Address,
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
    pub enum KhalaniSetterCalls {
        InitializeRemoteRequestProcessor(InitializeRemoteRequestProcessorCall),
        RegisterRemoteAdapter(RegisterRemoteAdapterCall),
    }
    impl ::ethers::core::abi::AbiDecode for KhalaniSetterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <InitializeRemoteRequestProcessorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitializeRemoteRequestProcessor(decoded));
            }
            if let Ok(decoded) = <RegisterRemoteAdapterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterRemoteAdapter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for KhalaniSetterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::InitializeRemoteRequestProcessor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterRemoteAdapter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for KhalaniSetterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializeRemoteRequestProcessor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterRemoteAdapter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InitializeRemoteRequestProcessorCall>
    for KhalaniSetterCalls {
        fn from(value: InitializeRemoteRequestProcessorCall) -> Self {
            Self::InitializeRemoteRequestProcessor(value)
        }
    }
    impl ::core::convert::From<RegisterRemoteAdapterCall> for KhalaniSetterCalls {
        fn from(value: RegisterRemoteAdapterCall) -> Self {
            Self::RegisterRemoteAdapter(value)
        }
    }
}
