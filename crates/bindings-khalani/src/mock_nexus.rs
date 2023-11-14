pub use mock_nexus::*;
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
pub mod mock_nexus {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("processRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("processRequest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_origin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("ProcessRequestCalled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProcessRequestCalled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_origin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_message"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
    pub static MOCKNEXUS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x01{\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xC2\xA1\xCB\x9D\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\0\x88V[a\0EV[\0[\x7F\x01\xE2\xE1c\x04X\xD1\xF4\xE8\x82\xD2\xA7\x97uY\x1C2\x98&,a\xB6\x95\x1D\x8Dx^\xF2_\x9C\xE3\xE7\x84\x84\x84\x84`@Qa\0z\x94\x93\x92\x91\x90a\x01\x08V[`@Q\x80\x91\x03\x90\xA1PPPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\0\x9EW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xC4W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\0\xD8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\0\xE7W`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\0\xF9W`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[\x84\x81R\x83` \x82\x01R```@\x82\x01R\x81``\x82\x01R\x81\x83`\x80\x83\x017`\0\x81\x83\x01`\x80\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xD7\xE0\x90\xA4\x7Ff\x07\x0EACxX\x81s\t8Z\x02D%u\0\x19Ak\rp5\x99\xB3\xCD\xC1dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MOCKNEXUS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0+W`\x005`\xE0\x1C\x80c\xC2\xA1\xCB\x9D\x14a\x000W[`\0\x80\xFD[a\0Ca\0>6`\x04a\0\x88V[a\0EV[\0[\x7F\x01\xE2\xE1c\x04X\xD1\xF4\xE8\x82\xD2\xA7\x97uY\x1C2\x98&,a\xB6\x95\x1D\x8Dx^\xF2_\x9C\xE3\xE7\x84\x84\x84\x84`@Qa\0z\x94\x93\x92\x91\x90a\x01\x08V[`@Q\x80\x91\x03\x90\xA1PPPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\0\x9EW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\xC4W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\0\xD8W`\0\x80\xFD[\x815\x81\x81\x11\x15a\0\xE7W`\0\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\0\xF9W`\0\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV[\x84\x81R\x83` \x82\x01R```@\x82\x01R\x81``\x82\x01R\x81\x83`\x80\x83\x017`\0\x81\x83\x01`\x80\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xD7\xE0\x90\xA4\x7Ff\x07\x0EACxX\x81s\t8Z\x02D%u\0\x19Ak\rp5\x99\xB3\xCD\xC1dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKNEXUS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockNexus<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockNexus<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockNexus<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockNexus<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockNexus<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockNexus)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockNexus<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKNEXUS_ABI.clone(),
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
                MOCKNEXUS_ABI.clone(),
                MOCKNEXUS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `processRequest` (0xc2a1cb9d) function
        pub fn process_request(
            &self,
            origin: ::ethers::core::types::U256,
            sender: [u8; 32],
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 161, 203, 157], (origin, sender, message))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ProcessRequestCalled` event
        pub fn process_request_called_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProcessRequestCalledFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProcessRequestCalledFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockNexus<M> {
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
    #[ethevent(
        name = "ProcessRequestCalled",
        abi = "ProcessRequestCalled(uint256,bytes32,bytes)"
    )]
    pub struct ProcessRequestCalledFilter {
        pub origin: ::ethers::core::types::U256,
        pub sender: [u8; 32],
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `processRequest` function with signature `processRequest(uint256,bytes32,bytes)` and selector `0xc2a1cb9d`
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
    #[ethcall(name = "processRequest", abi = "processRequest(uint256,bytes32,bytes)")]
    pub struct ProcessRequestCall {
        pub origin: ::ethers::core::types::U256,
        pub sender: [u8; 32],
        pub message: ::ethers::core::types::Bytes,
    }
}
