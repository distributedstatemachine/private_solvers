pub use remote_setter::*;
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
pub mod remote_setter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assetReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("khalaniReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("khalaniChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hyperlaneAdapter"),
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
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assetReserves"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("khalaniReceiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("khalaniChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("hyperlaneAdapter"),
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
    pub static REMOTESETTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x02\r\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xBE 0\x94\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01\x8AV[a\0EV[\0[a\0Ma\0\xE4V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x81\x16` \x83\x01R\x81\x83\x01\x85\x90R\x83\x16``\x82\x01R\x90Q\x7F\\|\xE4\xFA\x9F(\xA7\x98\xB9_\xFF!\xDD\x8D\x82\x95n\xFB\x7F\xDFX\ra\xAAT_\x1Ah\xCF\x95\xABk\x91\x81\x90\x03`\x80\x01\x90\xA1`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x04\x80T\x94\x86\x16\x94\x82\x16\x94\x90\x94\x17\x90\x93U`\x05\x91\x90\x91U`\x02\x80T\x91\x90\x93\x16\x91\x16\x17\x90UV[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1C`\x03\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01lW\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1FT`@\x80Q`\x01b\xBE\xD85`\xE0\x1B\x03\x19\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01RQ\x90\x81\x90\x03`D\x01\x90\xFD[V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x85W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01\xA0W`\0\x80\xFD[a\x01\xA9\x85a\x01nV[\x93Pa\x01\xB7` \x86\x01a\x01nV[\x92P`@\x85\x015\x91Pa\x01\xCC``\x86\x01a\x01nV[\x90P\x92\x95\x91\x94P\x92PV\xFE\xA2dipfsX\"\x12 JYP\x89\x83\xA8\xB1\xA2\xE7\xCE\x90\xD8_N\xC3!\xFA\x9B\x96\x06\r\xD9\xB9u7\x1A\xC6\xA0\xEA;\xB1\xD0dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static REMOTESETTER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xBE 0\x94\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\x01\x8AV[a\0EV[\0[a\0Ma\0\xE4V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x81\x16` \x83\x01R\x81\x83\x01\x85\x90R\x83\x16``\x82\x01R\x90Q\x7F\\|\xE4\xFA\x9F(\xA7\x98\xB9_\xFF!\xDD\x8D\x82\x95n\xFB\x7F\xDFX\ra\xAAT_\x1Ah\xCF\x95\xABk\x91\x81\x90\x03`\x80\x01\x90\xA1`\x03\x80T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x04\x80T\x94\x86\x16\x94\x82\x16\x94\x90\x94\x17\x90\x93U`\x05\x91\x90\x91U`\x02\x80T\x91\x90\x93\x16\x91\x16\x17\x90UV[\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1C`\x03\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01lW\x7F\xC8\xFC\xAD\x8D\xB8M<\xC1\x8BLA\xD5Q\xEA\x0E\xE6m\xD5\x99\xCD\xE0h\xD9\x98\xE5}^\t3,\x13\x1FT`@\x80Q`\x01b\xBE\xD85`\xE0\x1B\x03\x19\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01RQ\x90\x81\x90\x03`D\x01\x90\xFD[V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x85W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01\xA0W`\0\x80\xFD[a\x01\xA9\x85a\x01nV[\x93Pa\x01\xB7` \x86\x01a\x01nV[\x92P`@\x85\x015\x91Pa\x01\xCC``\x86\x01a\x01nV[\x90P\x92\x95\x91\x94P\x92PV\xFE\xA2dipfsX\"\x12 JYP\x89\x83\xA8\xB1\xA2\xE7\xCE\x90\xD8_N\xC3!\xFA\x9B\x96\x06\r\xD9\xB9u7\x1A\xC6\xA0\xEA;\xB1\xD0dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static REMOTESETTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RemoteSetter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RemoteSetter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RemoteSetter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RemoteSetter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RemoteSetter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RemoteSetter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RemoteSetter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    REMOTESETTER_ABI.clone(),
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
                REMOTESETTER_ABI.clone(),
                REMOTESETTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `initialize` (0xbe203094) function
        pub fn initialize(
            &self,
            asset_reserves: ::ethers::core::types::Address,
            khalani_receiver: ::ethers::core::types::Address,
            khalani_chain_id: ::ethers::core::types::U256,
            hyperlane_adapter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [190, 32, 48, 148],
                    (
                        asset_reserves,
                        khalani_receiver,
                        khalani_chain_id,
                        hyperlane_adapter,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RemoteSetter<M> {
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
        name = "Initialized",
        abi = "Initialized(address,address,uint256,address)"
    )]
    pub struct InitializedFilter {
        pub asset_reserves: ::ethers::core::types::Address,
        pub khalani_receiver: ::ethers::core::types::Address,
        pub khalani_chain_id: ::ethers::core::types::U256,
        pub hyperlane_adapter: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint256,address)` and selector `0xbe203094`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,uint256,address)")]
    pub struct InitializeCall {
        pub asset_reserves: ::ethers::core::types::Address,
        pub khalani_receiver: ::ethers::core::types::Address,
        pub khalani_chain_id: ::ethers::core::types::U256,
        pub hyperlane_adapter: ::ethers::core::types::Address,
    }
}
