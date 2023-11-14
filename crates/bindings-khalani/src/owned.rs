pub use owned::*;
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
pub mod owned {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nominateNewOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nominateNewOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_owner"),
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
                    ::std::borrow::ToOwned::to_owned("nominatedOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nominatedOwner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("OwnerChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OwnerChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnerNominated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OwnerNominated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
    pub static OWNED_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x03\xF08\x03\x80a\x03\xF0\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\xE6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\0\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FOwner address cannot be 0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x7F\xB52\x07;8\xC81E\xE3\xE5\x13Sw\xA0\x8B\xF9\xAA\xB5[\xC0\xFD|\x11y\xCDO\xB9\x95\xD2\xA5\x15\x9C\x91\x01`@Q\x80\x91\x03\x90\xA1Pa\x01\x16V[`\0` \x82\x84\x03\x12\x15a\0\xF8W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x0FW`\0\x80\xFD[\x93\x92PPPV[a\x02\xCB\x80a\x01%`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x16'T\x0C\x14a\0QW\x80cS\xA4{\xB7\x14a\0fW\x80cy\xBAP\x97\x14a\0\x95W\x80c\x8D\xA5\xCB[\x14a\0\x9DW[`\0\x80\xFD[a\0da\0_6`\x04a\x02eV[a\0\xB0V[\0[`\x01Ta\0y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0da\x01{V[`\0Ta\0y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FOnly the contract owner may perf`D\x82\x01Rn7\xB96\x90:44\xB9\x900\xB1\xBA4\xB7\xB7`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90j\x1Ck\xD7\xE3\t\x1E\xA8f\x93\xDD\x02\x9A\x83\x1C\x19\x04\x9C\xE7\x7F\x1D\xCE,\xE0\xBA\xB1\xCA\xCB\xAB\xCE\"\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FYou must be nominated before you`D\x82\x01Rt\x02\x066\x16\xE2\x06\x1666W\x07B\x06\xF7v\xE6W'6\x86\x97`\\\x1B`d\x82\x01R`\x84\x01a\x01\x1EV[`\0T`\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x7F\xB52\x07;8\xC81E\xE3\xE5\x13Sw\xA0\x8B\xF9\xAA\xB5[\xC0\xFD|\x11y\xCDO\xB9\x95\xD2\xA5\x15\x9C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01\x80T`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90\x91U\x16\x90UV[`\0` \x82\x84\x03\x12\x15a\x02wW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x8EW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x11\xBC%\xEC\xA1\r6\x0E4b\xDDV\x17\x13\xDC\x88\xF3\x07c\xEET\xAA59(W\xE0\xB2]\xB6N\x95dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static OWNED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x16'T\x0C\x14a\0QW\x80cS\xA4{\xB7\x14a\0fW\x80cy\xBAP\x97\x14a\0\x95W\x80c\x8D\xA5\xCB[\x14a\0\x9DW[`\0\x80\xFD[a\0da\0_6`\x04a\x02eV[a\0\xB0V[\0[`\x01Ta\0y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0da\x01{V[`\0Ta\0y\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FOnly the contract owner may perf`D\x82\x01Rn7\xB96\x90:44\xB9\x900\xB1\xBA4\xB7\xB7`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90j\x1Ck\xD7\xE3\t\x1E\xA8f\x93\xDD\x02\x9A\x83\x1C\x19\x04\x9C\xE7\x7F\x1D\xCE,\xE0\xBA\xB1\xCA\xCB\xAB\xCE\"\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FYou must be nominated before you`D\x82\x01Rt\x02\x066\x16\xE2\x06\x1666W\x07B\x06\xF7v\xE6W'6\x86\x97`\\\x1B`d\x82\x01R`\x84\x01a\x01\x1EV[`\0T`\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x7F\xB52\x07;8\xC81E\xE3\xE5\x13Sw\xA0\x8B\xF9\xAA\xB5[\xC0\xFD|\x11y\xCDO\xB9\x95\xD2\xA5\x15\x9C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x01\x80T`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90\x91U\x16\x90UV[`\0` \x82\x84\x03\x12\x15a\x02wW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02\x8EW`\0\x80\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x11\xBC%\xEC\xA1\r6\x0E4b\xDDV\x17\x13\xDC\x88\xF3\x07c\xEET\xAA59(W\xE0\xB2]\xB6N\x95dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static OWNED_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Owned<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Owned<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Owned<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Owned<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Owned<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Owned)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Owned<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    OWNED_ABI.clone(),
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
                OWNED_ABI.clone(),
                OWNED_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nominateNewOwner` (0x1627540c) function
        pub fn nominate_new_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 39, 84, 12], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nominatedOwner` (0x53a47bb7) function
        pub fn nominated_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([83, 164, 123, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnerChanged` event
        pub fn owner_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnerChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnerNominated` event
        pub fn owner_nominated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnerNominatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnedEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Owned<M> {
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
    #[ethevent(name = "OwnerChanged", abi = "OwnerChanged(address,address)")]
    pub struct OwnerChangedFilter {
        pub old_owner: ::ethers::core::types::Address,
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "OwnerNominated", abi = "OwnerNominated(address)")]
    pub struct OwnerNominatedFilter {
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum OwnedEvents {
        OwnerChangedFilter(OwnerChangedFilter),
        OwnerNominatedFilter(OwnerNominatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for OwnedEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OwnerChangedFilter::decode_log(log) {
                return Ok(OwnedEvents::OwnerChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnerNominatedFilter::decode_log(log) {
                return Ok(OwnedEvents::OwnerNominatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for OwnedEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnerChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnerNominatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OwnerChangedFilter> for OwnedEvents {
        fn from(value: OwnerChangedFilter) -> Self {
            Self::OwnerChangedFilter(value)
        }
    }
    impl ::core::convert::From<OwnerNominatedFilter> for OwnedEvents {
        fn from(value: OwnerNominatedFilter) -> Self {
            Self::OwnerNominatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
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
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `nominateNewOwner` function with signature `nominateNewOwner(address)` and selector `0x1627540c`
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
    #[ethcall(name = "nominateNewOwner", abi = "nominateNewOwner(address)")]
    pub struct NominateNewOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `nominatedOwner` function with signature `nominatedOwner()` and selector `0x53a47bb7`
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
    #[ethcall(name = "nominatedOwner", abi = "nominatedOwner()")]
    pub struct NominatedOwnerCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
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
    pub enum OwnedCalls {
        AcceptOwnership(AcceptOwnershipCall),
        NominateNewOwner(NominateNewOwnerCall),
        NominatedOwner(NominatedOwnerCall),
        Owner(OwnerCall),
    }
    impl ::ethers::core::abi::AbiDecode for OwnedCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <NominateNewOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NominateNewOwner(decoded));
            }
            if let Ok(decoded) = <NominatedOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NominatedOwner(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OwnedCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NominateNewOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NominatedOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for OwnedCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::NominateNewOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::NominatedOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for OwnedCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<NominateNewOwnerCall> for OwnedCalls {
        fn from(value: NominateNewOwnerCall) -> Self {
            Self::NominateNewOwner(value)
        }
    }
    impl ::core::convert::From<NominatedOwnerCall> for OwnedCalls {
        fn from(value: NominatedOwnerCall) -> Self {
            Self::NominatedOwner(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for OwnedCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    ///Container type for all return fields from the `nominatedOwner` function with signature `nominatedOwner()` and selector `0x53a47bb7`
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
    pub struct NominatedOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
